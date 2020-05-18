use crate::compiler_options::CompilerOptions;

use libunic_compile_session::CompileSession;
use libunic_fs::discover_source_files;
use std::io;

#[derive(Debug)]
pub struct Compiler<'a> {
  pub cwd: &'a str,
  pub options: CompilerOptions,
  session: CompileSession,
}

impl<'a> Compiler<'a> {
  pub fn new(cwd: &'a str, options: CompilerOptions) -> Self {
    Compiler {
      cwd,
      options,
      session: CompileSession::new(),
    }
  }

  pub fn compile(&mut self) -> Result<(), io::Error> {
    self.update_files()?;
    // self.update_ast();
    // let entryPoints = self.identify_entry_points();
    // if self.main_index.is_some() {
    //   self.compile_binary()?;
    // }
    Ok(())
  }
  /**
   * In the real unic,
   * this method will discover all source files,
   * compare and invalidate any changes (add,change,delete),
   * so that tokens for those files will be rebuilt.
   */
  fn update_files(&mut self) -> Result<(), io::Error> {
    self.session.update_files(discover_source_files(self.cwd)?);
    Ok(())
  }

  // fn identify_entry_points(&mut self) {
  //   let main = format!("{}/{}", self.cwd, "src/main.uni");
  //   let lib = format!("{}/{}", self.cwd, "src/lib.uni");

  //   let mut found = 0;
  //   for (i, file) in self.session.files.iter().enumerate() {
  //     if *file == main {
  //       self.main_index = Some(i);
  //       found += 1;
  //     }
  //     if *file == lib {
  //       self.lib_index = Some(i);
  //       found += 1;
  //     }
  //     if found == 2 {
  //       break;
  //     }
  //   }
  // }

  // fn compile_binary(&mut self) -> Result<(), io::Error> {
  //   Ok(())
  // }
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
