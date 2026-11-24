from pathlib import Path
from tempfile import TemporaryDirectory
import unittest

from scripts.check_error_codes import check


class MalformedErrorCodeRows(unittest.TestCase):
    def setUp(self):
        temp = TemporaryDirectory()
        self.addCleanup(temp.cleanup)
        root = Path(temp.name)
        self.source = root / "lib.rs"
        self.source.write_text(
            "#[contracterror]\npub enum ContractError {\n    ExampleError = 1,\n}\n",
            encoding="utf-8",
        )
        self.doc = root / "errors.md"

    def test_valid_rows_and_unrelated_tables(self):
        self.doc.write_text(
            "| Code | Language | Message |\n"
            "| ---- | -------- | ------- |\n"
            "| 1 | en | Example message |\n\n"
            "| Code | Variant | Meaning |\n"
            "| ---: | ------- | ------- |\n"
            "| 1 | `ExampleError` | A \\| B |\n",
            encoding="utf-8",
        )
        self.assertEqual(check(self.doc, [self.source]), [])

    def test_malformed_row_diagnostics(self):
        for row, diagnostic in [
            ("| nope | ExampleError | Meaning |", "invalid numeric error code 'nope'"),
            ("| 1 |  | Meaning |", "empty error-code cells: Variant"),
            ("| 1 | ExampleError |", "expected 3 cells in error-code row, found 2"),
            ("| 1 |", "expected 3 cells in error-code row, found 1"),
        ]:
            with self.subTest(row=row):
                self.doc.write_text(
                    "| Code | Variant | Meaning |\n"
                    "| ---: | ------- | ------- |\n"
                    "| 1 | ExampleError | Valid entry |\n"
                    f"{row}\n",
                    encoding="utf-8",
                )
                problems = check(self.doc, [self.source])
                self.assertEqual(len(problems), 1)
                self.assertTrue(problems[0].startswith("errors.md:4: "))
                self.assertIn(diagnostic, problems[0])

    def test_quick_index_validates_second_pair(self):
        self.doc.write_text(
            "| Code | Variant | Code | Variant |\n"
            "| ---: | ------- | ---: | ------- |\n"
            "| 1 | ExampleError | nope | AnotherError |\n",
            encoding="utf-8",
        )
        self.assertEqual(
            check(self.doc, [self.source]),
            ["errors.md:3: invalid numeric error code 'nope'"],
        )
