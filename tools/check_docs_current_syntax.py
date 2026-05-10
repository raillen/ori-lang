#!/usr/bin/env python3
"""Check current-facing docs for stale Zenith syntax.

The check is intentionally small. It does not reject historical decisions or
explicit migration notes. It rejects stale syntax when a current user/reference
doc appears to teach it as normal syntax.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]

SCAN_ROOTS = [
    Path("docs/public"),
    Path("docs/reference"),
    Path("docs/spec/language"),
]

HISTORICAL_FILES = {
    Path("docs/spec/language/decision-conflict-audit.md"),
}

ALLOW_WORDS = (
    "historical",
    "legacy",
    "migration",
    "migrated",
    "deprecated",
    "superseded",
    "old ",
    "older",
    "not ",
    "do not",
    "instead",
)


CHECKS = [
    ("dyn", re.compile(r"(?<![A-Za-z0-9_])dyn(?:\s|<)"), "`dyn` is historical; use `any<Trait>`"),
    ("case-default", re.compile(r"case\s+default\b"), "`case default` is historical; use `case else`"),
    ("fmt-interpolation", re.compile(r'fmt\s+"'), '`fmt "..."` is historical; use `f"..."`'),
    ("assert-keyword", re.compile(r"(?<![A-Za-z0-9_])assert(?!ion|ive|s|ed|ing)(?![A-Za-z0-9_])"), "`assert` is historical; use `check`"),
    ("uint-alias", re.compile(r"(?<![A-Za-z0-9_])uint(?:8|16|32|64)(?![A-Za-z0-9_])"), "prefer `u8/u16/u32/u64`"),
]


def read_text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        return path.read_text(encoding="latin-1")


def iter_docs() -> list[Path]:
    files: list[Path] = []
    for root in SCAN_ROOTS:
        base = ROOT / root
        if not base.exists():
            continue
        files.extend(base.rglob("*.md"))
    return sorted(files)


def is_allowed_context(line: str) -> bool:
    low = line.lower()
    if any(word in low for word in ALLOW_WORDS):
        return True
    # Compatibility mapping tables may show old -> current spelling.
    if low.strip().startswith("|") and "|" in low[1:]:
        return True
    return False


def main() -> int:
    issues: list[str] = []

    for path in iter_docs():
        rel = path.relative_to(ROOT)
        if rel in HISTORICAL_FILES:
            continue
        text = read_text(path)
        for line_no, line in enumerate(text.splitlines(), 1):
            if is_allowed_context(line):
                continue
            for check_id, pattern, message in CHECKS:
                if pattern.search(line):
                    issues.append(f"{rel.as_posix()}:{line_no}: {check_id}: {message}: {line.strip()}")

    if issues:
        print("docs current syntax check failed")
        for issue in issues:
            print(issue)
        return 1

    print("docs current syntax check ok")
    return 0


if __name__ == "__main__":
    sys.exit(main())
