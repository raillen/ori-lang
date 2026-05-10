use smol_str::SmolStr;
use std::collections::HashMap;
use ori_diagnostics::Span;

/// A unique identifier for a top-level definition within a compilation session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DefId(pub u32);

impl std::fmt::Display for DefId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "def#{}", self.0)
    }
}

/// What kind of thing a `DefId` refers to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefKind {
    Struct,
    Enum,
    Trait,
    Func,
    Const,
    Var,
    TypeAlias,
    Extern,
}

/// A single registered definition.
#[derive(Debug, Clone)]
pub struct Def {
    pub id:   DefId,
    pub kind: DefKind,
    /// Simple (unqualified) name: `"User"`, `"connect"`.
    pub name: SmolStr,
    /// Fully-qualified path: `"app.user.User"`, `"ori.io.print"`.
    pub path: SmolStr,
    pub span: Span,
}

/// Maps fully-qualified names to their definitions.
///
/// Populated during name resolution; queried by the type checker.
#[derive(Debug, Default)]
pub struct DefMap {
    defs:    Vec<Def>,
    by_path: HashMap<SmolStr, DefId>,
}

impl DefMap {
    /// Register a new definition. Returns its `DefId`.
    ///
    /// If `path` is already registered, returns the existing `DefId` without
    /// inserting a duplicate (the caller should emit a `name.duplicate` error).
    pub fn register(
        &mut self,
        kind: DefKind,
        name: SmolStr,
        path: SmolStr,
        span: Span,
    ) -> DefId {
        if let Some(&existing) = self.by_path.get(&path) {
            return existing;
        }
        let id = DefId(self.defs.len() as u32);
        self.defs.push(Def { id, kind, name, path: path.clone(), span });
        self.by_path.insert(path, id);
        id
    }

    pub fn lookup(&self, path: &str) -> Option<DefId> {
        self.by_path.get(path).copied()
    }

    pub fn get(&self, id: DefId) -> &Def {
        &self.defs[id.0 as usize]
    }

    pub fn all_defs(&self) -> &[Def] {
        &self.defs
    }

    pub fn len(&self) -> usize {
        self.defs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.defs.is_empty()
    }
}
