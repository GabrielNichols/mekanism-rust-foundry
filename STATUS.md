# Implementation status

This independent local project contains the Rust validation engine and NeoForge bridge. It has no rewritten Mekanism gameplay and no Cargo dependency on Horizon. The future native mod targets Horizon through a versioned adaptation boundary, while remaining outside the game's source tree. The repository-local [planning bundle](docs/planning/README.md) is canonical for contributors; workspace reports remain historical entry points.

## Implemented and exercised

| Area | Available evidence | Scope |
| --- | --- | --- |
| Core | 140 passing tests, warning-free Clippy and formatting | Supported types/precision, comparison, artifacts, guards, capture, measurements, observation and reconciliation |
| Processes | Actual probe processes and JVMs | Arguments, cwd, logs, deadlines, supervision and fresh directories |
| Original server | A 13-item barrel fixture and one-tick advancement | Actual NeoForge/Mekanism; administrative component setup |
| Original pilot | Three recipes, 3 ores → 6 dust, 12,000 FE consumed; blocking and resumption | 635 observed ticks; declared administrative output fixture; FE supplied through the public API |
| Original client | Placement, item consumption, menu, three-ore transfer, processing and two-dust pickup | Actual keybinding/Screen handling, checked on server and client; no OS input qualification |
| Repeatability | Three fresh detailed runs and three fresh terminal runs; all same-mode traces match | 644 frames per detailed run, 45 per terminal run; three observer comparisons and four A/A comparisons pass |
| Capture | Continuity, phase, owner thread, endpoint and separate clocks; 635-tick pilot | Only implemented phases; the full roadmap's causal barriers remain open |
| Persistence and recovery | Independent Rust NBT/zlib/Anvil reading, fresh JVM reopen, and live next-tick checks | Single administrative Enrichment Chamber fixture with explicit save boundaries |
| Reconciliation | Slots, aliases and NBT describe the same items in every sample; rejected payload retained | Pilot inventory schema; not all machine or resource types |
| Performance | Actual Pre/Post, snapshot, CPU, GC and sampled-heap collection; separate percentiles | Functional measurement; no demonstrated Java/Rust improvement |

The baseline campaign is stored locally under `.lab/evidence/acceptance-01/server-1`, `server-2`, `server-3` and `client-1`. Its public metadata/hash summary is `evidence/first-campaign.json`. The successful exploratory processing pilots are summarized in `evidence/processing-pilots.json`; their raw records include `.lab/evidence/processing-04`, `lifecycle-01` and `client-processing-03`.

The expanded fresh cohort `.lab/evidence/processing-acceptance-04` completed with seven functional passes: three `every_tick` runs, three `terminal` runs and `client-regression`. All three observer-pair reports and all four same-mode A/A reports pass. Each server run advances 635 ticks, consumes three ores, produces six natural dust and spends 12,000 FE. The fixture adds 62 dust and extraction removes 64, leaving four. Progress remains at 5 while energy is absent, then resumes. Independent persisted NBT, per-sample inventory reconciliation and the Rust work ledger pass for every server run.

The final real-client run contains 223 trace frames and 200 world ticks. Actual placement, three-ore input, two-dust production and two-dust Screen pickup are confirmed on server and client. Its client control/render counters remain raw observations; this single run does not qualify client A/A repetition or OS input. The public metadata summary is [processing-campaign.json](evidence/processing-campaign.json).

[Offline comparator controls](evidence/comparator-controls.json) accept an unchanged roundtrip and reject a transient progress difference at `frames[432].state.machine.progress` and a missing intermediate frame at `frames[10].tick`, with the original Java reference unchanged. These qualify the stated offline comparisons, not an in-game sensor-fault campaign.

Raw outputs, images, resources and worlds remain in ignored `.lab/`. Earlier attempts, including Windows publication locking, exceeded boot budgets, rejected bindings and client bootstrap failures, are preserved and do not count as passes.

All results retain `qualified=false`, `port_equivalence_proven=false` and `optimization_proven=false`. A scenario's functional pass does not close G4/G5/G8 or qualify the complete machine pilot. No skip, receipt or health check substituted for the actual client flow.

The [lifecycle campaign](evidence/lifecycle-campaign.json) passed all 15 treatments across three cohorts and 25 fresh JVM processes, including the seed. Bridge-absent and every-tick runs matched the same live checkpoints and terminal NBT. Before flush, forced termination recovered the seed at progress zero. Graceful restart and forced termination after flush recovered progress 73. The next powered tick reached 74, and all treatments ended with two dust and no stored energy. The existing dedicated-server inventory scenario also passed against the rebuilt runner. Private run directories are `.lab/evidence/recovery-04` and `.lab/evidence/recovery-reference-regression-01`.

The simulated observer campaign accepted the clean control and detected eight injected faults. The retained replies include a stale state with a valid clock, and a world mutation after an otherwise valid observer read. The simulator uses the production receipt checks and its own independent world witness. [Simulated fault evidence](evidence/simulated-observer-faults.json)

The [live observer campaign](evidence/observer-controls-campaign.json) passed 15 continuous treatments across 18 JVM processes, including three seeds. The bridge-absent, installed-disabled and every-tick modes matched all four live checkpoints and terminal disk NBT across the 20 ticks/s serial, 100 ticks/s serial and 100 ticks/s three-worker profiles. The accelerated three-worker profile ran three fresh cohorts. Disabled processes loaded the pinned bridge jar and recorded the startup guard without opening a bridge session.

The [live fault audit](evidence/live-observer-audit.json) accepted three clean controls and identified six injected faults in nine treatments across ten fresh JVMs, including the seed. Three stale replies stamped with the correct tick reported progress zero while console and disk showed 73. Three authentic replies reported one ore before the observer changed the live inventory to two. The console and full typed region NBT confirmed that the ore count was the only saved change.

The [trace seal controls](evidence/trace-seal-controls.json) verified identical trace bytes from two fresh reference runs. The verifier rejected a valid-looking shortened trace, a same-length value edit, a result from a different attempt, and a missing seal. The reference and processing campaign scripts now verify new traces before comparison.

The [phase and chunk campaign](evidence/phase-chunk-controls.json) verified three real client/server runs and six dedicated chunk trials. Each client began with a hidden GLFW context. The premature capture was blocked even though the client already predicted three ore on its cursor. The server confirmed the transfer and issued revision 2. The client applied that revision and captured a subsequent rendered frame. Every trace seal verified. The runs reproduced the same semantic phases with their own client and render counters.

The nonloading chunk lookup preserved residency, holder and effective ticket state through a three-tick observation window and left no persisted target record. The loading control produced one load event and one persisted target record in each run. Its holder was already gone at the delayed read, making the event and disk witnesses necessary.

The [Core runtime bootstrap](evidence/core-catalog-bootstrap.json) matched a 20 ticks/s serial catalog with three isolated 100 ticks/s workers. It recorded loaded modules, eight registry groups, 1,578 loaded Mekanism recipe bindings, active server packs and configuration fingerprints. This diagnostic has no gameplay reference or qualification verdict.

## Qualification boundary

[VALIDATOR-CONTRACT.md](VALIDATOR-CONTRACT.md) defines the implemented contracts and limits. The lifecycle pilot adds a bridge-absent control and an independent world read after process exit. RF-CLOCK-07 has all three no-fault Java modes and a live post-read mutation detected by independent console and disk witnesses for the Enrichment Chamber fixture. Broader observation neutrality and client corroboration remain open. RF-PILOT-09 is exercised at a paused mid-recipe checkpoint in the administrative fixture. An administratively filled output stack does not close RF-PILOT-06, which requires player-driven removal of the blockage. The original roadmap's criteria and coverage denominator remain unchanged.

The next closures are a reviewed effective manifest and reference catalog; wider action, render and chunk observation coverage, and session and transport faults; menu gauges and visual checks during processing; invalid inputs, batch boundaries, and wider recovery coverage. Performance proof still needs warmup, native memory, representative workloads and an equivalent candidate. OS input remains a separate, unqualified ingress level.

## Roadmap traceability

| Source items | Prototype status |
| --- | --- |
| RF-SCOPE-01/02/03/05 | A pinned Core subset, hashes and before/after configurations; full reference closure remains unqualified |
| RF-MODEL-01/02/03/04/05/07/08/09/10 | Contracts, typed NBT and FE units exercised in the pilot; complete types/states/protocol and all verdict classes remain open |
| RF-INSTANCE-01/02/04/05/06/07/08 | Initial actual instances and teardown; isolated recovery clones exercised for one machine, with broader adverse process and storage conditions open |
| RF-BRIDGE-01/02/04/05/06 | Initial bridge and correlation; the in-game RED detected acknowledgment without the expected effect |
| RF-CLIENT-01/02/03/05/06/07/08 | Player, placement, menu, processing and Screen pickup; movement input, breaking, two clients and UI variations remain open |
| RF-CLOCK-03/04 and RF-CLIENT-06 | Frozen server precondition, single real Screen pickup, independent server application, authoritative menu revision, and a later render. Early capture is rejected even with a predicted cursor |
| RF-CLOCK-06 | Three fresh pairs of resident-only and allocating lookups, checked through owner-thread witnesses, load events, three ticks and independent region lookup |
| RF-CLOCK-03/07/08 and RF-EVIDENCE-07 | Frozen audit with a phase/owner envelope, independent console and disk witnesses, three clean runs, and two live fault types. Broader phases and client effects remain open |
| RF-EVIDENCE-01/02/04 | New reference trace seal and attempt binding exercised against real archived traces. Sealing observations, timing and other raw artifacts remains open |
| RF-PILOT-01/02/03/05/06/08 | Recipe, resources, blocking and processing slots exercised; blocked-output removal still uses a fixture/capability; gauges, invalid input and batch cases remain open |
| RF-PILOT-09 / RF-LIFE-01/02/06/08 | Mid-recipe pause, fresh reopen, durable continuation, and process kill before/after flush in one controlled fixture |
| RF-NET / RF-UX | Network and client recovery remain open |

These rows describe evidence for subsets, not completion of entire work items. The 138 planned items retain their exit criteria; this prototype has not promoted a broad gate to qualified.

## Next increment

1. Extend phase and chunk coverage beyond the pickup and single-coordinate fixtures, and characterize session and transport failures.
2. Expand the typed protocol and effective catalog, including negotiated actions/observables and classified errors.
3. Complete the pilot's invalid-input, batch, gauge and wider restart cases; review and seal Java expected behavior before any port gameplay.
4. Extend time, memory and completed-work measurement with full instrumentation controls, warmup and repeated trials.
5. Start the Horizon adapter and first Rust mod behavior only after the applicable scope gate, preserving the Java reference.

Windows x64 is the exercised environment. Linux, the declared MSRV, complete transport/Job Object security, universal NBT support, shrinking, audio and interoperability are not qualified. The independent lab and first-wave issue queue are public. [Lab repository](https://github.com/GabrielNichols/mekanism-rust-lab) · [Umbrella issue](https://github.com/GabrielNichols/mekanism-rust-lab/issues/7)
