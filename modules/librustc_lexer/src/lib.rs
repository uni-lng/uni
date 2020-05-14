//! Low-level Uni lexer.
//!
//! This is very similar to the Rust lexer and a lot of code are borrowed from it.
//! However, there are some differences thus the code is copied and modified for lexing Uni code.
//!
mod cursor;
pub mod errors;
mod literal_kind;
mod token;
mod token_kind;
pub mod tokenize;
pub mod unescape;
