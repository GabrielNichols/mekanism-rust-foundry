"""Choose CodeQL languages from the verified Git diff."""

import json
import os
import re
import subprocess
from pathlib import Path
from types import ModuleType

ALL = ("actions", "java-kotlin", "python", "rust")
SHA = re.compile(r"[0-9a-f]{40}")


def git(*args):
    return subprocess.run(["git", *args], check=True, capture_output=True).stdout


def policy_at(commit):
    policy = ModuleType("trusted_ci_policy")
    source = git("show", f"{commit}:tools/ci_policy.py")
    exec(compile(source, "tools/ci_policy.py", "exec"), policy.__dict__)
    return policy


def languages(raw, policy):
    fields = raw.decode("utf-8").split("\0")
    if fields[-1] or len(fields) < 3 or len(fields) % 2 != 1:
        return ALL
    selected = set()
    for header, path in zip(fields[:-1:2], fields[1:-1:2]):
        if not re.fullmatch(r":(?:000000|100644) 100644 [0-9a-f]{40} [0-9a-f]{40} [AM]", header):
            return ALL
        if path == policy.FUNDING or policy.editorial_markdown(path):
            plan = policy.classify(f"{header}\0{path}\0".encode(), policy.read_blob)
            if not plan.rust and plan.metadata == "none":
                continue
        if re.fullmatch(r"(src|tests)/.+\.rs", path) or path in {"Cargo.toml", "Cargo.lock"}:
            selected.add("rust")
        elif path.startswith("adapters/neoforge/") and path.endswith((".java", ".kt", ".kts", ".gradle")):
            selected.add("java-kotlin")
        elif path.startswith(("tools/", "tests/")) and path.endswith(".py") and path not in {
            "tools/ci_policy.py", "tools/codeql_scope.py",
        }:
            selected.add("python")
        else:
            return ALL
    return tuple(language for language in ALL if language in selected)


def choose(event, expected, base, head):
    if event not in {"pull_request", "push"}:
        return ALL
    if not SHA.fullmatch(expected) or not SHA.fullmatch(base) or base == "0" * 40:
        return ALL
    if git("rev-parse", "HEAD").decode().strip() != expected:
        return ALL
    policy = policy_at(base if event == "pull_request" else expected)
    if event == "pull_request":
        if not SHA.fullmatch(head):
            return ALL
        parents = git("rev-list", "--parents", "-n", "1", expected).decode().split()
        if parents != [expected, base, head]:
            return ALL
        origins = git("merge-base", "--all", base, head).decode().split()
        if len(origins) != 1:
            return ALL
        tested = policy.diff(base, expected)
        if policy.diff(origins[0], head) != tested:
            return ALL
    else:
        git("merge-base", "--is-ancestor", base, expected)
        tested = policy.diff(base, expected)
    return languages(tested, policy)


def main():
    try:
        selected = choose(os.environ["GITHUB_EVENT_NAME"], os.environ["GITHUB_SHA"],
                          os.getenv("PR_BASE_SHA", ""), os.getenv("PR_HEAD_SHA", ""))
    except (OSError, subprocess.CalledProcessError, ValueError):
        selected = ALL
    with Path(os.environ["GITHUB_OUTPUT"]).open("a", encoding="utf-8") as output:
        output.write(f"languages={json.dumps(selected)}\n")
    print("Selected CodeQL languages:", ", ".join(selected) or "none (verified editorial changes)")


if __name__ == "__main__":
    main()
