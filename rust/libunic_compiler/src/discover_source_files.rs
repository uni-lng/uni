use std::fs;
use std::io;

pub fn discover_source_files(cwd: &str) -> Result<Vec<String>, io::Error> {
  let mut src = String::from(cwd);
  src.push_str("/src");

  let mut files = Vec::new();
  discover_source_files_recur(&mut files, &src);
  Ok(files)
}

fn discover_source_files_recur(files: &mut Vec<String>, dir: &str) {
  try_discover_source_files_recur(files, dir).unwrap_or(())
}

fn try_discover_source_files_recur(files: &mut Vec<String>, dir: &str) -> Result<(), io::Error> {
  for dir_result in fs::read_dir(dir)? {
    let path = dir_result?.path();
    let p = path.to_str().unwrap();
    if path.is_dir() {
      discover_source_files_recur(files, p);
    } else {
      files.push(String::from(p))
    }
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn with_single_file() {
    match discover_source_files("fixtures/binary_single_file") {
      Ok(files) => {
        assert_eq!(1, files.len());
        assert_eq!("fixtures/binary_single_file/src/main.uni", files[0])
      }
      _ => assert!(false, "failed"),
    }
  }

  #[test]
  fn with_multi_files() {
    match discover_source_files("fixtures/binary_multi_files") {
      Ok(files) => {
        assert_eq!(2, files.len());
        assert_eq!("fixtures/binary_multi_files/src/foo.uni", files[0]);
        assert_eq!("fixtures/binary_multi_files/src/main.uni", files[1]);
      }
      _ => assert!(false, "failed"),
    }
  }

  #[test]
  fn with_sub_folder() {
    match discover_source_files("fixtures/binary_sub_folder") {
      Ok(files) => {
        assert_eq!(2, files.len());
        assert_eq!("fixtures/binary_sub_folder/src/sub/mod.uni", files[0]);
        assert_eq!("fixtures/binary_sub_folder/src/main.uni", files[1]);
      }
      _ => assert!(false, "failed"),
    }
  }
}
