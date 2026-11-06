import subprocess
import unittest
from unittest.mock import patch

from ci_policy import classify, decide


def changes(*entries):
    records = []
    for status, path in entries:
        old_mode = "000000" if status == "A" else "100644"
        old_oid = ("0" if status == "A" else "a") * 40
        records.append(f":{old_mode} 100644 {old_oid} {'b' * 40} {status}\0{path}\0")
    return "".join(records).encode()


class PolicyTests(unittest.TestCase):
    def test_small_allowlist(self):
        self.assertEqual(classify(changes(("A", ".github/FUNDING.yml")))[0], "light")
        self.assertEqual(classify(changes(("M", "docs/WIKI.md")))[0], "light")

    def test_non_editorial_inputs_and_statuses(self):
        for path in (
            "README.md", "AGENTS.md", "SECURITY.md", "docs/OPERATIONS.md",
            "docs/releases/README.md", "docs/planning/roadmap.json",
            "docs/planning/GITHUB-ROADMAP.md", "cases/server-inventory.json",
            "src/process.rs", "adapters/neoforge/build.ps1", "Cargo.lock",
            "tests/contracts.rs", "tools/check-public-tree.ps1", ".github/CODEOWNERS",
            ".github/workflows/ci.yml", "new.md", "docs/wiki.md",
        ):
            with self.subTest(path=path):
                self.assertEqual(classify(changes(("M", path)))[0], "rust")
        for status in ("A", "D", "T", "R100", "C100", "U"):
            with self.subTest(status=status):
                self.assertEqual(classify(changes((status, "docs/WIKI.md")))[0], "rust")

    def test_mixed_rename_and_incomplete_diff(self):
        for raw in (
            changes(("M", "docs/WIKI.md"), ("M", "src/main.rs")),
            changes(("M", "docs/WIKI.md"), ("M", ".github/FUNDING.yml")),
            changes(("M", "docs/WIKI.md")).replace(b" 100644 ", b" 100755 "),
            changes(("M", "docs/WIKI.md")).replace(b"docs/WIKI.md", b"\xff"),
            changes(("D", "old.md"), ("A", "docs/WIKI.md")),
            changes(("M", "docs/WIKI.md"), ("M", "docs/WIKI.md")),
            b"", b"M\0", b"M\0docs/WIKI.md", b"M\0\xff\0",
        ):
            with self.subTest(raw=raw):
                self.assertEqual(classify(raw)[0], "rust")

    def test_manual_selection_and_revision_mismatch(self):
        sha = "a" * 40
        with patch("ci_policy.git", return_value=sha.encode()):
            self.assertEqual(decide("workflow_dispatch", sha, "", "", "true")[0], "matrix")
            self.assertEqual(decide("workflow_dispatch", sha, "", "", "false")[0], "rust")
            with self.assertRaises(ValueError):
                decide("workflow_dispatch", "b" * 40, "", "", "true")
            with self.assertRaises(ValueError):
                decide("workflow_dispatch", sha, "", "", "")

    def test_exact_pr_parents_both_diffs_and_git_failure(self):
        merge, base, head, ancestor = [letter * 40 for letter in "abcd"]
        refs = [merge.encode(), f"{merge} {base} {head}".encode(), ancestor.encode()]
        wiki = changes(("M", "docs/WIKI.md"))
        code = changes(("M", "src/main.rs"))
        with patch("ci_policy.git", side_effect=[*refs, wiki, wiki]):
            self.assertEqual(decide("pull_request", merge, base, head, "")[0], "light")
        funding = changes(("M", ".github/FUNDING.yml"))
        for diffs in ([wiki, code], [wiki, funding], [code],
                      [subprocess.CalledProcessError(128, "git")]):
            with patch("ci_policy.git", side_effect=[*refs, *diffs]):
                self.assertEqual(decide("pull_request", merge, base, head, "")[0], "rust")
        with patch("ci_policy.git", side_effect=[merge.encode(), head.encode()]):
            with self.assertRaises(ValueError):
                decide("pull_request", merge, base, head, "")


if __name__ == "__main__":
    unittest.main()
