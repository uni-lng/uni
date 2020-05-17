use assert_cmd::prelude::*;
// use predicates::prelude::*;
use std::process::Command;

#[test]
fn no_args_print_help() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("unic0")?;
  cmd.assert();

  Ok(())
}
