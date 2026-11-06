# Foundry CI policy

The workflow reports one verdict for pull requests against main and for main pushes. The required Foundry PR gate checks that each selected job succeeded.

| Lane | Selection | Evidence |
| --- | --- | --- |
| Change scope and gate | Every PR, main push and manual dispatch | Complete Git diffs, checked-out revision and parents, relevant funding syntax or prose checks and a final verdict |
| Policy fixtures and public tree | Rust, metadata or conservative changes | Routing contracts and tracked file safety |
| Metadata | Changed catalog or triage inputs, conservative changes and manual validation | Relevant offline fixtures. A triage change runs its triage fixture. A catalog change runs its catalog fixtures. |
| Windows Rust | Rust, conservative changes and manual validation | Pinned formatting, Clippy, locked build and contracts |
| Dependency review | PRs with root Cargo.toml or Cargo.lock changes in the tested merge | Known advisories on added dependencies from the GitHub dependency graph. Findings warn, and service errors fail the separate advisory job. |
| Linux and macOS Rust | Manual full validation | Independent x64 results with the same locked Rust commands |
| Headless Java and parity | Reviewed refs in isolated environments | Pinned reference, semantic witnesses, seals and teardown. [Platform audit](https://github.com/GabrielNichols/mekanism-rust-foundry/issues/141). |
| Release and packaging | Maintainer review and explicit publication | Verified immutable tag, installed artifacts, checksums and provenance. [Release policy](releases/README.md). |

Funding and prose-only changes to ordinary Markdown at the repository root or under docs may share the light path. Agent instructions, security policy, CI policy and packaged release documentation still use conservative validation. The funding check accepts the native configuration for the current beneficiary. The prose check protects links, commands, indentation, fenced blocks and raw HTML. Pure Rust changes skip metadata. Mixed changes take the union of applicable checks, and unknown paths, incomplete diffs, renames and deletes select both Rust and metadata. Governance, workflows, executable documentation inputs and generated manifests without a dedicated contract use conservative validation. The contributor operations PR includes executable triage tooling and governance docs, so it selects its triage fixture and the conservative Rust path.

The base branch supplies the classifier for PR selection. The classifier uses local Git, including both the proposed diff and the effect on the tested merge. It issues no GraphQL or REST calls. Initial PR adoption runs Rust and metadata. Main pushes classify the complete previous-to-landed range with the landed policy and run conservative validation for an unverifiable range. A mismatch between the checked-out merge and the event fails the gate. A conflicting PR may have no merge ref and requires conflict resolution before validation.

The final gate requires success for every selected job. It permits a skip only when the policy explicitly leaves that job unselected. A failed, cancelled or unexpectedly skipped selected job cannot produce a successful gate. PR work uses ephemeral standard runners, pinned checkout, a read-only token and no persisted checkout credentials or Actions artifacts. The gate has no token permissions and checks out no code. Workflow changes need maintainer review because binding a required check to the Actions app does not make proposed workflow YAML immutable. Keep trusted runners, private Java artifacts and publishing credentials outside PR workflows.

## PR evidence and triage

The advisory workflow checks the PR description and reports suggested labels from changed paths. It compares explicit closing declarations for local issues across open PRs. `Refs #N` indicates partial work and does not trigger an overlap notice. The job writes only its own summary and uses a read-only token for the GitHub API. Drafts and title-only edits do not start a runner. Maintainers review all suggestions before updating labels or coordinating with contributors.

## Windows ZIP proof

The separate ZIP proof runs on packaging and bundled documentation or fixture changes, or on manual dispatch at the reviewed revision. It runs archive controls and extracts and exercises the offline package on Windows. It does not publish or upload artifacts. The required PR gate and the Java-backed release proof remain independent. See the [ZIP dry run](releases/windows-zip-dry-run.md).

## Advisory dependency review

The advisory job uses a read-only Actions token to compare the verified PR base and tested merge in the GitHub dependency graph. It does not check out the PR, post comments or upload artifacts. The workflow calls the same REST endpoint as the official dependency review action. The current action can upload oversized summaries as artifacts, so this lane uses the endpoint directly. The required gate has the same dependencies and verdicts.

Review the advisory job on the current revision when it runs. A service failure leaves the dependency review unavailable. An empty diff can also mean the graph did not recognize a change, so review the Cargo diff directly. This lane covers the root Cargo files. Java runtime pins retain their separate artifact verification. The PR that introduces this lane uses the existing base classifier and has no dependency review output until the new classifier is on main.

## CodeQL

GitHub default setup independently selects Actions, Rust and Java/Kotlin. Those three analyses can still run on editorial PRs against main and on the resulting main push. Default setup does not offer Foundry's content-aware routing. These are CodeQL analysis jobs, separate from GitHub GraphQL and from the PR policy's offline metadata tests.

Advanced setup supports custom triggers and schedules, so an equivalent path-aware workflow is possible in principle. Before proposing a switch, verify all three language configurations and findings on reviewed code and workflow fixtures. Scan classifier and workflow changes with Actions analysis and require broad analysis for uncertain diffs or dependency changes. Keep periodic full scans of main and manual full replay. Check exact revision coverage, fork SARIF handling and least privilege for uploads. Compare equivalent results in an isolated trial before requesting any change to default setup, which can override advanced analysis. The rollout retains default setup and current security policies. [Default setup](https://docs.github.com/en/code-security/concepts/code-scanning/setup-types) · [Advanced setup](https://docs.github.com/en/code-security/how-tos/find-and-fix-code-vulnerabilities/configure-code-scanning/configuring-advanced-setup-for-code-scanning)

## Protection transition

1. Review and land the shadow workflow with the old Windows job unchanged. The shadow verification PRs target the policy branch and must never be merged.
2. Verify the funding-only and failing Rust revisions reached their intended outcomes. Confirm the actual gate check and GitHub Actions app. Keep strict branch protection and conversation resolution.
3. After reviewing the evidence, add the new gate alongside the Windows requirement. Read protection back and verify both contexts with a normal main-targeting PR. Only then remove the old required context. Preserve CodeQL and branch and tag settings.
4. Remove duplicate legacy PR work after confirming protection. A subsequent main workflow can select work from the complete push range against the exact landed revision. Run Rust for landed code, workflows and ambiguous history. Continue release checks at the exact tagged commit.
5. If the new gate misses or misreports a result, restore the previous Windows requirement before reverting policy. Preserve the affected check history.

The manual lane uses the established Windows runner and the x64 Ubuntu and macOS targets from the [platform trial](https://github.com/GabrielNichols/mekanism-rust-foundry/actions/runs/42156105640). Java and release verification have separate permissions and triggers.

[GitHub job dependencies](https://docs.github.com/en/actions/how-tos/write-workflows/choose-what-workflows-do/use-jobs) · [GitHub status checks](https://docs.github.com/en/pull-requests/reference/status-checks)
