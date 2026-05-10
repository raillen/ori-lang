use smol_str::SmolStr;
use ori_diagnostics::Span;
use ori_lexer::TokenKind;
use ori_ast::stmt::{
    AssignStmt, Block, CheckStmt, CompoundAssignStmt, CompoundOp, ForStmt, IfSomeStmt,
    IfStmt, LocalConst, LocalVar, LoopStmt, LValue, MatchCase, MatchStmt,
    RepeatStmt, ReturnStmt, Stmt, UsingStmt, WhileSomeStmt, WhileStmt,
};
use ori_ast::expr::Expr;
use crate::parser::Parser;

/// Tokens that terminate a block (without consuming them).
const BLOCK_TERMINATORS: &[TokenKind] = &[
    TokenKind::End, TokenKind::Else, TokenKind::Case,
];

impl<'src> Parser<'src> {
    /// Parse a block: zero or more statements, stopping at a terminator token.
    pub fn parse_block(&mut self) -> Option<Block> {
        let start = self.current_span();
        let mut stmts = Vec::new();
        while !self.at_any(BLOCK_TERMINATORS) && !self.at_eof() {
            match self.parse_stmt() {
                Some(s) => stmts.push(s),
                None    => {
                    // Error recovery: skip to next statement boundary
                    self.synchronize(BLOCK_TERMINATORS);
                    break;
                }
            }
        }
        let end = stmts.last().map(stmt_span).unwrap_or(start);
        Some(Block { stmts, span: start.cover(end) })
    }

    pub fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.peek_kind()? {
            TokenKind::Const   => self.parse_local_const(),
            TokenKind::Var     => self.parse_local_var(),
            TokenKind::Return  => self.parse_return(),
            TokenKind::Break   => { let s = self.advance().unwrap().span; Some(Stmt::Break(s)) }
            TokenKind::Continue => { let s = self.advance().unwrap().span; Some(Stmt::Continue(s)) }
            TokenKind::If      => self.parse_if_stmt(),
            TokenKind::While   => self.parse_while_stmt(),
            TokenKind::For     => self.parse_for_stmt(),
            TokenKind::Repeat  => self.parse_repeat_stmt(),
            TokenKind::Loop    => self.parse_loop_stmt(),
            TokenKind::Match   => self.parse_match_stmt(),
            TokenKind::Using   => self.parse_using_stmt(),
            TokenKind::Check   => self.parse_check_stmt(),
            _ => {
                // Expression statement or assignment
                let expr = self.parse_expr()?;
                // Check for assignment operators
                if let Some(op) = self.peek_compound_assign_op() {
                    self.advance();
                    let value = self.parse_expr()?;
                    let span = expr_lvalue_span(&expr).cover(value.span());
                    let lvalue = expr_to_lvalue(expr)?;
                    return Some(Stmt::CompoundAssign(CompoundAssignStmt { lvalue, op, value: Box::new(value), span }));
                }
                if self.at(&TokenKind::Eq) {
                    self.advance();
                    let value = self.parse_expr()?;
                    let span = expr_lvalue_span(&expr).cover(value.span());
                    let lvalue = expr_to_lvalue(expr)?;
                    return Some(Stmt::Assign(AssignStmt { lvalue, value: Box::new(value), span }));
                }
                Some(Stmt::Expr(Box::new(expr)))
            }
        }
    }

    fn parse_local_const(&mut self) -> Option<Stmt> {
        let start = self.advance().unwrap().span; // const
        let name = self.parse_name()?;
        self.expect(&TokenKind::Colon)?;
        let ty = self.parse_type()?;
        self.expect(&TokenKind::Eq)?;
        let value = self.parse_expr()?;
        let span = start.cover(value.span());
        Some(Stmt::Const(LocalConst { name, ty, value: Box::new(value), span }))
    }

    fn parse_local_var(&mut self) -> Option<Stmt> {
        let start = self.advance().unwrap().span; // var
        let name = self.parse_name()?;
        self.expect(&TokenKind::Colon)?;
        let ty = self.parse_type()?;
        self.expect(&TokenKind::Eq)?;
        let value = self.parse_expr()?;
        let span = start.cover(value.span());
        Some(Stmt::Var(LocalVar { name, ty, value: Box::new(value), span }))
    }

    fn parse_return(&mut self) -> Option<Stmt> {
        let start = self.advance().unwrap().span; // return
        let value = if self.at_any(BLOCK_TERMINATORS) || self.at_eof() {
            None
        } else {
            Some(Box::new(self.parse_expr()?))
        };
        let span = value.as_ref().map(|v| start.cover(v.span())).unwrap_or(start);
        Some(Stmt::Return(ReturnStmt { value, span }))
    }

    fn parse_if_stmt(&mut self) -> Option<Stmt> {
        let start = self.advance().unwrap().span; // if

        // `if some(binding) = expr` — optional binding form
        if self.at(&TokenKind::Some) && self.peek_nth_kind(1) == Some(&TokenKind::LParen) {
            self.advance(); // some
            self.expect(&TokenKind::LParen)?;
            let binding = self.parse_name()?;
            self.expect(&TokenKind::RParen)?;
            self.expect(&TokenKind::Eq)?;
            let value = self.parse_expr()?;
            let then_block = self.parse_block()?;
            let else_block = if self.eat(&TokenKind::Else) {
                let b = self.parse_block()?;
                Some(b)
            } else {
                None
            };
            let end = self.expect(&TokenKind::End)?;
            return Some(Stmt::IfSome(IfSomeStmt {
                binding, value: Box::new(value), then_block, else_block,
                span: start.cover(end),
            }));
        }

        let condition = self.parse_expr()?;
        let then_block = self.parse_block()?;
        let mut else_ifs = Vec::new();
        let mut else_block = None;
        loop {
            if self.at(&TokenKind::Else) {
                self.advance();
                if self.at(&TokenKind::If) {
                    self.advance(); // else if
                    let cond = self.parse_expr()?;
                    let block = self.parse_block()?;
                    else_ifs.push((Box::new(cond), block));
                } else {
                    else_block = Some(self.parse_block()?);
                    break;
                }
            } else {
                break;
            }
        }
        let end = self.expect(&TokenKind::End)?;
        Some(Stmt::If(IfStmt { condition: Box::new(condition), then_block, else_ifs, else_block, span: start.cover(end) }))
    }

    fn parse_while_stmt(&mut self) -> Option<Stmt> {
        let start = self.advance().unwrap().span; // while
        // `while some(binding) = expr`
        if self.at(&TokenKind::Some) && self.peek_nth_kind(1) == Some(&TokenKind::LParen) {
            self.advance(); // some
            self.expect(&TokenKind::LParen)?;
            let binding = self.parse_name()?;
            self.expect(&TokenKind::RParen)?;
            self.expect(&TokenKind::Eq)?;
            let value = self.parse_expr()?;
            let body = self.parse_block()?;
            let end = self.expect(&TokenKind::End)?;
            return Some(Stmt::WhileSome(WhileSomeStmt {
                binding, value: Box::new(value), body, span: start.cover(end),
            }));
        }
        let condition = self.parse_expr()?;
        let body = self.parse_block()?;
        let end = self.expect(&TokenKind::End)?;
        Some(Stmt::While(WhileStmt { condition: Box::new(condition), body, span: start.cover(end) }))
    }

    fn parse_for_stmt(&mut self) -> Option<Stmt> {
        let start = self.advance().unwrap().span; // for
        let binding = self.parse_name()?;
        let second_binding = if self.eat(&TokenKind::Comma) { Some(self.parse_name()?) } else { None };
        self.expect(&TokenKind::In)?;
        let iterable = self.parse_expr()?;
        let body = self.parse_block()?;
        let end = self.expect(&TokenKind::End)?;
        Some(Stmt::For(ForStmt { binding, second_binding, iterable: Box::new(iterable), body, span: start.cover(end) }))
    }

    fn parse_repeat_stmt(&mut self) -> Option<Stmt> {
        let start = self.advance().unwrap().span; // repeat
        let count = self.parse_expr()?;
        // Optional contextual `times` keyword
        if self.at(&TokenKind::Ident) {
            let tok = self.peek().unwrap();
            if self.slice(tok.span) == "times" { self.advance(); }
        }
        // Also handle reserved `times` keyword
        self.eat(&TokenKind::Times);
        let body = self.parse_block()?;
        let end = self.expect(&TokenKind::End)?;
        Some(Stmt::Repeat(RepeatStmt { count: Box::new(count), body, span: start.cover(end) }))
    }

    fn parse_loop_stmt(&mut self) -> Option<Stmt> {
        let start = self.advance().unwrap().span; // loop
        let body = self.parse_block()?;
        let end = self.expect(&TokenKind::End)?;
        Some(Stmt::Loop(LoopStmt { body, span: start.cover(end) }))
    }

    fn parse_match_stmt(&mut self) -> Option<Stmt> {
        let start = self.advance().unwrap().span; // match
        let scrutinee = self.parse_expr()?;
        let mut cases = Vec::new();
        while self.at(&TokenKind::Case) {
            let case_span = self.advance().unwrap().span; // case
            if self.at(&TokenKind::Else) {
                self.advance(); // else
                self.expect(&TokenKind::Colon)?;
                let body = self.parse_case_body()?;
                let end = body.last().map(stmt_span).unwrap_or(case_span);
                cases.push(MatchCase::Else { body, span: case_span.cover(end) });
                break; // else must be last
            }
            let pattern = self.parse_pattern()?;
            let guard = if self.eat(&TokenKind::If) { Some(Box::new(self.parse_expr()?)) } else { None };
            self.expect(&TokenKind::Colon)?;
            let body = self.parse_case_body()?;
            let end = body.last().map(stmt_span).unwrap_or(case_span);
            cases.push(MatchCase::Pattern { pattern, guard, body, span: case_span.cover(end) });
        }
        let end = self.expect(&TokenKind::End)?;
        Some(Stmt::Match(MatchStmt { scrutinee: Box::new(scrutinee), cases, span: start.cover(end) }))
    }

    /// Statements belonging to one case arm (stop at `case` or `end`).
    fn parse_case_body(&mut self) -> Option<Vec<Stmt>> {
        let mut stmts = Vec::new();
        let stop = &[TokenKind::Case, TokenKind::End];
        while !self.at_any(stop) && !self.at_eof() {
            match self.parse_stmt() {
                Some(s) => stmts.push(s),
                None    => { self.synchronize(stop); break; }
            }
        }
        Some(stmts)
    }

    fn parse_using_stmt(&mut self) -> Option<Stmt> {
        let start = self.advance().unwrap().span; // using
        let name = self.parse_name()?;
        self.expect(&TokenKind::Colon)?;
        let ty = self.parse_type()?;
        self.expect(&TokenKind::Eq)?;
        let value = self.parse_expr()?;
        let span = start.cover(value.span());
        Some(Stmt::Using(UsingStmt { name, ty, value: Box::new(value), span }))
    }

    fn parse_check_stmt(&mut self) -> Option<Stmt> {
        let start = self.advance().unwrap().span; // check
        let condition = self.parse_expr()?;
        let message = if self.eat(&TokenKind::Comma) {
            match self.peek_kind() {
                Some(TokenKind::StrLit) => {
                    let tok = self.advance().unwrap();
                    let raw = self.slice(tok.span);
                    Some(SmolStr::new(&raw[1..raw.len()-1]))
                }
                _ => None,
            }
        } else {
            None
        };
        let span = start.cover(condition.span());
        Some(Stmt::Check(CheckStmt { condition: Box::new(condition), message, span }))
    }

    // ── Assignment helpers ────────────────────────────────────────────────────

    fn peek_compound_assign_op(&self) -> Option<CompoundOp> {
        match self.peek_kind() {
            Some(TokenKind::PlusEq)  => Some(CompoundOp::Add),
            Some(TokenKind::MinusEq) => Some(CompoundOp::Sub),
            Some(TokenKind::StarEq)  => Some(CompoundOp::Mul),
            Some(TokenKind::SlashEq) => Some(CompoundOp::Div),
            _ => None,
        }
    }
}

fn expr_to_lvalue(expr: Expr) -> Option<LValue> {
    match expr {
        Expr::Ident(n) => Some(LValue::Ident(n)),
        Expr::QualifiedIdent(_) => None, // multi-segment paths are not lvalues
        Expr::Field { object, field, span } => {
            let base = expr_to_lvalue(*object)?;
            Some(LValue::Field { base: Box::new(base), field, span })
        }
        Expr::Index { object, index, span } => {
            if let ori_ast::expr::IndexExpr::Single(idx) = index {
                let base = expr_to_lvalue(*object)?;
                Some(LValue::Index { base: Box::new(base), index: idx, span })
            } else {
                None
            }
        }
        _ => None,
    }
}

fn expr_lvalue_span(expr: &Expr) -> Span {
    expr.span()
}

fn stmt_span(s: &Stmt) -> Span {
    match s {
        Stmt::Break(sp) | Stmt::Continue(sp) => *sp,
        Stmt::Const(c)  => c.span,
        Stmt::Var(v)    => v.span,
        Stmt::Assign(a) => a.span,
        Stmt::CompoundAssign(c) => c.span,
        Stmt::Return(r) => r.span,
        Stmt::If(i)     => i.span,
        Stmt::IfSome(i) => i.span,
        Stmt::While(w)  => w.span,
        Stmt::WhileSome(w) => w.span,
        Stmt::For(f)    => f.span,
        Stmt::Repeat(r) => r.span,
        Stmt::Loop(l)   => l.span,
        Stmt::Match(m)  => m.span,
        Stmt::Using(u)  => u.span,
        Stmt::Check(c)  => c.span,
        Stmt::Expr(e)   => e.span(),
    }
}
