use crate::compiler_options::CompilerOptions;
use crate::discover_source_files::discover_source_files;

#[derive(Debug)]
pub struct Compiler<'a> {
  pub cwd: &'a str,
  pub options: CompilerOptions,
  files: Vec<String>,
  main_index: Option<usize>,
  lib_index: Option<usize>,
}

impl<'a> Compiler<'a> {
  pub fn new(cwd: &'a str, options: CompilerOptions) -> Self {
    Compiler {
      cwd,
      options,
      files: Vec::new(),
      main_index: None,
      lib_index: None,
    }
  }

  pub fn compile(&mut self) -> Result<(), std::io::Error> {
    self.files = discover_source_files(self.cwd)?;
    self.identify_entry_points();
    if self.main_index.is_some() {
      self.compile_binary();
    }
    Ok(())
  }

  fn identify_entry_points(&mut self) {
    let main = format!("{}/{}", self.cwd, "src/main.uni");
    let lib = format!("{}/{}", self.cwd, "src/lib.uni");

    let mut found = 0;
    for (i, file) in self.files.iter().enumerate() {
      if *file == main {
        self.main_index = Some(i);
        found += 1;
      }
      if *file == lib {
        self.lib_index = Some(i);
        found += 1;
      }
      if found == 2 {
        break;
      }
    }
  }

  fn compile_binary(&mut self) {
    
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn empty() {
    let mut c = Compiler::new("fixtures/binary_single_file", CompilerOptions::new());

    assert!(c.compile().is_ok());
  }
}
