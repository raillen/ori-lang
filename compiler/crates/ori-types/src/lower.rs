use crate::def::DefMap;
use crate::ty::{OpaqueTy, Ty};
use ori_ast::common::QualifiedName;
use ori_ast::ty::Type as AstType;
use ori_diagnostics::{Diagnostic, DiagnosticSink, FileId, Label};
use smol_str::SmolStr;
use std::collections::HashMap;

/// Lower an AST type into a canonical `Ty`.
///
/// `module_path` is the current namespace (e.g. `"app.user"`).
/// `type_params` are the in-scope generic type parameter names.
pub fn lower_type(
    ast_ty: &AstType,
    module_path: &str,
    type_params: &[SmolStr],
    def_map: &DefMap,
    file_id: FileId,
    sink: &mut DiagnosticSink,
) -> Ty {
    let aliases = HashMap::new();
    lower_type_with_aliases(
        ast_ty,
        module_path,
        type_params,
        def_map,
        file_id,
        sink,
        &aliases,
    )
}

pub fn lower_type_with_aliases(
    ast_ty: &AstType,
    module_path: &str,
    type_params: &[SmolStr],
    def_map: &DefMap,
    file_id: FileId,
    sink: &mut DiagnosticSink,
    aliases: &HashMap<SmolStr, SmolStr>,
) -> Ty {
    macro_rules! rec {
        ($t:expr) => {
            lower_type_with_aliases(
                $t,
                module_path,
                type_params,
                def_map,
                file_id,
                sink,
                aliases,
            )
        };
    }
    match ast_ty {
        // ── Primitives ────────────────────────────────────────────────────────
        AstType::Bool(_) => Ty::Bool,
        AstType::Int(_) => Ty::Int,
        AstType::Int8(_) => Ty::Int8,
        AstType::Int16(_) => Ty::Int16,
        AstType::Int32(_) => Ty::Int32,
        AstType::Int64(_) => Ty::Int64,
        AstType::U8(_) => Ty::U8,
        AstType::U16(_) => Ty::U16,
        AstType::U32(_) => Ty::U32,
        AstType::U64(_) => Ty::U64,
        AstType::Float(_) => Ty::Float,
        AstType::Float32(_) => Ty::Float32,
        AstType::Float64(_) => Ty::Float64,
        AstType::String(_) => Ty::String,
        AstType::Bytes(_) => Ty::Bytes,
        AstType::Void(_) => Ty::Void,

        // ── Built-in generic types ────────────────────────────────────────────
        AstType::Optional(inner, _) => Ty::Optional(Box::new(rec!(inner))),
        AstType::Result(ok, err, _) => Ty::Result(Box::new(rec!(ok)), Box::new(rec!(err))),
        AstType::List(elem, _) => Ty::List(Box::new(rec!(elem))),
        AstType::Map(key, val, _) => Ty::Map(Box::new(rec!(key)), Box::new(rec!(val))),
        AstType::Set(elem, _) => Ty::Set(Box::new(rec!(elem))),
        AstType::Range(elem, _) => Ty::Range(Box::new(rec!(elem))),
        AstType::Lazy(inner, _) => Ty::Lazy(Box::new(rec!(inner))),
        AstType::Tuple(elems, _) => Ty::Tuple(elems.iter().map(|t| rec!(t)).collect()),
        AstType::Any(trait_name, span) => {
            let id = resolve_name(
                trait_name,
                module_path,
                def_map,
                file_id,
                *span,
                sink,
                aliases,
            );
            Ty::Any(id.unwrap_or(crate::def::DefId(u32::MAX)))
        }

        // ── Callable type ─────────────────────────────────────────────────────
        AstType::Func {
            params, return_ty, ..
        } => {
            let ps = params.iter().map(|t| rec!(t)).collect();
            let ret = return_ty.as_ref().map_or(Ty::Void, |t| rec!(t));
            Ty::Func {
                params: ps,
                ret: Box::new(ret),
            }
        }

        // ── Named / generic types ─────────────────────────────────────────────
        AstType::Named(name) => lower_named(
            name,
            &[],
            module_path,
            type_params,
            def_map,
            file_id,
            sink,
            aliases,
        ),

        AstType::Generic { name, args, .. } => {
            let lowered_args: Vec<Ty> = args.iter().map(|t| rec!(t)).collect();
            lower_named(
                name,
                &lowered_args,
                module_path,
                type_params,
                def_map,
                file_id,
                sink,
                aliases,
            )
        }
    }
}

fn lower_named(
    name: &QualifiedName,
    args: &[Ty],
    module_path: &str,
    type_params: &[SmolStr],
    def_map: &DefMap,
    file_id: FileId,
    sink: &mut DiagnosticSink,
    aliases: &HashMap<SmolStr, SmolStr>,
) -> Ty {
    // Check if it's an in-scope type parameter (must be a single-segment name)
    if name.is_single() {
        let n = name.last().as_str();
        if let Some(idx) = type_params.iter().position(|p| p == n) {
            return Ty::Param {
                index: idx as u32,
                name: SmolStr::new(n),
            };
        }
    }
    let expanded = expand_alias(&name.to_string(), aliases);
    if let Some(ty) = lower_builtin_concurrency_type(&expanded, args) {
        return ty;
    }
    let span = name.span;
    match resolve_name(name, module_path, def_map, file_id, span, sink, aliases) {
        Some(id) => Ty::Named(id, args.to_vec()),
        None => Ty::Error,
    }
}

fn lower_builtin_concurrency_type(path: &str, args: &[Ty]) -> Option<Ty> {
    match path {
        "future" => Some(Ty::Future(Box::new(
            args.first().cloned().unwrap_or(Ty::Infer(0)),
        ))),
        "ori.task.Job" => Some(Ty::TaskJob(Box::new(
            args.first().cloned().unwrap_or(Ty::Infer(0)),
        ))),
        "ori.task.JoinError" => Some(Ty::TaskJoinError),
        "ori.channel.Channel" => Some(Ty::Channel(Box::new(
            args.first().cloned().unwrap_or(Ty::Infer(0)),
        ))),
        "ori.channel.SendError" => Some(Ty::ChannelSendError),
        "ori.channel.ReceiveError" => Some(Ty::ChannelReceiveError),
        "ori.atomic.AtomicInt" => Some(Ty::AtomicInt),
        "ori.deque.Deque" => Some(Ty::Opaque {
            kind: OpaqueTy::Deque,
            args: vec![args.first().cloned().unwrap_or(Ty::Infer(0))],
        }),
        "ori.queue.Queue" => Some(Ty::Opaque {
            kind: OpaqueTy::Queue,
            args: vec![args.first().cloned().unwrap_or(Ty::Infer(0))],
        }),
        "ori.stack.Stack" => Some(Ty::Opaque {
            kind: OpaqueTy::Stack,
            args: vec![args.first().cloned().unwrap_or(Ty::Infer(0))],
        }),
        "ori.linked_list.LinkedList" => Some(Ty::Opaque {
            kind: OpaqueTy::LinkedList,
            args: vec![args.first().cloned().unwrap_or(Ty::Infer(0))],
        }),
        "ori.doubly_linked_list.DoublyLinkedList" => Some(Ty::Opaque {
            kind: OpaqueTy::DoublyLinkedList,
            args: vec![args.first().cloned().unwrap_or(Ty::Infer(0))],
        }),
        "ori.tree.Tree" => Some(Ty::Opaque {
            kind: OpaqueTy::Tree,
            args: vec![args.first().cloned().unwrap_or(Ty::Infer(0))],
        }),
        "ori.tree.NodeId" => Some(Ty::Opaque {
            kind: OpaqueTy::NodeId,
            args: vec![],
        }),
        "ori.hash_table.HashTable" => Some(Ty::Opaque {
            kind: OpaqueTy::HashTable,
            args: vec![
                args.first().cloned().unwrap_or(Ty::Infer(0)),
                args.get(1).cloned().unwrap_or(Ty::Infer(1)),
            ],
        }),
        "ori.graph.Graph" => Some(Ty::Opaque {
            kind: OpaqueTy::Graph,
            args: vec![args.first().cloned().unwrap_or(Ty::Infer(0))],
        }),
        "ori.heap.Heap" => Some(Ty::Opaque {
            kind: OpaqueTy::Heap,
            args: vec![args.first().cloned().unwrap_or(Ty::Infer(0))],
        }),
        "ori.fs.File" => Some(Ty::Opaque {
            kind: OpaqueTy::File,
            args: vec![],
        }),
        "ori.task.CancelToken" => Some(Ty::Opaque {
            kind: OpaqueTy::CancelToken,
            args: vec![],
        }),
        _ => None,
    }
}

fn resolve_name(
    name: &QualifiedName,
    module_path: &str,
    def_map: &DefMap,
    file_id: FileId,
    span: ori_diagnostics::Span,
    sink: &mut DiagnosticSink,
    aliases: &HashMap<SmolStr, SmolStr>,
) -> Option<crate::def::DefId> {
    let path_str = name.to_string();
    let expanded = expand_alias(&path_str, aliases);
    // Try fully-qualified first
    if let Some(id) = def_map.lookup(&expanded) {
        return Some(id);
    }
    // Try with module prefix
    let local = format!("{}.{}", module_path, expanded);
    if let Some(id) = def_map.lookup(&local) {
        return Some(id);
    }
    sink.emit(
        Diagnostic::error(
            "type.undefined_name",
            format!("undefined type `{}`", path_str),
        )
        .with_label(Label::primary(file_id, span, "not defined in scope"))
        .with_action("ensure the type is defined in this namespace or imported with `import`"),
    );
    None
}

fn expand_alias(name: &str, aliases: &HashMap<SmolStr, SmolStr>) -> String {
    let mut prefix_end = name.len();
    loop {
        let prefix = &name[..prefix_end];
        if let Some(full_ns) = aliases.get(prefix) {
            let suffix = &name[prefix_end..];
            if suffix.is_empty() {
                return full_ns.to_string();
            }
            return format!("{}{}", full_ns, suffix);
        }
        if let Some(dot) = name[..prefix_end].rfind('.') {
            prefix_end = dot;
        } else {
            break;
        }
    }
    name.to_string()
}
