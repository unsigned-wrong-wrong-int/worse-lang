mod core;

use self::core::*;

#[derive(Debug)]
pub struct Program(Value);

impl Program {
   pub fn parse(src: impl IntoIterator<Item = u8>) -> Option<Self> {
      let mut stack = vec![];
      for c in src {
         match c {
            b'+' => stack.push(Value::PLUS),
            b'-' => stack.push(Value::MINUS),
            b'.' => {
               let x = stack.pop()?;
               let y = stack.pop()?;
               stack.push(x.apply(y));
            }
            _ => {}
         }
      }
      match *stack {
         [x] => Some(Self(x)),
         _ => None,
      }
   }

   pub fn wrap(self, input: impl Read) -> impl Read {
      Runtime {
         input,
         code: self.0,
      }
   }
}

use std::io::{Read, Result, Error, ErrorKind,};

#[derive(Debug)]
struct Runtime<In: Read> {
   input: In,
   code: Value,
}

impl<In: Read> Read for Runtime<In> {
   fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
      if buf.len() == 0 {
         return Ok(0)
      }
      let mut list = self.code;
      loop {
         list = list.eval::<Pure>();
         match list.apply(Value::CONST).eval::<Decoder>() {
            Leaf::True => {
               list = list.apply(Value::ZERO).eval::<Pure>();
               match list.apply(Value::CONST).eval::<Decoder>() {
                  Leaf::True => return Ok(0),
                  Leaf::Byte(0) => {
                     let mut buf = [0u8];
                     let x = loop {
                        match self.input.read(&mut buf) {
                           Ok(0) => break Value::CONST,
                           Ok(_) => break Value::number(buf[0] as u32),
                           Err(e) if e.kind() == ErrorKind::Interrupted => {}
                           Err(e) => return Err(e),
                        }
                     };
                     list = list.apply(Value::ZERO).apply(x);
                  }
                  _ => break
               }
            }
            Leaf::Byte(b) => {
               buf[0] = b;
               self.code = list.apply(Value::ZERO);
               return Ok(1)
            }
            _ => break
         }
      }
      Err(Error::new(ErrorKind::InvalidData, "failed to decode value"))
   }
}
