use ori_diagnostics::{Diagnostic as OriDiagnostic, DiagnosticSink, Label, Severity, SourceCache};
use std::path::{Path, PathBuf};
use tower_lsp::lsp_types::{
    Diagnostic as LspDiagnostic, DiagnosticSeverity, NumberOrString, Position, Range,
};

use crate::utils::position;
use crate::utils::uri;

/// Convert Ori diagnostics for a specific file into LSP diagnostics.
pub fn diagnostics_for_path(
    cache: &SourceCache,
    diagnostics: &[OriDiagnostic],
    target: &Path,
) -> Vec<LspDiagnostic> {
    let target = uri::canonical_path(target);
    diagnostics
        .iter()
        .filter_map(|diagnostic| {
            let label = diagnostic
                .labels
                .iter()
                .find(|label| label_points_to_path(cache, label, &target));

            if !diagnostic.labels.is_empty() && label.is_none() {
                return None;
            }

            Some(LspDiagnostic {
                range: label
                    .map(|label| range_for_label(cache, label))
                    .unwrap_or_else(position::default_range),
                severity: Some(match diagnostic.severity {
                    Severity::Error => DiagnosticSeverity::ERROR,
                    Severity::Warning => DiagnosticSeverity::WARNING,
                }),
                code: Some(NumberOrString::String(diagnostic.code.to_string())),
                code_description: None,
                source: Some("ori".to_string()),
                message: diagnostic_message(diagnostic, label),
                related_information: None,
                tags: None,
                data: None,
            })
        })
        .collect()
}

/// Construct a file-level error diagnostic (e.g. when the file can't be read).
pub fn file_error_diagnostic(message: String) -> LspDiagnostic {
    LspDiagnostic {
        range: position::default_range(),
        severity: Some(DiagnosticSeverity::ERROR),
        code: Some(NumberOrString::String("lsp.file".to_string())),
        code_description: None,
        source: Some("ori".to_string()),
        message,
        related_information: None,
        tags: None,
        data: None,
    }
}

fn label_points_to_path(cache: &SourceCache, label: &Label, target: &Path) -> bool {
    cache
        .get(label.file_id)
        .map(|file| uri::canonical_path(&file.path) == target)
        .unwrap_or(false)
}

fn range_for_label(cache: &SourceCache, label: &Label) -> Range {
    let Some(file) = cache.get(label.file_id) else {
        return position::default_range();
    };
    let content_len = file.content.len() as u32;
    let start = label.span.start.min(content_len);
    let mut end = label.span.end.min(content_len);
    if end <= start && start < content_len {
        end = start + 1;
    }

    Range::new(
        position::position_for_byte_offset(&file.content, start as usize),
        position::position_for_byte_offset(&file.content, end as usize),
    )
}

fn diagnostic_message(diagnostic: &OriDiagnostic, label: Option<&Label>) -> String {
    let mut message = diagnostic.message.clone();
    if let Some(label) = label {
        if !label.message.is_empty() {
            message.push('\n');
            message.push_str(&label.message);
        }
    }
    if let Some(why) = &diagnostic.why {
        message.push_str("\nwhy: ");
        message.push_str(why);
    }
    if let Some(action) = &diagnostic.action {
        message.push_str("\naction: ");
        message.push_str(action);
    }
    for note in &diagnostic.notes {
        message.push_str("\nnote: ");
        message.push_str(note);
    }
    message
}
