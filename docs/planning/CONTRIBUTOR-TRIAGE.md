# Contributor triage

Use the [Wiki index](../WIKI.md) for navigation, [roadmap issues](GITHUB-ROADMAP.md) for acceptance and ownership, [Project Status](https://github.com/users/GabrielNichols/projects/2/views/3) for the current action, and [milestones](MILESTONES.md) for phase exit criteria. [Label and workflow policy](GITHUB-TAXONOMY.md)

## Discussion entry points

| Existing category | Suggested starting post | Links and context |
| --- | --- | --- |
| [General](https://github.com/GabrielNichols/mekanism-rust-foundry/discussions/categories/general) | Start here and find a small task | [Onboarding](../wiki/ONBOARDING.md), [Start Here issue](https://github.com/GabrielNichols/mekanism-rust-foundry/issues/8), [Ready work](https://github.com/users/GabrielNichols/projects/2/views/3?filterQuery=status%3AReady%20is%3Aissue%20is%3Aopen%20no%3Aassignee) |
| [Q&A](https://github.com/GabrielNichols/mekanism-rust-foundry/discussions/categories/q-a) | Ask for help reproducing a case | Source or tag, case and profile, sanitized failure phase, expected witness and related issue |
| [Ideas](https://github.com/GabrielNichols/mekanism-rust-foundry/discussions/categories/ideas) | Propose a finite scenario or lab improvement | Requirement or roadmap ID, [authoring guide](../wiki/SCENARIO-AUTHORING.md), acceptance, negative control and any prerequisite |
| [Announcements](https://github.com/GabrielNichols/mekanism-rust-foundry/discussions/categories/announcements) | Summarize a verified preview or its reviewed scope | [Releases](../wiki/RELEASES.md), exact tag or tracking issue and corresponding Project release slice |
| [Show and tell](https://github.com/GabrielNichols/mekanism-rust-foundry/discussions/categories/show-and-tell) | Share a reproduced case | Sanitized evidence, result and limitations, and issue link |

When a maintainer accepts a proposal, link the new issue to the Discussion and add the issue link to the Discussion. Keep acceptance, evidence and blocking prerequisites on the issue. Use Polls for informal preference gathering. Route vulnerability reports through the [private reporting policy](../../SECURITY.md).

## Project views

| View | Filter and presentation | Operational rule |
| --- | --- | --- |
| Ready to Claim | `status:Ready is:issue is:open no:assignee`. Table with title, assignee, milestone and labels. [Preview filter](https://github.com/users/GabrielNichols/projects/2/views/3?filterQuery=status%3AReady%20is%3Aissue%20is%3Aopen%20no%3Aassignee) | Filter by Project Status. Keep `status:ready-to-claim` as the issue-search mirror. Start Here is a directory. |
| Next Release | Proposed Project single-select field `Target release`. Select the exact reviewed preview tag as the filter value. Display Status, Milestone and Assignees, including Done items. | Include reviewed release-slice items only. Keep M00–M14 as phase milestones. Link scope and exit evidence from the release tracking issue and announcement. The audit found no saved Next Release view. Tag and scope selection need maintainer review. |
| Actual Blockers | `status:Blocked is:issue is:open`. Table with title, assignee and milestone. [Preview filter](https://github.com/users/GabrielNichols/projects/2/views/3?filterQuery=status%3ABlocked%20is%3Aissue%20is%3Aopen) | Link the prerequisite from the issue and describe the exact missing evidence and the action it stops. Recheck the evidence before clearing Blocked. |

The existing [Phase](https://github.com/users/GabrielNichols/projects/2/views/1), [Family](https://github.com/users/GabrielNichols/projects/2/views/2) and [Status](https://github.com/users/GabrielNichols/projects/2/views/3) views retain their roles. The saved [Ready to Claim](https://github.com/users/GabrielNichols/projects/2/views/4) view currently uses the issue label. Saving views and fields needs a separate maintainer operation. [GitHub Project filters](https://docs.github.com/en/issues/planning-and-tracking-with-projects/customizing-views-in-your-project/filtering-projects)

## Reconciliation policy

- Decide status for the current action. A contributor can draft a fixture while the effective profile, observation surfaces or reviewed evidence still prevent execution. Make draft acceptance explicit and link the later execution gate.
- Keep historical `status:blocked` and `status:planned` as import classifications. Use the [dependency DAG](roadmap.json) and [issue crosswalk](github-roadmap.json) to identify prerequisites, then review the actual evidence before proposing status.
- The [bounded proposal](triage-review.json) compares its review baseline with a fresh snapshot. It preserves changed assignments, labels, milestones and statuses, flags discovery mirror drift for review and proposes Blocked only with a concrete open issue link and missing evidence. A partial manual application is a conflict to review. Recheck prerequisite acceptance even when an issue remains open.
- Close a milestone after reviewing its evidence and gate criteria. First-wave drafting and navigation issues can stay outside phase milestones. [Milestone policy](MILESTONES.md)

### Reproduce the dry run

Run from the repository root with PowerShell 7 and GitHub CLI. Use a fresh output directory for each attempt.

```powershell
$out = '.lab/evidence/triage-new'
New-Item -ItemType Directory -Path $out
./tools/read-github-triage.ps1 | Set-Content "$out/snapshot.json"
./tools/preview-github-triage.ps1 -ObservedSnapshot "$out/snapshot.json" | Set-Content "$out/preview.json"
./tools/test-github-triage.ps1
```

The reader joins paginated REST issues and labels to the public Project Phase view and checks the rows against the Status board counts. It stops if the public page data, Project identity or partitions change. The requests are separate, so recheck issues and prerequisite links before using the preview. The offline planner also accepts an export normalized from the official Projects API. Use schema 1 with repository, project, known_labels and issues. For each issue, include number, stable task_id, state, assigned (Boolean), labels, Project status and milestone title. Use null for a missing status or milestone. [November 6 audit](TRIAGE-AUDIT-2026-11-06.md)

[Wiki index](../WIKI.md)
