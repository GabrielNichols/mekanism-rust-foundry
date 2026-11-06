# Scenario authoring

1. Choose one observable behavior and link its requirement and work item. Check [scenario designs](../planning/SCENARIOS.md) and [binding provenance](../../cases/SOURCES.md) for the relevant profile and gaps.
2. Declare the pinned artifacts and effective configuration, topology, initial state, actors, ingress, typed values and units, phases, tick budget and deadline. Mark unresolved bindings and name the evidence that would resolve them. [Work packages](../planning/FOUNDRY-WORK-PACKAGES.md)
3. Specify independent checks for the effect and its order. A client case distinguishes consumed input, server application, fresh client revision and later render where relevant. A restart case identifies the flush boundary, persisted witness and next transition in a fresh process.
4. Define a negative control that would expose a failure even when final totals agree. A transient progress difference with unchanged output should fail at the first affected phase. Classify setup failures separately from the intended assertion. [TDD contract](../planning/TDD.md)
5. When runtime bindings are available, materialize the case, use fresh isolated attempts, run the matching controls and verify the trace. Review and seal the Java expectation before any future candidate comparison. [Game validation](../../CONTRIBUTING.md#actual-game-validation) · [Validator contract](../../VALIDATOR-CONTRACT.md)

For a [straight chemical line draft](https://github.com/GabrielNichols/mekanism-rust-foundry/issues/4), specify one source, sink and chemical, then write an incompatible-terminal control. Document the source debit, residual buffer, sink credit and exact unit. Link the binding and ledger gaps before proposing execution.

[Wiki index](../WIKI.md)
