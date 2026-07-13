use ori_ast::common::{Name, QualifiedName, TypeParam, WhereClause, WhereConstraint};
use ori_diagnostics::{Diagnostic, DiagnosticSink, FileId, Label, Span};
use ori_lexer::{Token, TokenKind};
use smol_str::SmolStr;

// ── Parser core ───────────────────────────────────────────────────────────────

pub(crate) struct Parser<'src> {
    pub tokens: &'src [Token],
    pub pos: usize,
    pub source: &'src str,
    pub file_id: FileId,
    pub sink: &'src mut DiagnosticSink,
}

impl<'src> Parser<'src> {
    pub fn new(
        tokens: &'src [Token],
        source: &'src str,
        file_id: FileId,
        sink: &'src mut DiagnosticSink,
    ) -> Self {
        let mut p = Self {
            tokens,
            pos: 0,
            source,
            file_id,
            sink,
        };
        p.skip_trivia();
        p
    }

    fn skip_trivia(&mut self) {
        while self.pos < self.tokens.len() && self.tokens[self.pos].is_trivia() {
            self.pos += 1;
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    pub fn peek_kind(&self) -> Option<&TokenKind> {
        self.peek().map(|t| &t.kind)
    }

    pub fn at(&self, kind: &TokenKind) -> bool {
        self.peek_kind() == Some(kind)
    }

    pub fn at_any(&self, kinds: &[TokenKind]) -> bool {
        self.peek_kind().map_or(false, |k| kinds.contains(k))
    }

    pub fn at_contextual(&self, keyword: &str) -> bool {
        self.peek_kind() == Some(&TokenKind::Ident)
            && self
                .peek()
                .is_some_and(|tok| self.slice(tok.span) == keyword)
    }

    pub fn at_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    /// Peek at the nth non-trivia token ahead (0 = current).
    pub fn peek_nth_kind(&self, n: usize) -> Option<&TokenKind> {
        let mut count = 0usize;
        let mut i = self.pos;
        while i < self.tokens.len() {
            if !self.tokens[i].is_trivia() {
                if count == n {
                    return Some(&self.tokens[i].kind);
                }
                count += 1;
            }
            i += 1;
        }
        None
    }

    pub fn advance(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let tok = self.tokens[self.pos].clone();
            self.pos += 1;
            self.skip_trivia();
            Some(tok)
        } else {
            None
        }
    }

    /// Consume the current token if it matches `kind`. Returns its span.
    pub fn expect(&mut self, kind: &TokenKind) -> Option<Span> {
        if self.peek_kind() == Some(kind) {
            Some(self.advance().unwrap().span)
        } else {
            let span = self.current_span();
            let found = self
                .peek_kind()
                .map(|k| k.display_name())
                .unwrap_or("end of file");
            self.error(
                "parse.unexpected_token",
                format!("expected {}, found {}", kind.display_name(), found),
                span,
            );
            None
        }
    }

    pub fn expect_block_end(&mut self, start: Span, block_name: &'static str) -> Option<Span> {
        if self.peek_kind() == Some(&TokenKind::End) {
            return Some(self.advance().unwrap().span);
        }

        if self.at_eof() {
            let diag = Diagnostic::error(
                "parse.unterminated_block",
                format!("{block_name} block is not closed"),
            )
            .with_label(Label::primary(self.file_id, start, "block starts here"))
            .with_why("Ori blocks must be closed with `end`")
            .with_action(format!("add `end` to close this {block_name} block"));
            self.sink.emit(diag);
            return None;
        }

        self.expect(&TokenKind::End)
    }

    /// Consume and return `true` if the current token matches `kind`.
    pub fn eat(&mut self, kind: &TokenKind) -> bool {
        if self.peek_kind() == Some(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn eat_contextual(&mut self, keyword: &str) -> bool {
        if self.at_contextual(keyword) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn current_span(&self) -> Span {
        self.peek().map(|t| t.span).unwrap_or_else(|| {
            self.tokens
                .last()
                .map(|t| Span::new(t.span.end as usize, t.span.end as usize))
                .unwrap_or(Span::DUMMY)
        })
    }

    pub fn slice(&self, span: Span) -> &str {
        &self.source[span.as_range()]
    }

    pub fn error(&mut self, code: &'static str, msg: impl Into<String>, span: Span) {
        let diag =
            Diagnostic::error(code, msg).with_label(Label::primary(self.file_id, span, "here"));
        self.sink.emit(diag);
    }

    // ── Common grammar helpers ────────────────────────────────────────────────

    pub fn parse_name(&mut self) -> Option<Name> {
        match self.peek_kind() {
            Some(TokenKind::Ident) => {
                let tok = self.advance().unwrap();
                let text = SmolStr::new(self.slice(tok.span));
                Some(Name::new(text, tok.span))
            }
            _ => {
                let span = self.current_span();
                self.error("parse.expected_identifier", "expected identifier", span);
                None
            }
        }
    }

    pub fn parse_member_name(&mut self) -> Option<Name> {
        match self.peek_kind() {
            Some(TokenKind::Ident | TokenKind::Or) => {
                let tok = self.advance().unwrap();
                let text = SmolStr::new(self.slice(tok.span));
                Some(Name::new(text, tok.span))
            }
            _ => {
                let span = self.current_span();
                self.error("parse.expected_identifier", "expected identifier", span);
                None
            }
        }
    }

    fn parse_qualified_name_part(&mut self) -> Option<Name> {
        if !is_qualified_name_part(self.peek_kind()) {
            let span = self.current_span();
            self.error("parse.expected_identifier", "expected identifier", span);
            return None;
        }
        let tok = self.advance().unwrap();
        let text = SmolStr::new(self.slice(tok.span));
        Some(Name::new(text, tok.span))
    }

    /// Parse `ident (.ident)*` in a type or import context.
    pub fn parse_qualified_name(&mut self) -> Option<QualifiedName> {
        let first = self.parse_qualified_name_part()?;
        let mut span = first.span;
        let mut parts = vec![first];
        // Continue only if Dot is followed by Ident (not a field access)
        while self.at(&TokenKind::Dot) && is_qualified_name_part(self.peek_nth_kind(1)) {
            self.advance(); // dot
            let name = self.parse_qualified_name_part()?;
            span = span.cover(name.span);
            parts.push(name);
        }
        Some(QualifiedName { parts, span })
    }

    /// Parse optional `<T, U, …>` type parameters.
    pub fn parse_type_params_opt(&mut self) -> Vec<TypeParam> {
        if !self.at(&TokenKind::Lt) {
            return Vec::new();
        }
        // Is this `<ident` (type param) or `<expr` (comparison)?
        // In declaration context, always parse as type params.
        self.advance(); // <
        let mut params = Vec::new();
        loop {
            if self.at_eof() || self.at(&TokenKind::Gt) {
                break;
            }
            let is_const = self.eat(&TokenKind::Const);
            if let Some(name) = self.parse_name() {
                if is_const && self.eat(&TokenKind::Colon) {
                    let _ = self.parse_type();
                }
                params.push(TypeParam { name });
                if self.at(&TokenKind::Lt) {
                    self.skip_angle_group();
                }
            } else {
                break;
            }
            if !self.eat(&TokenKind::Comma) {
                break;
            }
        }
        self.expect(&TokenKind::Gt);
        params
    }

    fn skip_angle_group(&mut self) {
        if !self.at(&TokenKind::Lt) {
            return;
        }
        let mut depth = 0usize;
        while !self.at_eof() {
            if self.at(&TokenKind::Lt) {
                depth += 1;
                self.advance();
                continue;
            }
            if self.at(&TokenKind::Gt) {
                self.advance();
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    break;
                }
                continue;
            }
            self.advance();
        }
    }

    /// Parse optional `where T is Trait, U is not Trait` clause.
    pub fn parse_where_clause_opt(&mut self) -> Option<WhereClause> {
        if !self.at(&TokenKind::Where) {
            return None;
        }
        let start = self.advance().unwrap().span; // `where`
        let mut constraints = Vec::new();
        let grouped = self.eat(&TokenKind::LParen);
        loop {
            if grouped && self.at(&TokenKind::RParen) {
                break;
            }
            let param = self.parse_name()?;
            let negated = if self.eat(&TokenKind::Is) {
                let neg = self.eat(&TokenKind::Not);
                neg
            } else {
                self.expect(&TokenKind::Is)?;
                false
            };
            let bound = self.parse_qualified_name()?;
            let span = param.span.cover(bound.span);
            if negated {
                constraints.push(WhereConstraint::IsNot { param, bound, span });
            } else {
                constraints.push(WhereConstraint::Is { param, bound, span });
            }
            if self.eat(&TokenKind::Comma) || self.eat(&TokenKind::And) {
                if grouped && self.at(&TokenKind::RParen) {
                    break;
                }
                continue;
            } else {
                break;
            }
        }
        if grouped {
            self.expect(&TokenKind::RParen)?;
        }
        let end = constraints
            .last()
            .map(|c| match c {
                WhereConstraint::Is { span, .. } => *span,
                WhereConstraint::IsNot { span, .. } => *span,
            })
            .unwrap_or(start);
        Some(WhereClause {
            constraints,
            span: start.cover(end),
        })
    }

    /// Advance past all tokens that are NOT in the sync set (error recovery).
    pub fn synchronize(&mut self, sync: &[TokenKind]) {
        while !self.at_eof() && !self.at_any(sync) {
            self.advance();
        }
    }
}

fn is_qualified_name_part(kind: Option<&TokenKind>) -> bool {
    matches!(
        kind,
        Some(TokenKind::Ident)
            // Allow `module` as a path segment (`pkg.module.name`) even though it is
            // a keyword for the file header.
            | Some(TokenKind::Module)
            | Some(TokenKind::Any)
            | Some(TokenKind::Optional)
            | Some(TokenKind::ResultKw)
            | Some(TokenKind::List)
            | Some(TokenKind::Map)
            | Some(TokenKind::Set)
            | Some(TokenKind::Range)
            | Some(TokenKind::Repeat)
            | Some(TokenKind::Void)
            | Some(TokenKind::Tuple)
            | Some(TokenKind::Lazy)
            | Some(TokenKind::BoolTy)
            | Some(TokenKind::IntTy)
            | Some(TokenKind::Int8Ty)
            | Some(TokenKind::Int16Ty)
            | Some(TokenKind::Int32Ty)
            | Some(TokenKind::Int64Ty)
            | Some(TokenKind::U8Ty)
            | Some(TokenKind::U16Ty)
            | Some(TokenKind::U32Ty)
            | Some(TokenKind::U64Ty)
            | Some(TokenKind::FloatTy)
            | Some(TokenKind::Float32Ty)
            | Some(TokenKind::Float64Ty)
            | Some(TokenKind::StringTy)
            | Some(TokenKind::BytesTy)
    )
}
