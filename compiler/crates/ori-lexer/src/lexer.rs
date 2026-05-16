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
    let initial_offset = if source.starts_with('\u{feff}') {
        '\u{feff}'.len_utf8()
    } else {
        0
    };
    let lex_end = match find_unclosed_block_comment(source, initial_offset) {
        Some(span) => {
            sink.emit(
                Diagnostic::error("lex.unclosed_block_comment", "block comment is not closed")
                    .with_label(Label::primary(file_id, span, "comment starts here"))
                    .with_action("close the block comment with `|--`"),
            );
            span.start as usize
        }
        None => source.len(),
    };
    let mut lexer = TokenKind::lexer(&source[initial_offset..lex_end]);

    while let Some(result) = lexer.next() {
        let raw_span = lexer.span();
        let span = Span::new(
            raw_span.start + initial_offset,
            raw_span.end + initial_offset,
        );

        match result {
            Ok(kind) => tokens.push(Token::new(kind, span)),
            Err(()) => {
                // logos returns `Err(())` for unrecognised characters.
                let bad = &source[span.start as usize..span.end as usize];
                let (message, action) = if bad == "\u{feff}" {
                    (
                        "byte order mark is only allowed at the start of a file".to_string(),
                        "remove this byte order mark",
                    )
                } else {
                    (
                        format!("unexpected character `{}`", bad.escape_default()),
                        "remove or replace this character",
                    )
                };
                sink.emit(
                    Diagnostic::error("lex.unexpected_character", message)
                        .with_label(Label::primary(file_id, span, "not a valid token"))
                        .with_action(action),
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

fn find_unclosed_block_comment(source: &str, start_at: usize) -> Option<Span> {
    let bytes = source.as_bytes();
    let mut cursor = start_at;
    while cursor < bytes.len() {
        if bytes[cursor..].starts_with(b"--|") {
            let start = cursor;
            let body_start = start + 3;
            match source[body_start..].find("|--") {
                Some(relative_end) => {
                    cursor = body_start + relative_end + 3;
                }
                None => return Some(Span::new(start, source.len())),
            }
            continue;
        }

        if bytes[cursor..].starts_with(b"--") {
            cursor = skip_line_comment(bytes, cursor + 2);
            continue;
        }

        if bytes[cursor..].starts_with(br#"f"""#) {
            cursor = skip_triple_quoted(source, cursor + 4);
            continue;
        }
        if bytes[cursor..].starts_with(br#"""""#) {
            cursor = skip_triple_quoted(source, cursor + 3);
            continue;
        }
        if bytes[cursor..].starts_with(br#"f""#) || bytes[cursor..].starts_with(br#"b""#) {
            cursor = skip_quoted(bytes, cursor + 2);
            continue;
        }
        if bytes[cursor] == b'"' {
            cursor = skip_quoted(bytes, cursor + 1);
            continue;
        }

        cursor += 1;
    }
    None
}

fn skip_line_comment(bytes: &[u8], mut cursor: usize) -> usize {
    while cursor < bytes.len() && bytes[cursor] != b'\n' {
        cursor += 1;
    }
    cursor
}

fn skip_quoted(bytes: &[u8], mut cursor: usize) -> usize {
    while cursor < bytes.len() {
        match bytes[cursor] {
            b'\\' => cursor = (cursor + 2).min(bytes.len()),
            b'"' => return cursor + 1,
            _ => cursor += 1,
        }
    }
    cursor
}

fn skip_triple_quoted(source: &str, cursor: usize) -> usize {
    match source[cursor..].find("\"\"\"") {
        Some(relative_end) => cursor + relative_end + 3,
        None => source.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ori_diagnostics::FileId;

    fn lex_for_test(source: &str) -> (Vec<Token>, Vec<ori_diagnostics::Diagnostic>) {
        let mut sink = DiagnosticSink::default();
        let tokens = lex(source, FileId(0), &mut sink);
        (tokens, sink.into_diagnostics())
    }

    #[test]
    fn ignores_utf8_bom_at_file_start_and_preserves_spans() {
        let (tokens, diagnostics) = lex_for_test("\u{feff}namespace app.main\n");

        assert!(diagnostics.is_empty(), "{diagnostics:?}");
        assert_eq!(
            tokens.first().map(|token| &token.kind),
            Some(&TokenKind::Namespace)
        );
        assert_eq!(tokens.first().map(|token| token.span.start), Some(3));
    }

    #[test]
    fn reports_utf8_bom_outside_file_start() {
        let (_tokens, diagnostics) = lex_for_test("namespace app.main\n\u{feff}\n");

        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "lex.unexpected_character"),
            "{diagnostics:?}"
        );
    }

    #[test]
    fn reports_unclosed_block_comment_without_generic_lex_error() {
        let (_tokens, diagnostics) = lex_for_test("namespace app.main\n--|\nmissing close\n");

        let codes: Vec<_> = diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code)
            .collect();
        assert_eq!(codes, vec!["lex.unclosed_block_comment"]);
    }

    #[test]
    fn accepts_valid_block_comment() {
        let (tokens, diagnostics) = lex_for_test("namespace app.main\n--|\nclosed\n|--\n");

        assert!(diagnostics.is_empty(), "{diagnostics:?}");
        assert!(tokens
            .iter()
            .any(|token| token.kind == TokenKind::BlockComment));
    }

    #[test]
    fn ignores_block_comment_openers_inside_string_literals() {
        let source = concat!(
            "namespace app.main\n",
            "const plain: string = \"--| text\"\n",
            "const bytes: bytes = b\"--| bytes\"\n",
            "const fstr: string = f\"--| {1}\"\n",
            "const triple: string = \"\"\"--| text\"\"\"\n",
            "const triple_fstr: string = f\"\"\"--| {1}\"\"\"\n",
        );

        let (tokens, diagnostics) = lex_for_test(source);

        assert!(diagnostics.is_empty(), "{diagnostics:?}");
        assert!(tokens.iter().any(|token| token.kind == TokenKind::StrLit));
        assert!(tokens.iter().any(|token| token.kind == TokenKind::BytesLit));
        assert!(tokens.iter().any(|token| token.kind == TokenKind::FStrLit));
        assert!(tokens
            .iter()
            .any(|token| token.kind == TokenKind::TripleStrLit));
        assert!(tokens
            .iter()
            .any(|token| token.kind == TokenKind::TripleFStrLit));
    }
}
