use std::time::SystemTime;

#[derive(Debug)]
pub struct SourceFile {
  pub path: String,
  pub modified: SystemTime,
  pub source: Option<String>,
}

impl SourceFile {
  pub fn new(path: &str, modified: SystemTime) -> Self {
    SourceFile {
      path: String::from(path),
      modified,
      source: None,
    }
  }
}
