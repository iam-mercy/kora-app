#!/usr/bin/env python3
"""Verify that docs/error-codes.md matches the #[contracterror] enums in the tree.

The document marks each table it owns with a comment of the form

    <!-- check-error-codes: enum=ContractError file=path/to/file.rs -->
    <!-- check-error-codes: seed-messages -->

The script parses those tables, parses the corresponding Rust enums (and the
`initialize_error_messages` literals), and fails when the two disagree.

Run from the repository root:

    python3 scripts/check_error_codes.py
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DOC = ROOT / "docs" / "error-codes.md"
MAIN_LIB = "stellar-contracts/src/lib.rs"

ENUM_ANCHOR = re.compile(r"<!--\s*check-error-codes:\s*enum=(\w+)\s+file=(\S+?)\s*-->")
SEED_ANCHOR = re.compile(r"<!--\s*check-error-codes:\s*seed-messages\s*-->")
VARIANT_LINE = re.compile(r"^\s*([A-Za-z][A-Za-z0-9_]*)\s*=\s*(\d+)\s*,$", re.M)
SEEDED_MESSAGE = re.compile(
    r"ErrorMessage\s*\{\s*"
    r"code:\s*(\d+),\s*"
    r'language:\s*String::from_str\(&env,\s*"([^"]*)"\),\s*'
    r'message:\s*String::from_str\(&env,\s*"((?:[^"\\]|\\.)*)"\),',
    re.S,
)


def rows_in_range(lines: list[str], index: int, stop: int) -> list[list[str]]:
    """Return the rows of every markdown table in `lines[index:stop]`."""
    rows: list[list[str]] = []
    cursor = index + 1
    while cursor < stop:
        line = lines[cursor].strip()
        if not line.startswith("|"):
            cursor += 1
            continue
        cells = [cell.strip() for cell in line.strip("|").split("|")]
        if not all(set(cell) <= set("-: ") for cell in cells):
            rows.append(cells)
        cursor += 1
    return rows


def documented(path: Path) -> tuple[dict[str, list[tuple[int, str]]], dict[tuple[int, str], str]]:
    """Parse every anchored table range out of the document."""
    lines = path.read_text(encoding="utf-8").splitlines()
    anchors = [
        index
        for index, line in enumerate(lines)
        if ENUM_ANCHOR.match(line.strip()) or SEED_ANCHOR.match(line.strip())
    ]

    enums: dict[str, list[tuple[int, str]]] = {}
    seeds: dict[tuple[int, str], str] = {}

    for position, index in enumerate(anchors):
        stop = anchors[position + 1] if position + 1 < len(anchors) else len(lines)
        rows = rows_in_range(lines, index, stop)

        enum_match = ENUM_ANCHOR.match(lines[index].strip())
        if enum_match:
            key = f"{enum_match.group(2)}::{enum_match.group(1)}"
            pairs: list[tuple[int, str]] = []
            for row in rows:
                if len(row) < 2 or not row[0].isdigit() or not row[1].startswith("`"):
                    continue
                pairs.append((int(row[0]), row[1].strip("`")))
            enums[key] = pairs
            continue

        for row in rows:
            if len(row) >= 3 and row[0].isdigit():
                seeds[(int(row[0]), row[1].strip("`"))] = row[2]

    return enums, seeds


def seeded_messages() -> dict[tuple[int, str], str]:
    """Parse the literals passed to batch_set_error_messages by initialize_*."""
    source = (ROOT / MAIN_LIB).read_text(encoding="utf-8")
    start = source.index("fn initialize_error_messages")
    end = source.index("\n    /// ", start + 1)
    return {
        (int(code), language): message
        for code, language, message in SEEDED_MESSAGE.findall(source[start:end])
    }


def enum_in_file(path: Path, name: str) -> dict[int, str]:
    """Map discriminant -> variant for `pub enum <name>` in `path`."""
    source = path.read_text(encoding="utf-8")
    match = re.search(rf"pub enum {re.escape(name)} \{{(.*?)\n\}}", source, re.S)
    if not match:
        raise SystemExit(f"!! {path.relative_to(ROOT)}: no `pub enum {name}` found")
    return {int(code): variant for variant, code in VARIANT_LINE.findall(match.group(1))}


def never_raised(rel: str, name: str, codes: dict[int, str]) -> list[str]:
    """Variants of `name` that the declaring file never references."""
    source = (ROOT / rel).read_text(encoding="utf-8")
    body = re.search(rf"pub enum {re.escape(name)} \{{.*?\n\}}", source, re.S).group(0)
    return sorted(
        variant
        for variant in codes.values()
        if f"{name}::{variant}" not in source.replace(body, "")
    )


def main() -> int:
    enums, seeds = documented(DOC)
    problems: list[str] = []
    warnings: list[str] = []

    for key, pairs in sorted(enums.items()):
        rel, name = key.split("::")
        actual = enum_in_file(ROOT / rel, name)

        for code, variant in pairs:
            if code not in actual:
                problems.append(f"{rel}::{name}: {DOC.name} documents code {code} as {variant}, the enum does not declare that code")
            elif actual[code] != variant:
                problems.append(
                    f"{rel}::{name}: code {code} is {actual[code]} in the enum but {variant} in {DOC.name}"
                )
        for code in sorted(set(actual) - {code for code, _ in pairs}):
            problems.append(f"{rel}::{name}: the enum declares {actual[code]} = {code}, {DOC.name} does not list it")

        missing = never_raised(rel, name, actual)
        if missing:
            warnings.append(f"{rel}::{name}: declared but never raised: {', '.join(missing)}")
        print(f"{rel}::{name}: {len(actual)} codes in the enum, {len({code for code, _ in pairs})} documented")

    actual_seeds = seeded_messages()
    for key in sorted(set(actual_seeds) | set(seeds)):
        if key not in seeds:
            problems.append(f"seeded message {key} is in {MAIN_LIB} but not in {DOC.name}")
        elif key not in actual_seeds:
            problems.append(f"seeded message {key} is in {DOC.name} but not in {MAIN_LIB}")
        elif actual_seeds[key] != seeds[key]:
            problems.append(
                f"seeded message {key}: {MAIN_LIB} says {actual_seeds[key]!r}, "
                f"{DOC.name} says {seeds[key]!r}"
            )
    print(f"initialize_error_messages: {len(actual_seeds)} literals, {len(seeds)} rows in the document")

    for warning in warnings:
        print(f"warning: {warning}")

    if problems:
        print()
        for problem in problems:
            print(f"error: {problem}")
        print(f"\n{len(problems)} mismatch(es) between the enums and {DOC.name}")
        return 1

    print("\nAll error code tables match the enums.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
