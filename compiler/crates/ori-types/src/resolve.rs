use smol_str::SmolStr;
use std::collections::HashMap;
use ori_diagnostics::{Diagnostic, DiagnosticSink, FileId, Label};
use ori_ast::item::{Item, SourceFile};
use crate::def::{DefId, DefKind, DefMap};
use crate::lower::lower_type_with_aliases;
use crate::ty::Ty;

/// Resolved type information for a single function (signature only for now).
#[derive(Debug, Clone)]
pub struct FuncSig {
    pub def_id:    DefId,
    pub params:    Vec<Ty>,
    pub return_ty: Ty,
}

#[derive(Debug, Clone)]
pub struct ValueSig {
    pub def_id: DefId,
    pub ty:     Ty,
}

/// The output of resolving a single source file.
#[derive(Debug)]
pub struct ResolvedModule {
    pub def_map:    DefMap,
    pub func_sigs:  Vec<FuncSig>,
    pub value_sigs: Vec<ValueSig>,
    pub namespace:  SmolStr,
}

/// Build a `ResolvedModule` from a `SourceFile`.
///
/// Phase 1 — register all top-level definitions.
/// Phase 2 — lower all type annotations to `Ty`.
pub fn resolve(file: &SourceFile, file_id: FileId, sink: &mut DiagnosticSink) -> ResolvedModule {
    let namespace = SmolStr::new(file.namespace.name.to_string());
    resolve_many(&[(file, file_id)], namespace, sink)
}

pub fn resolve_many<S: Into<SmolStr>>(
    files: &[(&SourceFile, FileId)],
    entry_namespace: S,
    sink: &mut DiagnosticSink,
) -> ResolvedModule {
    let mut def_map = DefMap::default();

    // ── Phase 1: register definitions ────────────────────────────────────────
    for (file, file_id) in files {
        let namespace = SmolStr::new(file.namespace.name.to_string());
        for item in &file.items {
            register_item(&item.item, &namespace, &mut def_map, *file_id, sink);
        }
    }

    // ── Phase 2: lower function signatures ───────────────────────────────────
    let mut func_sigs = Vec::new();
    let mut value_sigs = Vec::new();
    for (file, file_id) in files {
        let namespace = SmolStr::new(file.namespace.name.to_string());
        let aliases = import_aliases(file);
        for item in &file.items {
            match &item.item {
                Item::Func(f) => {
                    let tp: Vec<SmolStr> = f.type_params.iter().map(|p| p.name.text.clone()).collect();
                    let params = f.params.iter()
                        .map(|p| lower_type_with_aliases(&p.ty, &namespace, &tp, &def_map, *file_id, sink, &aliases))
                        .collect();
                    let return_ty = f.return_ty.as_ref()
                        .map(|t| lower_type_with_aliases(t, &namespace, &tp, &def_map, *file_id, sink, &aliases))
                        .unwrap_or(Ty::Void);
                    let path = format!("{}.{}", namespace, f.name.text);
                    if let Some(def_id) = def_map.lookup(&path) {
                        func_sigs.push(FuncSig { def_id, params, return_ty });
                    }
                }
                Item::Const(c) => {
                    let path = format!("{}.{}", namespace, c.name.text);
                    if let Some(def_id) = def_map.lookup(&path) {
                        let ty = lower_type_with_aliases(&c.ty, &namespace, &[], &def_map, *file_id, sink, &aliases);
                        value_sigs.push(ValueSig { def_id, ty });
                    }
                }
                Item::Var(v) => {
                    let path = format!("{}.{}", namespace, v.name.text);
                    if let Some(def_id) = def_map.lookup(&path) {
                        let ty = lower_type_with_aliases(&v.ty, &namespace, &[], &def_map, *file_id, sink, &aliases);
                        value_sigs.push(ValueSig { def_id, ty });
                    }
                }
                _ => {}
            }
        }
    }

    ResolvedModule { def_map, func_sigs, value_sigs, namespace: entry_namespace.into() }
}

// ── Registration helpers ──────────────────────────────────────────────────────

fn register_item(
    item:      &Item,
    ns:        &str,
    def_map:   &mut DefMap,
    file_id:   FileId,
    sink:      &mut DiagnosticSink,
) {
    let mut reg = |def_map: &mut DefMap, kind, name: &SmolStr, span| {
        let path = SmolStr::new(format!("{}.{}", ns, name));
        // Duplicate check
        if def_map.lookup(&path).is_some() {
            sink.emit(
                Diagnostic::error("name.duplicate", format!("duplicate definition `{}`", name))
                    .with_label(Label::primary(file_id, span, "defined again here"))
                    .with_action("rename or remove one of the definitions"),
            );
            return;
        }
        def_map.register(kind, name.clone(), path, span);
    };

    match item {
        Item::Struct(s)    => reg(def_map, DefKind::Struct,    &s.name.text, s.span),
        Item::Enum(e)      => reg(def_map, DefKind::Enum,      &e.name.text, e.span),
        Item::Trait(t)     => reg(def_map, DefKind::Trait,     &t.name.text, t.span),
        Item::Func(f)      => reg(def_map, DefKind::Func,      &f.name.text, f.span),
        Item::Alias(a)     => reg(def_map, DefKind::TypeAlias, &a.name.text, a.span),
        Item::Const(c)     => reg(def_map, DefKind::Const,     &c.name.text, c.span),
        Item::Var(v)       => reg(def_map, DefKind::Var,       &v.name.text, v.span),
        Item::Extern(ext)  => {
            for member in &ext.members {
                match member {
                    ori_ast::item::ExternMember::Func { name, span, .. } =>
                        reg(def_map, DefKind::Extern, &name.text, *span),
                    ori_ast::item::ExternMember::Var { name, span, .. } =>
                        reg(def_map, DefKind::Var, &name.text, *span),
                }
            }
        }
        Item::Implement(_) => {} // not directly named at top level
    }
}

fn import_aliases(file: &SourceFile) -> HashMap<SmolStr, SmolStr> {
    let mut aliases = HashMap::new();
    for import in &file.imports {
        let alias = import.alias.as_ref()
            .map(|a| a.text.clone())
            .unwrap_or_else(|| import.path.last().text.clone());
        aliases.insert(alias, SmolStr::new(import.path.to_string()));
    }
    aliases
}
