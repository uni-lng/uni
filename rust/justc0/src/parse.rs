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

//! Parsing Just code to AST?.
//!
//! This parser learn from Rust lexer and parser as well as TypeScript parser.
//!
//! The design is closer to TypeScript than Rust,
//! because the responsivness of language service is King.
//!
//! The design must able to do incremental parse and update of the AST efficiently.
//! This mean the Rust pre-lexing approach will not work.
//!
//! The general idea is parsing the code and updating AST will be done in a single pass,
//! while syntax validation and reference verification will be done in a separate worker.
//!
//! Multiple update can be performed before the syntax checker is invoked,
//! and it can be stopped and restart when another AST update have taken place.
//!
//! Things to check in the future:
//! - TypeScript tsbuildinfo
pub fn parse(source: &str) {

}
