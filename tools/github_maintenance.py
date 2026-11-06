"""Trusted event handlers for PR CI cancellation and ready issue claims."""

import argparse
import json
import os
import subprocess
from pathlib import Path
from urllib.parse import quote

READY = "status:ready-to-claim"
ACTIVE = {"queued", "in_progress", "waiting", "pending", "requested"}


def api(path, method="GET", fields=(), paginated=False, payload=None):
    command = ["gh", "api", "--method", method, path]
    if paginated:
        command += ["--paginate", "--slurp"]
    for field in fields:
        command += ["-f", field]
    if payload is not None:
        command += ["--input", "-"]
    response = subprocess.run(
        command, input=json.dumps(payload) if payload is not None else None,
        check=True, capture_output=True, text=True, timeout=60,
    )
    return json.loads(response.stdout) if response.stdout.strip() else None


def head_key(head):
    return ((head.get("repo") or {}).get("full_name"), head["ref"])


def matches_run(run, pr, same_head_in_use):
    head_repo, head_ref = head_key(pr["head"])
    if (run["status"] not in ACTIVE or run.get("event") != "pull_request"
            or (run.get("head_repository") or {}).get("full_name") != head_repo
            or run.get("head_branch") != head_ref):
        return False
    linked = {link["number"] for link in run.get("pull_requests", [])}
    if linked:
        return linked == {pr["number"]}
    return not same_head_in_use and run.get("head_sha") == pr["head"]["sha"]


def cancel(repository, event):
    pr = event["pull_request"]
    if event["action"] != "closed" or pr["state"] != "closed":
        return "Cancellation applies to closed pull requests."
    live = api(f'repos/{repository}/pulls/{pr["number"]}')
    if live["state"] != "closed" or live["head"]["sha"] != pr["head"]["sha"]:
        return "The pull request changed after closure. No runs were cancelled."
    head_repo, branch = head_key(pr["head"])
    if not head_repo:
        return "The head repository is unavailable. No runs were cancelled."
    owner = head_repo.split("/")[0]
    peers = api(
        f"repos/{repository}/pulls",
        fields=("state=open", f"head={owner}:{branch}", "per_page=100"),
        paginated=True,
    )
    same_head_in_use = any(
        head_key(other["head"]) == (head_repo, branch) for page in peers for other in page
    )
    cancelled = set()
    for status in sorted(ACTIVE):
        pages = api(
            f"repos/{repository}/actions/runs",
            fields=(f"branch={branch}", "event=pull_request", f"status={status}", "per_page=100"),
            paginated=True,
        )
        for page in pages:
            for run in page["workflow_runs"]:
                run_id = run["id"]
                if run_id in cancelled or not matches_run(run, pr, same_head_in_use):
                    continue
                try:
                    api(f"repos/{repository}/actions/runs/{run_id}/cancel", method="POST")
                except subprocess.CalledProcessError:
                    # A run can finish between listing it and sending the cancellation.
                    if api(f"repos/{repository}/actions/runs/{run_id}")["status"] != "completed":
                        raise
                else:
                    cancelled.add(run_id)
    return f"Requested cancellation for {len(cancelled)} unfinished CI run(s)."


def claim(repository, event):
    comment = event["comment"]
    actor = comment["user"]
    number = event["issue"]["number"]
    if (event["action"] != "created" or "pull_request" in event["issue"]
            or actor["type"] != "User" or comment["body"] != "/claim"):
        return "No supported issue claim command."
    path = f"repos/{repository}/issues/{number}"
    issue = api(path)
    if issue["state"] != "open" or issue["assignees"] or not any(
        label["name"] == READY for label in issue["labels"]
    ):
        return "Claim skipped. The issue must be open, unassigned and ready to claim."
    assigned = api(f"{path}/assignees", method="POST", payload={"assignees": [actor["login"]]})
    assignees = {user["id"] for user in assigned["assignees"]}
    if actor["id"] not in assignees:
        return "GitHub did not accept the requested assignee. A maintainer can assign it manually."
    if assignees != {actor["id"]}:
        api(f"{path}/assignees", method="DELETE", payload={"assignees": [actor["login"]]})
        return "A concurrent assignment was detected. The bot left the other assignee in place."
    api(f"{path}/labels/{quote(READY, safe='')}", method="DELETE")
    return "Assigned the issue to the claimant and removed its ready-to-claim label."


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("operation", choices=("cancel", "claim"))
    operation = parser.parse_args().operation
    with Path(os.environ["GITHUB_EVENT_PATH"]).open(encoding="utf-8") as source:
        event = json.load(source)
    handler = cancel if operation == "cancel" else claim
    message = handler(os.environ["GITHUB_REPOSITORY"], event)
    print(message)
    with Path(os.environ["GITHUB_STEP_SUMMARY"]).open("a", encoding="utf-8") as summary:
        summary.write(message + "\n")


if __name__ == "__main__":
    main()
