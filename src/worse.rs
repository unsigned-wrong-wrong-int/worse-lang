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
            // application operator
            b'.' => {
               let y = stack.pop().ok_or_else(syntax_error)?;
               let x = stack.pop().ok_or_else(syntax_error)?;
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
            // whitespaces
            b' ' | b'\t' | b'\n' | b'\r' => {}
            _ => return Err(syntax_error()),
         }
      }
      match *stack {
         [x] => Ok(Self(x)),
         _ => Err(syntax_error()),
      }
   }

   pub fn wrap(self, input: impl Read) -> impl Read {
      Runtime {
         input: input.bytes(),
         code: self.0,
      }
   }
}

#[derive(Debug)]
struct Runtime<In: Read> {
   input: std::io::Bytes<In>,
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
            // exit
            Some(256) => break Ok(0),
            // read
            Some(257) => {
               let n = match self.input.next() {
                  Some(b) => b? as u32,
                  None => 256, // EOF
               };
               list = list.apply(Value::ZERO).apply(Value::number(n));
            }
            // write
            Some(n) if n < 256 => {
               buf[0] = n as u8;
               self.code = list.apply(Value::ZERO);
               break Ok(1)
            }
            _ => break Err(Error::new(ErrorKind::Other, "failed to decode value")),
         }
      }
   }
}
