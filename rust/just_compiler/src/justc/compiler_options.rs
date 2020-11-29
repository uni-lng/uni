use clap::ArgMatches;

#[derive(Debug)]
pub struct CompilerOptions {
  pub cwd: String,
  pub files: Vec<String>,
}

impl CompilerOptions {
  pub fn from_arg_matches(matches: &ArgMatches) -> Self {
    let folder = matches.value_of("folder").unwrap();
    CompilerOptions {
      cwd: String::from(folder),
      files: vec![String::from("src/main.just")],
    }
  }
}
