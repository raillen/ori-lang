use super::*;

impl<'a> Checker<'a> {
    pub(super) fn check_match_exhaustiveness(
        &mut self,
        scr_ty: &Ty,
        cases: &[ori_ast::stmt::MatchCase],
        span: ori_diagnostics::Span,
    ) {
        if scr_ty.is_error() || scr_ty.contains_infer() {
            return;
        }

        if cases
            .iter()
            .any(|case| self.case_is_unguarded_catch_all(case, scr_ty))
        {
            return;
        }

        match scr_ty {
            Ty::Bool => {
                let mut seen_true = false;
                let mut seen_false = false;
                for case in cases {
                    let ori_ast::stmt::MatchCase::Pattern {
                        pattern,
                        guard: None,
                        ..
                    } = case
                    else {
                        continue;
                    };
                    if let Pattern::Literal(expr) = pattern {
                        match expr.as_ref() {
                            Expr::BoolLit(true, _) => seen_true = true,
                            Expr::BoolLit(false, _) => seen_false = true,
                            _ => {}
                        }
                    }
                }
                let mut missing = Vec::new();
                if !seen_true {
                    missing.push("true".to_string());
                }
                if !seen_false {
                    missing.push("false".to_string());
                }
                self.emit_match_non_exhaustive(span, missing);
            }
            Ty::Optional(_) => {
                let mut seen_some = false;
                let mut seen_none = false;
                for case in cases {
                    let ori_ast::stmt::MatchCase::Pattern {
                        pattern,
                        guard: None,
                        ..
                    } = case
                    else {
                        continue;
                    };
                    match pattern {
                        Pattern::Some(_, _) => seen_some = true,
                        Pattern::None(_) => seen_none = true,
                        _ => {}
                    }
                }
                let mut missing = Vec::new();
                if !seen_some {
                    missing.push("some(...)".to_string());
                }
                if !seen_none {
                    missing.push("none".to_string());
                }
                self.emit_match_non_exhaustive(span, missing);
            }
            Ty::Result(_, _) => {
                let mut seen_success = false;
                let mut seen_error = false;
                for case in cases {
                    let ori_ast::stmt::MatchCase::Pattern {
                        pattern,
                        guard: None,
                        ..
                    } = case
                    else {
                        continue;
                    };
                    match pattern {
                        Pattern::Success(_, _) => seen_success = true,
                        Pattern::Error(_, _) => seen_error = true,
                        _ => {}
                    }
                }
                let mut missing = Vec::new();
                if !seen_success {
                    missing.push("success(...)".to_string());
                }
                if !seen_error {
                    missing.push("error(...)".to_string());
                }
                self.emit_match_non_exhaustive(span, missing);
            }
            Ty::Named(def_id, _) if self.def_map.get(*def_id).kind == DefKind::Enum => {
                let Some(enum_sig) = self.enum_sig(*def_id) else {
                    return;
                };
                let mut covered = HashSet::new();
                for case in cases {
                    let ori_ast::stmt::MatchCase::Pattern {
                        pattern,
                        guard: None,
                        ..
                    } = case
                    else {
                        continue;
                    };
                    if let Some(name) = self.covered_enum_variant(pattern, enum_sig) {
                        covered.insert(name);
                    }
                }
                let missing: Vec<String> = enum_sig
                    .variants
                    .iter()
                    .filter(|variant| !covered.contains(&variant.name))
                    .map(|variant| variant.name.to_string())
                    .collect();
                self.emit_match_non_exhaustive(span, missing);
            }
            _ => {}
        }
    }

    fn case_is_unguarded_catch_all(&self, case: &ori_ast::stmt::MatchCase, scr_ty: &Ty) -> bool {
        match case {
            ori_ast::stmt::MatchCase::Else { .. } => true,
            ori_ast::stmt::MatchCase::Pattern {
                pattern,
                guard: None,
                ..
            } => self.pattern_is_catch_all(pattern, scr_ty),
            ori_ast::stmt::MatchCase::Pattern { .. } => false,
        }
    }

    fn pattern_is_catch_all(&self, pattern: &Pattern, scr_ty: &Ty) -> bool {
        match pattern {
            Pattern::Wildcard(_) => true,
            Pattern::Binding(name) => {
                if let Ty::Named(def_id, _) = scr_ty {
                    if let Some(enum_sig) = self.enum_sig(*def_id) {
                        return !enum_sig
                            .variants
                            .iter()
                            .any(|variant| variant.name == name.text);
                    }
                }
                true
            }
            _ => false,
        }
    }

    fn covered_enum_variant(&self, pattern: &Pattern, enum_sig: &EnumSig) -> Option<SmolStr> {
        match pattern {
            Pattern::VariantUnit { name, .. } | Pattern::Binding(name) => {
                let variant = enum_sig
                    .variants
                    .iter()
                    .find(|variant| variant.name == name.text)?;
                if variant.fields.is_empty() {
                    Some(name.text.clone())
                } else {
                    None
                }
            }
            Pattern::VariantNamed { name, fields, .. } => {
                let variant = enum_sig
                    .variants
                    .iter()
                    .find(|variant| variant.name == name.text)?;
                if fields.len() != variant.fields.len() {
                    return None;
                }

                let mut seen = HashSet::new();
                for field in fields {
                    if !seen.insert(field.name.text.clone()) {
                        return None;
                    }
                    if !variant
                        .fields
                        .iter()
                        .any(|(field_name, _)| field_name == &field.name.text)
                    {
                        return None;
                    }
                }
                Some(name.text.clone())
            }
            _ => None,
        }
    }

    fn emit_match_non_exhaustive(&mut self, span: ori_diagnostics::Span, missing: Vec<String>) {
        if missing.is_empty() {
            return;
        }
        self.sink.emit(
            Diagnostic::error(
                "match.non_exhaustive",
                format!("match is not exhaustive; missing {}", missing.join(", ")),
            )
            .with_label(Label::primary(self.file_id, span, "match checked here"))
            .with_action("add the missing cases or a `case else` arm"),
        );
    }
}
