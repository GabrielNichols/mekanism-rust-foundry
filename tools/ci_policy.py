"""Choose Foundry validation from complete Git diffs of a verified PR merge."""

import difflib
import os
import re
import subprocess
from pathlib import Path
from typing import NamedTuple


SHA = re.compile(r"[0-9a-f]{40}")
GOVERNANCE_MARKDOWN = {"AGENTS.md", "SECURITY.md", "docs/CI-POLICY.md"}
FUNDING = ".github/FUNDING.yml"
FUNDING_LINE = re.compile(r"github:[ \t]*(?:GabrielNichols|\[\s*GabrielNichols\s*\])")
CATALOG = {
    "tools/preview-issues.ps1", "tools/sync-roadmap.ps1",
    "tools/plan-github-labels.ps1", "tools/sync-roadmap-milestones.ps1",
    "tools/test-issue-preview.ps1", "tools/test-roadmap-sync.ps1",
    "tools/test-github-labels.ps1", "tools/test-roadmap-milestones.ps1",
    "docs/planning/issue-queue.json", "docs/planning/github-roadmap.json",
    "docs/planning/github-labels.json",
}
TRIAGE = {
    "tools/preview-github-triage.ps1", "tools/read-github-triage.ps1",
    "tools/test-github-triage.ps1", "docs/planning/triage-review.json",
}


class Plan(NamedTuple):
    rust: bool
    metadata: str
    platforms: bool
    reason: str


def broad(reason: str) -> Plan:
    return Plan(True, "all", False, reason)


def prose_lines(lines: list[str]) -> list[bool]:
    """Mark plain prose outside fenced or raw HTML blocks."""
    fence = ""
    allowed = []
    for line in lines:
        marker = re.match(r"^\s*(`{3,}|~{3,})", line)
        if fence:
            allowed.append(False)
            if line.strip().startswith(fence) and not line.strip().rstrip(fence[0]):
                fence = ""
        elif marker:
            fence = marker[1]
            allowed.append(False)
        else:
            # Preserve markup, indentation, links and inline commands exactly.
            allowed.append(not line or (
                not line[0].isspace()
                and not re.search(r"[`\[\]{}<>|\\$~]", line)
                and not re.fullmatch(r"[-=]+", line)
            ))
    return allowed


def prose_only(old: str, new: str) -> bool:
    a, b = old.splitlines(), new.splitlines()
    a_prose, b_prose = prose_lines(a), prose_lines(b)
    # Multiline raw HTML has context-sensitive semantics. Keep such files on full validation.
    if re.search(r"(?m)^\s*<(?:[A-Za-z!/]|\?)", old) or re.search(r"(?m)^\s*<(?:[A-Za-z!/]|\?)", new):
        return False
    return bool(new.strip()) and all(
        all(a_prose[i:j]) and all(b_prose[k:l])
        for tag, i, j, k, l in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes()
        if tag != "equal"
    )


def editorial_markdown(path: str) -> bool:
    return (
        path.endswith(".md")
        and ("/" not in path or path.startswith("docs/"))
        and path not in GOVERNANCE_MARKDOWN
        and not path.startswith("docs/releases/")
        and not path.endswith("/AGENTS.md")
    )


def classify(raw: bytes, blob) -> Plan:
    """Interpret NUL-separated --raw --no-renames output."""
    try:
        fields = raw.decode("utf-8").split("\0")
    except UnicodeDecodeError:
        return broad("non-UTF-8 paths")
    if fields[-1] != "" or len(fields) < 3 or len(fields) % 2 != 1:
        return broad("empty or incomplete changed-file list")
    changes = {}
    for entry, path in zip(fields[:-1:2], fields[1:-1:2]):
        header = entry.split()
        if len(header) != 5 or not path or path in changes:
            return broad("invalid or duplicate changed-file record")
        old_mode, new_mode, old_sha, new_sha, status = header
        if not SHA.fullmatch(old_sha) or not SHA.fullmatch(new_sha):
            return broad("incomplete changed-file object identity")
        if (status not in {"A", "M"} or new_mode != "100644" or
                old_mode != (":000000" if status == "A" else ":100644")):
            return broad("rename, deletion, type change or unfamiliar file status")
        changes[path] = (status, old_sha, new_sha)

    rust = False
    metadata = set()
    uncertain = False
    for path, (status, old_sha, new_sha) in changes.items():
        if path == FUNDING:
            lines = [line.strip() for line in blob(new_sha).splitlines()
                     if line.strip() and not line.lstrip().startswith("#")]
            if len(lines) != 1 or not FUNDING_LINE.fullmatch(lines[0]):
                raise ValueError("funding must use the approved native GabrielNichols configuration")
        elif editorial_markdown(path) and prose_only(
            "" if status == "A" else blob(old_sha), blob(new_sha)
        ):
            continue
        elif path in CATALOG:
            metadata.add("catalog")
        elif path in TRIAGE:
            metadata.add("triage")
        elif path in {"Cargo.toml", "Cargo.lock"} or re.fullmatch(r"(?:src|tests)/.+\.rs", path):
            rust = True
        else:
            uncertain = True
    if uncertain:
        return broad("a change needs conservative validation")
    lane = "all" if len(metadata) > 1 else next(iter(metadata), "none")
    reason = "applicable Rust or metadata checks" if rust or metadata else "funding or editorial prose only"
    return Plan(rust, lane, False, reason)


def git(*args: str) -> bytes:
    return subprocess.run(["git", *args], check=True, capture_output=True).stdout


def diff(before: str, after: str) -> bytes:
    return git("diff", "--raw", "-z", "--no-renames", "--no-abbrev",
               "--no-ext-diff", "--no-textconv", before, after, "--")


def read_blob(sha: str) -> str:
    return git("cat-file", "blob", sha).decode("utf-8")


def decide(event: str, expected: str, base: str, head: str, platforms: str) -> Plan:
    if not SHA.fullmatch(expected) or git("rev-parse", "HEAD").decode().strip() != expected:
        raise ValueError("checkout does not match the Actions revision")
    if event == "workflow_dispatch":
        if platforms not in {"true", "false"}:
            raise ValueError("missing or invalid manual platform selection")
        return Plan(True, "all", platforms == "true", "manual validation")
    if event == "push":
        if not SHA.fullmatch(base) or base == "0" * 40:
            return broad("push has no previous revision")
        try:
            git("merge-base", "--is-ancestor", base, expected)
            tested = diff(base, expected)
        except (OSError, subprocess.CalledProcessError):
            return broad("Git could not verify the complete push range")
        return classify(tested, read_blob)
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
            return broad("PR ancestry is ambiguous")
        proposed, tested = diff(merge_bases[0], head), diff(base, expected)
    except (OSError, subprocess.CalledProcessError, UnicodeError):
        return broad("Git could not produce the complete changed-file list")
    if proposed != tested:
        return broad("proposed and tested diffs differ")
    return classify(tested, read_blob)


def main() -> None:
    plan = decide(
        os.environ["GITHUB_EVENT_NAME"], os.environ["GITHUB_SHA"],
        os.getenv("PR_BASE_SHA", ""), os.getenv("PR_HEAD_SHA", ""),
        os.getenv("FULL_PLATFORMS", ""),
    )
    with Path(os.environ["GITHUB_OUTPUT"]).open("a", encoding="utf-8") as output:
        output.write(f"rust={str(plan.rust).lower()}\nmetadata={plan.metadata}\n"
                     f"platforms={str(plan.platforms).lower()}\nreason={plan.reason}\n")


if __name__ == "__main__":
    main()
