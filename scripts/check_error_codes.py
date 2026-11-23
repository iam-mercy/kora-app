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
    ROOT / "stellar-contracts" / "contracts" / "pet-transfer-adoption" / "src" / "escrow.rs",
    ROOT / "stellar-contracts" / "contracts" / "pet-transfer-adoption" / "src" / "vet_registry.rs",
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

ENUM_ANCHOR_RE = re.compile(r"<!--\s*check-error-codes:\s*enum=(\w+)\s+file=(\S+)\s*-->")
SEED_ANCHOR_RE = re.compile(r"<!--\s*check-error-codes:\s*seed-messages\s*-->")
SEED_FN = "initialize_error_messages"
SEED_LANGUAGES = ("en", "es")
SEED_TUPLE_RE = re.compile(r'\(\s*(\d+)\s*,\s*"((?:[^"\\]|\\[\s\S])*)"\s*\)')
RAW_SEED_RE = re.compile(r'\(\s*\d+\s*,\s*b?r#*"')
SIMPLE_ESCAPES = {"n": "\n", "r": "\r", "t": "\t", "\\": "\\", "0": "\0", "'": "'", '"': '"'}


class UnsupportedLiteral(ValueError):
    pass


def line_of(text: str, offset: int) -> int:
    return text.count("\n", 0, offset) + 1


def decode_rust_string(raw: str) -> str:
    """Decode the body of a normal Rust string literal (without the quotes)."""
    out: list[str] = []
    i = 0
    while i < len(raw):
        ch = raw[i]
        if ch != "\\":
            out.append(ch)
            i += 1
            continue
        i += 1
        esc = raw[i] if i < len(raw) else ""
        if esc in SIMPLE_ESCAPES:
            out.append(SIMPLE_ESCAPES[esc])
            i += 1
        elif esc == "x" and re.fullmatch(r"[0-7][0-9a-fA-F]", raw[i + 1 : i + 3]):
            out.append(chr(int(raw[i + 1 : i + 3], 16)))
            i += 3
        elif esc == "u" and (m := re.match(r"\{([0-9a-fA-F_]{1,13})\}", raw[i + 1 :])):
            out.append(chr(int(m.group(1).replace("_", ""), 16)))
            i += 1 + m.end()
        elif esc in ("\n", "\r"):
            # Line continuation: drop the newline and leading whitespace of the next line.
            i += 1
            while i < len(raw) and raw[i] in " \t\r\n":
                i += 1
        else:
            raise UnsupportedLiteral(f"unsupported escape sequence '\\{esc}'")
    return "".join(out)


def function_body(text: str, name: str) -> tuple[int, str] | None:
    """Return (offset, body) of the balanced `{...}` body of `fn name`."""
    m = re.search(rf"\bfn\s+{re.escape(name)}\b", text)
    if not m:
        return None
    i = text.find("{", m.end())
    if i < 0:
        return None
    start, depth, n = i, 0, len(text)
    while i < n:
        c = text[i]
        if text.startswith("//", i):
            j = text.find("\n", i)
            i = n if j < 0 else j
            continue
        if text.startswith("/*", i):
            j = text.find("*/", i + 2)
            i = n if j < 0 else j + 2
            continue
        if c == '"':
            i += 1
            while i < n and text[i] != '"':
                i += 2 if text[i] == "\\" else 1
        elif c == "'" and (cm := re.match(r"'(?:\\.[^']*|[^'\\])'", text[i:])):
            i += cm.end() - 1
        elif c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0:
                return start, text[start + 1 : i]
        i += 1
    return None


def parse_sources(sources: list[Path]) -> dict[tuple[Path, str], dict[int, str]]:
    """Return {(source, enum_name): {discriminant: variant}} for every error enum."""
    enums: dict[tuple[Path, str], dict[int, str]] = {}
    for source in sources:
        text = source.read_text(encoding="utf-8")
        for name, body in ENUM_RE.findall(text):
            enums[(source, name)] = {
                int(d): v for v, d in VARIANT_RE.findall(body)
            }
    return enums


def table_rows(lines: list[str], anchor_idx: int) -> list[tuple[int, list[str]]]:
    """Return (line_number, cells) for data rows following an anchor.

    Rows are collected until the next anchor or a `##`/`###` heading, so an enum
    table may be split under `####` sub-headings.
    """
    rows: list[tuple[int, list[str]]] = []
    for i in range(anchor_idx + 1, len(lines)):
        line = lines[i].strip()
        if line.startswith("<!--") or re.match(r"#{1,3}\s", line) or line == "---":
            break
        if line.startswith("|"):
            cells = [c.strip().replace("\\|", "|") for c in re.split(r"(?<!\\)\|", line)[1:-1]]
            if cells and cells[0].isdigit():
                rows.append((i + 1, cells))
    return rows


def parse_document(doc: Path, problems: list[str]):
    """Return ({(enum, file): (line, {code: variant})}, {(code, lang): msg} | None)."""
    lines = doc.read_text(encoding="utf-8").splitlines()
    tables: dict[tuple[str, str], tuple[int, dict[int, str]]] = {}
    seeds: dict[tuple[int, str], tuple[int, str]] | None = None
    seed_anchor_line = None
    for idx, line in enumerate(lines):
        if m := ENUM_ANCHOR_RE.search(line):
            key = (m.group(1), m.group(2))
            if key in tables:
                problems.append(
                    f"{doc.name}:{idx + 1}: duplicate enum anchor {key[0]} ({key[1]}); "
                    f"first anchor is at line {tables[key][0]}"
                )
                continue
            codes: dict[int, str] = {}
            for ln, cells in table_rows(lines, idx):
                code, variant = int(cells[0]), cells[1].strip("`")
                if code in codes:
                    problems.append(f"{doc.name}:{ln}: {key[0]} code {code} is listed twice")
                codes[code] = variant
            tables[key] = (idx + 1, codes)
        elif SEED_ANCHOR_RE.search(line):
            if seed_anchor_line is not None:
                problems.append(
                    f"{doc.name}:{idx + 1}: duplicate seed-messages anchor; "
                    f"first anchor is at line {seed_anchor_line}"
                )
                continue
            seed_anchor_line = idx + 1
            seeds = {}
            for ln, cells in table_rows(lines, idx):
                if len(cells) < 3:
                    continue
                key2 = (int(cells[0]), cells[1].strip("`"))
                if key2 in seeds:
                    problems.append(
                        f"{doc.name}:{ln}: duplicate seeded message key {key2}; "
                        f"first row is at line {seeds[key2][0]}"
                    )
                    continue
                seeds[key2] = (ln, cells[2])
    return tables, seeds


def seeded_messages(source: Path, problems: list[str]) -> dict[tuple[int, str], tuple[int, str]]:
    """Return {(code, lang): (line, message)} seeded by initialize_error_messages."""
    text = source.read_text(encoding="utf-8")
    found = function_body(text, SEED_FN)
    if found is None:
        problems.append(f"{source.name}: cannot find the body of {SEED_FN}")
        return {}
    offset, body = found
    # Blank out comments so commented-out rows are ignored (keeps offsets stable).
    body = re.sub(r"//[^\n]*|/\*[\s\S]*?\*/", lambda m: re.sub(r"[^\n]", " ", m.group(0)), body)
    for m in RAW_SEED_RE.finditer(body):
        problems.append(
            f"{source.name}:{line_of(text, offset + 1 + m.start())}: raw string literals "
            f"are not supported in seeded messages; use a normal string literal"
        )
    seeds: dict[tuple[int, str], tuple[int, str]] = {}
    for m in SEED_TUPLE_RE.finditer(body):
        line = line_of(text, offset + 1 + m.start())
        try:
            message = decode_rust_string(m.group(2))
        except UnsupportedLiteral as err:
            problems.append(f"{source.name}:{line}: seeded message {m.group(1)}: {err}")
            continue
        for lang in SEED_LANGUAGES:
            key = (int(m.group(1)), lang)
            if key in seeds:
                problems.append(
                    f"{source.name}:{line}: duplicate seeded message key {key}; "
                    f"first defined at line {seeds[key][0]}"
                )
                continue
            seeds[key] = (line, message)
    return seeds


def check(
    doc: Path = DOC,
    sources: list[Path] | None = None,
    seed_source: Path | None = None,
) -> list[str]:
    """Return a list of human-readable problems; empty means the document is valid."""
    if sources is None:
        sources = SOURCES
    if seed_source is None:
        seed_source = sources[0]

    problems: list[str] = []
    enums = parse_sources(sources)
    tables, doc_seeds = parse_document(doc, problems)

    if not enums:
        problems.append("no #[contracterror] enums found in the contract sources")
        return problems

    matched: set[tuple[Path, str]] = set()
    for (enum_name, file), (anchor_line, documented) in tables.items():
        hits = [k for k in enums if k[1] == enum_name and k[0].as_posix().endswith(file)]
        if not hits:
            problems.append(f"{doc.name}:{anchor_line}: no enum {enum_name} found in {file}")
            continue
        matched.add(hits[0])
        variants = enums[hits[0]]
        for code, variant in sorted(variants.items()):
            doc_variant = documented.get(code)
            if doc_variant is None:
                problems.append(f"{enum_name}::{variant} = {code} is missing from {doc.name}")
            elif doc_variant != variant:
                problems.append(
                    f"{enum_name} discriminant {code} is {variant} in the contract "
                    f"but {doc_variant} in {doc.name}"
                )
        for code in sorted(set(documented) - set(variants)):
            problems.append(f"{enum_name} code {code} is documented but not in the contract")
    for key in enums:
        if key not in matched:
            problems.append(f"enum {key[1]} in {key[0].name} has no anchor in {doc.name}")

    if doc_seeds is not None:
        src_seeds = seeded_messages(seed_source, problems)
        for key, (line, message) in sorted(src_seeds.items()):
            if key not in doc_seeds:
                problems.append(f"seeded message {key} is missing from {doc.name}")
            elif doc_seeds[key][1] != message:
                problems.append(
                    f"seeded message {key} is {message!r} in the contract "
                    f"but {doc_seeds[key][1]!r} in {doc.name}:{doc_seeds[key][0]}"
                )
        for key, (line, _) in sorted(doc_seeds.items()):
            if key not in src_seeds:
                problems.append(f"{doc.name}:{line}: seeded message {key} is not seeded by the contract")

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
