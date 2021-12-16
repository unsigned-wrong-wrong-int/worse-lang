#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Value {
   raw: u64,
}

impl Value {
   const fn primitive(id: u8) -> Self {
      Self { raw: (id as u64) << 1 | 0b1 }
   }
   pub const PLUS: Self = Self::primitive(0);
   pub const MINUS: Self = Self::primitive(1);
   pub const PRED: Self = Self::primitive(2);
   pub const WRAP: Self = Self::primitive(3);
   pub const CONST: Self = Self::primitive(4);
   pub const ROTATE: Self = Self::primitive(5);
   pub const MARKER_INC: Self = Self::primitive(6);
   pub const MARKER_INIT: Self = Self::primitive(7);

   pub const fn number(n: u32) -> Self {
      Self { raw: (n as u64) << 4 | 0b0010 }
   }
   pub const ZERO: Self = Self::number(0);
   pub const ONE: Self = Self::number(1);

   const fn pair_13(x: Self, y: Self) -> Self {
      Self { raw: x.raw << 8 | y.raw << 4 | 0b0110 }
   }
   const fn pair_30(x: Self, y: Self) -> Self {
      Self { raw: x.raw << 17 | y.raw << 4 | 0b1010 }
   }
   const fn pair_64(x: Self, y: Self) -> Self {
      Self { raw: x.raw << 34 | y.raw << 4 | 0b1110 }
   }
   const PLUS_MINUS: Self = Self::pair_13(Self::PLUS, Self::MINUS);
   const MINUS_PLUS: Self = Self::pair_13(Self::MINUS, Self::PLUS);
   const MINUS_MINUS: Self = Self::pair_13(Self::MINUS, Self::MINUS);
   const CONST_CONST: Self = Self::pair_13(Self::CONST, Self::CONST);
   const CONST_ZERO: Self = Self::pair_13(Self::CONST, Self::ZERO);
   const ROTATE_CONST: Self = Self::pair_13(Self::ROTATE, Self::CONST);
   const ROTATE_ZERO: Self = Self::pair_13(Self::ROTATE, Self::ZERO);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum PairWidth {
   _13,
   _30,
   _64,
   Ptr,
}

impl Value {
   fn pair_width(self) -> PairWidth {
      match self.raw & 0b11 {
         0b00 => PairWidth::Ptr,
         0b10 => match self.raw & 0b1100 {
            0b0000 => {
               let n = (self.raw >> 4) as u32;
               if n < 0x200 {
                  PairWidth::_30
               } else if n < 0x40000000 {
                  PairWidth::_64
               } else {
                  PairWidth::Ptr
               }
            }
            0b0100 => PairWidth::_30,
            0b1000 => PairWidth::_64,
            _ => PairWidth::Ptr,
         }
         _ => PairWidth::_13,
      }
   }
}

#[derive(Debug)]
struct Pair(Value, Value, usize);

impl Value {
   fn bind(self, x: Value) -> Value {
      match std::cmp::max(self.pair_width(), x.pair_width()) {
         PairWidth::_13 => Value::pair_13(self, x),
         PairWidth::_30 => Value::pair_30(self, x),
         PairWidth::_64 => Value::pair_64(self, x),
         _ => {
            let v = Value { raw: Box::into_raw(Box::new(Pair(self, x, 0))) as u64 };
            v.inc_ref();
            v
         }
      }
   }

   fn inc_ref(self) {
      if self.raw & 0b11 == 0b00 {
         unsafe {
            let pair = &mut*(self.raw as *mut Pair);
            pair.2 += 1;
         }
      }
   }

   fn dec_ref(self) {
      if self.raw & 0b11 == 0b00 {
         unsafe {
            let pair = &mut *(self.raw as *mut Pair);
            pair.2 -= 1;
            if pair.2 == 0 {
               Self::dec_ref(pair.0);
               Self::dec_ref(pair.1);
               drop(Box::from_raw(pair as *mut _));
            }
         }
      }
   }

   fn duplicate(self) -> Self {
      self.inc_ref();
      self
   }

   fn ignore(self, x: Value) -> Self {
      x.dec_ref();
      self
   }
}

#[derive(Debug, Clone)]
pub enum Data {
   None,
   Number(u32),
   Pair(Value, Value),
}

impl Value {
   pub fn data(self) -> Data {
      match self.raw & 0b11 {
         0b00 => unsafe {
            let pair = &*(self.raw as *const Pair);
            Data::Pair(pair.0, pair.1)
         },
         0b10 => match self.raw & 0b1100 {
            0b0100 => Data::Pair(Self { raw: self.raw >> 8 },
               Self { raw: self.raw >> 4 & 0xf }),
            0b1000 => Data::Pair(Self { raw: self.raw >> 17 },
               Self { raw: self.raw >> 4 & 0x1fff }),
            0b1100 => Data::Pair(Self { raw: self.raw >> 34 },
               Self { raw: self.raw >> 4 & 0x3fffffff }),
            _ => Data::Number((self.raw >> 2) as u32),
         },
         _ => Data::None,
      }
   }

   pub fn apply(self, x: Value) -> Value {
      match self {
         Value::PLUS => if matches!(x, Value::CONST) {
            return Value::CONST_CONST
         }
         Value::PRED => if matches!(x, Value::PRED) {
            return Value::CONST_ZERO
         } else if let Data::Number(n) = x.data() {
            return Value::number(n.saturating_sub(1))
         }
         Value::ZERO => return Value::ONE.ignore(x),
         Value::ONE => return x,
         Value::MINUS_PLUS => if matches!(x, Value::PLUS_MINUS | Value::ONE) {
            return Value::ROTATE
         }
         Value::MINUS_MINUS => if x == Value::MINUS {
            return Value::ZERO
         }
         Value::ROTATE_CONST => if x == Value::ROTATE {
            return Value::ZERO
         }
         Value::ROTATE_ZERO => if x == Value::ROTATE {
            return Value::CONST
         }
         _ => match self.data() {
            Data::Number(m) => match x.data() {
               Data::Number(n) => if let Some(v) = n.checked_pow(m) {
                  return Value::number(v)
               }
               Data::Pair(u, Value::PLUS) => if let Data::Number(n) = u.data() {
                  if let Some(v) = m.checked_mul(n) {
                     return Value::number(v)
                  }
               }
               _ => {}
            }
            Data::Pair(Value::PLUS, u) => if let Data::Number(m) = u.data() {
               if let Data::Number(n) = x.data() {
                  if let Some(v) = m.checked_add(n) {
                     return Value::number(v)
                  }
               }
            }
            Data::Pair(Value::MINUS, u) => if let Data::Number(m) = u.data() {
               if let Data::Number(n) = x.data() {
                  return Value::number(m.saturating_sub(n))
               }
            }
            Data::Pair(Value::CONST, u) => return u.ignore(x),
            _ => {}
         },
      }
      self.bind(x)
   }
}

pub trait Context {
   type Result;
   fn init() -> (Vec<Value>, Self);
   fn result(self, x: Value, stack: Vec<Value>) -> Self::Result;
   fn add(&mut self, n: u32, stack: &mut Vec<Value>) -> Option<Value>;
}

#[derive(Debug)]
pub struct Pure;

impl Context for Pure {
   type Result = Value;

   fn init() -> (Vec<Value>, Self) {
      (vec![], Self)
   }

   fn result(self, mut x: Value, stack: Vec<Value>) -> Value {
      for v in stack {
         x = x.apply(v)
      }
      x
   }

   fn add(&mut self, _: u32, _: &mut Vec<Value>) -> Option<Value> {
      unreachable!()
   }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Leaf {
   True,
   Byte(u8),
   Invalid,
}

#[derive(Debug)]
pub struct Decoder(Option<u8>);

impl Context for Decoder {
   type Result = Leaf;

   fn init() -> (Vec<Value>, Self) {
      (vec![Value::MARKER_INIT, Value::MARKER_INC], Self(Some(0)))
   }

   fn result(self, x: Value, stack: Vec<Value>) -> Leaf {
      if !stack.is_empty() {
         return Leaf::Invalid
      }
      match (self.0, x) {
         (Some(0), Value::MARKER_INC) => Leaf::True,
         (Some(n), Value::MARKER_INIT) => Leaf::Byte(n),
         _ => Leaf::Invalid,
      }
   }

   fn add(&mut self, n: u32, stack: &mut Vec<Value>) -> Option<Value> {
      if stack.len() != 1 {
         return None
      }
      self.0 = match self.0 {
         Some(m) if n < 256 => m.checked_add(n as u8),
         _ => None,
      };
      stack.pop()
   }
}

impl Value {
   fn eval_plus(stack: &mut Vec<Value>) -> Option<Value> {
      if stack.len() < 4 {
         return None
      }
      let m = stack.pop().unwrap();
      let n = stack.pop().unwrap();
      if let Data::Number(i) = m.data() {
         if let Data::Number(j) = n.data() {
            return i.checked_add(j).map(Value::number)
         }
      }
      let f = stack.pop().unwrap();
      let x = stack.pop().unwrap();
      if m == Value::CONST {
         return Some(f.ignore(n).ignore(x))
      }
      stack.push(n.apply(f).apply(x));
      Some(m)
   }

   fn eval_minus(stack: &mut Vec<Value>) -> Option<Value> {
      if stack.len() < 2 {
         return None
      }
      let m = stack.pop().unwrap();
      let n = stack.pop().unwrap();
      match m {
         Value::PLUS => if matches!(n, Value::PLUS_MINUS | Value::ONE) {
            return Some(Value::ROTATE)
         }
         Value::MINUS => if n == Value::MINUS {
            return Some(Value::ZERO)
         }
         _ => if let Data::Number(i) = m.data() {
            if let Data::Number(j) = n.data() {
               return Some(Value::number(i.saturating_sub(j)))
            }
         }
      }
      stack.push(m);
      stack.push(Value::PRED);
      Some(n)
   }

   fn eval_pred(stack: &mut Vec<Value>) -> Option<Value> {
      if stack.len() < 3 {
         return None
      }
      let m = stack.pop().unwrap();
      if let Data::Number(i) = m.data() {
         return Some(Value::number(i.saturating_sub(1)))
      }
      let f = stack.pop().unwrap();
      let x = stack.pop().unwrap();
      if m == Value::PRED {
         return Some(Value::ONE.ignore(f).ignore(x))
      }
      stack.push(Value::ONE);
      stack.push(Value::CONST.bind(x));
      stack.push(Value::WRAP.bind(f));
      Some(m)
   }

   fn eval_wrap(stack: &mut Vec<Value>) -> Option<Value> {
      if stack.len() < 3 {
         return None
      }
      let f = stack.pop().unwrap();
      let g = stack.pop().unwrap();
      let h = stack.pop().unwrap();
      stack.push(g.apply(f));
      Some(h)
   }

   fn eval_const(stack: &mut Vec<Value>) -> Option<Value> {
      if stack.len() < 2 {
         return None
      }
      let x = stack.pop().unwrap();
      let y = stack.pop().unwrap();
      Some(x.ignore(y))
   }

   fn eval_rotate(stack: &mut Vec<Value>) -> Option<Value> {
      if stack.len() < 3 {
         return None
      }
      let x = stack.pop().unwrap();
      let y = stack.pop().unwrap();
      if y == Value::ROTATE {
         match x {
            Value::CONST => return Some(Value::ZERO),
            Value::ZERO => return Some(Value::CONST),
            _ => {}
         }
      }
      let z = stack.pop().unwrap();
      stack.push(x);
      stack.push(z);
      Some(y)
   }

   fn eval_zero(stack: &mut Vec<Value>) -> Option<Value> {
      if stack.len() < 2 {
         return None
      }
      let x = stack.pop().unwrap();
      let y = stack.pop().unwrap();
      Some(y.ignore(x))
   }

   fn eval_one(stack: &mut Vec<Value>) -> Option<Value> {
      stack.pop()
   }

   fn eval_number(i: u32, f: Value, stack: &mut Vec<Value>) -> Option<Value> {
      match f.data() {
         Data::Pair(n, Value::PLUS) => if let Data::Number(j) = n.data() {
            if let Some(k) = i.checked_mul(j) {
               return Some(Value::number(k))
            }
         }
         Data::Number(j) => if let Some(k) = j.checked_pow(i) {
            return Some(Value::number(k))
         }
         _ => {}
      }
      let x = stack.pop().unwrap();
      stack.push(Value::number(i - 1).apply(f).apply(x));
      Some(f.duplicate())
   }

   pub fn eval<T: Context>(mut self) -> T::Result {
      let (mut stack, mut context) = T::init();
      loop {
         let x = match self {
            Value::PLUS => Value::eval_plus(&mut stack),
            Value::MINUS => Value::eval_minus(&mut stack),
            Value::PRED => Value::eval_pred(&mut stack),
            Value::WRAP => Value::eval_wrap(&mut stack),
            Value::CONST => Value::eval_const(&mut stack),
            Value::ROTATE => Value::eval_rotate(&mut stack),
            Value::ZERO => Value::eval_zero(&mut stack),
            Value::ONE => Value::eval_one(&mut stack),
            Value::MARKER_INC => context.add(1, &mut stack),
            _ => match self.data() {
               Data::Pair(u, v) => {
                  stack.push(v);
                  Some(u)
               }
               Data::Number(i) if stack.len() >= 2 => match stack.pop().unwrap() {
                  Value::MARKER_INC => context.add(i, &mut stack),
                  v => Value::eval_number(i, v, &mut stack),
               }
               _ => None,
            }
         };
         match x {
            Some(x) => self = x,
            None => break,
         }
      }
      context.result(self, stack)
   }
}
