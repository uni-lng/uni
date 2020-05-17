pub struct Program {
  name: String,
  // config: Config,
  compiler_options: CompilerOptions,
  source_files: Vec<SourceFile>
}

/**
 * The "Compilation Context" refers to all files involved in a "program".
 * The context is created by inspecting all files passed in to the compiler.
 **/
pub struct CompilationContext {

}

pub struct SourceFile {
  name: String,
  source_text: String,
  nodes: Vec<Node>,
  symbols: Vec<Symbol>
}
pub fn parse(source: &str) {

}
