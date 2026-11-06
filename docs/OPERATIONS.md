# Foundry operations

## Sources of truth

The [roadmap](planning/roadmap.json) owns task contracts and dependencies. The [issue map](planning/GITHUB-ROADMAP.md) links each stable ID to one issue. Issues record claims, acceptance and evidence. Project Status tracks the current action. The [work packages](planning/FOUNDRY-WORK-PACKAGES.md) own the inventory crosswalk. GitHub Projects views are filters over that work. [Discussions](https://github.com/GabrielNichols/mekanism-rust-foundry/discussions) host introductions, Q&A, ideas and announcements. The [handbook](WIKI.md) links the relevant versioned policies.

## Issue synchronization

Preview first, choose a batch with `-Ids` when appropriate, review the content, then apply the unchanged preview against the verified repository ID. Use only one writer at a time. The script preserves existing titles, bodies, assignees and state. It creates labels only when missing, waits between writes, and stops on ambiguous results. Re-preview before retrying an interrupted batch.

```powershell
./tools/sync-roadmap.ps1 -Repository GabrielNichols/mekanism-rust-foundry -Ids RF-SCOPE-01 -PreviewPath .lab/reviewed.json
./tools/sync-roadmap.ps1 -Repository GabrielNichols/mekanism-rust-foundry -Ids RF-SCOPE-01 -ApplyPreview .lab/reviewed.json -ExpectedRepositoryId 1447583916
```

[Contributor triage](planning/CONTRIBUTOR-TRIAGE.md) describes the view filters and the read-only reconciliation. Refresh the observed snapshot before reviewing changes.

## Quotas and cost

GitHub authenticated REST and GraphQL primary limits are exposed by `gh api rate_limit`. The observed account snapshot was REST 72 of 5,000 and GraphQL 11 of 5,000 consumed in their respective windows before the roadmap apply. The limits are account-wide and the snapshot is not a per-task cost. GitHub secondary limits are separate and can change. The script serializes writes and pauses between them. Prefer a single checked-in batch and a read-only verification over repeated individual calls.

CI uses GitHub-hosted Windows for Rust tests. Batch documentation and metadata changes when practical. Codex agent usage belongs to the goal runtime. GitHub Copilot quotas are separate and are not observable through the GitHub REST counters. This rollout uses no Copilot review automation. [GitHub REST rate limits](https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api)

Use only standard GitHub-hosted runners while this repository is public. GitHub provides their compute free and without drawing on the account's Actions compute allowance. Do not enable larger runners, paid services, uploaded CI artifacts, or paid review automation. Maintainers run game characterization and package verification locally. Keep untrusted fork code on ephemeral runners. [GitHub Actions billing](https://docs.github.com/en/billing/concepts/product-billing/github-actions)

[Milestone synchronization](planning/MILESTONES.md) uses the same verified issue IDs as roadmap synchronization. It previews missing milestones and assignments and preserves manual choices.
