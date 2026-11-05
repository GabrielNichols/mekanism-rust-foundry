# Compatibility planning baseline

This repository-local bundle is the canonical requirements and qualification baseline for contributors. It preserves the original IDs, schemas, counts and acceptance criteria while making the authored descriptions available in English. The workspace reports outside this repository remain historical entry points; a standalone clone uses this directory.

Start with the [roadmap](ROADMAP.md), [specification](SPECIFICATION.md), [inventory](INVENTORY.md), [TDD contract](TDD.md), [functional laboratory](FUNCTIONAL-LAB.md) and [scenario designs](SCENARIOS.md). [Sources](SOURCES.md) distinguish nominal facts, runtime evidence and engineering proposals.

Machine-readable companion files are `roadmap.json`, `inventory.json`, `requirements.json`, `functional-scenarios.json`, `instance-profiles.template.json` and `profiles.reference.template.json`. Draft templates remain non-executable until their required bindings, artifact identities, expected results and qualification controls are resolved.

The baseline contains 117 behavior families, 253 requirements, 138 granular work items and 22 scenario designs. These are coverage and planning counts, not counts of implemented or qualified behavior. Historical planning-time fields such as `planned_not_executed`, missing runtime bindings or zero qualified requirements are retained rather than silently promoted by later infrastructure work.

[Current implementation status](../../STATUS.md), the [validator contract](../../VALIDATOR-CONTRACT.md) and the [TDD evidence index](../../evidence/tdd/README.md) describe actual delivered scope. A functional run, a unit-test total or acceptance of one supporting work item does not close a broad gate or cover all dependent families. Gameplay porting begins only after the applicable validator/reference scope has passed its qualification gate.


[GitHub issue index](GITHUB-ROADMAP.md) maps all 138 roadmap IDs to issues. [Foundry work packages](FOUNDRY-WORK-PACKAGES.md) group the inventory into contribution slices.

## First wave

The [umbrella issue](https://github.com/GabrielNichols/mekanism-rust-foundry/issues/7) links the six reviewed issues. The [starter issue](https://github.com/GabrielNichols/mekanism-rust-foundry/issues/1) is suitable for a small crosswalk PR.

Use the [collaboration plan](COLLABORATION-PLAN.md), [first-wave workstreams and starter tasks](FIRST-WAVE-WORKSTREAMS.md), [object-to-evidence crosswalk](FIRST-WAVE-INVENTORY.md), and [English contribution drafts](CONTRIBUTION-DRAFTS.md). The [issue queue](issue-queue.json) is the versioned input for the read-only [batch preview](../../CONTRIBUTING.md#preview-an-issue-batch).
