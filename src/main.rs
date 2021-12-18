use std::env;
use std::io;
use std::fs::File;

mod worse;

use worse::Program;

const VERSION: &'static str = "0.1.0";
const HELP: &'static str = r"USAGE:
   worse [OPTIONS] FILE
OPTIONS:
   -v, --version  Prints version information
   -h, --help     Displays this message";

fn main() -> io::Result<()> {
   let mut args = env::args();
   let mut file = None;
   let mut version = false;
   let mut help = false;
   // skip path of the executable
   let _ = args.next();
   while let Some(arg) = args.next() {
      match arg.as_str() {
         "-v" | "--version" => version = true,
         "-h" | "--help" => help = true,
         s if s.starts_with('-') => {
            println!("unknown option: {}", s);
            help = true;
            file = None;
            break
         }
         s => if file.is_some() {
            println!("too many arguments");
            help = true;
            file = None;
            break
         } else {
            file = Some(File::open(s));
         }
      }
   }
   if version {
      println!("worse interpreter {}", VERSION);
   }
   if help || file.is_none() && !version {
      println!("{}", HELP);
   }
   if let Some(file) = file {
      let program = Program::load(file?)?;
      let _ = io::copy(&mut program.wrap(io::stdin().lock()), &mut io::stdout().lock())?;
   }
   Ok(())
}
