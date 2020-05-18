use crate::source_file::SourceFile;

#[derive(Debug)]
pub struct CompileSession {
  pub files: Vec<SourceFile>,
}

impl CompileSession {
  pub fn new() -> Self {
    CompileSession { files: Vec::new() }
  }

  pub fn update_files(&mut self, files: Vec<SourceFile>) {
    self.files = files;
  }
}
