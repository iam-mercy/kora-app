import tempfile
import unittest
from pathlib import Path

import check_error_codes as c

RS = "#[contracterror]\n#[repr(u32)]\npub enum E {\n%s}\n"
ANCHOR = "<!-- check-error-codes: enum=E file=lib.rs -->\n\n| Code | Variant |\n|--|--|\n"


def run(rs_body, doc_text):
    with tempfile.TemporaryDirectory() as d:
        d = Path(d)
        (d / "lib.rs").write_text(RS % rs_body)
        (d / "doc.md").write_text(doc_text)
        return c.check(d / "doc.md", [d / "lib.rs"])


class CheckErrorCodes(unittest.TestCase):
    def test_valid(self):
        self.assertEqual(run("    A = 1,\n    B = 2,\n", ANCHOR + "| 1 | `A` |\n| 2 | `B` |\n"), [])

    def test_source_collision(self):
        p = run("    A = 1,\n    B = 1,\n", ANCHOR + "| 1 | `A` |\n")
        self.assertTrue(any("discriminant 1 appears more than once" in x for x in p))

    def test_all_anchors_removed(self):
        p = run("    A = 1,\n", "# Doc\n\n| 1 | `A` |\n")
        self.assertTrue(any("no anchor for E" in x for x in p))

    def test_identical_duplicate_row(self):
        p = run("    A = 1,\n", ANCHOR + "| 1 | `A` |\n| 1 | `A` |\n")
        self.assertTrue(any("table" in x and "more than once" in x for x in p))

    def test_conflicting_duplicate_row(self):
        p = run("    A = 1,\n    B = 2,\n", ANCHOR + "| 1 | `A` |\n| 1 | `B` |\n| 2 | `B` |\n")
        self.assertTrue(any("discriminant 1 appears" in x for x in p))
        self.assertTrue(any("variant B appears" in x for x in p))


if __name__ == "__main__":
    unittest.main()
