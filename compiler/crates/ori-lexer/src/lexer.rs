use crate::token::TokenKind;
use logos::Logos;
use ori_diagnostics::{Diagnostic, DiagnosticSink, FileId, Label, Span};

/// A single token produced by the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    /// Returns `true` if this token is trivia (comment) that most passes ignore.
    pub fn is_trivia(&self) -> bool {
        self.kind.is_trivia()
    }
}

/// Lexes an entire source file into a `Vec<Token>`, emitting diagnostics for
/// any unrecognised characters.
///
/// Trivia tokens (line comments, block comments) are **included** in the output
/// so that `ori doc` can process documentation comments.  The parser uses
/// `Token::is_trivia()` to skip them.
pub fn lex(source: &str, file_id: FileId, sink: &mut DiagnosticSink) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut lexer = TokenKind::lexer(source);

    while let Some(result) = lexer.next() {
        let raw_span = lexer.span();
        let span = Span::new(raw_span.start, raw_span.end);

        match result {
            Ok(kind) => tokens.push(Token::new(kind, span)),
            Err(()) => {
                // logos returns `Err(())` for unrecognised characters.
                let bad = &source[raw_span.clone()];
                sink.emit(
                    Diagnostic::error(
                        "lex.unexpected_character",
                        format!("unexpected character `{}`", bad.escape_default()),
                    )
                    .with_label(Label::primary(file_id, span, "not a valid token"))
                    .with_action("remove or replace this character"),
                );
            }
        }
    }

    tokens
}

#[allow(dead_code)]
/// An unclosed block comment was detected during lexing.
pub fn check_unclosed_block_comments(
    tokens: &[Token],
    source: &str,
    file_id: FileId,
    sink: &mut DiagnosticSink,
) {
    // If lex_block_comment callback returns false, logos emits Err(()).
    // An unclosed `--|` will already have been reported as an unexpected
    // character sequence above.  This function is a secondary pass to look for
    // partial `--|` without a matching `|--` — currently a no-op placeholder.
    let _ = (tokens, source, file_id, sink);
}
