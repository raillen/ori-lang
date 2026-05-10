#!/usr/bin/env python3
"""Check repository-local paths referenced by Markdown docs.

This is intentionally conservative:
- it checks repository paths in backticks and Markdown links;
- it skips generated outputs, vendor trees, and historical raw archives;
- it skips historical decision/report files that intentionally preserve old
  paths for audit context;
- it does not try to validate URLs.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]

SCAN_ROOTS = [
    Path("README.md"),
    Path("docs"),
    Path("packages"),
    Path("runtime"),
    Path("stdlib"),
    Path("tests"),
    Path("tools"),
    Path("reports"),
]

PATH_PATTERN = re.compile(r"`([^`]+)`|\[[^\]]+\]\(([^)]+)\)")

PATH_STARTS = (
    "docs/",
    "language/",
    "packages/",
    "runtime/",
    "stdlib/",
    "tests/",
    "tools/",
    "reports/",
    "compiler/",
    ".github/",
    "examples/",
    "installer/",
)

IGNORED_PREFIXES = (
    "reports/perf/",
    "reports/suites/",
    "docs/internal/reports/release/artifacts/",
    ".selfhost-artifacts/",
    "wiki_repo/",
    "runtime/sdk/",
    "packages/borealis/native/raylib/windows-x64/raylib.dll",
    "packages/borealis/native/raylib/linux-x64/libraylib.so",
    "packages/borealis/native/raylib/macos-arm64/libraylib.dylib",
    "packages/borealis/native/raylib/lib",
)

IGNORED_DOC_PREFIXES = (
    "docs/internal/archive/",
    "docs/internal/reports/",
    "docs/internal/decisions/language/",
    "reports/",
)

IGNORED_DOC_PATHS = {
    "docs/internal/planning/cascade-v1.md",
    "docs/internal/planning/cascade-v2.md",
    "docs/internal/planning/lsp-1.0-roadmap.md",
    "docs/internal/planning/selfhosted-checklist-v1.md",
    "docs/internal/planning/selfhosted-roadmap-v1.md",
    "docs/internal/planning/v1-completeness-discussion.md",
    "docs/internal/planning/v1-implementation-plan.md",
    "packages/borealis/borealis-redesign-v2.md",
}


def should_scan_doc(rel_file: str) -> bool:
    if rel_file in IGNORED_DOC_PATHS:
        return False
    if rel_file.startswith(IGNORED_DOC_PREFIXES):
        return False
    if "/decisions/" in rel_file:
        return False
    return True


def read_text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        return path.read_text(encoding="latin-1")


def iter_doc_files() -> list[Path]:
    files: list[Path] = []
    for base in SCAN_ROOTS:
        path = ROOT / base
        if path.is_file():
            files.append(path)
            continue
        if not path.exists():
            continue
        files.extend(path.rglob("*.md"))
        files.extend(path.rglob("*.txt"))
    return sorted(files)


def normalize_candidate(raw: str) -> str | None:
    value = raw.strip()
    if not value:
        return None
    if value.startswith(("http://", "https://", "#", "mailto:")):
        return None
    value = value.split("#", 1)[0].strip().strip(".,;:")
    value = value.replace("\\", "/")
    if " " in value or value.startswith(("./", "../")):
        return None
    if any(char in value for char in "*{}<>:"):
        return None
    if "/build/" in value:
        return None
    if value.endswith((".exe", ".dll", ".so", ".dylib", ".a")):
        return None
    if value.endswith("/"):
        value = value[:-1]
    if not value.startswith(PATH_STARTS):
        return None
    if value.startswith(IGNORED_PREFIXES):
        return None
    return value


def main() -> int:
    missing: list[tuple[str, int, str]] = []
    seen: set[tuple[str, int, str]] = set()

    for file_path in iter_doc_files():
        rel_file = file_path.relative_to(ROOT).as_posix()
        if "node_modules/" in rel_file:
            continue
        if rel_file.startswith("docs/internal/reports/raw/"):
            continue
        if not should_scan_doc(rel_file):
            continue

        for line_number, line in enumerate(read_text(file_path).splitlines(), 1):
            for match in PATH_PATTERN.finditer(line):
                raw = (match.group(1) or match.group(2) or "").strip()
                candidate = normalize_candidate(raw)
                if candidate is None:
                    continue
                key = (rel_file, line_number, candidate)
                if key in seen:
                    continue
                seen.add(key)
                if not (ROOT / candidate).exists():
                    missing.append(key)

    if missing:
        for rel_file, line_number, candidate in missing:
            print(f"{rel_file}:{line_number}: missing {candidate}")
        print(f"docs path check failed: {len(missing)} missing path(s)")
        return 1

    print("docs path check ok")
    return 0


if __name__ == "__main__":
    sys.exit(main())
