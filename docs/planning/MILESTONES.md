# Foundry delivery milestones

GitHub Milestones M00–M14 are delivery groupings derived from the versioned roadmap's `legacy_phase` field. The [issue map](github-roadmap.json) binds each stable ID to its GitHub issue. Dependencies and qualification still follow the [roadmap DAG and gate criteria](ROADMAP.md). The [Project Phase view](https://github.com/users/GabrielNichols/projects/2/views/1) groups by the issue's GitHub Milestone. Project Status tracks ongoing work independently. The milestones have no invented dates or iteration cadence.

The milestone descriptions below summarize the intended evidence. Review every mapped item's acceptance and applicable gate before closing its milestone. A Project column change never qualifies a scope or closes work.

## M00

Reference scope and locks. Exit with reviewed, reproducible Java profile identities, artifact and configuration locks, and explicit reference and candidate boundaries.

## M01

Engine contracts. Exit with accepted and rejected cases for strict scenario parsing, canonical state, typed values, comparison, and public verdicts.

## M02

Isolated dedicated instances. Exit with bounded startup, stop, teardown and adverse-process proofs across disposable instance directories.

## M03

Bridge and clocks. Exit with independently witnessed dedicated and client phase barriers, tick and chunk observations, and observer controls.

## M04

Effective catalog discovery. Exit with loaded modules, configuration, registries and capabilities tied to verified runtime identities.

## M05

Lifecycle and persistence. Exit with durable save, restart, reload and failure continuation evidence using independent observations of saved state.

## M06

Evidence and validator sensitivity. Exit with complete, linked and sealed artifacts and known-change controls that expose the first divergence.

## M07

Post-load recipe discovery. Exit with runtime recipe keys, serializers and resolved ingredients linked to their pinned configuration.

## M08

Processing pilot and configuration. Exit with bounded Enrichment Chamber and selected configuration controls, independent FE and inventory evidence, and repeatable outcomes.

## M09

Network slices. Exit with reviewed terminal balances and topology controls for declared resource and failure cases.

## M10

Deep behavior and module slices. Exit with scoped multiblock, world interaction and add-on behavior on pinned profiles, including negative controls.

## M11

Real client and interaction coverage. Exit with hidden-client receipts for declared actions, synchronization, presentation, and client-specific failure paths.

## M12

Generated and comparative controls. Exit with supported case generation and reduction, plus reviewed reference and measurement controls for the scoped audit.

## M13

Scoped audit and reproducibility. Exit with the declared platform matrix, exclusions and signed-off evidence that the chosen scope meets the pre-port criteria.

## M14

Future Java/Rust TDD entry. Exit with a reviewed fixture and sealed Java expectation that can expose the first real candidate behavior mismatch. This milestone grants no earlier implementation authority.

## Maintainer synchronization

Preview from a reviewed checkout. Use an optional `-Ids RF-SCOPE-01` to limit the batch. Existing manual assignments are surfaced and preserved. A missing identity, a closed destination for an unassigned issue, or an unreviewed write stops application.

```powershell
./tools/sync-roadmap-milestones.ps1 -PreviewPath .lab/milestones-reviewed.json
./tools/sync-roadmap-milestones.ps1 -ApplyPreview .lab/milestones-reviewed.json -PreviewPath .lab/milestones-after.json
./tools/test-roadmap-milestones.ps1
```

Only one maintainer should apply at a time. GitHub's built-in Project automation imports open issues. Add linked PRs and reconcile their milestones manually. Native Project automation does not create repository milestones. Run authenticated writes separately from contributor PR workflows.

[GitHub built-in automations](https://docs.github.com/en/issues/planning-and-tracking-with-projects/automating-your-project/using-the-built-in-automations)
