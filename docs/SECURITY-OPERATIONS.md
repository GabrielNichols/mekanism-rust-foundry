# Security and quality operations

## Merge policy

GitHub protects `main` for every role, including the maintainer. Changes go through pull requests. The base must be current, review conversations must be resolved, and the required `Windows x64 / Rust 1.95.0` check must come from the GitHub Actions app. Force pushes and branch deletion are disabled.

CODEOWNERS requests the maintainer on contributor PRs. The maintainer reviews contributor changes, evidence and workflow edits before merging. GitHub requires zero independent approvals while Foundry has one maintainer so maintainer-authored fixes can pass through the same PR and check gate. The sole maintainer deferred mandatory code-owner approval until a second active maintainer with repository write access can review. At that point, verify owner resolution, require code-owner approval, retain stale-review dismissal, and test a critical-path PR that waits for an eligible reviewer.

## Untrusted contributions

Pull requests run on GitHub-hosted ephemeral Windows runners using `pull_request`. The workflow grants `contents: read`, passes no repository secrets and keeps Git credentials out of the checkout. Actions use immutable commits. The job checks formatting, Clippy, Rust contracts and offline issue mapping. It does not run a game or publish assets. Do not add privileged pull request triggers, persistent runners, shared credentials, or release permissions to this job. [GitHub Actions secure use](https://docs.github.com/en/actions/reference/security/secure-use)

Maintainers review edits to workflows and build scripts before trusting their results. The check app binding authenticates the check producer. Review is still required for changes to what the workflow actually runs.

## Alerts and release controls

Private vulnerability reporting, dependency alerts, secret scanning, and provider push protection are enabled. Dependabot alerts and any scanner findings require maintainer triage. Keep Java and game artifacts on the pinned private reference path. Use [preview packaging](releases/README.md) to verify exact inputs, the release archive and extracted execution.

The active version-tag ruleset blocks updates and deletion of `v*` tags. GitHub immutable releases also lock the published tag and assets. The maintainer creates releases from verified commits with an authenticated release session outside PR CI. Publish checksum and verification metadata with each first-party archive. Publish corrections as a new version.

The only standing review exception is the zero independent approval count described above. Log a policy exception with its owner, scope, expiration, and restoration proof before applying one.

CodeQL runs independently on standard GitHub-hosted runners. The first default-setup run completed analysis for Rust, Java and GitHub Actions. Treat individual analyzer status and findings as security evidence. A passing scan does not qualify parity. [CodeQL workflow](https://github.com/GabrielNichols/mekanism-rust-foundry/actions/workflows/github-code-scanning/codeql)
