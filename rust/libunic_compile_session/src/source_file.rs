use std::time::SystemTime;

#[derive(Debug)]
pub struct SourceFile {
  pub path: String,
  pub modified: SystemTime,
}
