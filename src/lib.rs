//! LIFT OPS · Trainer
//!
//! Content is data. Knowledge is an AST.
//! Renderers (HTML, flash, terminal, future SAGCO agent) consume the same tree.

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod enumerator;
pub mod render;

pub use ast::*;
