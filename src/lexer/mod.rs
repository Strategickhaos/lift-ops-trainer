//! Lexer for a future training DSL.
//! Right now content is YAML; this module is the extension point
//! when we move to a custom FlameLang / BRICKS-style surface syntax.

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    StringLit(String),
    Number(f64),
    Colon,
    Dash,
    LBracket,
    RBracket,
    Newline,
    Eof,
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    /// Placeholder — returns a single Eof for now.
    /// Real implementation will come when we define the DSL.
    pub fn next_token(&mut self) -> Token {
        Token::Eof
    }
}
