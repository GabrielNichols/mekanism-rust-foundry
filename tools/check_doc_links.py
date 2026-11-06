"""Find newly broken repository-relative links in changed Markdown."""

import json
import os
import posixpath
import re
import subprocess
from functools import lru_cache
from pathlib import Path
from urllib.parse import unquote, urlsplit

FENCED = re.compile(r"(?ms)^ {0,3}(\x60{3,}|~{3,})[^\n]*\n.*?^ {0,3}\1[ \t]*$")
COMMENT = re.compile(r"<!--.*?-->", re.S)
INLINE = re.compile(r"(?<!\\)\[[^\]\n]*\]\(\s*(<[^>\n]+>|[^)\s]+)")
REFERENCE = re.compile(r"(?m)^ {0,3}\[[^\]\n]+\]:[ \t]*(<[^>\n]+>|\S+)")
HTML_ANCHOR = re.compile(r"""<(?:a|span)\b[^>]*\b(?:name|id)=["']([^"']+)["']""", re.I)
SHA = re.compile(r"[0-9a-f]{40}")


def visible(text):
    return FENCED.sub("", COMMENT.sub("", text))


def targets(text):
    text = re.sub(r"(\x60+)[^\n]*?\1", "", visible(text))
    return {match[1].strip("<>") for pattern in (INLINE, REFERENCE)
            for match in pattern.finditer(text)}


def anchors(text):
    text = visible(text)
    used = set()
    for line in text.splitlines():
        heading = re.match(r"^ {0,3}#{1,6}\s+(.+?)\s*#*\s*$", line)
        if not heading:
            continue
        title = re.sub(r"\[([^]]+)\]\([^)]+\)", r"\1", heading[1])
        title = re.sub(r"<[^>]+>", "", title)
        slug = "".join(char for char in title.lower().strip()
                       if char.isalnum() or char in " _-").replace(" ", "-")
        unique = slug
        suffix = 1
        while unique in used:
            unique = f"{slug}-{suffix}"
            suffix += 1
        used.add(unique)
    return used | set(HTML_ANCHOR.findall(text))


@lru_cache(maxsize=512)
def object_type(revision, path):
    result = subprocess.run(["git", "cat-file", "-t", f"{revision}:{path}"],
                            capture_output=True, check=False, text=True)
    return result.stdout.strip() if result.returncode == 0 else None


@lru_cache(maxsize=128)
def read_markdown(revision, path):
    return subprocess.check_output(["git", "show", f"{revision}:{path}"]).decode("utf-8")


def broken_links(revision, source):
    if object_type(revision, source) != "blob":
        return set()
    broken = set()
    for target in targets(read_markdown(revision, source)):
        try:
            url = urlsplit(target)
        except ValueError:
            continue
        if url.scheme or url.netloc or target.startswith("//"):
            continue
        path = unquote(url.path)
        resolved = source
        if path:
            resolved = posixpath.normpath(path.lstrip("/") if path.startswith("/") else
                                          posixpath.join(posixpath.dirname(source), path))
        if resolved == ".." or resolved.startswith("../") or not object_type(revision, resolved):
            broken.add(target)
        elif url.fragment and resolved.lower().endswith(".md"):
            if object_type(revision, resolved) != "blob" or unquote(url.fragment) not in anchors(
                read_markdown(revision, resolved)
            ):
                broken.add(target)
    return broken


def main():
    base = os.getenv("PR_BASE_SHA", "")
    revision = os.environ["GITHUB_SHA"]
    if not SHA.fullmatch(base) or base == "0" * 40:
        print("No comparable base revision for the documentation advisory.")
        return
    if not SHA.fullmatch(revision) or subprocess.check_output(
        ["git", "rev-parse", "HEAD"], text=True
    ).strip() != revision:
        raise ValueError("Documentation comparison must match the checked-out Actions revision")
    changed = subprocess.check_output([
        "git", "diff", "--no-renames", "--name-only", "--diff-filter=AM",
        "-z", base, revision, "--", "*.md",
    ]).decode("utf-8").split("\0")
    changed = list(filter(None, changed))
    if not changed:
        return
    count = 0
    for source in changed:
        new = broken_links(revision, source) - broken_links(base, source)
        for target in sorted(new):
            # JSON escapes line breaks and terminal control characters in contributor text.
            print("New broken relative link:", json.dumps([source, target], ensure_ascii=True))
            count += 1
    message = f"Documentation link advisory found {count} new broken relative link(s)."
    if count:
        print(f"::warning::{message}")
    print(message)
    with Path(os.environ["GITHUB_STEP_SUMMARY"]).open("a", encoding="utf-8") as summary:
        summary.write(message + "\n")


if __name__ == "__main__":
    main()
