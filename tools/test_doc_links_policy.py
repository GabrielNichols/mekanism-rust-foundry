import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

SCRIPT = Path(__file__).with_name("check_doc_links.py").resolve()


class DocumentationLinks(unittest.TestCase):
    def test_reports_new_relative_failures_and_ignores_old_and_external_links(self):
        with tempfile.TemporaryDirectory(dir=Path.cwd()) as directory:
            repo = Path(directory)
            self.git(repo, "init", "-q")
            (repo / "docs").mkdir()
            (repo / "docs/Guide File.md").write_text(
                "# Setup\n\n## Repeated Heading\n\n## Repeated Heading\n", encoding="utf-8",
            )
            readme = repo / "README.md"
            readme.write_text("[Existing issue](legacy.md)\n", encoding="utf-8")
            base = self.commit(repo, "base")
            readme.write_text(
                readme.read_text(encoding="utf-8")
                + "[Guide](docs/Guide%20File.md#setup)\n"
                + "[Repeated](<docs/Guide File.md#repeated-heading-1>)\n"
                + "[Wrong anchor](docs/Guide%20File.md#wrong-anchor)\n"
                + "[External](https://example.invalid/missing)\n"
                + "[new-ref]: new-missing.md\n"
                + "<!-- [Comment](comment.md) -->\n"
                + "\x60\x60\x60md\n[Example](example.md)\n\x60\x60\x60\n",
                encoding="utf-8",
            )
            head = self.commit(repo, "docs")
            summary = repo / ".git" / "summary"
            env = dict(os.environ, PR_BASE_SHA=base, GITHUB_SHA=head, GITHUB_STEP_SUMMARY=str(summary))
            output = subprocess.check_output([sys.executable, "-I", "-B", str(SCRIPT)],
                                             cwd=repo, env=env, text=True)
            self.assertIn("wrong-anchor", output)
            self.assertIn("new-missing.md", output)
            for excluded in ("legacy.md", "example.invalid", "comment.md", "example.md"):
                self.assertNotIn(excluded, output)
            self.assertIn("2 new broken relative link(s)", summary.read_text(encoding="utf-8"))

    @staticmethod
    def git(repo, *args):
        return subprocess.check_output(["git", *args], cwd=repo, text=True).strip()

    def commit(self, repo, message):
        self.git(repo, "add", ".")
        self.git(repo, "-c", "user.name=Fixture", "-c", "user.email=fixture@example.invalid",
                 "commit", "-qm", message)
        return self.git(repo, "rev-parse", "HEAD")


if __name__ == "__main__":
    unittest.main()
