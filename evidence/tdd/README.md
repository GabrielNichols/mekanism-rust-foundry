# Recorded TDD cycles

These logs come from actual executions. Library tests, probe processes and Minecraft scenarios have different scopes; none is reclassified as parity of the future port. Raw evidence is immutable. It may contain localized operating-system messages; authored documentation, comments and explanations are maintained in English.

| Increment | RED | GREEN | Scope |
| --- | --- | --- | --- |
| Types, comparison and artifacts | `01-contracts-red.log`: 12 failures; one schema check already passed | `02-contracts-green.log`: 13 tests | Rust contracts with independent fixtures |
| Supervision | `03-processes-red.log`: two failures | `04-processes-green.log`: two tests | Actual probe processes, arguments/cwd and deadline |
| Scenario guards | `05-scenarios-red.log`: six failures; one valid case already passed | `06-scenarios-green.log`: seven tests | Reject missing assertions, invalid actions/topologies and mutation retry |
| Windows transport | `07-transport-red.log`: accepted socket did not wait for a fragmented hello | `08-transport-green.log`: test passes | Actual loopback sockets; inherited nonblocking-mode regression |
| Server inventory | `09-server-inventory-red.json`: expected 13, observed zero | `09-server-inventory-green.json`: scenario passes | Actual NeoForge/Mekanism barrel fixture; not player interaction |
| Measurement comparison | `10-benchmarks-red.log`: six failures | `11-benchmarks-green.log`: six tests | Independent synthetic arithmetic; no game benchmark |
| Client/menu observation | `12-client-observer-red.json`: expected open menu, observed false | `12-client-observer-green.json`: scenario passes | Actual client, keybinding/Screen handlers and server/client witnesses |
| Native libraries | `13-platform-red.log`: two failures | `14-platform-green.log`: four tests | Exclude x86/ARM64/Linux/macOS DLLs from Windows x64 |
| Required observations and arguments | `18-observation-guards-red.log`: four new failures | `19-observation-guards-green.log`: 12 scenario tests | Missing/invalid ticks never default to zero; invalid arguments/selectors fail |
| Concurrent Windows publication | `22-atomic-publication-red.log`: rename blocked by an actual reader | `23-atomic-publication-green.log`: publication succeeds after reader release | Bounded file-commit retry; no repeated game mutation |
| Per-tick capture | `26-capture-nbt-red.log`: six capture failures | `27-capture-nbt-green.log`: seven capture tests | Continuity, phase, thread, endpoint and terminal mode |
| Independent binary NBT | `26b-nbt-red.log`: six failures | `27-capture-nbt-green.log`: six NBT tests | Width, long above 2^53, bits, truncation and duplicate keys |
| Machine actions | `28-machine-contracts-red.log`: six failures | `29-machine-contracts-green.log`: six tests | Arguments, units, inputs and advancement budget |
| Actual measurement contracts | `30-measurements-red.log`: six failures | `31-measurements-green.log`: six tests | Required counters, ticks and independent percentile expectations |
| Persisted witness | `32-persistence-red.log`: three failures | `33-persistence-green.log`: three tests | Only `keepPacked Byte(0)` may be projected |
| Capture-mode comparison | `34-observer-red.log`, `34b-observer-red.log`: six/eight failures | `35-observer-green.log`: eight tests | Cases, artifacts, configurations, endpoints, capture and persistence |
| Blocking/extraction/energy guards | `36-machine-operations-red.log`: three new failures | `37-machine-operations-green.log`: nine tests | Explicit bounded operations; not proof of their Java execution |
| Region reading | `38-region-red.log`: one new failure; three tests already passed | `39-region-green.log`: four tests | Correct coordinate types, zlib, duplicate targets and external compression |
| Observer receipts | `40-receipt-red.log`: different receipts were accepted | `41-receipt-green.log`: nine tests | Nontiming metadata must also match |
| Work accounting | `45-work-red.log`: seven failures; one check already passed | `46-work-green.log`: eight tests | Recipe, FE, receipts and exclusion of fixture production |
| Boot budget | `47-boot-budget-red.log`: one new failure | `48-boot-budget-green.log`: three tests | Explicit bounded budget through 900 seconds, without silent clamping |
| Item identity | `52-work-binding-red.log`: slot order was assumed | `53-work-binding-green.log`: nine tests | Item-based ledger; complete comparison retains slot positions |
| Client output selection | `57-client-output-red.log`: output selector rejected | `58-client-output-green.log`: ten machine-contract tests | Actual output-selector support in the scenario contract |
| Clock domains | `59-clock-domains-red.log`: separate clocks were collapsed | `60-clock-domains-green.log`: 15 contract tests | Global sequence, per-domain regression validation |
| Dedicated client counter | `61-client-counter-red.log`: three failures | `62-client-counter-green.log`: three tests | Counter independent of world time, required and canonical |
| Required client-world observation | `63-client-world-clock-red.log`: missing world time was accepted | `64-client-world-clock-green.log`: four client-clock tests | Retain the observed world clock without substituting it for the control counter |
| Relative instance paths | `67-instance-paths-red.log`: two failures | `68-instance-paths-green.log`: two tests | Relative and absolute destinations resolve to the same absolute root |
| Private data homes | `69-private-homes-red.log`: actual child lacked a private home | `70-private-homes-green.log`: actual server/client probes pass | Disjoint owned data directories received by actual processes |
| Inventory reconciliation | `73-witness-reconciliation-red.log`: eight failures | `74-witness-reconciliation-green.log`: eight tests | NBT/capability/alias agreement, types and unique slots |
| Lifecycle witnesses | `80-lifecycle-red.log` | `81-lifecycle-green.log` | Reject a one-tick progress loss, wrong NBT width, extra inventory, and clock undershoot or overshoot |
| Console boundary | `82-console-marker-red.log` | `83-console-marker-green.log` | Match the marker emitted by the pinned offline server, preserve partial lines, reject duplicate replies |
| Crash recovery region record | `85-crash-region-red.log` | `86-crash-region-green.log` | Read a complete record without sector padding and still reject short payloads and writes beyond the declared allocation |
| Retained rejection payload | `75b-retained-rejection-red.log`: stale alias was accepted | `76-retained-rejection-green.log`: test passes | Reject inconsistent state while retaining the original payload |
| Simulated observer faults | `89b-observer-faults-red.log` | `90-observer-faults-green.log` | Validate the capture and timing envelope, then use independent witnesses for relabelled state and post-read mutation |
| Installed-disabled profile | `89c-disabled-profile-red.log` | `91-disabled-profile-green.log` | Require the installed bridge jar while withholding the active bridge session |
| Live observer audit classifier | `94-live-audit-red.log` | `95-live-audit-green.log` | Distinguish stale capture from an actual post-read mutation and require stable time and persisted corroboration |
| Reference trace seal | `98-trace-seal-red.log` | `99-trace-seal-green.log` | Reject a structurally valid prefix and a same-length value edit |
| Causal client barrier | `102-causal-barrier-red.log` | `104-causal-barrier-green.log` | Require fresh revision, correct owner and a later render. Refuse capture before the barrier |
| Chunk observation controls | `103-chunk-controls-red.log` | `105-chunk-controls-green.log` | Distinguish unchanged residency and holder state from a loading lookup, with event and disk corroboration |
| Delayed chunk observation | `106-delayed-chunk-red.log` | `107-delayed-chunk-green.log` | Detect delayed load events or holders across an exact three-tick window |
| Effective catalog comparison | `110-effective-catalog-red.log` | `111-effective-catalog-green.log` | Reject missing registry entries, rebound recipes, module drift and generated configuration drift |

## Regression checkpoints

- `25-final-regression.log`: 39 passing tests, none ignored; `24-clippy.log` records warnings denied.
- `42-suite-green.log`: 83 passing tests, none ignored.
- `54-final-regression.log`: 95 passing tests, none ignored; `55-clippy.log` records warning-free Clippy.
- `65-final-regression.log`: 102 passing tests; `66-clippy.log` records Clippy.
- `71-final-regression.log`: 105 passing tests; `72-clippy.log` records Clippy.
- `77-final-regression.log`: 114 passing tests, none ignored; `78-clippy.log` records warning-free Clippy. Formatting also passed.

- `88-lifecycle-final-regression.log`: 122 passing tests, none ignored. `87-lifecycle-clippy.log` records warnings denied, and formatting passed.

- `93-observer-regression.log`: 127 passing tests. `92-observer-clippy.log` records warnings denied, and formatting passed. The filtered RED runs isolate the intended failures from the sandbox errors in `89-observer-controls-red.log`.

- `101-final-regression.log`: 132 passing tests. `100-final-clippy.log` records warnings denied, and formatting passed.

- `109-phase-chunk-regression.log`: 139 passing tests. `108-phase-chunk-clippy.log` records warnings denied, and formatting passed.

- `113-catalog-regression.log`: 140 passing Rust tests. `112-catalog-clippy.log` records warnings denied. Raw execution logs stay in private local storage. This index preserves the local names and tested contracts.

Earlier logs preserve the 33- and 38-test regressions. The bridge's own RED source states remain in `.java.txt` files; their jars remain in ignored `.lab/cache`.

## Failure classification and evidence integrity

Files prefixed `00-`, `05a-`, `07a-` and `10a-` retain preparation, compilation or access errors/interruption. `75-retained-rejection-red.log` also contains an initial sandbox temporary-file issue; the intended semantic RED is `75b-retained-rejection-red.log`. These setup failures are not semantic RED evidence. Early boot attempts and DLL hash rejection likewise do not count as Mekanism gameplay failures.

`49-boot-infrastructure.json` separates exceeded boot budgets from functional tests. `56-chunk-fixture-red.json` preserves the observed machine stall while world time advanced. Neither is a port RED. `processing-discovery-01` failed on the storage marker difference; `processing-discovery-02` passed with raw and projected records retained. `processing-01` rejected the initial insertion binding; subsequent discovery established the public face/slot bindings and required placement components.

The barrel expectation stayed at 13. Client placement, item consumption and three-ore transfer expectations remained intact. Recipe quantities were taken from pinned recipe data and actual Java characterization. Binding discovery and declared fixture corrections do not authorize changing expected results to fit candidate code. Starting terrain, pose, inventory, forced-chunk anchoring and output-filling setup remain explicit in the cases and documentation.

The lifecycle campaign uses the original Java game. Open roadmap controls include installed-but-disabled observation, adversarial mutations, complete phases, multiple clients, and a per-family recovery corpus. [VALIDATOR-CONTRACT.md](../../VALIDATOR-CONTRACT.md) defines the capture-frequency comparison's limits. Test counts are not a promise of complete behavior coverage.

## Runtime campaign and comparator summaries

The [processing campaign](../processing-campaign.json) records seven fresh functional passes, three observer-pair comparisons and four same-mode A/A comparisons. The [offline comparator controls](../comparator-controls.json) accept an unchanged roundtrip and reject a known transient progress change and a missing intermediate frame in an actual Java trace, preserving the original reference. These are offline comparator controls, not in-game sensor-fault qualification. Broad qualification and optimization flags remain false.


The [simulated observer campaign](../simulated-observer-faults.json) accepted one clean scenario and rejected eight mutations with their expected diagnostics. The [live observer campaign](../observer-controls-campaign.json) separately verifies the installed-disabled guard and compares the three no-fault Java modes under serial and parallel scheduling.

The [live audit](../live-observer-audit.json) verifies the two observer faults against real console and persisted NBT witnesses. The [trace seal campaign](../trace-seal-controls.json) verifies two fresh Java reference traces and rejects four archive mutations. Its first exploratory same-length mutation changed a phase and was rejected by phase validation. The final control edits a valid numeric value so the hash check is the rejecting layer.

## Lifecycle discovery runs

The initial bridge-free run reached a live Minecraft server and froze it, then timed out because the console marker had a `[Not Secure]` prefix. The parser regression test uses that actual line. Two later attempts reached the real pre-flush termination. In the first, the selected region record started at byte 929,792 and contained all 916 bytes declared by its header. The process exited before close-time sector padding was written. The reader initially rejected this file. The region RED and GREEN preserve the distinction between complete records, truncated payloads, and allocations that extend beyond their declared sector count. Those early campaign attempts remain non-passes.

The corrected reader then passed all 15 treatments across 25 JVM processes. The rebuilt runner also passed the existing dedicated-server inventory case. [Lifecycle campaign](../lifecycle-campaign.json)

The raw files are retained under `.lab/evidence/recovery-01`, `recovery-02`, and `recovery-03`. They include the original console messages, exact shutdown conditions, and selected world files. Keep them private with the other local game data.


## Client and chunk discovery runs

The exploratory client attempt used the previous windowed launcher and rejected the source pickup precondition. A case-file edit during setup exposed that the runner could archive different bytes from those it parsed. The runner now reads each input once and uses those bytes for execution, archival and the case hash. The later client runs require the hidden GLFW path and verify the actual render artifact.

The first chunk campaign established that the loading control produced an actual target load and a persisted region slot. The final campaign adds three real world ticks and a follow-up witness to check for delayed effects. Private traces and worlds remain under `.lab/evidence`.
