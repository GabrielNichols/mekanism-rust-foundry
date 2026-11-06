# Foundry CI policy

The candidate workflow reports one verdict for pull requests against main. During shadow validation, the existing required Windows check and main workflow remain authoritative.

| Lane | When it runs | Evidence |
| --- | --- | --- |
| Selection, public tree and metadata | Every PR and manual dispatch | Complete Git diffs, tested revision and parents, policy fixtures, public tree and metadata contracts |
| Windows Rust | Changes outside the editorial allowlist and manual validation | Pinned Rust formatting, Clippy, locked build and contracts |
| Linux and macOS Rust | Manual dispatch with full platforms selected | Independent x64 results with the same locked Rust commands |
| Headless Java and parity | Reviewed refs in isolated environments | Pinned reference, semantic witnesses, seals and teardown. [Platform audit](https://github.com/GabrielNichols/mekanism-rust-foundry/issues/141) |
| CodeQL | Existing GitHub default setup | Separate Actions, Rust and Java/Kotlin analyses |
| Release and packaging | Maintainer review and explicit publication | Verified immutable tag, installed artifacts, checksums and provenance. [Release policy](releases/README.md) |

The light path accepts a single regular-file modification to the Wiki landing page or a single addition or modification to GitHub's funding file. Executable documentation and generated manifests stay on the Rust path. Git treats a rename as a deletion and addition for this check. Incomplete or ambiguous diffs select Rust. A mismatch between the checked-out merge and the event fails the gate. The base branch supplies the classifier for PR selection once it exists. Initial adoption runs Rust.

The metadata lane also runs the contributor triage fixture when the triage tools are present. A PR that changes those executable tools and their fixtures uses the metadata lane and the conservative Rust path.

The final gate requires success for every selected job. It permits a skip only when the policy explicitly leaves that job unselected. A failed, cancelled or unexpectedly skipped required job cannot produce a successful gate. A conflicting PR may have no merge ref and requires conflict resolution before validation.

PR work uses ephemeral standard runners, pinned checkout, a read-only token and no persisted checkout credentials or Actions artifacts. The gate has no token permissions and never checks out code. Policy and workflow changes still require maintainer review. A contributor can propose changes to workflow YAML, so the GitHub Actions app binding alone does not make workflow logic immutable. Keep trusted runners, Java artifacts and publishing credentials outside the PR workflow.

## Protection transition

1. Review and land the shadow workflow with the old Windows job unchanged. The shadow verification PRs target the policy branch and must never be merged.
2. Verify that the exact funding-only and failing code revisions reached the intended outcomes. Confirm the gate's actual check name and GitHub Actions app ID. Keep strict branch protection and conversation resolution.
3. After reviewing the evidence, add the new gate alongside the Windows requirement. Read protection back and verify both contexts with a normal main-targeting PR. Only then remove the old required context. Leave CodeQL and branch and tag settings in place.
4. Remove duplicate PR work from the legacy workflow after confirming the protection change. For a future selective main run, use the exact landed revision and complete push range. Skip Rust only for independently verified editorial changes. Run Rust on landed code, workflow changes and ambiguous history. Release verification continues at the exact tagged commit.
5. If the new gate misses or misreports a result, restore the previous Windows requirement before reverting policy. Preserve the affected check history.

The manual lane uses the established Windows runner and the x64 Ubuntu and macOS targets from the [platform trial](https://github.com/GabrielNichols/mekanism-rust-foundry/actions/runs/42156105640). Java and release verification use their own lanes.

[GitHub job dependencies](https://docs.github.com/en/actions/how-tos/write-workflows/choose-what-workflows-do/use-jobs) · [GitHub status checks](https://docs.github.com/en/pull-requests/reference/status-checks)
