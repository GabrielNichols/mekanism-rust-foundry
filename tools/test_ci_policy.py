import subprocess
import unittest
from unittest.mock import patch

from ci_policy import classify, decide, prose_only


def changes(*entries):
    records = []
    for status, path in entries:
        old_mode = "000000" if status == "A" else "100644"
        old_oid = ("0" if status == "A" else "a") * 40
        oid = ("c" if path == ".github/FUNDING.yml" else "b") * 40
        records.append(f":{old_mode} 100644 {old_oid} {oid} {status}\0{path}\0")
    return "".join(records).encode()


BLOBS = {
    "a" * 40: "Old prose.\n", "b" * 40: "Revised prose.\n",
    "c" * 40: "github: [GabrielNichols]\n",
}


class PolicyTests(unittest.TestCase):
    def check(self, raw, rust, metadata):
        plan = classify(raw, BLOBS.__getitem__)
        self.assertEqual((plan.rust, plan.metadata, plan.platforms), (rust, metadata, False))

    def test_funding_and_contributing_alone_and_together(self):
        funding = ("A", ".github/FUNDING.yml")
        prose = ("M", "CONTRIBUTING.md")
        for entries in ((funding,), (prose,), (funding, prose), (("M", "docs/WIKI.md"),)):
            with self.subTest(entries=entries):
                self.check(changes(*entries), False, "none")
        with self.assertRaises(ValueError):
            classify(changes(funding), lambda _: "patreon: example")

    def test_prose_respects_executable_markup(self):
        for old, new in (
            ("```sh\ncargo test\n```\n", "```sh\ncargo check\n```\n"),
            ("Use `cargo test`.\n", "Use `cargo check`.\n"),
            ("    cargo test\n", "    cargo check\n"),
            ("<script>\nold\n</script>\n", "<script>\nnew\n</script>\n"),
            ("# Heading\n", "# Heading\n[reference]: https://example.org\n"),
        ):
            with self.subTest(new=new):
                self.assertFalse(prose_only(old, new))
        self.assertTrue(prose_only(
            "Old text.\n\n```sh\ncargo test\n```\n",
            "New text.\n\n```sh\ncargo test\n```\n",
        ))

    def test_relevance_and_mixed_changes(self):
        self.check(changes(("M", "tests/contracts.rs")), True, "none")
        self.check(changes(("M", "Cargo.lock")), True, "none")
        self.check(changes(("A", "tools/test-github-triage.ps1")), False, "triage")
        self.check(changes(("M", "docs/planning/github-roadmap.json")), False, "catalog")
        self.check(changes(("M", "src/main.rs"), ("A", "tools/test-github-triage.ps1")), True, "triage")
        self.check(changes(("M", "docs/WIKI.md"), ("M", "src/main.rs")), True, "none")
        for path in (
            "README.md", "AGENTS.md", "SECURITY.md", "docs/OPERATIONS.md",
            "docs/releases/README.md", "docs/planning/roadmap.json",
            "docs/planning/GITHUB-ROADMAP.md", "cases/server-inventory.json",
            "adapters/neoforge/build.ps1", "tools/check-public-tree.ps1",
            ".github/CODEOWNERS", ".github/workflows/ci.yml", "new.md", "docs/wiki.md",
        ):
            with self.subTest(path=path):
                self.check(changes(("M", path)), True, "all")

    def test_ambiguous_and_incomplete_diffs(self):
        for raw in (
            changes(("D", "old.md"), ("A", "docs/WIKI.md")),
            changes(("M", "docs/WIKI.md"), ("M", "docs/WIKI.md")),
            changes(("M", "docs/WIKI.md")).replace(b" 100644 ", b" 100755 "),
            changes(("M", "docs/WIKI.md")).replace(b"docs/WIKI.md", b"\xff"),
            b"", b"M\0", b"M\0docs/WIKI.md",
        ):
            with self.subTest(raw=raw):
                self.check(raw, True, "all")

    def test_manual_selection_and_revision_mismatch(self):
        sha = "a" * 40
        with patch("ci_policy.git", return_value=sha.encode()):
            self.assertTrue(decide("workflow_dispatch", sha, "", "", "true").platforms)
            self.assertFalse(decide("workflow_dispatch", sha, "", "", "false").platforms)
            with self.assertRaises(ValueError):
                decide("workflow_dispatch", "b" * 40, "", "", "true")
            with self.assertRaises(ValueError):
                decide("workflow_dispatch", sha, "", "", "")

    def test_exact_pr_parents_both_diffs_and_git_failure(self):
        merge, base, head, ancestor = [letter * 40 for letter in "abcd"]
        refs = [merge.encode(), f"{merge} {base} {head}".encode(), ancestor.encode()]
        wiki = changes(("M", "docs/WIKI.md"))
        code = changes(("M", "src/main.rs"))
        with patch("ci_policy.git", side_effect=[*refs, wiki, wiki]), patch("ci_policy.read_blob", side_effect=BLOBS.__getitem__):
            self.assertFalse(decide("pull_request", merge, base, head, "").rust)
        for diffs in ([wiki, code], [subprocess.CalledProcessError(128, "git")]):
            with patch("ci_policy.git", side_effect=[*refs, *diffs]):
                plan = decide("pull_request", merge, base, head, "")
                self.assertEqual((plan.rust, plan.metadata), (True, "all"))
        with patch("ci_policy.git", side_effect=[merge.encode(), head.encode()]):
            with self.assertRaises(ValueError):
                decide("pull_request", merge, base, head, "")


if __name__ == "__main__":
    unittest.main()
