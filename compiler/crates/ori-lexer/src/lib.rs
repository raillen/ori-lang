mod lexer;
mod token;

pub use lexer::{lex, Token};
pub use token::TokenKind;
pub use ori_diagnostics::Span;
