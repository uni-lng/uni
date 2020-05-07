pub struct CompileOptions {
  /**
   * output file name
   */
  pub out: String
}

pub fn compile(_source_code: &str, _options: CompileOptions) {
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
