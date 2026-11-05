# First-wave workstreams

The six IDs below identify local workstreams. Canonical requirements, scenarios and qualification gates remain in the [planning baseline](README.md). The owner labels are proposed roles. Each lane remains open for a contributor to claim after publication approval. Drafting a lane, writing its case, observing Java, and approving its reference are separate steps.

Use the [initial inventory crosswalk](FIRST-WAVE-INVENTORY.md) for object-level status, the [status tracker](../../STATUS.md) for existing evidence and the [draft issues](CONTRIBUTION-DRAFTS.md) for contributor assignments. Keep the Enrichment Chamber pilot and its negative controls as regressions. These lanes are follow-up contributions beyond the small setup and reproduction PR.

## Ownership and dependencies

| ID | Proposed owner | Owned artifacts or contracts | Independent start | Required before live acceptance |
| --- | --- | --- | --- | --- |
| W1-CAT | Reference catalog contributor | Effective catalog export contract, profile census and object crosswalk | Static census from verified local JARs and archived run identity | Integrator-approved read-only post-load export and effective lock |
| W1-PROC | Processing scenario contributor | Factory scenario, lane witnesses and work ledger expectations | Factory case design and Enrichment pilot replay | Effective recipe, tier and slot map from W1-CAT. Per-lane observables from W1-ENG |
| W1-ENER | Generation scenario contributor | Generators profile proposal and Heat Generator fixture | Profile checklist, fuel and idle fixture design | Pinned Generators runtime registration from W1-CAT. Typed energy and fuel observation from W1-ENG |
| W1-TRANS | Chemical transport scenario contributor | Straight tube topology, chemical identity and terminal balance | Chemical identity and topology checklist | Runtime chemical and tube bindings from W1-CAT. Typed terminals and network buffer from W1-ENG |
| W1-LIFE | Multiblock and lifecycle scenario contributor | Dynamic Tank fixture, durable checkpoint and restart expectations | Geometry diagram and block/port checklist | Runtime block and port bindings from W1-CAT. Formation, identity and restart witnesses from W1-ENG |
| W1-ENG | Validator contributor | Comparator controls, worker supervision, measurements and evidence | Synthetic fault fixtures and existing Java control replay | Integrator review of shared protocol, runner and evidence changes |
| Integration | One appointed maintainer or reviewer | Shared bridge, schema, runner, profile manifests and final validation | Review contracts and triage gaps in parallel | Independent evidence review and an explicit scope verdict |

### W1-CAT · Effective Core catalog

- **Slice.** Export loaded module IDs and versions, selected Core registry entries and the resolved recipe for one Enrichment Chamber input. Record effective configuration and datapack fingerprints. Separate archive paths, nominal definitions, runtime registry data and resolved recipes. Identify menus, resources and configuration domains that need additional profile or client evidence.
- **Roadmap.** RF-SCOPE-05/06 and RF-DISCOVER-01/02/03/07. RF-DISCOVER-04/05/06 retain their own coverage requirements.
- **Acceptance and control.** Two fresh dedicated boots from the verified profile produce an exact canonical export for the selected entries. Every exported ID has a family or an explicit gap. Deliberately declare an absent module or missing live recipe and require the checker to identify the discrepancy before reference selection.
- **Evidence.** Profile and artifact IDs, loaded module list, selected registry/recipe keys and types, effective fingerprints, canonical diff and gap rows. The diagnostic bootstrap and reviewed effective lock are separate.
- **Starter W1-CAT-01.** Reconcile one Core block and recipe key across the planning row, pinned archive and available runtime witness.

### W1-PROC · Two observable factory lanes

- **Slice.** Characterize one Basic Enriching Factory with two observable active lanes after discovering tier, recipe, slots and sorting mode. Supply controlled FE through a declared fixture. Keep the Enrichment Chamber case and ledger intact.
- **Roadmap.** MK-MAC-010.R01/R02, RF-DISCOVER-03/04/06 and RF-EVIDENCE-07. SC-LAB-003/004 provide pilot context.
- **Acceptance and control.** Record lane input, output, progress and shared energy expenditure at completion and blocked boundaries. An actual blocked lane must have an independent witness. An offline transient lane mutant with unchanged final totals must fail at the first incorrect lane and phase. A separate real-client Screen scenario is needed for menu interaction credit.
- **Evidence.** Recipe and lane bindings, initial state, ordered trajectory, work ledger and independent saved or client witness. Sorting, heterogeneous recipes, contention, upgrades and reload keep their own cases.
- **Starter W1-PROC-01.** Draft one blocked-lane case with two boundary witnesses, and mark every unresolved bridge binding.

### W1-ENER · Fuel-driven Heat Generator

- **Slice.** Pair a fueled and unfueled Heat Generator under a separately pinned Core + Generators profile. Discover fuel and declare the thermal environment. Exclude passive environmental energy from the fixture or measure it separately. Generator-to-consumer delivery remains a follow-up.
- **Roadmap.** MK-GEN-001.R01 and RF-MODS-07. MK-GEN-001.R02, RF-NET-01 and SC-LAB-021 cover later delivery proof.
- **Acceptance and control.** Verify the loaded module and block identity. Observe fuel debit and remainder, exact buffer change and environmental preconditions. Distinguish produced, stored and delivered energy. Hold the environment and initial buffer fixed, remove fuel, and require an independent idle or passive baseline. Reject a trace that credits fuel-driven energy without the corresponding debit.
- **Evidence.** Profile and config fingerprints, fuel provenance, exact typed ledger, tick trace and independent terminal witness.
- **Starter W1-ENER-01.** Draft the Generators profile entry and two controlled preconditions. Leave unresolved artifact identities and observations explicit.

### W1-TRANS · One straight chemical line

- **Slice.** Characterize one Basic Pressurized Tube line, one source, one sink and one compatible chemical in a declared component fixture. Exercise a separate incompatible terminal or resource. Branches, merge/split, fluids and cross-chunk transport remain separate.
- **Roadmap.** MK-CHE-003.R01, RF-DISCOVER-05 and RF-NET-02. SC-LAB-009 supplies the incompatibility design seed.
- **Acceptance and control.** Verify connection identity and compare source debit, network residual, sink credit, exact unit and transfer order at tick boundaries. Use an incompatible chemical with the same numeric quantity. Verify Java rejection and require the comparator to detect loss of chemical identity. The straight-line fixture leaves the two-client requirement open.
- **Evidence.** Topology manifest, typed terminal and buffer ledger, accepted and rejected action receipts and independent terminal reads.
- **Starter W1-TRANS-01.** Draw the smallest tube line, label each read point with the resource ID and unit, and identify the unbound capabilities.

### W1-LIFE · Dynamic Tank formation and durable reopen

- **Slice.** Discover a supported minimum Dynamic Tank geometry and valve layout. Form it, invalidate it by removing one component, then reopen a saved checkpoint in a fresh JVM and observe reconstruction. Keep administrative setup and a real hidden-client build/break action separate.
- **Roadmap.** MK-STO-003.R01/R02, RF-DEEP-01/02 and RF-LIFE-01/02. The full SC-LAB-013 design retains its two-client scope.
- **Acceptance and control.** Establish formation and invalidation causally through observed block changes. Observe identity, ports, capacity and typed contents. Corroborate the saved checkpoint independently and inspect identity and the next transition after reload. Remove a required component or reopen a pre-durability checkpoint and require the appropriate divergence. Detect reconstruction that credits the same contents twice.
- **Evidence.** Geometry, ordered trace, flush/close acknowledgement, region or saved-data witness and new JVM identity. Keep pristine and failed worlds private.
- **Starter W1-LIFE-01.** Write a block-position checklist and the missing-block control. Keep geometry and port bounds provisional until runtime discovery.

### W1-ENG · Validator, workers and evidence

- **Slice.** Add a deterministic comparator mutant with a known first divergence and one bounded session fault in simulation. Exercise current worker profiles on supported Java work first. Preserve prior observer and seal controls.
- **Roadmap.** RF-EVIDENCE-01/02/04/05/06/08, RF-CLOCK-08 and RF-AUDIT-06.
- **Acceptance and control.** An unchanged fixture passes. A changed intermediate type, value or phase fails at the expected first difference even when endpoints match. A wrong-session or invalid-frame control cannot publish a pass. Record worker allocation, fingerprints, fresh attempt identities and every outcome. Track remaining session faults and non-trace sealing separately.
- **Evidence.** Intended validator RED/GREEN, synthetic input hashes, exact diagnostics, live replay where claimed and a measured profile report.
- **Starter W1-ENG-01.** Add a transient typed-value fixture with an independently specified difference path.

## Isolation and integration

1. Use separate implementation worktrees once a repository baseline exists. Until then, use disjoint authoring paths in the local lab. Keep each lane's game directories, configs, logs, temporary files, worlds, owned processes and loopback ports isolated. Use a fresh private attempt directory under `.lab/evidence/first-wave/<lane>/<attempt>`. Keep the pristine seed and reference inputs read-only.
2. Lane owners propose the smallest contract change they need. One integrator serializes edits to the bridge, schema, runner, and shared reference values. The integrator records affected scenario IDs and reruns their regressions against the pinned integration artifacts. Domain-specific case authoring can proceed in parallel.
3. A handoff records profile, artifact and effective fingerprints, family and scenario IDs, topology and ingress, preconditions, exact typed observations, independent witness, discriminating control and verdict. Attach attempt IDs, digests and sanitized metadata. Preserve raw artifacts, negative controls and failed attempts privately. A follow-up run gets a fresh attempt ID.
4. A reviewer checks the trace seal, provenance and attempt binding, then assesses the declared scope against G8 with open required cells visible. Java characterization, validator TDD and future G9 gameplay TDD are separate. A fixture pass or trace seal alone cannot qualify a family or authorize gameplay porting.

## Measured campaign gate

The existing Java observer campaign exercised 20 ticks/s with one worker and 100 ticks/s with one and three workers. Live checkpoints and terminal disk NBT matched for that workload. The serial cohort took 110.93 seconds at 20 ticks/s and 117.76 seconds at 100 ticks/s. The 100 ticks/s three-worker spans ranged from 39.86 to 116.27 seconds. Startup and contention affect these harness measurements. [Observer campaign](../../evidence/observer-controls-campaign.json) · [Benchmark method](../../BENCHMARKS.md)

1. **Establish the serial reference.** Run each new scenario at 20 ticks/s with one worker. Retain at least three fresh Java attempts and applicable controls for reference selection. Use dedicated servers without a UI. When menu, input or render proof requires a real client, use the GLFW context hidden from creation and record the applicable causal barriers.
2. **Isolate tick rate.** Compare an otherwise equivalent 100 ticks/s single-worker run. Pin artifacts, effective configs, scenario, seed, ingress and planned work. Verify phase order, commanded and completed ticks, exact typed state and quantities, per-resource debits and credits, and applicable independent disk or client witnesses. Client scenarios whose input or rendering can affect semantics need their own measured comparison.
3. **Isolate worker count.** Compare 100/1 and 100/3 on the same job set, with one isolated Java instance per independent job. Keep every attempt and measure the increase before considering greater concurrency. Divergence, missing evidence, skipped work and resource-pressure failures block acceptance for that scenario/profile.
4. **Separate measurements and semantics.** Report boot/readiness, scheduled and verified tick execution, observer cost and I/O, flush, exit and total campaign wall time. Distinguish overlap from summed worker time. Record host, worker count, warmup and setup assumptions. Observer modes require their own semantic comparison. Preserve raw clocks and causal relationships and declare any comparison projection.
5. **Select by evidence.** A reviewed serial Java attempt and its verified controls anchor the scenario. Seal the selected trace and record its provenance, comparison scope and gaps. Keep `qualified`, `port_equivalence_proven`, and `optimization_proven` independent. Infrastructure errors and unsupported actions remain separate verdicts from gameplay mismatch. [TDD and reference selection](TDD.md)
