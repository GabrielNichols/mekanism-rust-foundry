# Mekanism parity engine specification

Document version: 0.2, 2026-11-05. Status: **implementation proposal**, with no engine, dynamic bridge or qualification runs in this delivery. The engine and all its qualification results remain pending. The [inventory](INVENTORY.md) now contains 117 families/253 requirements; the [roadmap](ROADMAP.md) details 138 microdeliveries. The [laboratory](FUNCTIONAL-LAB.md), [TDD](TDD.md), [scenarios](SCENARIOS.md) and [non-executable profile](profiles.reference.template.json) complete this contract.

## 1. Objective and compatibility boundaries

The product is a tooling engine written in Rust that prepares scenarios, supervises processes, controls execution, collects traces, compares states, generates variations, reduces failures and reports coverage. Java is restricted to an original development bridge inside the original game. The new engine requires no Python in its execution path; legacy Python tools remain independent. This architecture contains no automatic Java→Rust translator.

The profiles support different conclusions. `behavior` compares the semantic effects of actions and states over time. `persistence-semantic` compares preservation and evolution after save/load without requiring binary-identical files. `interoperability`, if implemented, will need additional contracts for saves, migrations and communication. Neither of the first two implies binary jar compatibility, Minecraft 1.21.1↔26.3 network protocol compatibility or native reading of the same save.

The first candidate reference is tag `v1.21.1-10.7.19.85`, commit `a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80`. The tag metadata declares Java 21, Minecraft 1.21.1, build NeoForge 21.1.200 and minimum 21.1.194. NeoForge 21.1.259, used by the existing oracle, is an explicit candidate that has not yet been qualified in this combination. ([Tag](https://api.github.com/repos/mekanism/Mekanism/git/ref/tags/v1.21.1-10.7.19.85), [metadata](https://github.com/mekanism/Mekanism/blob/v1.21.1-10.7.19.85/gradle.properties)).

Module profiles: Core; Core+Generators; Core+Tools; Core+Additions; and all four together. Absent and present modules are distinct conditions. Add-ons and external integrations use their own profiles, with pinned licenses, versions, dependencies and contracts; the baseline does not represent the complete ATM10 pack.

## 2. Rust architecture and adaptation points

Start with a dev-only package independent of the simulation and renderer, with a testable library and CLI. Working name: `mod-compat`. Split into crates as boundaries prove necessary; this delivery created no new crate. Dependencies and toolchain will be pinned in `Cargo.lock` and the implementation manifest after checking MSRV and builds on the declared platforms.

| Proposed module | Responsibility | Required independence |
|---|---|---|
| `model` | IDs, scenarios, typed values, events, verdicts | No Minecraft, IO or renderer |
| `protocol` | Frames, handshake, sequence, capabilities and limits | Same contract in Rust and the Java bridge |
| `runner` | Processes, scratch, clock, barriers and shutdown | Versioned adapter interface |
| `discover` | Effective catalog and reconciliation | Runtime data, without inferring registration from files |
| `trace` | Snapshots, events and integrity | Raw evidence preserved |
| `compare` | Typed equality and first divergence | Does not produce or modify expected results |
| `generate` | Boundaries, sequences and combinations | Seed, catalog and expansion recorded |
| `shrink` | Scenario reduction with reproduction | Preserves assumptions and failure class |
| `coverage` | Requirements→scenarios→observables→evidence | Does not promote a fixture to integration |
| `report` | JSON, human diagnostics and JUnit | Derived from intact evidence |

`JavaNeoForgeAdapter` controls the real reference through an original bridge. `ReplayAdapter` exercises codecs/diff and remains identified as synthetic. `HorizonAdapter` belongs to the later port phase. The Mekanism action and observable vocabulary stays separate from generic framing, supervision and comparison. The Horizon adapter must not freeze the current mixin structure: the modding API continues evolving toward typed contracts, Host/handles/Parts. ([Current direction](https://github.com/arismoko/horizon/issues/771)).

Proposed CLI operations: validate specification, discover catalog, run, compare, generate, reduce, report coverage and qualify. These names describe requirements; **they are not available commands**. Execution does not silently download versions, and offline comparison neither launches the game nor rewrites goldens.

## 3. Provenance and executable reference

The lock must resolve SHA-256 hashes for the game, four selected modules, transitive dependencies, loader, JDK vendor/build, bridge, runner and policies. Record effective settings after load, datapack/resourcepack order, initial world, seeds, accepted JVM arguments, platform, scheduler and schema. A source commit does not replace the hash of the executed jar. A null hash or unknown required catalog blocks qualified execution.

Discovery records the modules actually loaded and enumerates blocks/states, items/components, chemicals/fluids, entities/block entities, menus, recipes/serializers/resolved ingredients, effects, attributes, sounds, particles and any other revealed registries. It also records publicly accessible tiers, modes, upgrades, modules and settings. Each object needs a link to a capability or an explicit classification as passive data, vanilla, dependency or justified exclusion. An orphan object is a gap. Complete registration does not prove complete behavioral coverage.

Whenever any effective input changes, generate a new fingerprint and invalidate affected evidence. Capture catalog differences before approving the update. Do not import a legacy green result as dynamic qualification. The current exporter and harness are sources of knowledge and formats, not the new executor. ([Legacy oracle](https://github.com/horizon-rust/horizon/blob/main/tools/neoforge_oracle.py), [existing Rust formats](https://github.com/horizon-rust/horizon/blob/main/crates/horizon-port-pack/src/oracle.rs)).

## 4. Session, transport and failures

The state machine is `spawned → hello → negotiated → ready → world_open → fixture_ready → running/paused → closing → stopped`. Rejected transitions must terminate with diagnostics. The session negotiates protocol/schema, environment, dedicated/integrated/client mode, limits, action/observation capabilities and clock semantics. An incompatible version, unknown type or missing required capability is an error before comparison.

The spike must select a local transport with one versioned framing format. Proposal: length-prefixed frames with typed JSON payloads; logs use a separate channel. The channel uses an ephemeral session, monotonically increasing sequence, action correlation, tick/phase and payload integrity. Do not expose the channel on external interfaces. Local authentication and final transport need a recorded decision before qualified execution.

Reject excessive size, truncation, invalid checksum, duplicates, gaps and invalid order. Reconnection does not authorize repeating a mutation: a lost ACK leaves the run inconclusive until idempotency by ID is demonstrated. Do not blindly retransmit insertions or extractions. A missing required snapshot/event is never equality.

Run verdicts: `pass`, `mismatch`, `unsupported`, `inconclusive`, `flaky`, `infra_error`, `invalid_spec`, `cancelled`. All except `pass` block a mandatory requirement. `not_applicable` requires explicit profile exclusion and its own denominator. Do not use zero comparisons, a process exiting 0, an existing file, a receipt or the absence of mismatch as proof of success.

## 5. Clock and capture without changing the world

The simulation tick is an integer, separate from wall clock, render frame and transport. Proposed phases: `before_action`, `after_action`, `end_server_tick`, `client_applied`. The location of each phase must be demonstrated in the bridge. Actions and capture occur on the world-owning thread. Advancing N ticks preserves normal server processing rather than calling only a machine's tick.

A snapshot is atomic at its declared barrier. Getters must not advance RNG, load chunks, recalculate networks, clear queues or change ticker order as a side effect of observation. Asynchronous operations require an explicit causal barrier; elapsed time is not equivalent to ticks. The environment pins and records controllable RNG sources and execution order; unexplained nondeterminism blocks the exact profile.

Independent controls compare bridge absent where feasible, bridge loaded/capture off, and capture on. Where a bridge-free control does not exist, disable observation while retaining common control and compare saved state, inventories through the player/client or another independent observable. The spike must demonstrate equivalent schedules for these controls. A/A with two identical observers may share the same error and does not replace this control.

## 6. Lossless representation and equality

State is a typed tree, not a map of `f64` numbers. Integers use explicit width and validated decimal representation, including values above 2^53. Floats carry bits, for example `{"type":"f32","bits":"80000000"}`. Preserve signed zero, subnormals, infinities and NaNs. Energy and chemicals must use **the actual discovered representation and unit**; do not assume long, float, decimal or J↔FE. No unit conversion is implicit.

NBT preserves tags, homogeneous lists and distinct arrays. `absent`, `null`, an empty collection, an empty tank and an unsupported observable are not equivalent. Out-of-range integers are codec errors. Overflow in future calculations must follow Java semantics where applicable, demonstrated by tests; do not rely on Rust debug/release mode. ([NBT](https://docs.neoforged.net/docs/1.21.1/datastorage/nbt/), [Java types](https://docs.oracle.com/javase/specs/jls/se21/html/jls-4.html), [Rust overflow](https://doc.rust-lang.org/reference/expressions/operator-expr.html#overflow)).

Integers, IDs, booleans, enums, quantities, slots, ticks and events are exact. Floats use bits by default; an exception requires a justification per field, explicit boundary and negative controls inside/outside the tolerance. There is no universal epsilon, global rounding or spatial grid to obtain a green result. In-memory state and serialized format may have different NaN rules; the policy must distinguish both and preserve raw values.

Compound/map canonicalization is permitted only when order has been demonstrated to lack semantics. Do not indiscriminately sort slots, lists, events, priorities or nodes. Mapping ephemeral IDs to aliases requires a bijection by session, role, birth and dimension; detect duplicate aliases and dangling references. Owners, security, frequencies, respawn and causal order remain comparable. Keep the raw mapping in local evidence without using real accounts.

The legacy comparator uses 1/32 grid occupancy for shapes and four decimal places for attributes. These approximations **are not part of the new exact default**; any reuse requires explicit policy and discriminating tests. ([Legacy comparator](https://github.com/horizon-rust/horizon/blob/main/tools/mod_port_harness.py)).

## 7. Mandatory scenarios and observables

Each scenario declares profile, capabilities, integration level, preconditions, fixture, action sequence, barriers, observables, limits and expected-result origin. Administrative setup is allowed with a component label and precondition verification. Injecting energy tests consumption; it does not prove a generator/cable. Injecting an already formed network does not prove formation. Player actions, placement, menus and permissions need their own connected scenarios.

The proposed pilot is `MK-MAC-001` Enrichment Chamber with a recipe and inputs selected from discovery, controlled energy and per-tick snapshots. Characterize normal processing, insufficient energy, full output, resumption and save/reopen. Quantities, rates, capacity, duration and results are collected in Java; the [template](examples/scenario.template.json) contains no assumed numeric results. If a required observable is inaccessible, the scenario is insufficient to qualify that transition.

Each capability must record states/transitions, zero/one/limit−1/limit/limit+1 boundaries where valid, rejection without effect, incompatible types, partial transfers, interruption/resumption, faces/modes, settings and applicable lifecycle. Generating pairwise combinations complements these boundaries; known high-risk interactions require 3+ factors, such as upgrade×full output×redstone or network×chunk unload×restart. Make finite domains and anything not enumerated explicit.

Accounting conservation applies only under demonstrated assumptions: account for generation, consumption, disposal, transformation, buffers and creative resources. Metamorphic relations—splitting N ticks into smaller calls, saving/reopening without a tick, rotating a layout—are hypotheses until confirmed in Java. Rotation may change network priority; do not impose universal symmetry.

Worldgen and probabilistic processes have their own seed and RNG profiles. When exact control is impossible, separate statistical qualification with reviewed hypothesis, distribution, sample and criterion; statistical approval does not become exact equivalence through normalization.

## 8. Persistence, multiplayer and client surfaces

Separate `save_flush → close → restart → load`, chunk unload/reload, dimension changes and crash recovery. Record durability ACK and crash point. Compare state and subsequent transitions; terminal NBT equality alone is insufficient. Frequencies, security, radiation and other global files form part of the environment. Java determines what survives a crash, without promising operations that are not yet durable.

Multiplayer uses causal correlation `client_sent → server_applied → server_tick_end → client1_applied/client2_applied`. Compare the authoritative server and each client after its barrier. Two players use separate identities, menus and inventories, including scheduled concurrency, reconnection and late join. Do not require packet equality between different versions: compare declared semantic projections.

UI, rendering, particles and audio require a real client. Pin locale, GUI scale, resolution, camera, resourcepacks and framebuffer. Compare menu structure/slots/values and events in addition to images. Visual tolerance must be regional and justified, with known changes detected; a global threshold must not hide a missing gauge, text or model. Audio separates event semantics and device capture. Dedicated-only does not qualify these capabilities.

In the future 1.21.1↔Horizon26.3 comparison, maintain a vanilla baseline without the mod and a ledger of differences per field/case. Capture raw data and the versioned transformation. Effects on `minecraft:*` blocks, damage, water and redstone remain part of the mod when caused by it; filtering entire namespaces would erase relevant behavior. An unexplained vanilla difference remains a blocker.

## 9. Evidence, reproduction and supervision

Each attempt produces an effective lock, expanded scenario, discovery, raw trace, normalized trace, policy ledger, delimited stdout/stderr, necessary saves/crash reports and result. A final manifest contains hashes, tick/frame counts and a `complete` marker, published atomically. A partial attempt must not reuse an earlier green result. Freshness issue [#9444](https://github.com/horizon-rust/horizon/issues/9444) and [PR #9453, open at the previous check](https://github.com/horizon-rust/horizon/pull/9453) should be tracked without duplicating the patch.

The cache includes every effective input, toolchain/JDK, flags, bridge/runner/schema, normalizers, relevant platform/scheduler and expanded scenario bytes. Qualification requires verified `fresh` executions; caches are labeled and do not count as independent repetitions. Golden updates are separate operations with difference review and rationale; they never happen automatically to fix a red test.

Use structured Rust `Command` arguments without a composed shell. Supervise the owned tree, deadlines for boot/action/tick/save/shutdown, graceful stop and forced termination limited to processes belonging to the run. Windows Job Object is a candidate; prove the same contracts on Linux. Verify canonical paths before cleanup and preserve failure evidence. ([Rust Command](https://doc.rust-lang.org/std/process/struct.Command.html), [Windows Job Objects](https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects)).

NeoForge uses new gameDir, saves, configs and logs for each run. For any future Horizon/launcher use fresh directories in `HORIZON_HOME`, `HORIZON_LAUNCHER_HOME`, `XDG_DATA_HOME`, `XDG_CONFIG_HOME`; set `HORIZON_LAUNCHER_NO_SELF_UPDATE=1`, `HORIZON_NO_AUDIO=1`; the launcher also uses `HORIZON_LAUNCHER_TRASH`, `HORIZON_LAUNCHER_CLIPBOARD`, `HORIZON_LAUNCHER_DESKTOP` under scratch. Private jars/assets/facts/saves stay outside the public tree, supplied by the user. Do not inherit player defaults or secret flags. ([Project policy](https://github.com/horizon-rust/horizon/blob/main/AGENTS.md), [modpack policy](https://github.com/horizon-rust/horizon/blob/main/docs/modpacks.md#licences)).

## 10. Qualification and metrics

| Validator family | Mandatory discriminating controls |
|---|---|
| Types | i8/i16/i32/i64 extremes, >2^53, overflow, ±0, NaN, infinities, distinct arrays/lists/tags |
| Diff | First differing tick, same final state with different transient state, changed slots/order/events |
| Quantities | Swapped value/type/unit, partial extraction, permitted source/sink |
| Collection | Removed field, mixed snapshot, mutating getter, chunk loaded by observation |
| Transport | EOF/truncation, duplication, gap, reordering, excessive size, unknown schema |
| Processes | Failed boot, timeout, cancellation during save, surviving child, cleanup outside scratch |
| Identity | Swapped owner/frequency, ambiguous alias, merged respawn, dangling reference |
| Evidence | Changed hash/config, corrupt blob, old green result, partial manifest, fresh run that does not execute |
| Generation/reduction | Reproducible seed and expanded actions; assumptions/failure preserved after shrinking |
| Coverage | Skip/flaky/unsupported do not increase approval; orphans and missing observables prevent closure |

Pre-port qualification requires three fresh Java executions per mandatory scenario, without selecting the best result or retry-until-green. Every mandatory divergence class has a negative control detected by the engine. Real game controls confirm actions were applied; mutating JSON does not prove action application or correct capture.

The ledger relates `profile → capability → object → state/transition → scenario → action → observable → negative control → evidence`. Record catalogued, specified, observable, controlled, executed, repeatable, negative-control-qualified and integration-covered separately. Discovered IDs, mandatory requirements, transitions and observables have different denominators; an unknown denominator remains unknown. A global test percentage does not certify the mod.

M13 closes only the scope declared in the [roadmap](ROADMAP.md). Handoff to Rust gameplay requires a sealed environment and catalog, zero mandatory gaps, qualified collection, repeatable fresh corpus, detected negative controls and real integration paths. This conclusion is finite and versioned, with no universal guarantee. M14 will need new Java/Rust evidence; do not reuse A/A as proof of the port.

## 11. Open decisions

Actual artifact selection and hashes, JDK vendor/build, final NeoForge choice, non-mutating public observation surfaces, exact tick/observer-off mechanism, local transport/authentication, resource limits and soak duration remain unresolved. Windows x64 and Linux x64 are proposed initial platforms with no proven execution yet. Mekanism fields, rates and internal types were not inferred from Java. Each decision needs evidence before enabling its profile.

## 12. Laboratory and functional input contract

The executor must materialize the nine canonical [instance](instance-profiles.template.json) profiles and the scenarios in [functional-scenarios.json](functional-scenarios.json). A design is not an executable DSL: bindings, step IDs, limits, types/units, preconditions, predicates, observers and expected-result origin are mandatory. The laboratory controls boot, world, login, chunks, screen and phase separately; a port or hello does not prove the player can act.

The first real client is brought forward in the RF-CLIENT backlog and gate G3. Proof traverses the declared ingress, authoritative application and each client's update. OS input, real queue, action pipeline, protocol bot and fixture/fake-player have distinct limits. Enqueue, focus or receipt alone do not prove an effect; silent fallback to a direct call is rejected. The [laboratory specification](FUNCTIONAL-LAB.md) defines isolation, processes, display, resources, lifecycle and evidence per step.

Gates are finite, explicit projections. G3/G6 do not authorize claiming broad coverage or skipping the G8 pre-port audit. M00–M14 are historical origins rather than a blocking order: completing every machine is not required to test input. Future Java/Rust comparison begins after the applicable gate and produces new evidence.

## 13. TDD contracts and requirement traceability

The 117 families break down into 253 characterization requirements, with specific actions, observables, boundaries and witnesses in [requirements.json](requirements.json). All lack real objects/parameters and captured expected results. A scenario link to a family is context, not automatic coverage of its children. Each requirement→case→evidence link must be reviewed; the 22 scenario designs do not represent the entire corpus.

The [TDD contract](TDD.md) separates engine testing, Java characterization and the port's future semantic RED. Missing hash, failed boot, unapplied action, missing capability and partial capture block progress; they are not divergences in Mekanism behavior. GREEN must not come from reducing assertions, recalculating a golden or replacing a real client with a stub. Refactoring preserves domain, expected results and integration level.

Mutants have classification and applicability: executed valid/discriminating, equivalent, out of domain, invalid/not applied and unresolved. The score does not include invalid mutants as proof of sensitivity. JSON comparator controls are complemented by real effects and independent witnesses to qualify actions/collection. No score, RED/GREEN or instance was executed in this revision.

## Two-stage locks and qualification

Before any game, resolve and seal an artifact-lock with hashes of every selected artifact, JDK/loader/dependencies and tooling. Diagnostic bootstrap in scratch may start with a draft effective-lock to generate post-load settings and catalog; it produces neither expected results nor a qualification pass. After review, seal the effective-reference-lock, checkpoint and bindings; the qualifying attempt is new and confirms that state after load. Changes or omissions block approval. Do not allow a null jar hash or promote a bootstrap draft. The [reference profile](profiles.reference.template.json) defines both contracts, and RF-SCOPE-05 depends on real boot/bridge.


