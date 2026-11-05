# Implementation in progress

The laboratory lives in an [independent project](../../README.md), with its own Cargo workspace and local Git repository. It has no Cargo dependency on Horizon and does not change Horizon's source. The future Mekanism fork remains separate from the game and will target Horizon through a versioned adapter.

[Status and traceability](../../STATUS.md) record the implemented subset, its limits and the next work. The [TDD index](../../evidence/tdd/README.md) preserves actual RED/GREEN runs while separating preparation errors. The [performance protocol](../../BENCHMARKS.md) defines the requirement to demonstrate lower cost under equivalent work and results. [CONTRIBUTING.md](../../CONTRIBUTING.md) explains how others can extend the validator and its evidence.

The original Java runtime now exercises a dedicated-server recipe pilot and a real client. The pilot records per-tick input/output/energy/progress, full-output blocking, finite-energy pause and resumption, independently decoded saved NBT and a Rust work ledger. The real client places the Enrichment Chamber, opens its menu, transfers ore, observes production and picks up output through Screen handling, with server/client checks. The expanded fresh cohort compares capture frequencies and same-mode A/A repetition; its final outcome is reported in the laboratory status and campaign record.

These results do not establish future-port parity or optimization. The 117 families, 253 requirements and 138 granular work items remain the coverage contract. No complete family or broad gate has been promoted to qualified. The planning bundle remains a requirements baseline rather than a record of completed runtime qualification.

