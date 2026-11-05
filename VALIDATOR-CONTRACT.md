# Executable validator contract

The laboratory runs the original Mekanism, pinned by SHA-256, and observes its behavior. The Java bridge uses public Minecraft/NeoForge APIs. The controller, comparison, region reader and measurement analysis are written in Rust. No port gameplay exists yet. The repository-local [planning bundle](docs/planning/README.md) is the canonical source of broader requirements and qualification gates for contributors.

## Time, actions and evidence

- A fresh world starts frozen with recorded seed and configuration. Each `step` declares 1–20 ticks. `advance_until` declares a goal, batch size and budget of at most 1,000 ticks. Every batch is a new command with its own sequence. A failed mutation ends the case without automatic retry.
- Boot has an explicit operational budget of 1–900 seconds, archived in the manifest. Preparation defaults to 300 seconds and accepts `-BootTimeoutSeconds`. Exceeding this budget is an infrastructure failure; it does not become a mod mismatch or authorize removing expectations.
- Relative destinations resolve to absolute instance directories before launch. Server and client receive separate gameDirs, temporary directories and Horizon/XDG homes, plus the audio/self-update flags. Process probes check the paths actually received.
- `every_tick` requires exactly one sample per advanced tick; `terminal` requires only the last sample. The engine checks continuity, agreement between state and envelope ticks, phase and owner-thread confirmation. The phase corresponds to the pinned bridge's `ServerTickEvent.Post` listener after the world update. It is not qualified against every possible ordering of other listeners.
- `advance_until` assertions are checked at each batch endpoint. Completion may occur inside the batch; detailed samples preserve that instant. The response tick is not necessarily the first completion tick.
- The final snapshot must equal the last sample. Performance timings and counters are stored in `measurements.json`, separately from `trace.json`. Original observations, case, manifest, configurations and logs are retained in the private run directory. Parsed rejected observations are archived before witness validation so the failure payload remains available.
- Sequence order is global, while envelope clocks have separate domains. Server phases use world time. `client_sent`, `client_applied`, and `client_rendered` use a dedicated `ClientTickEvent.Post` counter. Observed client-world time remains in `state.tick`, and `state.client_tick` is mandatory. Regressions are checked within each domain. A client clock is never replaced with the server clock. This version supports one client and does not yet qualify a client A/A corpus or every causal barrier in the roadmap.

## Independent persistence and inventory witnesses

With `require_persisted_machine`, the server must exit successfully before Rust reads `world/region/r.0.0.mca`. The reader selects exactly one machine at `(0,80,0)`, requires NBT `Int` coordinates and compares its complete typed NBT with the final live read.

The only permitted storage projection removes `keepPacked` when its value is exactly `Byte(0)`. This field was observed in the region file and is absent from `saveWithFullMetadata`. A different value, width or missing marker fails. Both raw and projected records are retained; there is no generic field removal.

Every observed pilot inventory must also reconcile public unsided capability slots, scenario aliases and typed NBT. Slot identities must be valid and unique, counts must retain their required width, and quantities must agree. This inventory-schema check complements the independent final region read; it does not qualify every resource container or energy representation.

The reader supports internal zlib records, explicit depth/size limits and the 12 NBT payload types. It rejects external compression, unsupported formats and strings containing isolated UTF-16 surrogates. This is not general Anvil/NBT qualification or a world reopen/resumption test.

## Observer effects

`compare-observer` accepts two completed runs with archived cases and verified case hashes. The only permitted case difference is `observer_mode`: `every_tick` versus `terminal`. Artifacts, controller binary and configurations must match. The command compares complete response states and metadata, including receipts, as well as persisted NBT. Both sample and measurement series are validated separately.

This check covers capture frequencies using the same bridge and controller. It does not establish neutrality against bridge-absent execution. It does not authorize lowering the proof level, updating goldens or declaring future-port equivalence.

## Observed performance

`bridge_pre_to_post_ns` measures the interval between the bridge's Pre/Post listeners, before detailed snapshot capture. `snapshot_ns` measures typed-sample construction. `batch_wall_ns` includes deliberate 20 TPS server waiting and is not pure simulation cost. Measurements also record JVM process CPU, GC pauses/collections and sampled heap usage. Heap is neither RSS nor total system memory.

`performance.json` uses nearest-rank percentiles and explicit units. These are functional-campaign measurements, without qualified steady-state warmup, Java/Rust comparison or an optimization conclusion. Future proof must follow [BENCHMARKS.md](BENCHMARKS.md), including equivalent work and outputs.

The JVM interval ends before final response serialization/transmission. It excludes part of the bridge overhead, transport and controller parsing/writing. Client render and tick counters remain raw in the trace; projecting them for future cross-runtime comparison requires a separately reviewed policy.

`mek-work RUN_DIRECTORY` performs offline Rust accounting for the pilot's declared bindings. It checks energy/extraction receipts, recipe transitions and final item/FE balances. The 62 dust added when filling output from 2 to 64 are fixture input, not production. The report records hashes of the analyzer and reference runner. It does not turn terminal capture into proof of intermediate transitions or replace complete trace comparison.

The ledger aggregates items by identifier in the unsided capability. Slot order and complete contents remain in the trace and complete comparison; equal aggregates do not make different layouts equivalent. Insertion and extraction use the case's declared face/slot, discovered with public simulation before mutations. `isItemValid` alone does not prove operational acceptance. Simulation probes in snapshots are part of the observed cost; cache neutrality still requires the broader RF-CLOCK-06/07 controls.

## Client bootstrap profile

The private client disables the NeoForge loader splash with `earlyWindowControl=false` and creates its GLFW window hidden from the start. The runner requires the offscreen bridge artifact before launching the client. The bridge checks GLFW visibility at creation and on the client tick and render paths. The real rendering, menu and input handlers use the Windows OpenGL stack. The client profile records its configuration seed, effective configuration hashes, render distance 2 and valid simulation distance 5. Effective client configuration is checked again after the scenario.


## Bridge absence and process recovery

The lifecycle command uses a fixed administrative fixture. It places one Enrichment Chamber in a forced chunk, inserts one iron ore via server NBT, and persists the empty-energy seed. Every treatment gets a fresh world and config copy. The installed-disabled and enabled treatments both install the same pinned jar. Only the enabled treatment opens a control connection and registers observation hooks. The recovery suite uses the absent and every-tick modes, and the continuous observer controls include the installed-disabled treatment. Both modes use the vanilla tick scheduler. The bridge-free runs issue steps from the local console, while the enabled runs request steps through the every-tick bridge. A separate console clock must reach the exact end of every batch.

Each treatment advances 73 powered ticks, then waits 64 dry ticks to settle the machine's update delay at zero. The runner compares full live checkpoints and independently decodes the machine from disk after the process exits. Reopen happens in a fresh JVM against the same world. The crash-before-flush control verifies the original machine on disk and replays the discarded prefix. The graceful and crash-after-flush paths verify progress 73 on disk and in the reopened server. All paths explicitly check progress 74 after one resumed tick, then finish the 127-tick resume pulse and validate two dust in the output slot. The clock probe records startup separately from commanded work.

These checks cover a single server machine with no connected clients and a controlled autosave policy. The process kill exercises JVM interruption on the same host. System crashes, client recovery and wider save compatibility remain separate gates.


## Simulated observer faults

The simulator runs a fresh three-tick synthetic world for each fault. It validates the same capture and measurement envelope as the live bridge. A fixed independent scenario contract checks observed progress and ore at each tick, then checks the world again after the observer read. This second witness catches an observer that returns a valid snapshot and then changes inventory. Each case archives the synthetic reply, the independent world, and the first rejection diagnostic. Synthetic fields in the reply are test inputs. Validator timing measures actual Rust validation only, over repeated checks of each generated reply.

The live disabled profile loads the same pinned bridge jar as the enabled profile. A constructor guard returns before registering server or client hooks. A startup marker and the jar manifest confirm which profile ran. All profiles use the console clock and an independently decoded terminal region file. Within a live campaign, divergence at any checkpoint makes the campaign a non-pass. The dedicated live audit exercises deliberate observer faults in a separate fixture.


## Live observer audit

The audit brackets a bridge read with vanilla console witnesses while the world clock is frozen. The bridge receipt identifies the server post-event command phase and owner thread. The runner corroborates live progress and inventory against the typed bridge snapshot, then flushes, closes the JVM and independently decodes the target from the region file. The campaign compares the full typed saved state against the clean control, allowing only the single expected ore-count change in the inventory mutation treatment.

The clean control, stale capture and post-read mutation each run in three fresh JVMs at a target of 100 ticks/s with three workers. The stale treatment replays the actual initial machine snapshot with the current tick stamped on it. The mutation treatment captures the authentic state and then changes the real ore count before returning. The bridge enables these controls only for the dedicated audit profile and the one-shot checkpoint request. A clock change, failed fixture or missing disk witness makes the attempt a non-pass.

## Reference trace seals

A fresh `run-reference` commits a schema 2 result after it writes and validates the complete trace and rechecks the artifact lock. The atomic result binds the attempt ID, case hash and execution fingerprint to the exact trace frame count, byte count and SHA-256. `verify-reference` requires a matching run and result, the archived case, and the trace bytes. The campaign adapters verify the seal before using new traces. Historical comparisons remain available through the existing comparator. A seal detects accidental truncation, edits and mismatched results. Authenticity and sealing the remaining raw artifacts are separate concerns.


## Real-client menu barrier

The source-slot fixture captures an owner-thread server precondition with three ore in the source slot and an empty cursor. It sends one Screen click and polls read-only until the server independently shows the empty source slot and three ore on the cursor. The server sends a full menu update and returns its fresh revision. The client must observe the same menu, revision, and contents on a later client tick. Only then can the one-shot render gate capture at a subsequent render-post event. The premature request is deliberately sent before the authoritative revision is issued and must leave the output PNG absent. The runner verifies and hashes the subsequent PNG.

This fixture uses the vanilla menu synchronization path and checks one pickup in one dedicated server and one real client. The phase evidence records owner checks, separate clocks, revision, and per-request round-trip timings. General action fencing, multiplayer actor identity, OS input and image equivalence remain separate scopes.

## Nonloading chunk lookup

The dedicated fixture probes overworld chunk (2000, 2000), away from spawn and player tickets. The nonloading lookup uses the chunk cache's resident-only read. The negative control calls the loading lookup for a full chunk. An independently registered target-filtered chunk-load event, visible holder and ticket-level debug state, frozen console clock, and persisted region location serve as witnesses. The runner steps exactly three world ticks and rechecks for delayed effects before flushing and closing. It checks the specific target record in the region file after shutdown.

These controls cover one fresh unloaded coordinate and a bounded observation window. They exercise residency and effective visible ticket level. They leave other dimensions, overlapping players and direct inspection of every ticket queue open.
