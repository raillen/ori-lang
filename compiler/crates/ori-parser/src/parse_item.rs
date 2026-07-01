use crate::parser::Parser;
use ori_ast::common::{Attr, AttrArg, Visibility};
use ori_ast::item::{
    AbiLabel, AliasDecl, EnumDecl, EnumVariant, ExternBlock, ExternMember, FuncDecl, FuncSignature,
    ImplementDecl, ImportDecl, ImportItem, Item, ItemWithAttrs, NamedField, NamespaceDecl, Param,
    ParamKind, SourceFile, StructDecl, StructField, TopConst, TopVar, TraitDecl, TraitMember,
};
use ori_diagnostics::Span;
use ori_lexer::TokenKind;
use std::collections::HashSet;

impl<'src> Parser<'src> {
    /// Entry point: parse a full source file.
    pub fn parse_source_file(&mut self) -> SourceFile {
        let start = self.current_span();
        let namespace = if self.at(&TokenKind::Namespace) {
            self.parse_namespace()
        } else {
            self.error(
                "parse.namespace_missing",
                "source file must start with a namespace declaration",
                start,
            );
            None
        }
        .unwrap_or_else(|| NamespaceDecl {
            name: ori_ast::common::QualifiedName {
                parts: Vec::new(),
                span: Span::DUMMY,
            },
            span: Span::DUMMY,
        });
        let mut imports = Vec::new();
        while self.at(&TokenKind::Import)
            || (self.at(&TokenKind::Public) && self.peek_nth_kind(1) == Some(&TokenKind::Import))
        {
            if let Some(i) = self.parse_import() {
                imports.push(i);
            }
        }
        let mut items = Vec::new();
        while !self.at_eof() {
            let before = self.pos;
            if let Some(item) = self.parse_item_with_attrs() {
                items.push(item);
            } else {
                if self.pos == before {
                    self.advance();
                }
                self.synchronize(&[
                    TokenKind::Func,
                    TokenKind::Public,
                    TokenKind::Struct,
                    TokenKind::Enum,
                    TokenKind::Trait,
                    TokenKind::Implement,
                    TokenKind::Alias,
                    TokenKind::Const,
                    TokenKind::Var,
                    TokenKind::Extern,
                    TokenKind::Ident,
                    TokenKind::At,
                ]);
                if self.pos == before {
                    self.advance();
                }
            }
        }
        let end = self.tokens.last().map(|t| t.span).unwrap_or(start);
        SourceFile {
            namespace,
            imports,
            items,
            span: start.cover(end),
        }
    }

    fn parse_namespace(&mut self) -> Option<NamespaceDecl> {
        let start = self.expect(&TokenKind::Namespace)?;
        let name = self.parse_qualified_name()?;
        Some(NamespaceDecl {
            span: start.cover(name.span),
            name,
        })
    }

    fn parse_import(&mut self) -> Option<ImportDecl> {
        let start = self.current_span();
        let visibility = if self.eat(&TokenKind::Public) {
            Visibility::Public
        } else {
            Visibility::Private
        };
        self.expect(&TokenKind::Import)?;
        let path = self.parse_qualified_name()?;
        let (alias, selected) = if self.eat_contextual("only") {
            (None, self.parse_import_selection()?)
        } else if self.eat(&TokenKind::As) {
            match self.parse_name() {
                Some(alias) => (Some(alias), Vec::new()),
                None => {
                    if !self.at_eof() && !is_import_alias_recovery_boundary(self.peek_kind()) {
                        self.advance();
                    }
                    return None;
                }
            }
        } else {
            (None, Vec::new())
        };
        let end = selected
            .last()
            .map(|item| item.span)
            .or_else(|| alias.as_ref().map(|a| a.span))
            .unwrap_or(path.span);
        Some(ImportDecl {
            visibility,
            path,
            alias,
            selected,
            span: start.cover(end),
        })
    }

    fn parse_import_selection(&mut self) -> Option<Vec<ImportItem>> {
        self.expect(&TokenKind::LParen)?;
        let mut selected = Vec::new();
        while !self.at(&TokenKind::RParen) && !self.at_eof() {
            let name = self.parse_name()?;
            let alias = if self.eat(&TokenKind::As) {
                Some(self.parse_name()?)
            } else {
                None
            };
            let end = alias.as_ref().map(|a| a.span).unwrap_or(name.span);
            selected.push(ImportItem {
                span: name.span.cover(end),
                name,
                alias,
            });
            if !self.eat(&TokenKind::Comma) {
                break;
            }
        }
        self.expect(&TokenKind::RParen)?;
        Some(selected)
    }

    fn parse_item_with_attrs(&mut self) -> Option<ItemWithAttrs> {
        let mut attrs = Vec::new();
        while self.at(&TokenKind::At) {
            if let Some(attr) = self.parse_attr() {
                attrs.push(attr);
            }
        }
        let item = self.parse_item()?;
        Some(ItemWithAttrs { attrs, item })
    }

    fn parse_attr(&mut self) -> Option<Attr> {
        let start = self.advance().unwrap().span; // @
        let name = self.parse_name()?;
        let mut end = name.span;
        let args = if self.at(&TokenKind::LParen) {
            self.advance();
            let mut args = Vec::new();
            while !self.at(&TokenKind::RParen) && !self.at_eof() {
                let span = self.current_span();
                if self.at(&TokenKind::Ident) && self.peek_nth_kind(1) == Some(&TokenKind::Colon) {
                    let key = self.parse_name()?;
                    self.expect(&TokenKind::Colon)?;
                    let value = self.parse_name()?;
                    args.push(AttrArg::Named { key, value });
                } else if self.at(&TokenKind::StrLit) {
                    let tok = self.advance().unwrap();
                    let raw = self.slice(tok.span);
                    args.push(AttrArg::String(raw[1..raw.len() - 1].into(), span));
                } else {
                    break;
                }
                if !self.eat(&TokenKind::Comma) {
                    break;
                }
            }
            let rparen = self.expect(&TokenKind::RParen)?;
            end = rparen;
            args
        } else {
            Vec::new()
        };
        Some(Attr {
            name,
            args,
            span: start.cover(end),
        })
    }

    fn parse_visibility(&mut self) -> Visibility {
        if self.eat(&TokenKind::Public) {
            Visibility::Public
        } else {
            Visibility::Private
        }
    }

    fn parse_item(&mut self) -> Option<Item> {
        if self.at(&TokenKind::Namespace) {
            let span = self.current_span();
            self.error(
                "parse.namespace_not_first",
                "namespace must be the first declaration in a file",
                span,
            );
            let _ = self.parse_namespace();
            return None;
        }

        if self.at(&TokenKind::Import)
            || (self.at(&TokenKind::Public) && self.peek_nth_kind(1) == Some(&TokenKind::Import))
        {
            let span = self.current_span();
            self.error(
                "parse.import_after_declaration",
                "imports must appear before declarations",
                span,
            );
            let _ = self.parse_import();
            return None;
        }

        let vis = self.parse_visibility();
        if self.at_contextual("async") {
            return Some(Item::Func(self.parse_func_decl(vis)?));
        }
        match self.peek_kind()? {
            TokenKind::Func | TokenKind::Mut => Some(Item::Func(self.parse_func_decl(vis)?)),
            TokenKind::Struct => Some(Item::Struct(self.parse_struct_decl(vis)?)),
            TokenKind::Enum => Some(Item::Enum(self.parse_enum_decl(vis)?)),
            TokenKind::Trait => Some(Item::Trait(self.parse_trait_decl(vis)?)),
            TokenKind::Implement => Some(Item::Implement(self.parse_implement_decl()?)),
            TokenKind::Alias => Some(Item::Alias(self.parse_alias_decl(vis)?)),
            TokenKind::Const => Some(Item::Const(self.parse_top_const(vis)?)),
            TokenKind::Var => Some(Item::Var(self.parse_top_var(vis)?)),
            TokenKind::Extern => Some(Item::Extern(self.parse_extern_block()?)),
            _ => {
                let span = self.current_span();
                self.error("parse.expected_declaration", "expected a declaration", span);
                None
            }
        }
    }

    // ── Functions ─────────────────────────────────────────────────────────────

    pub fn parse_func_decl(&mut self, vis: Visibility) -> Option<FuncDecl> {
        let start = self.current_span();
        let (is_async, is_mut) = self.parse_func_modifiers();
        self.expect(&TokenKind::Func)?;
        let name = self.parse_name()?;
        let type_params = self.parse_type_params_opt();
        let params = self.parse_param_list()?;
        let return_ty = if self.eat(&TokenKind::Arrow) {
            Some(self.parse_type()?)
        } else {
            None
        };
        let where_clause = self.parse_where_clause_opt();
        let body = self.parse_block()?;
        let end = self.expect_block_end(start, "function")?;
        Some(FuncDecl {
            visibility: vis,
            is_async,
            is_mut,
            name,
            type_params,
            params,
            return_ty,
            where_clause,
            body,
            span: start.cover(end),
        })
    }

    fn parse_func_signature(&mut self, vis: Visibility) -> Option<FuncSignature> {
        let start = self.current_span();
        let (is_async, is_mut) = self.parse_func_modifiers();
        self.expect(&TokenKind::Func)?;
        let name = self.parse_name()?;
        let type_params = self.parse_type_params_opt();
        let params = self.parse_param_list()?;
        let return_ty = if self.eat(&TokenKind::Arrow) {
            Some(self.parse_type()?)
        } else {
            None
        };
        let where_clause = self.parse_where_clause_opt();
        let end = return_ty.as_ref().map(|t| t.span()).unwrap_or(name.span);
        Some(FuncSignature {
            visibility: vis,
            is_async,
            is_mut,
            name,
            type_params,
            params,
            return_ty,
            where_clause,
            span: start.cover(end),
        })
    }

    fn parse_param_list(&mut self) -> Option<Vec<Param>> {
        self.expect(&TokenKind::LParen)?;
        let mut params = Vec::new();
        while !self.at(&TokenKind::RParen) && !self.at_eof() {
            params.push(self.parse_param()?);
            if !self.eat(&TokenKind::Comma) {
                break;
            }
        }
        self.expect(&TokenKind::RParen)?;
        self.validate_param_list(&params);
        Some(params)
    }

    fn parse_func_modifiers(&mut self) -> (bool, bool) {
        let mut is_async = false;
        let mut is_mut = false;
        for _ in 0..2 {
            if !is_async && self.eat_contextual("async") {
                is_async = true;
                continue;
            }
            if !is_mut && self.eat(&TokenKind::Mut) {
                is_mut = true;
                continue;
            }
            break;
        }
        (is_async, is_mut)
    }

    fn parse_param(&mut self) -> Option<Param> {
        let start = self.current_span();
        // `self` parameter
        if self.at(&TokenKind::SelfKw) {
            let tok = self.advance().unwrap();
            let name = ori_ast::common::Name::new("self", tok.span);
            return Some(Param {
                name,
                ty: ori_ast::ty::Type::Named(ori_ast::common::QualifiedName::single(
                    ori_ast::common::Name::new("Self", tok.span),
                )),
                kind: ParamKind::Required,
                span: tok.span,
            });
        }
        let name = self.parse_name()?;
        self.expect(&TokenKind::Colon)?;
        let ty = self.parse_type()?;
        // Variadic: `name: Type...`.
        //
        // `Type..` is accepted for compatibility with early fixtures, but new
        // code should use the documented `Type...` form.
        if self.eat(&TokenKind::Ellipsis) || self.eat(&TokenKind::DotDot) {
            let span = start.cover(self.current_span());
            return Some(Param {
                name,
                ty,
                kind: ParamKind::Variadic,
                span,
            });
        }
        // Default: `name: Type = expr`
        let default = if self.eat(&TokenKind::Eq) {
            Some(Box::new(self.parse_expr()?))
        } else {
            None
        };
        // Contract: `name: Type if it > 0`
        let contract = if self.eat(&TokenKind::If) {
            Some(Box::new(self.parse_expr()?))
        } else {
            None
        };
        let kind = match (default, contract) {
            (None, None) => ParamKind::Required,
            (Some(d), None) => ParamKind::Default(d),
            (None, Some(c)) => ParamKind::Contract(c),
            (Some(d), Some(c)) => ParamKind::DefaultAndContract(d, c),
        };
        let span = start.cover(self.current_span());
        Some(Param {
            name,
            ty,
            kind,
            span,
        })
    }

    fn validate_param_list(&mut self, params: &[Param]) {
        let mut seen_default = false;
        let mut seen_names = HashSet::new();
        for (index, param) in params.iter().enumerate() {
            if !seen_names.insert(param.name.text.clone()) {
                self.error(
                    "bind.duplicate_param",
                    format!("duplicate parameter `{}`", param.name.text),
                    param.name.span,
                );
            }

            if matches!(param.kind, ParamKind::Variadic) && index + 1 != params.len() {
                self.error(
                    "parse.variadic_not_last",
                    "variadic parameter must be the last parameter",
                    param.span,
                );
            }

            if seen_default && param_is_required(&param.kind) {
                self.error(
                    "parse.default_before_required",
                    "required parameter cannot follow a default parameter",
                    param.span,
                );
            }

            if param_has_default(&param.kind) {
                seen_default = true;
            }
        }
    }

    // ── Structs ───────────────────────────────────────────────────────────────

    fn parse_struct_decl(&mut self, vis: Visibility) -> Option<StructDecl> {
        let start = self.advance().unwrap().span; // struct
        let name = self.parse_name()?;
        let type_params = self.parse_type_params_opt();
        let where_clause = self.parse_where_clause_opt();
        let mut fields = Vec::new();
        let mut field_names = HashSet::new();
        let mut methods = Vec::new();
        // Fields: ident : type [if expr]
        while self.at(&TokenKind::Ident) {
            let field = self.parse_struct_field()?;
            if !field_names.insert(field.name.text.clone()) {
                self.error(
                    "bind.duplicate_field",
                    format!("duplicate struct field `{}`", field.name.text),
                    field.name.span,
                );
            }
            fields.push(field);
        }
        // Methods: [public] [mut] func …
        while !self.at(&TokenKind::End) && !self.at_eof() {
            let mvis = self.parse_visibility();
            methods.push(self.parse_func_decl(mvis)?);
        }
        let end = self.expect_block_end(start, "struct")?;
        Some(StructDecl {
            visibility: vis,
            name,
            type_params,
            where_clause,
            fields,
            methods,
            span: start.cover(end),
        })
    }

    fn parse_struct_field(&mut self) -> Option<StructField> {
        let start = self.current_span();
        let name = self.parse_name()?;
        self.expect(&TokenKind::Colon)?;
        let ty = self.parse_type()?;
        let contract = if self.eat(&TokenKind::If) {
            Some(Box::new(self.parse_expr()?))
        } else {
            None
        };
        let span = start.cover(self.current_span());
        Some(StructField {
            name,
            ty,
            contract,
            span,
        })
    }

    // ── Enums ─────────────────────────────────────────────────────────────────

    fn parse_enum_decl(&mut self, vis: Visibility) -> Option<EnumDecl> {
        let start = self.advance().unwrap().span; // enum
        let name = self.parse_name()?;
        let type_params = self.parse_type_params_opt();
        let mut variants = Vec::new();
        let mut variant_names = HashSet::new();
        while !self.at(&TokenKind::End) && !self.at_eof() {
            let variant = self.parse_enum_variant()?;
            if !variant_names.insert(variant.name.text.clone()) {
                self.error(
                    "bind.duplicate_variant",
                    format!("duplicate enum variant `{}`", variant.name.text),
                    variant.name.span,
                );
            }
            variants.push(variant);
        }
        let end = self.expect_block_end(start, "enum")?;
        Some(EnumDecl {
            visibility: vis,
            name,
            type_params,
            variants,
            span: start.cover(end),
        })
    }

    fn parse_enum_variant(&mut self) -> Option<EnumVariant> {
        let start = self.current_span();
        let name = self.parse_name()?;
        let fields = if self.at(&TokenKind::LParen) {
            self.advance();
            let mut fs = Vec::new();
            while !self.at(&TokenKind::RParen) && !self.at_eof() {
                let n = self.parse_name()?;
                self.expect(&TokenKind::Colon)?;
                let ty = self.parse_type()?;
                let sp = n.span.cover(ty.span());
                fs.push(NamedField {
                    name: n,
                    ty,
                    span: sp,
                });
                if !self.eat(&TokenKind::Comma) {
                    break;
                }
            }
            self.expect(&TokenKind::RParen)?;
            fs
        } else {
            Vec::new()
        };
        let end = self.current_span();
        Some(EnumVariant {
            name,
            fields,
            span: start.cover(end),
        })
    }

    // ── Traits ────────────────────────────────────────────────────────────────

    fn parse_trait_decl(&mut self, vis: Visibility) -> Option<TraitDecl> {
        let start = self.advance().unwrap().span; // trait
        let name = self.parse_name()?;
        let type_params = self.parse_type_params_opt();
        let where_clause = self.parse_where_clause_opt();
        let mut members = Vec::new();
        while !self.at(&TokenKind::End) && !self.at_eof() {
            let associated_type = self
                .peek()
                .is_some_and(|tok| tok.kind == TokenKind::Ident && self.slice(tok.span) == "type");
            if associated_type {
                self.advance(); // consume "type"
                let name = self.parse_name()?;
                members.push(TraitMember::Type(name));
                continue;
            }
            let mvis = self.parse_visibility();
            let is_mut = self.at(&TokenKind::Mut);
            // Peek: is there a body? Advance copy of pos to find out
            let sig = self.parse_func_signature(mvis)?;
            // If next is a statement-starting token or the func signature is followed
            // by something other than `func`/`public`/`end`/`mut`, it has a body
            let has_body = !self.at_any(&[
                TokenKind::Func,
                TokenKind::Public,
                TokenKind::Mut,
                TokenKind::End,
            ]);
            if has_body && !is_mut {
                let body = self.parse_block()?;
                let end = self.expect_block_end(sig.span, "trait method")?;
                let decl = FuncDecl {
                    visibility: sig.visibility,
                    is_async: sig.is_async,
                    is_mut: sig.is_mut,
                    name: sig.name.clone(),
                    type_params: sig.type_params.clone(),
                    params: sig.params.clone(),
                    return_ty: sig.return_ty.clone(),
                    where_clause: sig.where_clause.clone(),
                    body,
                    span: sig.span.cover(end),
                };
                members.push(TraitMember::Default(decl));
            } else {
                members.push(TraitMember::Required(sig));
            }
        }
        let end = self.expect_block_end(start, "trait")?;
        Some(TraitDecl {
            visibility: vis,
            name,
            type_params,
            where_clause,
            members,
            span: start.cover(end),
        })
    }

    // ── Implement ─────────────────────────────────────────────────────────────

    fn parse_implement_decl(&mut self) -> Option<ImplementDecl> {
        let start = self.advance().unwrap().span; // implement
        let type_params = self.parse_type_params_opt();
        let trait_name = self.parse_qualified_name()?;
        self.expect(&TokenKind::For)?;
        let for_type = self.parse_qualified_name()?;
        let where_clause = self.parse_where_clause_opt();
        let mut methods = Vec::new();
        let mut associated_types = Vec::new();
        while !self.at(&TokenKind::End) && !self.at_eof() {
            let is_assoc = self
                .peek()
                .is_some_and(|tok| tok.kind == TokenKind::Ident && self.slice(tok.span) == "type");
            if is_assoc {
                self.advance(); // type
                let name = self.parse_name()?;
                self.expect(&TokenKind::Eq)?;
                let ty = self.parse_type()?;
                associated_types.push((name, ty));
                continue;
            }
            let mvis = self.parse_visibility();
            methods.push(self.parse_func_decl(mvis)?);
        }
        let end = self.expect_block_end(start, "implement")?;
        Some(ImplementDecl {
            type_params,
            trait_name,
            for_type,
            where_clause,
            methods,
            associated_types,
            span: start.cover(end),
        })
    }

    // ── Alias / Const / Var ───────────────────────────────────────────────────

    fn parse_alias_decl(&mut self, vis: Visibility) -> Option<AliasDecl> {
        let start = self.advance().unwrap().span; // alias
        let name = self.parse_name()?;
        let type_params = self.parse_type_params_opt();
        self.expect(&TokenKind::Eq)?;
        let ty = self.parse_type()?;
        let span = start.cover(ty.span());
        Some(AliasDecl {
            visibility: vis,
            name,
            type_params,
            ty,
            span,
        })
    }

    fn parse_top_const(&mut self, vis: Visibility) -> Option<TopConst> {
        let start = self.advance().unwrap().span; // const
        let name = self.parse_name()?;
        self.expect(&TokenKind::Colon)?;
        let ty = self.parse_type()?;
        self.expect(&TokenKind::Eq)?;
        let value = self.parse_expr()?;
        let span = start.cover(value.span());
        Some(TopConst {
            visibility: vis,
            name,
            ty,
            value: Box::new(value),
            span,
        })
    }

    fn parse_top_var(&mut self, vis: Visibility) -> Option<TopVar> {
        let start = self.advance().unwrap().span; // var
        let name = self.parse_name()?;
        self.expect(&TokenKind::Colon)?;
        let ty = self.parse_type()?;
        self.expect(&TokenKind::Eq)?;
        let value = self.parse_expr()?;
        let span = start.cover(value.span());
        Some(TopVar {
            visibility: vis,
            name,
            ty,
            value: Box::new(value),
            span,
        })
    }

    // ── Extern ────────────────────────────────────────────────────────────────

    fn parse_extern_block(&mut self) -> Option<ExternBlock> {
        let start = self.advance().unwrap().span; // extern
        let abi = if self.at(&TokenKind::Ident) {
            let tok = self.peek().unwrap();
            match self.slice(tok.span) {
                "c" | "C" => {
                    self.advance();
                    AbiLabel::C
                }
                "host" => {
                    self.advance();
                    AbiLabel::Host
                }
                abi => {
                    self.error(
                        "extern.unknown_abi",
                        format!("unknown extern ABI `{abi}`"),
                        tok.span,
                    );
                    self.advance();
                    AbiLabel::C
                }
            }
        } else {
            AbiLabel::C
        };
        let mut members = Vec::new();
        while !self.at(&TokenKind::End) && !self.at_eof() {
            let mvis = self.parse_visibility();
            members.push(self.parse_extern_member(mvis)?);
        }
        let end = self.expect_block_end(start, "extern")?;
        Some(ExternBlock {
            abi,
            members,
            span: start.cover(end),
        })
    }

    fn parse_extern_member(&mut self, vis: Visibility) -> Option<ExternMember> {
        let start = self.current_span();
        if self.at(&TokenKind::Func) {
            self.advance();
            let name = self.parse_name()?;
            let params = self.parse_param_list()?;
            let return_ty = if self.eat(&TokenKind::Arrow) {
                Some(self.parse_type()?)
            } else {
                None
            };
            let end = return_ty.as_ref().map(|t| t.span()).unwrap_or(name.span);
            Some(ExternMember::Func {
                visibility: vis,
                name,
                params,
                return_ty,
                span: start.cover(end),
            })
        } else if self.at(&TokenKind::Var) {
            self.advance();
            let name = self.parse_name()?;
            self.expect(&TokenKind::Colon)?;
            let ty = self.parse_type()?;
            let span = start.cover(ty.span());
            Some(ExternMember::Var {
                visibility: vis,
                name,
                ty,
                span,
            })
        } else {
            let span = self.current_span();
            self.error(
                "parse.expected_extern_member",
                "expected `func` or `var` in extern block",
                span,
            );
            None
        }
    }
}

fn is_import_alias_recovery_boundary(kind: Option<&TokenKind>) -> bool {
    matches!(
        kind,
        Some(TokenKind::Namespace)
            | Some(TokenKind::Import)
            | Some(TokenKind::Public)
            | Some(TokenKind::Func)
            | Some(TokenKind::Struct)
            | Some(TokenKind::Enum)
            | Some(TokenKind::Trait)
            | Some(TokenKind::Implement)
            | Some(TokenKind::Alias)
            | Some(TokenKind::Const)
            | Some(TokenKind::Var)
            | Some(TokenKind::Extern)
    )
}

fn param_has_default(kind: &ParamKind) -> bool {
    matches!(
        kind,
        ParamKind::Default(_) | ParamKind::DefaultAndContract(_, _)
    )
}

fn param_is_required(kind: &ParamKind) -> bool {
    matches!(kind, ParamKind::Required | ParamKind::Contract(_))
}
