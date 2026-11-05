# Contribution drafts

These are English drafts for review. The workstream IDs are planning identifiers, and every owner is a proposed role awaiting a contributor. Publication, repository creation, and external contact require explicit approval.

## Umbrella issue

### Open the first Mekanism Java characterization wave

We are building an independent Rust parity laboratory that observes pinned Mekanism Java behavior and prepares reviewed references for later reconstruction. The laboratory can stand on its own. Original Mekanism is the reference runtime, and a future Horizon adapter is the planned integration boundary. Gameplay implementation in Foundry starts after scoped pre-port qualification.

The current Enrichment Chamber pilot and its positive and negative controls are the baseline for the first wave. Preserve the original artifacts and observations. The status tracker distinguishes validated infrastructure, observed behavior, and open qualification. [Current status](../../STATUS.md)

- [ ] **W1-CAT** — Capture a small effective Core catalog and reconcile stable object IDs.
- [ ] **W1-PROC** — Characterize two observable lanes on a Basic Enriching Factory.
- [ ] **W1-ENER** — Characterize the Heat Generator with a fuel and environment control.
- [ ] **W1-TRANS** — Characterize a Basic Pressurized Tube on a straight line.
- [ ] **W1-LIFE** — Characterize minimal Dynamic Tank formation, break, and reopen.
- [ ] **W1-ENG** — Validate comparator sensitivity and the worker/evidence boundary.

A contribution starts with a single pinned profile, object, or reproducible control. Track mapping, runtime confirmation, scenario authoring, execution, and reference review and sealing separately. A trace seal verifies artifact integrity and the reference review decides whether the evidence is ready to serve as an expectation. Record excluded modules, variants, and unobserved outcomes. Treat family and pre-port qualification as explicit scoped reviews. [Roadmap](ROADMAP.md)

Use the 20 ticks/s serial profile for the Java reference. Validate each scenario's checkpoint semantics before accepting a faster run at 100 ticks/s with three isolated workers. Separate startup, tick execution, observer work, flush, and total time. Run dedicated scenarios headless. Create a real client hidden from startup when the scenario needs a menu, input, or rendering. [Measurement protocol](../../BENCHMARKS.md)

Keep JARs, worlds, screenshots, native and other private resources, and raw logs under the private evidence tree. Include reviewable manifests, summaries, and hashes in contributions after checking their contents. Keep authored material in English. [Contribution guide](../../CONTRIBUTING.md)

## First-wave issues

### W1-CAT — Reconcile one loaded Core profile

**Proposed owner** — Effective catalog contributor. **Starter task** — Bind the loaded Core module and Enrichment Chamber registry entry in a single profile.

On a fresh dedicated server, witness the loaded module and collect a small registry and resolved recipe slice. Add stable object IDs and link each object to its family, behavior, scenario, and evidence. Record the effective configuration fingerprint and distinguish JAR resources from loader-confirmed entries. Put candidate files under the catalog workstream directory. Route bridge or schema changes through the integrator.

**Acceptance** — A second fresh attempt reproduces the declared slice, and a missing required module or registry object is reported as a profile deviation. Retain the source artifact digest, loader/read receipt, resolved recipe provenance, and unmapped or excluded entries. [RF-DISCOVER-01 through RF-DISCOVER-04](ROADMAP.md)

### W1-PROC — Characterize two Basic Enriching Factory lanes

**Proposed owner** — Processing and factories contributor. **Starter task** — Resolve the runtime factory and recipe bindings, then write one two-lane case.

Choose one recipe in the pinned Core profile. Observe two lanes independently through input, progress, energy, and output. Declare fixture injection and leave the Enrichment Chamber pilot untouched. Start on a dedicated server. Record an explicit gap if the current observer cannot distinguish the lanes.

**Acceptance** — The serial trace identifies both lane histories and the observed result. A negative control substitutes the same aggregate output while moving a per-lane event to the other lane. The comparator rejects that difference at the affected lane. Submit the binding provenance, case, trace seal, and outcome summary. [Factory inventory and pilot trace discipline](INVENTORY.md#mk-mac-010--factories) · [RF-PILOT-03](ROADMAP.md)

### W1-ENER — Measure Heat Generator fuel and buffer behavior

**Proposed owner** — Energy and generation contributor. **Starter task** — Describe the isolated Generators profile and one independently observable fuel case.

Record the pinned module pair, declared environment, starting fuel and buffer, and owner-thread energy observations. Use a bounded dedicated case and select a control environment whose baseline behavior is characterized independently. Stage the generator-to-consumer cable path as a separate task.

**Acceptance** — The trace distinguishes fuel use, internal buffer changes, and externally delivered energy with typed units. The declared comparison catches continued fuel-derived output after removing the intended fuel contribution. Until the add-on profile boots and the fuel case runs, report the entries as mapped or blocked. [MK-GEN-001](INVENTORY.md#mk-gen-001--heat-generator) · [RF-MODS-07](ROADMAP.md)

### W1-TRANS — Trace one chemical through a Basic Pressurized Tube

**Proposed owner** — Transport and chemicals contributor. **Starter task** — Bind one chemical, a straight tube, and independent source and sink observations.

Characterize one accepted transfer and one incompatible-input or incompatible-receiver attempt on the pinned dedicated profile. Retain per-terminal quantities, units, resource identity, topology, and the first accepted or rejected transfer. Keep branch, cycle, merge, and chunk-unload behaviors as explicit follow-up gaps.

**Acceptance** — The supported path reconciles its terminal balances, and the incompatible control witnesses rejection without contamination or an unexplained debit. Submit both receipts and an evidence summary with provenance for the chosen chemical. [MK-CHE-003](INVENTORY.md#mk-che-003--pressurized-tubes) · [RF-NET-02](ROADMAP.md)

### W1-LIFE — Form, break, and reopen a minimal Dynamic Tank

**Proposed owner** — Multiblocks and lifecycle contributor. **Starter task** — Write a blueprint and the first missing-block control.

Characterize one valid structure, then remove an identified structural block and reopen the world in a fresh JVM. Record the formation action path, tank identity, valve and content state, save boundary, and first post-reopen transition. When a case claims real player construction, use a client created hidden and capture independent server effects. Label fixture construction separately.

**Acceptance** — The missing-block control fails formation or invalidates the previously formed tank as witnessed in the pinned Java profile. The fresh JVM provides the continuation evidence. Record contents and any disappearance or duplication without assuming how Java will behave. [MK-STO-003](INVENTORY.md#mk-sto-003--dynamic-tank) · [SC-LAB-013](SCENARIOS.md#sc-lab-013--multiblock-forms-and-invalidates-through-real-construction) · [RF-LIFE-02](ROADMAP.md)

### W1-ENG — Gate accelerated attempts on complete and equivalent evidence

**Proposed owner** — Engine and evidence contributor, with shared interfaces owned by the integrator. **Starter task** — Add one deterministic mutant with a named first-divergence assertion.

Use a private simulated trace to introduce an intermediate difference while preserving the endpoint. Preserve an unchanged positive control and the semantic RED/GREEN logs. Then use one already executable Java scenario to check 20 ticks/s serial and 100 ticks/s three-worker scheduling with isolated worlds. Record checkpoint and completed-work equivalence before comparing timing phases.

**Acceptance** — The mutant fails at its changed phase, and a missing worker result or stale attempt manifest remains an incomplete or rejected campaign. Preserve all attempted runs and record startup, tick execution, observation, flush, and total wall time separately. Declare warmup, cohort, and host constraints. Changes to the runner, schema, and bridge enter through the integrator. [RF-INSTANCE-09 and RF-EVIDENCE-01 through RF-EVIDENCE-06](ROADMAP.md) · [Benchmarks](../../BENCHMARKS.md)

## Copyable task template

```markdown
## Scope

- Workstream, stable inventory row, and roadmap requirement
- Object, variant, behavior, and scenario ID
- Pinned Minecraft, NeoForge, Mekanism module hashes and effective config
- Actor, input path, topology, fixture assumptions, and exclusions
- Proposed owner, writable paths, dependencies, and integrator review

## Observable contract

- Initial state and independently sourced expectation
- Action, phase, tick budget, units, and expected transition
- Independent witness, including the exact checkpoint or persisted state
- Negative control and expected first rejection reason

## Evidence checklist

- [ ] Mapped
- [ ] Confirmed in the pinned Java runtime
- [ ] Scenario written and reviewed
- [ ] Fresh run executed with each attempt outcome retained
- [ ] Reference reviewed and sealed for this exact scope

- Runtime fingerprint, invocation, trace seal, and sanitized summary
- Requested profile, workers, completed work, and separate timing phases
- TDD RED and GREEN for any validator change
- Remaining gaps and the current scoped qualification verdict
```

## First PR draft

### Reconcile the first Core registry and recipe pair

**Closes [#1](https://github.com/GabrielNichols/mekanism-rust-foundry/issues/1)**

- Review `mekanism:enrichment_chamber` and `mekanism:processing/iron/dust/from_ore` against the pinned archive and the Core runtime catalog.
- Add or refine one inventory row with the registered IDs, type, serializer, and evidence links. Keep tag resolution and behavior reference review explicit.
- Keep the PR scoped to inventory and sanitized evidence. The existing Enrichment Chamber pilot remains the machine regression.

**Verification plan**

```powershell
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo build --locked
./adapters/neoforge/prepare-reference.ps1 -Jdk '<JDK21-directory>'
./target/debug/mek-lab.exe run-reference .lab/environment.json cases/server-inventory.json "$PWD/.lab/evidence/onboarding-001"
./target/debug/mek-lab.exe verify-reference "$PWD/.lab/evidence/onboarding-001"
```

Use a different output name for each reproduction. Attach the sanitized attempt status and trace digest after the run. In a private copy, remove one trace frame and verify that the original seal rejects it. Keep the original evidence intact. This run verifies the bounded setup, process, and evidence path. It uses a barrel fixture in a mod-loaded world and leaves Mekanism machine characterization to the named cases and campaigns. [Setup and contract](../../README.md#run-on-windows-x64) · [Trace seal controls](../../evidence/trace-seal-controls.json)

## Integration discussion

Use Horizon Discord to invite contributors and discuss a future native mod interface. Keep the campaign and triage in the independent lab. Ideas for Horizon go through its hub or `/suggest`. A concrete upstream bug or bounded integration change can become an issue or PR after maintainer alignment. [Horizon contributor guide](https://github.com/horizon-rust/horizon/blob/main/CONTRIBUTING.md) · [Mod documentation](https://github.com/horizon-rust/horizon/blob/main/docs/mods.md)

## Horizon Discord message draft

We are opening an independent Rust parity lab for Mekanism. It uses pinned Mekanism Java as its reference and aims for a future native Horizon mod after scope qualification. You can help by [reconciling one registry and recipe key](https://github.com/GabrielNichols/mekanism-rust-foundry/issues/1), [drafting a factory lane fixture](https://github.com/GabrielNichols/mekanism-rust-foundry/issues/2), or [adding a worker-evidence control](https://github.com/GabrielNichols/mekanism-rust-foundry/issues/6). The [lab and task queue](https://github.com/GabrielNichols/mekanism-rust-foundry) contain the setup and evidence requirements. We welcome discussion with Horizon modding maintainers about eventual integration. This is an independent community effort.
