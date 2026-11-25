#!/usr/bin/env python3
"""Verify docs/error-codes.md matches the contract error enums.

The document is the reference clients use to map a numeric contract error
back to a variant. A renumbered enum that the document does not follow is a
silent breaking change, so this script parses both sides and compares them.

Input paths are injectable so the checker can be exercised against fixtures
independently of the live repository. Defaults preserve the historical
behaviour (repository root relative to this script).
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DOC = ROOT / "docs" / "error-codes.md"
SOURCES = [
    ROOT / "stellar-contracts" / "src" / "lib.rs",
    ROOT / "stellar-contracts" / "contracts" / "pet-transfer-adoption" / "src" / "lib.rs",
]

# `#[contracterror]` enums look like:
#   #[contracterror]
#   #[repr(u32)]
#   pub enum PetTransferError {
#       NotOwner = 1,
#       ...
#   }
ENUM_RE = re.compile(r"#\[contracterror\][\s\S]*?pub enum (\w+)\s*\{([\s\S]*?)\}")
VARIANT_RE = re.compile(r"^\s*(\w+)\s*=\s*(\d+)\s*,", re.MULTILINE)

# Document rows look like: `| 1 | `NotOwner` | ... |`
ROW_RE = re.compile(r"^\|\s*(\d+)\s*\|\s*`?(\w+)`?\s*\|", re.MULTILINE)
# Each enum table is preceded by: <!-- check-error-codes: enum=Name file=path -->
ANCHOR_RE = re.compile(r"<!--\s*check-error-codes:\s*enum=(\w+)\s+file=(\S+)\s*-->")
SECTION_END_RE = re.compile(r"^#{1,3}\s|<!--\s*check-error-codes:", re.MULTILINE)


def parse_sources(sources: list[Path]) -> dict[tuple[str, str], list[tuple[int, str]]]:
    """Return {(enum_name, file): [(discriminant, variant), ...]} in source order.

    A sequence (not a dict) is kept so repeated discriminants stay visible.
    """
    enums: dict[tuple[str, str], list[tuple[int, str]]] = {}
    for source in sources:
        try:
            file = source.resolve().relative_to(ROOT).as_posix()
        except ValueError:
            file = source.as_posix()
        text = source.read_text(encoding="utf-8")
        for name, body in ENUM_RE.findall(text):
            enums[(name, file)] = [
                (int(code), variant) for variant, code in VARIANT_RE.findall(body)
            ]
    return enums


def parse_document(doc: Path) -> dict[tuple[str, str], list[tuple[int, str]]]:
    """Return {(enum_name, file): [(discriminant, variant), ...]} per anchored table."""
    text = doc.read_text(encoding="utf-8")
    tables: dict[tuple[str, str], list[tuple[int, str]]] = {}
    for match in ANCHOR_RE.finditer(text):
        end = SECTION_END_RE.search(text, match.end())
        section = text[match.end() : end.start() if end else len(text)]
        tables[(match.group(1), match.group(2))] = [
            (int(code), variant) for code, variant in ROW_RE.findall(section)
        ]
    return tables


def find_duplicates(pairs: list[tuple[int, str]]) -> list[str]:
    """Describe every repeated discriminant or variant name in a sequence."""
    problems: list[str] = []
    for index, label in ((0, "discriminant"), (1, "variant")):
        seen: set = set()
        reported: set = set()
        for pair in pairs:
            value = pair[index]
            if value in seen and value not in reported:
                reported.add(value)
                problems.append(f"{label} {value} appears more than once")
            seen.add(value)
    return problems


def _same_file(anchor_file: str, source_file: str) -> bool:
    return anchor_file == source_file or source_file.endswith("/" + anchor_file)


def check(doc: Path = DOC, sources: list[Path] | None = None) -> list[str]:
    """Return a list of human-readable problems; empty means the document is valid."""
    if sources is None:
        sources = SOURCES

    problems: list[str] = []
    enums = parse_sources(sources)
    tables = parse_document(doc)

    if not enums:
        problems.append("no #[contracterror] enums found in the contract sources")
        return problems

    for (name, file), pairs in sorted(enums.items()):
        for problem in find_duplicates(pairs):
            problems.append(f"{name} in {file}: {problem} in the contract source")

        key = next(
            (k for k in tables if k[0] == name and _same_file(k[1], file)), None
        )
        if key is None:
            problems.append(f"{doc.name} has no anchor for {name} ({file})")
            continue

        rows = tables[key]
        for problem in find_duplicates(rows):
            problems.append(f"{name} table in {doc.name}: {problem}")

        documented = dict(rows)
        for code, variant in sorted(set(pairs)):
            documented_variant = documented.get(code)
            if documented_variant is None:
                problems.append(f"{name}::{variant} = {code} is missing from {doc.name}")
            elif documented_variant != variant:
                problems.append(
                    f"{name} discriminant {code} is {variant} in the contract "
                    f"but {documented_variant} in {doc.name}"
                )

    return problems


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--doc",
        type=Path,
        default=DOC,
        help="path to the error code document (default: docs/error-codes.md)",
    )
    parser.add_argument(
        "--source",
        type=Path,
        action="append",
        dest="sources",
        help="contract source file to scan (repeatable; default: repository sources)",
    )
    args = parser.parse_args(argv)

    problems = check(args.doc, args.sources)
    if problems:
        for problem in problems:
            print(f"error: {problem}", file=sys.stderr)
        return 1

    print("error code documentation matches the contract enums")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
