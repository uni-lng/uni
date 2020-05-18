#[derive(Debug)]
pub struct CompilerOptions {
  // at the beginning we don't even need this
  // pub files: Vec<String>,
}

impl CompilerOptions {
  pub fn new() -> Self {
    CompilerOptions {
      // files: Vec::new()
    }
  }
}
