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

VARIANT_CELL_RE = re.compile(r"`([A-Za-z_][A-Za-z_0-9]*)`|([A-Za-z_][A-Za-z_0-9]*)")


def parse_sources(sources: list[Path]) -> dict[str, dict[int, str]]:
    """Return {enum_name: {discriminant: variant}} for every contract error enum."""
    enums: dict[str, dict[int, str]] = {}
    for source in sources:
        text = source.read_text(encoding="utf-8")
        for name, body in ENUM_RE.findall(text):
            variants: dict[int, str] = {}
            for variant, discriminant in VARIANT_RE.findall(body):
                variants[int(discriminant)] = variant
            enums[name] = variants
    return enums


def parse_document(doc: Path, problems: list[str]) -> dict[int, str]:
    """Read error-code tables and append diagnostics for malformed rows."""
    documented: dict[int, str] = {}
    columns: list[str] = []
    header_line = 0
    for line_number, line in enumerate(doc.read_text(encoding="utf-8").splitlines(), 1):
        line = line.strip()
        if not line.startswith("|"):
            columns = []
            continue
        row = [cell.strip() for cell in re.split(r"(?<!\\)\|", line[1:].removesuffix("|"))]
        if row[:2] == ["Code", "Variant"]:
            columns = row
            header_line = line_number
            continue
        if not columns:
            continue
        if line_number == header_line + 1 and all(
            re.fullmatch(r":?-+:?", cell) for cell in row
        ):
            continue

        location = f"{doc.name}:{line_number}"
        if len(row) != len(columns):
            problems.append(
                f"{location}: expected {len(columns)} cells in error-code row, found {len(row)}"
            )
            continue

        empty_columns = [column for column, cell in zip(columns, row) if not cell]
        if empty_columns:
            problems.append(f"{location}: empty error-code cells: {', '.join(empty_columns)}")
            continue

        pair_indices = (0, 2) if columns == ["Code", "Variant"] * 2 else (0,)
        for index in pair_indices:
            code, variant_cell = row[index:index + 2]
            numeric = re.fullmatch(r"[0-9]+", code)
            variant = VARIANT_CELL_RE.fullmatch(variant_cell)
            if not numeric:
                problems.append(f"{location}: invalid numeric error code {code!r}")
            if not variant:
                problems.append(f"{location}: invalid error variant {variant_cell!r}")
            # The second pair is a quick-index entry, validated for shape only.
            if index == 0 and numeric and variant:
                documented[int(code)] = variant.group(1) or variant.group(2)
    return documented


def check(doc: Path = DOC, sources: list[Path] | None = None) -> list[str]:
    """Return a list of human-readable problems; empty means the document is valid."""
    if sources is None:
        sources = SOURCES

    problems: list[str] = []
    enums = parse_sources(sources)
    documented = parse_document(doc, problems)

    if not enums:
        problems.append("no #[contracterror] enums found in the contract sources")
        return problems

    for enum_name, variants in enums.items():
        for discriminant, variant in sorted(variants.items()):
            documented_variant = documented.get(discriminant)
            if documented_variant is None:
                problems.append(
                    f"{enum_name}::{variant} = {discriminant} is missing from {doc.name}"
                )
            elif documented_variant != variant:
                problems.append(
                    f"{enum_name} discriminant {discriminant} is {variant} in the contract "
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
