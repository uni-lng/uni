use std::time::SystemTime;

#[derive(Debug)]
pub struct SourceFile {
  pub path: String,
  pub modified: SystemTime,
  pub src: String,
}

impl SourceFile {
  pub fn new(path: String, src: String, modified: SystemTime) -> Self {
    SourceFile {
      path,
      modified,
      src,
    }
  }
}
