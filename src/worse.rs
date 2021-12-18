mod core;

use self::core::*;

#[derive(Debug)]
pub struct Program(Value);

use std::io::{Read, Result, Error, ErrorKind,};

fn syntax_error() -> Error {
   Error::new(ErrorKind::Other, "syntax error")
}

impl Program {
   pub fn load(src: impl Read) -> Result<Self> {
      let mut stack = vec![];
      let mut src = src.bytes();
   'parse:
      while let Some(b) = src.next() {
         match b? {
            // built-ins
            b'+' => stack.push(Value::PLUS),
            b'-' => stack.push(Value::MINUS),
            d @ b'0'..=b'9' => stack.push(Value::number((d - b'0') as u32)),
            a @ (b'A'..=b'Z' | b'a'..=b'z') => stack.push(Value::number(a as u32)),
            // application operator
            b'.' => {
               let x = stack.pop().ok_or_else(syntax_error)?;
               let y = stack.pop().ok_or_else(syntax_error)?;
               stack.push(x.apply(y));
            }
            // comment
            b'#' => loop {
               match src.next() {
                  Some(Ok(b'\n')) => break,
                  Some(Err(e)) => return Err(e),
                  None => break 'parse,
                  _ => {}
               }
            }
            _ => {}
         }
      }
      match *stack {
         [x] => Ok(Self(x)),
         _ => Err(syntax_error()),
      }
   }

   pub fn wrap(self, input: impl Read) -> impl Read {
      Runtime {
         input,
         code: self.0,
      }
   }
}

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
      Err(Error::new(ErrorKind::Other, "failed to decode value"))
   }
}
