import unittest
from unittest.mock import call, patch

import ci_policy
import codeql_scope
import github_maintenance as bot

REPO = "owner/foundry"
HEAD = {"repo": {"full_name": "fork/foundry"}, "ref": "fix", "sha": "a" * 40}
PR = {"number": 7, "state": "closed", "head": HEAD}


def diff_entry(path, status="M"):
    return f":100644 100644 {'1' * 40} {'2' * 40} {status}\0{path}\0".encode()


class CodeQLSelection(unittest.TestCase):
    def test_safe_editorial_changes_need_no_scan(self):
        for path, old, new in [
            ("README.md", "Earlier description.\n", "Updated description.\n"),
            (".github/FUNDING.yml", "", "github: GabrielNichols\n"),
        ]:
            with self.subTest(path=path), patch.object(
                ci_policy, "read_blob", side_effect=[old, new] if old else [new],
            ):
                self.assertEqual(codeql_scope.languages(diff_entry(path), ci_policy), ())

    def test_only_applicable_languages(self):
        changes = diff_entry("src/main.rs") + diff_entry("adapters/neoforge/src/Observer.java")
        self.assertEqual(codeql_scope.languages(changes, ci_policy), ("java-kotlin", "rust"))
        self.assertEqual(codeql_scope.languages(diff_entry("tools/lint.py"), ci_policy), ("python",))

    def test_sensitive_or_incomplete_changes_scan_everything(self):
        for changes in (
            diff_entry(".github/workflows/ci.yml"), diff_entry("tools/codeql_scope.py"),
            diff_entry("src/main.rs", "D"), b"", b"partial",
        ):
            self.assertEqual(codeql_scope.languages(changes, ci_policy), codeql_scope.ALL)
        self.assertEqual(codeql_scope.choose("schedule", "", "", ""), codeql_scope.ALL)


class Maintenance(unittest.TestCase):
    def test_cancellation_respects_fork_and_pr_identity(self):
        run = {"id": 19, "status": "queued", "event": "pull_request", "head_branch": "fix",
               "head_sha": HEAD["sha"], "head_repository": HEAD["repo"], "pull_requests": [{"number": 7}]}
        self.assertTrue(bot.matches_run(run, PR, True))
        self.assertFalse(bot.matches_run(dict(run, pull_requests=[{"number": 8}]), PR, False))
        self.assertFalse(bot.matches_run(dict(run, head_repository={"full_name": "other/foundry"}), PR, False))
        self.assertFalse(bot.matches_run(dict(run, event="push"), PR, False))
        self.assertFalse(bot.matches_run(dict(run, pull_requests=[]), PR, True))
        self.assertTrue(bot.matches_run(dict(run, pull_requests=[]), PR, False))

    def test_reopened_pr_is_not_cancelled(self):
        with patch.object(bot, "api", return_value=dict(PR, state="open")) as api:
            self.assertIn("No runs", bot.cancel(REPO, {"action": "closed", "pull_request": PR}))
            api.assert_called_once_with("repos/owner/foundry/pulls/7")

    def test_claim_only_ready_unassigned_issues(self):
        event = {"action": "created", "issue": {"number": 3},
                 "comment": {"body": "/claim", "user": {"login": "contributor", "id": 11, "type": "User"}}}
        ready = {"state": "open", "assignees": [], "labels": [{"name": bot.READY}]}
        with patch.object(bot, "api", side_effect=[ready, {"assignees": [{"id": 11}]}, None]) as api:
            self.assertIn("Assigned", bot.claim(REPO, event))
            self.assertEqual(api.call_args_list, [
                call("repos/owner/foundry/issues/3"),
                call("repos/owner/foundry/issues/3/assignees", method="POST", payload={"assignees": ["contributor"]}),
                call("repos/owner/foundry/issues/3/labels/status%3Aready-to-claim", method="DELETE"),
            ])
        with patch.object(bot, "api", return_value=dict(ready, assignees=[{"id": 12}])) as api:
            self.assertIn("Claim skipped", bot.claim(REPO, event))
            self.assertEqual(api.call_count, 1)
        with patch.object(bot, "api", side_effect=[ready, {"assignees": [{"id": 12}, {"id": 11}]}, None]) as api:
            self.assertIn("concurrent assignment", bot.claim(REPO, event))
            self.assertEqual(api.call_args, call(
                "repos/owner/foundry/issues/3/assignees", method="DELETE", payload={"assignees": ["contributor"]},
            ))


if __name__ == "__main__":
    unittest.main()
