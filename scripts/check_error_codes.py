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

# Document rows look like: `| 1 | NotOwner | ... |`
ROW_RE = re.compile(r"^\|\s*(\d+)\s*\|\s*(\w+)\s*\|", re.MULTILINE)


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


def parse_document(doc: Path) -> dict[int, str]:
    """Return {discriminant: variant} for the rows of the document table."""
    text = doc.read_text(encoding="utf-8")
    return {int(discriminant): variant for discriminant, variant in ROW_RE.findall(text)}


def check(doc: Path = DOC, sources: list[Path] | None = None) -> list[str]:
    """Return a list of human-readable problems; empty means the document is valid."""
    if sources is None:
        sources = SOURCES

    problems: list[str] = []
    enums = parse_sources(sources)
    documented = parse_document(doc)

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
