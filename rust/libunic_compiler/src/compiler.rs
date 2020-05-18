use crate::compiler_options::CompilerOptions;
use crate::discover_source_files::discover_source_files;

#[derive(Debug)]
pub struct Compiler<'a> {
  pub cwd: &'a str,
  pub options: CompilerOptions,
  files: Vec<String>,
  main: Option<&'a str>,
  lib: Option<&'a str>,
}

impl<'a> Compiler<'a> {
  pub fn new(cwd: &'a str, options: CompilerOptions) -> Self {
    Compiler {
      cwd,
      options,
      files: Vec::new(),
      main: None,
      lib: None,
    }
  }
  pub fn compile(&mut self) -> Result<(), std::io::Error> {
    self.files.push(format!("{}/{}", self.cwd, "src/main.uni"));
    self.files = discover_source_files(self.cwd)?;
    self.identify_entry_points();
    Ok(())
  }
  fn identify_entry_points(&mut self) {
    let main = format!("{}/{}", self.cwd, "src/main.uni");
    let lib = format!("{}/{}", self.cwd, "src/lib.uni");
    for file in &self.files {
      if *file == main {
        println!("has main");
        // self.main = Some(&file[..]);
      }
      if *file == lib {
        println!("has lib");
      }
    }
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
