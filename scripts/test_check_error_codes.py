import tempfile
import unittest
from pathlib import Path

import check_error_codes as c

ENUM_SRC = "#[contracterror]\n#[repr(u32)]\npub enum E {\n    A = 1,\n}\n"


def seed_src(rows: str, extra: str = "") -> str:
    return (
        ENUM_SRC
        + "/// doc\npub fn initialize_error_messages(env: Env) {\n"
        + f"    let defaults = [\n{rows}\n    ];\n    // }} comment brace\n    let s = \"}}\";\n}}\n"
        + extra
    )


def doc_text(enum_tables: str = "", seeds: str = "") -> str:
    out = enum_tables or (
        "<!-- check-error-codes: enum=E file=lib.rs -->\n\n| Code | Variant |\n|--|--|\n| 1 | `A` |\n"
    )
    if seeds:
        out += "\n<!-- check-error-codes: seed-messages -->\n\n| Code | Language | Msg |\n|--|--|--|\n" + seeds
    return out


class CheckTest(unittest.TestCase):
    def run_check(self, src: str, doc: str) -> list[str]:
        with tempfile.TemporaryDirectory() as d:
            sp, dp = Path(d) / "lib.rs", Path(d) / "doc.md"
            sp.write_text(src)
            dp.write_text(doc)
            return c.check(dp, [sp], sp)

    def test_valid(self):
        seeds = "| 1 | `en` | Hi |\n| 1 | `es` | Hi |\n"
        self.assertEqual(self.run_check(seed_src('        (1, "Hi"),'), doc_text(seeds=seeds)), [])

    def test_duplicate_enum_anchor_with_conflicting_first_table(self):
        tables = (
            "<!-- check-error-codes: enum=E file=lib.rs -->\n\n| 1 | `Wrong` |\n\n"
            "<!-- check-error-codes: enum=E file=lib.rs -->\n\n| 1 | `A` |\n"
        )
        problems = self.run_check(ENUM_SRC, doc_text(tables))
        self.assertTrue(any("duplicate enum anchor" in p and "line 1" in p and ":5:" in p for p in problems), problems)

    def test_duplicate_seed_keys_in_source_and_doc(self):
        seeds = "| 1 | `en` | A |\n| 1 | `en` | B |\n"
        problems = self.run_check(seed_src('        (1, "A"),\n        (1, "B"),'), doc_text(seeds=seeds))
        self.assertTrue(any("duplicate seeded message key (1, 'en')" in p and ".rs:" in p for p in problems))
        self.assertTrue(any("doc.md:" in p and "duplicate seeded message key" in p for p in problems))

    def test_escapes_decoded(self):
        seeds = '| 1 | `en` | say "hi"\\ |\n| 1 | `es` | say "hi"\\ |\n'
        src = seed_src(r'        (1, "say \"hi\"\\"),')
        self.assertEqual(self.run_check(src, doc_text(seeds=seeds)), [])

    def test_unsupported_escape_rejected(self):
        problems = self.run_check(seed_src(r'        (1, "bad \q"),'), doc_text(seeds=""))
        self.assertEqual(problems, [])  # no seed anchor in doc: seeds not checked
        seeds = "| 1 | `en` | x |\n"
        problems = self.run_check(seed_src(r'        (1, "bad \q"),'), doc_text(seeds=seeds))
        self.assertTrue(any("unsupported escape" in p for p in problems))

    def test_raw_string_rejected(self):
        problems = self.run_check(seed_src('        (1, r#"raw"#),'), doc_text(seeds="| 1 | `en` | raw |\n"))
        self.assertTrue(any("raw string" in p for p in problems))

    def test_function_boundary(self):
        text = seed_src('        (1, "A"), // (2, "ignored")\n        /* (3, "x") */', "/// next\npub fn other() { let d = [(9, \"Z\")]; }\n")
        body = c.function_body(text, "initialize_error_messages")[1]
        self.assertIn('(1, "A")', body)
        self.assertNotIn("(9", body)
        self.assertIn('let s = "}";', body)


if __name__ == "__main__":
    unittest.main()
