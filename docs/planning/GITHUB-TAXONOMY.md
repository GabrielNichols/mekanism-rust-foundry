# GitHub taxonomy and work tracking

The [label catalog](github-labels.json) defines names, colors, descriptions, eligible surfaces, and assignment authority. Contributors propose labels and provide links. Maintainers classify, change workflow labels, and update Project status. Preserve existing maintainer labels and use the catalog for new work.

- `area:*` describes the affected Mekanism behavior or lab subsystem. Apply every relevant area.
- `type:*` describes the primary deliverable. Choose one after triage.
- `family:*` identifies a roadmap workstream. This connects issues and PRs to the [canonical work items](GITHUB-ROADMAP.md).
- `phase:*` and GitHub Milestones represent roadmap phases. Use one phase per item when scope is clear. Apply the same milestone to a linked PR when its issues agree.
- `workflow:needs-triage` and `workflow:needs-evidence` record pending actions with explicit removal conditions.
- `status:ready-to-claim` is an issue-only discovery mirror for open, unassigned issues in Ready. The [Project](https://github.com/users/GabrielNichols/projects/2) Status field owns operational state. Earlier `status:planned` and `status:blocked` labels preserve historical prerequisite classification.

## From discussion to pull request

Use Q&A for support, Ideas for proposals, Announcements for project updates, and Show and tell for completed examples. Use labels for subject and deliverable. An accepted proposal becomes a scoped issue or sub-issue with its acceptance criteria, a link to the discussion, and a reciprocal link in the discussion. Keep the issue and linked PR in the Project as separate items.

| Event | Actor | Label change | Project and linkage |
| --- | --- | --- | --- |
| Discussion or item submitted | Contributor opens, maintainer triages | Add `workflow:needs-triage` if follow-up is needed | Select the appropriate Discussion category or issue template |
| Initial triage completed | Maintainer | Remove `workflow:needs-triage`, apply area and primary type | Document disposition and any remaining questions |
| Proposal becomes tracked work | Maintainer | Preview eligible durable classification on the issue | Link both ways and add the issue or sub-issue to the Project |
| Issue becomes ready | Maintainer | Remove triage and add `status:ready-to-claim` if open and unassigned | Set issue status to Ready after dependencies and acceptance are actionable |
| Issue claimed | Maintainer | Remove `status:ready-to-claim` | Assign the contributor. Move the issue to In Progress when work starts |
| Issue blocked | Maintainer | Remove `status:ready-to-claim` | Set Blocked and link the prerequisite or missing binding |
| Draft PR opened or linked | Contributor links, maintainer classifies | Propagate eligible area, type, family and phase through the planner | Add a separate PR item, use Draft for the PR and keep the issue status independent |
| PR marked ready for review | Contributor requests, maintainer tracks | Keep durable classification | Use In Review for the PR. The issue may remain Ready |
| Evidence requested or accepted | Reviewer requests, maintainer updates | Add or remove `workflow:needs-evidence` | Link the request and the accepted evidence. Keep review and parity verdicts on their original surfaces |
| PR merged | Maintainer | Keep durable classification and clear resolved workflow labels | Move the PR to Done. Use `Closes #N` only after each linked issue's acceptance and evidence are verified. Use `Refs #N` for partial work |

Copy eligible areas and roadmap families across reviewed links. For several issues, union their eligible areas and families. Conflicting type or phase values require a maintainer decision before propagation. A cross-cutting PR can retain direct links and an explicitly selected primary deliverable. Never propagate approvals, security checks, parity verdicts, evidence-request labels, or readiness from one item to another. Review each issue separately before closure.

## Preview and apply

From the repository root, preview the catalog or a proposed transition. A transition snapshot contains `surface`, `event`, `labels`, optional `project_status`, and any required `open`, `assigned`, or `sources` fields. A linked source contains `surface` and `labels`. The planner emits additions, removals, conflicts, and a proposed Project status.

```powershell
./tools/plan-github-labels.ps1 -PreviewPath .lab/labels-reviewed.json
./tools/plan-github-labels.ps1 -ApplyPreview .lab/labels-reviewed.json
./tools/plan-github-labels.ps1 -Transition .lab/transition.json
./tools/test-github-labels.ps1
```

Catalog apply creates only reviewed missing labels on the pinned repository ID. It preserves different live descriptions and colors for maintainer review. Repeat a preview after an ambiguous response. Apply a transition using authenticated GitHub controls after verifying the target and its links. Re-preview after applying it. CI runs the same planner against local fixtures with no write access. No workflow processing fork code receives metadata credentials.

[GitHub Discussions](https://docs.github.com/en/discussions/quickstart) · [Project item management](https://docs.github.com/en/issues/planning-and-tracking-with-projects/managing-items-in-your-project/adding-items-to-your-project)
