use crate::source_file::SourceFile;

#[derive(Debug)]
pub struct CompileSession {
  pub files: Vec<SourceFile>,
}

impl CompileSession {
  pub fn new() -> Self {
    CompileSession { files: Vec::new() }
  }

  // pub fn add_file()
  // pub fn update_file()
  // pub fn remove_file()
  pub fn update_files(&mut self, files: Vec<SourceFile>) {
    self.files = files;
  }

  /**
   * Build AST for all files.
   * This can be break off to be handled by multiple threads.
   */
  pub fn build_asts(&mut self) {
    // check each file to see if it is already parsed.
    // parse each file to AST.
    // capture any invalid token for error reporting
    // error reporting can use the AST to do in-depth analysis
    // to provide useful suggestions
    // type analysis, merging and error reporting is done in next phrase,
    // not here
    // for file in &self.files {

    // }
  }
}
