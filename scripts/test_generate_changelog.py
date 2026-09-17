import importlib.util
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

spec = importlib.util.spec_from_file_location(
    "generate_changelog", Path(__file__).with_name("generate_changelog.py")
)
generator = importlib.util.module_from_spec(spec)
spec.loader.exec_module(generator)


class ChangelogTests(unittest.TestCase):
    def test_untagged_version_is_unreleased_and_tagged_version_keeps_its_date(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            entries = root / "releases/changelog"
            entries.mkdir(parents=True)
            for version in ["v1.8.1", "v1.9.0"]:
                (entries / f"{version}.md").write_text("### Added\n\n- Example.\n")
            with patch.object(generator, "__file__", str(root / "scripts/generate_changelog.py")), patch.object(
                generator, "try_git_tag_date",
                side_effect=lambda _, tag: "2026-02-10" if tag == "v1.8.1" else None,
            ):
                self.assertEqual(generator.main([]), 0)
                first = (root / "CHANGELOG.md").read_text()
                self.assertIn("## [v1.9.0] - Unreleased", first)
                self.assertIn("## [v1.8.1] - 2026-02-10", first)
                self.assertLess(first.index("## [v1.9.0]"), first.index("## [v1.8.1]"))
                self.assertEqual(generator.main([]), 0)
                self.assertEqual((root / "CHANGELOG.md").read_text(), first)

    def test_explicit_date_overrides_tag_and_release_check_detects_stale_output(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            entries = root / "releases/changelog"
            entries.mkdir(parents=True)
            entry = entries / "v1.9.0.md"
            entry.write_text("Release date: 2026-09-17\n\n### Added\n\n- Example.\n")
            with patch.object(generator, "__file__", str(root / "scripts/generate_changelog.py")), patch.object(
                generator, "try_git_tag_date", return_value="2026-09-18"
            ) as tag_date:
                self.assertEqual(generator.main([]), 0)
                output = root / "CHANGELOG.md"
                original = output.read_text()
                self.assertIn("## [v1.9.0] - 2026-09-17", original)
                self.assertNotIn("Release date:", original)
                tag_date.assert_not_called()
                self.assertEqual(generator.main(["--check-release", "v1.9.0"]), 0)
                entry.write_text(entry.read_text().replace("2026-09-17", "2026-09-19"))
                with self.assertRaisesRegex(ValueError, "out of date"):
                    generator.main(["--check-release", "v1.9.0"])
                self.assertEqual(output.read_text(), original)
                generator.main([])
                self.assertEqual(generator.main(["--check-release", "v1.9.0"]), 0)
                with self.assertRaisesRegex(ValueError, "not found"):
                    generator.main(["--check-release", "v2.0.0"])

    def test_unconfirmed_release_is_rejected_even_with_a_tag(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            entries = root / "releases/changelog"
            entries.mkdir(parents=True)
            with patch.object(generator, "__file__", str(root / "scripts/generate_changelog.py")), patch.object(
                generator, "try_git_tag_date", return_value="2026-09-17"
            ):
                for metadata in ["", "Release date: Unreleased\n\n"]:
                    (entries / "v1.9.0.md").write_text(metadata + "### Added\n\n- Example.\n")
                    generator.main([])
                    if metadata:
                        self.assertIn("## [v1.9.0] - Unreleased", (root / "CHANGELOG.md").read_text())
                    with self.assertRaisesRegex(ValueError, "confirm Release date"):
                        generator.main(["--check-release", "v1.9.0"])

    def test_invalid_dates_are_rejected(self):
        for value in ["", "2026-02-30", "2026-9-17", "tomorrow", "20260917"]:
            with self.subTest(value=value), self.assertRaises(ValueError):
                generator.read_release_date(f"Release date: {value}\n\n### Added\n")


if __name__ == "__main__":
    unittest.main()
