use libunic_compile_session::SourceFile;
use std::fs;
use std::io;

pub fn discover_source_files(cwd: &str) -> Result<Vec<SourceFile>, io::Error> {
  let mut src = String::from(cwd);
  src.push_str("/src");

  let mut files = Vec::new();
  discover_source_files_recur(&mut files, &src).unwrap_or(());
  Ok(files)
}

fn discover_source_files_recur(files: &mut Vec<SourceFile>, dir: &str) -> Result<(), io::Error> {
  for dir_result in fs::read_dir(dir)? {
    let dir = dir_result?;
    let modified = dir.metadata()?.modified()?;
    let path = dir.path();
    let p = path.to_str().unwrap();
    if path.is_dir() {
      discover_source_files_recur(files, p)?;
    } else {
      files.push(SourceFile::new(p, modified));
    }
  }

  Ok(())
}

#[cfg(test)]

mod tests {
  use super::*;
  use path_slash::PathExt;
  use std::path::Path;

  #[test]
  fn with_single_file() {
    match discover_source_files("fixtures/binary_single_file") {
      Ok(files) => {
        assert_eq!(1, files.len());
        assert_eq!(
          "fixtures/binary_single_file/src/main.uni",
          Path::new(&files[0].path).to_slash().unwrap()
        );
      }
      _ => assert!(false, "failed"),
    }
  }

  #[test]
  fn with_multi_files() {
    match discover_source_files("fixtures/binary_multi_files") {
      Ok(files) => {
        assert_eq!(2, files.len());
        assert_eq!(
          "fixtures/binary_multi_files/src/foo.uni",
          Path::new(&files[0].path).to_slash().unwrap()
        );
        assert_eq!(
          "fixtures/binary_multi_files/src/main.uni",
          Path::new(&files[1].path).to_slash().unwrap()
        );
      }
      _ => assert!(false, "failed"),
    }
  }

  #[test]
  fn with_sub_folder() {
    match discover_source_files("fixtures/binary_sub_folder") {
      Ok(files) => {
        assert_eq!(2, files.len());
        let sorted: Vec<String> = files.iter().map(|x| x.path.to_string()).collect();
        assert_eq!(
          "fixtures/binary_sub_folder/src/main.uni",
          Path::new(&sorted[0]).to_slash().unwrap()
        );
        assert_eq!(
          "fixtures/binary_sub_folder/src/sub/mod.uni",
          Path::new(&sorted[1]).to_slash().unwrap()
        );
      }
      _ => assert!(false, "failed"),
    }
  }
}
