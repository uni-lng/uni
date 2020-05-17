use clap::ArgMatches;

#[derive(Debug)]
pub struct CompilerOptions {
  pub files: Vec<String>,
}

impl CompilerOptions {
  pub fn from_arg_matches(_matches: &ArgMatches) -> Self {
    CompilerOptions { files: Vec::new() }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn empty_args_ok() {
    let matches = ArgMatches::default();

    let co = CompilerOptions::from_arg_matches(&matches);

    assert_eq!(0, co.files.len());
  }
}
