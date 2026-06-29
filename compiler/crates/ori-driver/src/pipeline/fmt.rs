//! Source text formatter (`ori fmt`).
//!
//! Extracted from `pipeline.rs` as part of Etapa 8.3 monolith reduction.
//! Pure string processing — no dependencies on pipeline types or other
//! pipeline functions. The public entry point is re-exported from
//! `pipeline.rs` as `format_source_text` to preserve the public API.

pub fn format_source_text(source: &str) -> String {
    let mut indent = 0usize;
    let mut out = String::new();

    for raw_line in source.replace("\r\n", "\n").replace('\r', "\n").lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            out.push('\n');
            continue;
        }

        if should_dedent_before(line) {
            indent = indent.saturating_sub(1);
        }

        out.push_str(&"    ".repeat(indent));
        out.push_str(line);
        out.push('\n');

        if opens_block_after(line) {
            indent += 1;
        }
    }

    out
}

fn should_dedent_before(line: &str) -> bool {
    line == "end" || line == "else" || line.starts_with("else if ") || line.starts_with("case ")
}

fn opens_block_after(line: &str) -> bool {
    if is_comment_line(line) || line == "end" || line.starts_with("@") {
        return false;
    }

    line == "else"
        || line.starts_with("else if ")
        || line.starts_with("case ")
        || line == "loop"
        || line.starts_with("if ")
        || line.starts_with("while ")
        || line.starts_with("for ")
        || line.starts_with("repeat ")
        || line.starts_with("match ")
        || declaration_opens_block(line)
}

fn declaration_opens_block(line: &str) -> bool {
    let mut line = line;
    loop {
        let next = line
            .strip_prefix("public ")
            .or_else(|| line.strip_prefix("async "))
            .or_else(|| line.strip_prefix("mut "));
        let Some(next) = next else {
            break;
        };
        line = next;
    }
    line.starts_with("func ")
        || line.starts_with("struct ")
        || line.starts_with("enum ")
        || line.starts_with("trait ")
        || line.starts_with("implement ")
        || line.starts_with("extern ")
}

fn is_comment_line(line: &str) -> bool {
    line.starts_with("--") || line.starts_with("|--")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt_idempotent_on_simple_module() {
        let src = "namespace App\n\nfunc main()\n    io.println(\"hi\")\nend\n";
        let once = format_source_text(src);
        let twice = format_source_text(&once);
        assert_eq!(once, twice);
    }

    #[test]
    fn fmt_dedents_end_and_else() {
        let src = "namespace App\nfunc f()\nif x\nreturn 1\nelse\nreturn 2\nend\nend\n";
        let out = format_source_text(src);
        assert!(out.contains("    if x\n"));
        assert!(out.contains("    return 1\n"));
        assert!(out.contains("    else\n"));
        assert!(out.contains("    return 2\n"));
        assert!(out.contains("    end\n"));
        assert!(out.contains("func f()\n"));
    }
}
