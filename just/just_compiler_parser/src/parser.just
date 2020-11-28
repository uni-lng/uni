
pub type SourceFile = ParsedSource & {
  symbols: Vec<Symbol>
}

pub type ParsedSource = {
  name: String,
  source: String,
  ast: Vec<ASTNode>
}

/**
 *
 **/
pub let parse = (name: &str, source: &str) -> ParsedSource => {
  let mut ast = Vec.new<ASTNode>()
  ParsedSource {
    name: name.toString(),
    source: source.toString(),
    ast
  }
}
