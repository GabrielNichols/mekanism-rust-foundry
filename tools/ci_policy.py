"""Select CI work from complete Git diffs of a verified PR merge."""

import os
import re
import subprocess
from pathlib import Path


SHA = re.compile(r"[0-9a-f]{40}")
EDITORIAL = {"docs/WIKI.md"}
FUNDING = ".github/FUNDING.yml"


def classify(raw: bytes) -> tuple[str, str]:
    """Interpret NUL-separated --raw --no-renames output."""
    try:
        fields = raw.decode("utf-8").split("\0")
    except UnicodeDecodeError:
        return "rust", "non-UTF-8 paths"
    if fields[-1] != "" or len(fields) < 3 or len(fields) % 2 != 1:
        return "rust", "empty or incomplete changed-file list"
    paths = set()
    for entry, path in zip(fields[:-1:2], fields[1:-1:2]):
        header = entry.split()
        if len(header) != 5 or not header[0].startswith(":"):
            return "rust", "invalid changed-file record"
        old_mode, new_mode, old_sha, new_sha, status = header
        if not SHA.fullmatch(old_sha) or not SHA.fullmatch(new_sha):
            return "rust", "incomplete changed-file object identity"
        if new_mode != "100644" or old_mode != (":000000" if status == "A" else ":100644"):
            return "rust", "changed path is not a regular non-executable file"
        if not path or path in paths:
            return "rust", "invalid or duplicate changed path"
        paths.add(path)
        if not ((path in EDITORIAL and status == "M") or
                (path == FUNDING and status in {"A", "M"})):
            return "rust", "change outside the editorial allowlist"
    if len(paths) != 1:
        return "rust", "multiple changed paths"
    return "light", "only editorial handbook or GitHub funding changes"


def git(*args: str) -> bytes:
    return subprocess.run(["git", *args], check=True, capture_output=True).stdout


def diff(before: str, after: str) -> bytes:
    return git("diff", "--raw", "-z", "--no-renames", "--no-abbrev",
               "--no-ext-diff", "--no-textconv", before, after, "--")


def decide(event: str, expected: str, base: str, head: str,
           platform: str) -> tuple[str, str]:
    if not SHA.fullmatch(expected) or git("rev-parse", "HEAD").decode().strip() != expected:
        raise ValueError("checkout does not match the Actions revision")
    if event == "workflow_dispatch":
        if platform not in {"true", "false"}:
            raise ValueError("missing or invalid manual platform selection")
        return ("matrix" if platform == "true" else "rust"), "manual validation"
    if event != "pull_request":
        raise ValueError("unsupported workflow event")
    if not SHA.fullmatch(base) or not SHA.fullmatch(head):
        raise ValueError("missing or invalid PR revision")
    parents = git("rev-list", "--parents", "-n", "1", expected).decode().split()
    if parents != [expected, base, head]:
        raise ValueError("checkout parents do not match the PR base and head")

    try:
        merge_bases = git("merge-base", "--all", base, head).decode().split()
        if len(merge_bases) != 1 or not SHA.fullmatch(merge_bases[0]):
            return "rust", "PR ancestry is ambiguous"
        # Check both the contributor's change and its effect on the tested merge.
        proposed = diff(merge_bases[0], head)
        mode, reason = classify(proposed)
        if mode != "light":
            return mode, reason
        tested = diff(base, expected)
        mode, reason = classify(tested)
        if mode != "light":
            return mode, reason
        if proposed != tested:
            return "rust", "proposed and tested editorial diffs differ"
    except (OSError, subprocess.CalledProcessError, UnicodeError):
        return "rust", "Git could not produce the complete changed-file list"
    return "light", "only editorial handbook or GitHub funding changes"


def main() -> None:
    mode, reason = decide(
        os.environ["GITHUB_EVENT_NAME"], os.environ["GITHUB_SHA"],
        os.getenv("PR_BASE_SHA", ""), os.getenv("PR_HEAD_SHA", ""),
        os.getenv("FULL_PLATFORMS", ""),
    )
    with Path(os.environ["GITHUB_OUTPUT"]).open("a", encoding="utf-8") as output:
        output.write(f"mode={mode}\nreason={reason}\n")


if __name__ == "__main__":
    main()
