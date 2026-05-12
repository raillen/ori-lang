use crate::parser::Parser;
use ori_ast::ty::Type;
use ori_lexer::TokenKind;

impl<'src> Parser<'src> {
    /// Parse any type expression.
    pub fn parse_type(&mut self) -> Option<Type> {
        let span = self.current_span();
        match self.peek_kind()? {
            // ── Primitive types ───────────────────────────────────────────────
            TokenKind::BoolTy => {
                self.advance();
                Some(Type::Bool(span))
            }
            TokenKind::IntTy => {
                self.advance();
                Some(Type::Int(span))
            }
            TokenKind::Int8Ty => {
                self.advance();
                Some(Type::Int8(span))
            }
            TokenKind::Int16Ty => {
                self.advance();
                Some(Type::Int16(span))
            }
            TokenKind::Int32Ty => {
                self.advance();
                Some(Type::Int32(span))
            }
            TokenKind::Int64Ty => {
                self.advance();
                Some(Type::Int64(span))
            }
            TokenKind::U8Ty => {
                self.advance();
                Some(Type::U8(span))
            }
            TokenKind::U16Ty => {
                self.advance();
                Some(Type::U16(span))
            }
            TokenKind::U32Ty => {
                self.advance();
                Some(Type::U32(span))
            }
            TokenKind::U64Ty => {
                self.advance();
                Some(Type::U64(span))
            }
            TokenKind::FloatTy => {
                self.advance();
                Some(Type::Float(span))
            }
            TokenKind::Float32Ty => {
                self.advance();
                Some(Type::Float32(span))
            }
            TokenKind::Float64Ty => {
                self.advance();
                Some(Type::Float64(span))
            }
            TokenKind::StringTy => {
                self.advance();
                Some(Type::String(span))
            }
            TokenKind::BytesTy => {
                self.advance();
                Some(Type::Bytes(span))
            }
            TokenKind::Void => {
                self.advance();
                Some(Type::Void(span))
            }

            // ── Generic built-in types ────────────────────────────────────────
            TokenKind::Optional => {
                self.advance();
                self.expect(&TokenKind::Lt)?;
                let inner = self.parse_type()?;
                let end = self.expect(&TokenKind::Gt)?;
                Some(Type::Optional(Box::new(inner), span.cover(end)))
            }
            TokenKind::ResultKw => {
                self.advance();
                self.expect(&TokenKind::Lt)?;
                let ok = self.parse_type()?;
                self.expect(&TokenKind::Comma)?;
                let err = self.parse_type()?;
                let end = self.expect(&TokenKind::Gt)?;
                Some(Type::Result(Box::new(ok), Box::new(err), span.cover(end)))
            }
            TokenKind::List => {
                self.advance();
                self.expect(&TokenKind::Lt)?;
                let elem = self.parse_type()?;
                let end = self.expect(&TokenKind::Gt)?;
                Some(Type::List(Box::new(elem), span.cover(end)))
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
            TokenKind::Set => {
                self.advance();
                self.expect(&TokenKind::Lt)?;
                let elem = self.parse_type()?;
                let end = self.expect(&TokenKind::Gt)?;
                Some(Type::Set(Box::new(elem), span.cover(end)))
            }
            TokenKind::Range => {
                self.advance();
                self.expect(&TokenKind::Lt)?;
                let elem = self.parse_type()?;
                let end = self.expect(&TokenKind::Gt)?;
                Some(Type::Range(Box::new(elem), span.cover(end)))
            }
            TokenKind::Lazy => {
                self.advance();
                self.expect(&TokenKind::Lt)?;
                let inner = self.parse_type()?;
                let end = self.expect(&TokenKind::Gt)?;
                Some(Type::Lazy(Box::new(inner), span.cover(end)))
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
                // Check for generic args `<T, U>`
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
