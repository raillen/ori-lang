use crate::parser::Parser;
use ori_ast::ty::Type;
use ori_lexer::TokenKind;

macro_rules! primitive_type {
    ($self:expr, $span:expr, $variant:ident) => {{
        $self.advance();
        Some(Type::$variant($span))
    }};
}

macro_rules! single_generic_type {
    ($self:expr, $span:expr, $variant:ident) => {{
        $self.advance();
        $self.expect(&TokenKind::Lt)?;
        let inner = $self.parse_type()?;
        let end = $self.expect(&TokenKind::Gt)?;
        Some(Type::$variant(Box::new(inner), $span.cover(end)))
    }};
}

impl<'src> Parser<'src> {
    pub fn parse_type(&mut self) -> Option<Type> {
        let span = self.current_span();
        match self.peek_kind()? {
            // ── Primitive types ───────────────────────────────────────────────
            TokenKind::BoolTy => primitive_type!(self, span, Bool),
            TokenKind::IntTy => primitive_type!(self, span, Int),
            TokenKind::Int8Ty => primitive_type!(self, span, Int8),
            TokenKind::Int16Ty => primitive_type!(self, span, Int16),
            TokenKind::Int32Ty => primitive_type!(self, span, Int32),
            TokenKind::Int64Ty => primitive_type!(self, span, Int64),
            TokenKind::U8Ty => primitive_type!(self, span, U8),
            TokenKind::U16Ty => primitive_type!(self, span, U16),
            TokenKind::U32Ty => primitive_type!(self, span, U32),
            TokenKind::U64Ty => primitive_type!(self, span, U64),
            TokenKind::FloatTy => primitive_type!(self, span, Float),
            TokenKind::Float32Ty => primitive_type!(self, span, Float32),
            TokenKind::Float64Ty => primitive_type!(self, span, Float64),
            TokenKind::StringTy => primitive_type!(self, span, String),
            TokenKind::BytesTy => primitive_type!(self, span, Bytes),
            TokenKind::Void => primitive_type!(self, span, Void),

            // ── Single-arg generic built-in types ─────────────────────────────
            TokenKind::Optional => single_generic_type!(self, span, Optional),
            TokenKind::List => single_generic_type!(self, span, List),
            TokenKind::Set => single_generic_type!(self, span, Set),
            TokenKind::Range => single_generic_type!(self, span, Range),
            TokenKind::Lazy => single_generic_type!(self, span, Lazy),
            TokenKind::Handle => single_generic_type!(self, span, Handle),

            // ── Multi-arg generic built-in types ──────────────────────────────
            TokenKind::ResultKw => {
                self.advance();
                self.expect(&TokenKind::Lt)?;
                let ok = self.parse_type()?;
                self.expect(&TokenKind::Comma)?;
                let err = self.parse_type()?;
                let end = self.expect(&TokenKind::Gt)?;
                Some(Type::Result(Box::new(ok), Box::new(err), span.cover(end)))
            }
            TokenKind::Map => {
                self.advance();
                self.expect(&TokenKind::Lt)?;
                let key = self.parse_type()?;
                self.expect(&TokenKind::Comma)?;
                let val = self.parse_type()?;
                let end = self.expect(&TokenKind::Gt)?;
                Some(Type::Map(Box::new(key), Box::new(val), span.cover(end)))
            }
            TokenKind::Any => {
                self.advance();
                self.expect(&TokenKind::Lt)?;
                let trait_name = self.parse_qualified_name()?;
                let end = self.expect(&TokenKind::Gt)?;
                Some(Type::Any(trait_name, span.cover(end)))
            }
            TokenKind::Tuple => {
                self.advance();
                self.expect(&TokenKind::Lt)?;
                let args = self.parse_type_list(&TokenKind::Gt)?;
                let end = self.expect(&TokenKind::Gt)?;
                Some(Type::Tuple(args, span.cover(end)))
            }

            // ── Callable type: `func(T, U) -> R` ─────────────────────────────
            TokenKind::Func => {
                self.advance();
                self.expect(&TokenKind::LParen)?;
                let params = if self.at(&TokenKind::RParen) {
                    Vec::new()
                } else {
                    self.parse_type_list(&TokenKind::RParen)?
                };
                let end_paren = self.expect(&TokenKind::RParen)?;
                let return_ty = if self.eat(&TokenKind::Arrow) {
                    let ty = self.parse_type()?;
                    Some(Box::new(ty))
                } else {
                    None
                };
                let end = return_ty.as_ref().map(|t| t.span()).unwrap_or(end_paren);
                Some(Type::Func {
                    params,
                    return_ty,
                    span: span.cover(end),
                })
            }

            // ── User-defined type: `Name` or `Name<T, U>` ────────────────────
            TokenKind::Ident => {
                let name = self.parse_qualified_name()?;
                if self.at(&TokenKind::Lt) && self.peek_nth_kind(1) != Some(&TokenKind::Eq) {
                    self.advance(); // <
                    let args = self.parse_type_list(&TokenKind::Gt)?;
                    let end = self.expect(&TokenKind::Gt)?;
                    Some(Type::Generic {
                        name,
                        args,
                        span: span.cover(end),
                    })
                } else {
                    Some(Type::Named(name))
                }
            }

            TokenKind::IntLit | TokenKind::True | TokenKind::False => {
                let tok = self.advance().unwrap();
                let text = smol_str::SmolStr::new(self.slice(tok.span));
                let name = ori_ast::common::Name::new(text, tok.span);
                Some(Type::Named(ori_ast::common::QualifiedName {
                    parts: vec![name],
                    span: tok.span,
                }))
            }

            _ => {
                let span = self.current_span();
                self.error("parse.expected_type", "expected a type", span);
                None
            }
        }
    }

    /// Parse a comma-separated list of types, stopping before `stop`.
    pub fn parse_type_list(&mut self, stop: &TokenKind) -> Option<Vec<Type>> {
        let mut types = Vec::new();
        while !self.at(stop) && !self.at_eof() {
            types.push(self.parse_type()?);
            if !self.eat(&TokenKind::Comma) {
                break;
            }
        }
        Some(types)
    }
}
