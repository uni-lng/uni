mod compiler_options;

use clap::clap_app;
use compiler_options::CompilerOptions;

pub fn main() {
  let matches = clap_app!(myapp =>
    (version: "0.1")
    (author: "Homa wong <homawong@gmail.com>")
    (about: "Just stage 0 compiler")
    (@arg folder: +required "path to the justc0 project folder")
  )
  .get_matches();

  let options = CompilerOptions::from_arg_matches(&matches);
  println!("matches: {:?}", options);
}
