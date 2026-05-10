use smol_str::SmolStr;
use ori_lexer::TokenKind;
use ori_ast::expr::{Arg, ArgValue, BinaryOp, ClosureBody, ClosureExpr, ClosureParam,
                    Expr, FieldInit, FStrPart, IndexExpr, UnaryOp};
use ori_ast::stmt::Block;
use crate::parser::Parser;

// ── Pratt precedence ──────────────────────────────────────────────────────────

fn infix_prec(kind: &TokenKind) -> Option<(u8, u8)> {
    // Returns (left_prec, right_prec). right > left → right-associative.
    match kind {
        TokenKind::Pipe    => Some((1, 2)),  // |>  left-assoc
        TokenKind::Or      => Some((3, 4)),  // or
        TokenKind::And     => Some((5, 6)),  // and
        TokenKind::EqEq | TokenKind::BangEq
        | TokenKind::Lt  | TokenKind::LtEq
        | TokenKind::Gt  | TokenKind::GtEq  => Some((7, 8)),  // comparisons
        TokenKind::Plus  | TokenKind::Minus  => Some((9, 10)),
        TokenKind::Star  | TokenKind::Slash
        | TokenKind::Percent               => Some((11, 12)),
        _ => None,
    }
}

fn token_to_binop(kind: &TokenKind) -> Option<BinaryOp> {
    match kind {
        TokenKind::Plus    => Some(BinaryOp::Add),
        TokenKind::Minus   => Some(BinaryOp::Sub),
        TokenKind::Star    => Some(BinaryOp::Mul),
        TokenKind::Slash   => Some(BinaryOp::Div),
        TokenKind::Percent => Some(BinaryOp::Rem),
        TokenKind::EqEq    => Some(BinaryOp::Eq),
        TokenKind::BangEq  => Some(BinaryOp::Ne),
        TokenKind::Lt      => Some(BinaryOp::Lt),
        TokenKind::LtEq    => Some(BinaryOp::Le),
        TokenKind::Gt      => Some(BinaryOp::Gt),
        TokenKind::GtEq    => Some(BinaryOp::Ge),
        TokenKind::And     => Some(BinaryOp::And),
        TokenKind::Or      => Some(BinaryOp::Or),
        _ => None,
    }
}

// ── Public entry ──────────────────────────────────────────────────────────────

impl<'src> Parser<'src> {
    /// Parse an expression (top-level precedence).
    pub fn parse_expr(&mut self) -> Option<Expr> {
        self.parse_expr_prec(0)
    }

    fn parse_expr_prec(&mut self, min_prec: u8) -> Option<Expr> {
        let mut lhs = self.parse_unary()?;

        loop {
            // `is` type check — treated as a postfix binary at prec 7
            if self.at(&TokenKind::Is) && min_prec <= 7 {
                self.advance();
                let ty = self.parse_qualified_name()?;
                let span = lhs.span().cover(ty.span);
                lhs = Expr::IsCheck { value: Box::new(lhs), ty, span };
                continue;
            }

            // `|>` pipe — special: rhs is the function, result is Call
            if self.at(&TokenKind::Pipe) && min_prec <= 1 {
                self.advance();
                let func = self.parse_expr_prec(2)?;
                let span = lhs.span().cover(func.span());
                lhs = Expr::Pipe { value: Box::new(lhs), func: Box::new(func), span };
                continue;
            }

            // Standard binary operators
            if let Some(&(left_prec, right_prec)) =
                self.peek_kind().and_then(infix_prec).as_ref()
            {
                if left_prec < min_prec { break; }
                let op_tok = self.advance().unwrap();
                let op = token_to_binop(&op_tok.kind).unwrap();
                let rhs = self.parse_expr_prec(right_prec)?;
                let span = lhs.span().cover(rhs.span());
                lhs = Expr::Binary { op, lhs: Box::new(lhs), rhs: Box::new(rhs), span };
                continue;
            }

            break;
        }
        Some(lhs)
    }

    fn parse_unary(&mut self) -> Option<Expr> {
        let span = self.current_span();
        if self.eat(&TokenKind::Minus) {
            let operand = self.parse_unary()?;
            let s = span.cover(operand.span());
            return Some(Expr::Unary { op: UnaryOp::Neg, operand: Box::new(operand), span: s });
        }
        if self.eat(&TokenKind::Not) {
            let operand = self.parse_unary()?;
            let s = span.cover(operand.span());
            return Some(Expr::Unary { op: UnaryOp::Not, operand: Box::new(operand), span: s });
        }
        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Option<Expr> {
        let mut expr = self.parse_primary_expr()?;
        loop {
            let span_start = expr.span();
            match self.peek_kind() {
                // `.field` or `.integer` (tuple index)
                Some(TokenKind::Dot) => {
                    self.advance();
                    match self.peek_kind() {
                        Some(TokenKind::IntLit) => {
                            let tok = self.advance().unwrap();
                            let idx: u32 = self.slice(tok.span).parse().unwrap_or(0);
                            let span = span_start.cover(tok.span);
                            expr = Expr::TupleIndex { object: Box::new(expr), index: idx, span };
                        }
                        _ => {
                            let field = self.parse_name()?;
                            let span = span_start.cover(field.span);
                            expr = Expr::Field { object: Box::new(expr), field, span };
                        }
                    }
                }
                // `(args)` — call
                Some(TokenKind::LParen) => {
                    let args = self.parse_call_args()?;
                    let end_span = self.peek()
                        .map(|t| t.span)
                        .unwrap_or(span_start);
                    let span = span_start.cover(end_span);
                    expr = Expr::Call { callee: Box::new(expr), args, span };
                }
                // `[index]`
                Some(TokenKind::LBracket) => {
                    self.advance();
                    let index = self.parse_index_expr()?;
                    let end = self.expect(&TokenKind::RBracket)?;
                    let span = span_start.cover(end);
                    expr = Expr::Index { object: Box::new(expr), index, span };
                }
                // `?` propagation
                Some(TokenKind::Question) => {
                    let end = self.advance().unwrap().span;
                    let span = span_start.cover(end);
                    expr = Expr::Try { expr: Box::new(expr), span };
                }
                _ => break,
            }
        }
        Some(expr)
    }

    pub fn parse_primary_expr(&mut self) -> Option<Expr> {
        let span = self.current_span();
        match self.peek_kind()? {
            // Literals
            TokenKind::True  => { self.advance(); Some(Expr::BoolLit(true, span)) }
            TokenKind::False => { self.advance(); Some(Expr::BoolLit(false, span)) }
            TokenKind::None  => { self.advance(); Some(Expr::None(span)) }

            TokenKind::IntLit => {
                let tok = self.advance().unwrap();
                Some(Expr::IntLit { raw: SmolStr::new(self.slice(tok.span)), span })
            }
            TokenKind::FloatLit => {
                let tok = self.advance().unwrap();
                Some(Expr::FloatLit { raw: SmolStr::new(self.slice(tok.span)), span })
            }
            TokenKind::StrLit => {
                let tok = self.advance().unwrap();
                let raw = self.slice(tok.span);
                // Strip surrounding quotes; unescape sequences are left to a later pass
                let value = SmolStr::new(&raw[1..raw.len()-1]);
                Some(Expr::StrLit { value, span })
            }
            TokenKind::FStrLit => {
                let tok = self.advance().unwrap();
                // Store raw content; interpolation parsing deferred to a later pass
                let raw = self.slice(tok.span);
                let content = SmolStr::new(&raw[2..raw.len()-1]); // strip f" and "
                Some(Expr::FStrLit {
                    parts: vec![FStrPart::Literal(content)],
                    span,
                })
            }
            TokenKind::BytesLit => {
                let tok = self.advance().unwrap();
                let raw = self.slice(tok.span);
                let content = raw[2..raw.len()-1].as_bytes().to_vec();
                Some(Expr::BytesLit { bytes: content, span })
            }

            // `self`
            TokenKind::SelfKw => { self.advance(); Some(Expr::SelfExpr(span)) }

            // Grouped expression or tuple `(a, b)` or `(a)`
            TokenKind::LParen => {
                self.advance();
                if self.at(&TokenKind::RParen) {
                    // `()` — empty tuple
                    let end = self.advance().unwrap().span;
                    return Some(Expr::Tuple { elements: Vec::new(), span: span.cover(end) });
                }
                let first = self.parse_expr()?;
                if self.eat(&TokenKind::Comma) {
                    let mut elements = vec![first];
                    while !self.at(&TokenKind::RParen) && !self.at_eof() {
                        elements.push(self.parse_expr()?);
                        if !self.eat(&TokenKind::Comma) { break; }
                    }
                    let end = self.expect(&TokenKind::RParen)?;
                    Some(Expr::Tuple { elements, span: span.cover(end) })
                } else {
                    self.expect(&TokenKind::RParen)?;
                    Some(first) // just a grouped expr
                }
            }

            // List literal `[a, b, c]`
            TokenKind::LBracket => {
                self.advance();
                let mut elements = Vec::new();
                while !self.at(&TokenKind::RBracket) && !self.at_eof() {
                    elements.push(self.parse_expr()?);
                    if !self.eat(&TokenKind::Comma) { break; }
                }
                let end = self.expect(&TokenKind::RBracket)?;
                Some(Expr::List { elements, span: span.cover(end) })
            }

            // `do(params) -> T => expr` or `do(params) … end`
            TokenKind::Do => {
                self.advance();
                self.parse_closure_expr(span)
            }

            // `.Variant` — shorthand enum variant
            TokenKind::Dot => {
                self.advance();
                let variant = self.parse_name()?;
                if self.at(&TokenKind::LParen) {
                    let fields = self.parse_field_inits()?;
                    let end = self.peek().map(|t| t.span).unwrap_or(variant.span);
                    Some(Expr::EnumVariantNamed { ty: None, variant, fields, span: span.cover(end) })
                } else {
                    Some(Expr::EnumVariantUnit { ty: None, variant, span })
                }
            }

            // `if cond then a else b` — inline if expression
            TokenKind::If => {
                self.advance();
                let condition = self.parse_expr()?;
                self.expect(&TokenKind::Then)?;
                let then_expr = self.parse_expr()?;
                self.expect(&TokenKind::Else)?;
                let else_expr = self.parse_expr()?;
                let end = else_expr.span();
                Some(Expr::IfExpr {
                    condition: Box::new(condition),
                    then_expr: Box::new(then_expr),
                    else_expr: Box::new(else_expr),
                    span: span.cover(end),
                })
            }

            // Identifier or qualified name — also handles `Name(fields)` struct/enum lit
            TokenKind::Ident => {
                let name = self.parse_qualified_name()?;
                Some(Expr::QualifiedIdent(name))
            }

            _ => {
                let span = self.current_span();
                self.error("parse.expected_expression", "expected an expression", span);
                None
            }
        }
    }

    // ── Helpers ───────────────────────────────────────────────────────────────

    fn parse_call_args(&mut self) -> Option<Vec<Arg>> {
        self.expect(&TokenKind::LParen)?;
        let mut args = Vec::new();
        while !self.at(&TokenKind::RParen) && !self.at_eof() {
            let arg_span = self.current_span();
            // Named arg: `name: expr`
            if self.at(&TokenKind::Ident) && self.peek_nth_kind(1) == Some(&TokenKind::Colon) {
                let label = self.parse_name()?;
                self.expect(&TokenKind::Colon)?;
                let value = self.parse_expr()?;
                let span = arg_span.cover(value.span());
                args.push(Arg { label: Some(label), value: ArgValue::Expr(Box::new(value)), span });
            } else {
                let value = self.parse_expr()?;
                let span = arg_span.cover(value.span());
                args.push(Arg { label: None, value: ArgValue::Expr(Box::new(value)), span });
            }
            if !self.eat(&TokenKind::Comma) { break; }
        }
        self.expect(&TokenKind::RParen)?;
        Some(args)
    }

    fn parse_index_expr(&mut self) -> Option<IndexExpr> {
        // `a..b`, `a..`, `..b`, `..` — range index
        if self.at(&TokenKind::DotDot) {
            self.advance();
            let end = if !self.at(&TokenKind::RBracket) { Some(Box::new(self.parse_expr()?)) } else { None };
            return Some(IndexExpr::Range { start: None, end });
        }
        let expr = self.parse_expr()?;
        if self.eat(&TokenKind::DotDot) {
            let end = if !self.at(&TokenKind::RBracket) { Some(Box::new(self.parse_expr()?)) } else { None };
            Some(IndexExpr::Range { start: Some(Box::new(expr)), end })
        } else {
            Some(IndexExpr::Single(Box::new(expr)))
        }
    }

    pub fn parse_field_inits(&mut self) -> Option<Vec<FieldInit>> {
        self.expect(&TokenKind::LParen)?;
        let mut fields = Vec::new();
        while !self.at(&TokenKind::RParen) && !self.at_eof() {
            let name = self.parse_name()?;
            self.expect(&TokenKind::Colon)?;
            let value = self.parse_expr()?;
            let span = name.span.cover(value.span());
            fields.push(FieldInit { name, value: Box::new(value), span });
            if !self.eat(&TokenKind::Comma) { break; }
        }
        self.expect(&TokenKind::RParen)?;
        Some(fields)
    }

    fn parse_closure_expr(&mut self, start: ori_diagnostics::Span) -> Option<Expr> {
        self.expect(&TokenKind::LParen)?;
        let mut params = Vec::new();
        while !self.at(&TokenKind::RParen) && !self.at_eof() {
            let pspan = self.current_span();
            let name = self.parse_name()?;
            self.expect(&TokenKind::Colon)?;
            let ty = self.parse_type()?;
            params.push(ClosureParam { name, ty, span: pspan.cover(self.current_span()) });
            if !self.eat(&TokenKind::Comma) { break; }
        }
        self.expect(&TokenKind::RParen)?;
        let return_ty = if self.eat(&TokenKind::Arrow) { Some(self.parse_type()?) } else { None };
        // `=> expr` for single-expression closures
        let (body, end) = if self.eat(&TokenKind::FatArrow) {
            let e = self.parse_expr()?;
            let s = e.span();
            (ClosureBody::Expr(Box::new(e)), s)
        } else {
            let block = self.parse_block()?;
            let s = block.span;
            self.expect(&TokenKind::End)?;
            (ClosureBody::Block(block), s)
        };
        Some(Expr::Closure(Box::new(ClosureExpr {
            params, return_ty, body, span: start.cover(end),
        })))
    }
}
