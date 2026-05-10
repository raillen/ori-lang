use ori_lexer::TokenKind;
use ori_ast::pattern::{NamedPattern, Pattern};
use ori_ast::common::Name;
use crate::parser::Parser;

impl<'src> Parser<'src> {
    pub fn parse_pattern(&mut self) -> Option<Pattern> {
        let span = self.current_span();
        match self.peek_kind()? {
            // `_` wildcard
            TokenKind::Ident if self.slice(span) == "_" => {
                self.advance();
                Some(Pattern::Wildcard(span))
            }

            // `none`
            TokenKind::None => {
                self.advance();
                Some(Pattern::None(span))
            }

            // `some(pat)`
            TokenKind::Some => {
                self.advance();
                self.expect(&TokenKind::LParen)?;
                let inner = self.parse_pattern()?;
                let end = self.expect(&TokenKind::RParen)?;
                Some(Pattern::Some(Box::new(inner), span.cover(end)))
            }

            // `success(pat)`
            TokenKind::Success => {
                self.advance();
                self.expect(&TokenKind::LParen)?;
                let inner = self.parse_pattern()?;
                let end = self.expect(&TokenKind::RParen)?;
                Some(Pattern::Success(Box::new(inner), span.cover(end)))
            }

            // `error(pat)`
            TokenKind::ErrorKw => {
                self.advance();
                self.expect(&TokenKind::LParen)?;
                let inner = self.parse_pattern()?;
                let end = self.expect(&TokenKind::RParen)?;
                Some(Pattern::Error(Box::new(inner), span.cover(end)))
            }

            // `tuple(pat, pat, ...)`
            TokenKind::Tuple => {
                self.advance();
                self.expect(&TokenKind::LParen)?;
                let mut pats = Vec::new();
                while !self.at(&TokenKind::RParen) && !self.at_eof() {
                    pats.push(self.parse_pattern()?);
                    if !self.eat(&TokenKind::Comma) { break; }
                }
                let end = self.expect(&TokenKind::RParen)?;
                Some(Pattern::Tuple(pats, span.cover(end)))
            }

            // `.Variant` — shorthand enum variant
            TokenKind::Dot => {
                self.advance();
                let name = self.parse_name()?;
                self.parse_variant_pattern(name, true)
            }

            // Literal: `true`, `false`, integer, float, string
            TokenKind::True | TokenKind::False
            | TokenKind::IntLit | TokenKind::FloatLit
            | TokenKind::StrLit => {
                let expr = self.parse_primary_expr()?;
                Some(Pattern::Literal(Box::new(expr)))
            }
            TokenKind::Minus => {
                // Negative number literal in pattern
                let expr = self.parse_primary_expr()?;
                Some(Pattern::Literal(Box::new(expr)))
            }

            // Identifier: binding or `Variant` / `Variant(fields)`
            TokenKind::Ident => {
                let name = self.parse_name()?;
                // If followed by `(`, it's a named variant pattern
                if self.at(&TokenKind::LParen) {
                    self.parse_variant_pattern(name, false)
                } else {
                    // Plain binding
                    Some(Pattern::Binding(name))
                }
            }

            _ => {
                let span = self.current_span();
                self.error("parse.expected_pattern", "expected a pattern", span);
                None
            }
        }
    }

    fn parse_variant_pattern(&mut self, name: Name, shorthand: bool) -> Option<Pattern> {
        let start = name.span;
        if self.at(&TokenKind::LParen) {
            self.advance(); // (
            let mut fields = Vec::new();
            while !self.at(&TokenKind::RParen) && !self.at_eof() {
                let field_name = self.parse_name()?;
                let pattern = if self.eat(&TokenKind::Colon) {
                    self.parse_pattern()?
                } else {
                    // Shorthand: `field` means `field: field`
                    Pattern::Binding(field_name.clone())
                };
                let span = field_name.span.cover(pattern.span());
                fields.push(NamedPattern { name: field_name, pattern, span });
                if !self.eat(&TokenKind::Comma) { break; }
            }
            let end = self.expect(&TokenKind::RParen)?;
            Some(Pattern::VariantNamed { name, fields, shorthand, span: start.cover(end) })
        } else {
            Some(Pattern::VariantUnit { name, shorthand, span: start })
        }
    }
}
