//! Parser: YAML (current) → AST.
//! Future: custom DSL via the lexer.

use crate::ast::Document;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn parse_yaml_str(src: &str) -> Result<Document> {
    serde_yaml::from_str(src).context("failed to parse YAML into Document")
}

pub fn parse_yaml_file(path: impl AsRef<Path>) -> Result<Document> {
    let src = fs::read_to_string(path.as_ref())
        .with_context(|| format!("reading {}", path.as_ref().display()))?;
    parse_yaml_str(&src)
}
