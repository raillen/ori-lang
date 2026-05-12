use crate::parser::Parser;
use ori_ast::common::{Attr, AttrArg, Visibility};
use ori_ast::item::{
    AbiLabel, AliasDecl, EnumDecl, EnumVariant, ExternBlock, ExternMember, FuncDecl, FuncSignature,
    ImplementDecl, ImportDecl, Item, ItemWithAttrs, NamedField, NamespaceDecl, Param, ParamKind,
    SourceFile, StructDecl, StructField, TopConst, TopVar, TraitDecl, TraitMember,
};
use ori_diagnostics::Span;
use ori_lexer::TokenKind;

impl<'src> Parser<'src> {
    /// Entry point: parse a full source file.
    pub fn parse_source_file(&mut self) -> SourceFile {
        let start = self.current_span();
        let namespace = self.parse_namespace().unwrap_or_else(|| NamespaceDecl {
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
            if let Some(item) = self.parse_item_with_attrs() {
                items.push(item);
            } else {
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
                    TokenKind::At,
                ]);
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
        let alias = if self.eat(&TokenKind::As) {
            Some(self.parse_name()?)
        } else {
            None
        };
        let end = alias.as_ref().map(|a| a.span).unwrap_or(path.span);
        Some(ImportDecl {
            visibility,
            path,
            alias,
            span: start.cover(end),
        })
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
            self.expect(&TokenKind::RParen)?;
            args
        } else {
            Vec::new()
        };
        let end = args
            .last()
            .map(|_| self.current_span())
            .unwrap_or(name.span);
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
        let vis = self.parse_visibility();
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
        let is_mut = self.eat(&TokenKind::Mut);
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
        let end = self.expect(&TokenKind::End)?;
        Some(FuncDecl {
            visibility: vis,
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
        let is_mut = self.eat(&TokenKind::Mut);
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
        Some(params)
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
        // Variadic: `name: Type...`
        if self.eat(&TokenKind::DotDot) {
            let span = start.cover(self.current_span());
            return Some(Param {
                name,
                ty,
                kind: ParamKind::Variadic,
                span,
            });
        }
        // Contract: `name: Type if it > 0`
        let has_contract = self.at(&TokenKind::If);
        let contract = if has_contract {
            self.advance(); // if
            Some(Box::new(self.parse_expr()?))
        } else {
            None
        };
        // Default: `name: Type = expr`
        let default = if self.eat(&TokenKind::Eq) {
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

    // ── Structs ───────────────────────────────────────────────────────────────

    fn parse_struct_decl(&mut self, vis: Visibility) -> Option<StructDecl> {
        let start = self.advance().unwrap().span; // struct
        let name = self.parse_name()?;
        let type_params = self.parse_type_params_opt();
        let where_clause = self.parse_where_clause_opt();
        let mut fields = Vec::new();
        let mut methods = Vec::new();
        // Fields: ident : type [if expr]
        while self.at(&TokenKind::Ident) {
            fields.push(self.parse_struct_field()?);
        }
        // Methods: [public] [mut] func …
        while !self.at(&TokenKind::End) && !self.at_eof() {
            let mvis = self.parse_visibility();
            methods.push(self.parse_func_decl(mvis)?);
        }
        let end = self.expect(&TokenKind::End)?;
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
        while !self.at(&TokenKind::End) && !self.at_eof() {
            variants.push(self.parse_enum_variant()?);
        }
        let end = self.expect(&TokenKind::End)?;
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
                let end = self.expect(&TokenKind::End)?;
                let decl = FuncDecl {
                    visibility: sig.visibility,
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
        let end = self.expect(&TokenKind::End)?;
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
        while !self.at(&TokenKind::End) && !self.at_eof() {
            let mvis = self.parse_visibility();
            methods.push(self.parse_func_decl(mvis)?);
        }
        let end = self.expect(&TokenKind::End)?;
        Some(ImplementDecl {
            type_params,
            trait_name,
            for_type,
            where_clause,
            methods,
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
                _ => AbiLabel::C,
            }
        } else {
            AbiLabel::C
        };
        let mut members = Vec::new();
        while !self.at(&TokenKind::End) && !self.at_eof() {
            let mvis = self.parse_visibility();
            members.push(self.parse_extern_member(mvis)?);
        }
        let end = self.expect(&TokenKind::End)?;
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
