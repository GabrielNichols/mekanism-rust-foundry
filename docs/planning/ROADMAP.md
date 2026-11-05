# Segmented functional roadmap for the Rust engine

Implementation has started in an independent project: see the [first increment](IMPLEMENTATION.md) and [current status](../../STATUS.md). The phases and acceptance criteria below are the planning contract; their broad gates remain pending.

Revision 0.2, 2026-11-05. The plan contains **138 microdeliveries**, with dependencies, a discriminating test before implementation, minimum GREEN, functional acceptance, negative control, topology and evidence. All are pending. This package implemented no test, instance, dynamic bridge or port behavior.

The previous 25 MP IDs remain as epics and M00–M14 as a historical map. Ordering now follows the [JSON backlog](roadmap.json) DAG, not those phase numbers. In particular, RF-CLIENT tasks enter early: a real client, placement and slots precede the complete pilot and networks. [TDD](TDD.md), [laboratory](FUNCTIONAL-LAB.md), [253 requirements](requirements.json) and [22 scenarios](SCENARIOS.md) provide the proof contracts.

## First vertical path with a real client

The first slice traverses scenario → Rust CLI → scratch/processes → bridge → real client input → server application → authoritative/client state → trace → comparison → report. Start with a short vanilla interaction to diagnose control; this is not Mekanism parity. Then place and operate the Enrichment Chamber. No phase requires implementing every machine to test the first action.

| Milestone | Supporting items | In-game acceptance |
| --- | --- | --- |
| Reference and boot | RF-SCOPE-01..07; RF-MODEL-01..10; RF-INSTANCE-01..06; RF-BRIDGE-01..06 | Real server loaded and fingerprint checked; an error does not become green |
| Client and input | RF-CLIENT-01..04 | Login/chunks/identity; movement and aim with server+client witnesses |
| Placement and slots | RF-CLIENT-05..08 | Place/break, open a menu and move an item through the declared ingress; a receipt is insufficient |
| Clock and observer | RF-CLOCK-01..08; RF-EVIDENCE-01..08 | Complete phases, fresh A/A, independent controls and sensitivity |
| Pilot machine | RF-PILOT-01..10; RF-LIFE-01..02 | Recipe, invalid input, insufficient resource, full output, menu and continuation after restart |

## Gates by scope and proof level

| Gate | Slice | Exit condition | Limit |
| --- | --- | --- | --- |
| G0-reference | Two-stage reference | Artifact-lock before spawn; bootstrap exports config/catalog; effective-lock reviewed before qualification | Bootstrap produces no expected results/pass |
| G1-engine-contracts | Engine contracts | Independent type, diff, state and error tests with RED/GREEN recorded | Does not prove Java capture |
| G2-server-lifecycle | Real server | Boot, hello, world, modules, ownership and real shutdown | Does not prove input/GUI |
| G3-early-real-client | First real client | Login, movement, aiming, placement/breaking and slots with server+client effects | Not yet broad qualification of the clock or Mekanism |
| G4-observation-clock | Clock and observation | Complete phases/ticks, fresh A/A and observer on/off with an independent witness | Does not prove a port |
| G5-sensitive-validator | Sensitive validator | Discriminating controls for comparator, real action, capture, transport and evidence | Does not cover families without cases |
| G6-pilot-functional | Complete pilot machine | Recipe, invalid input, no energy, full output, menu and restart with trace | Does not authorize Rust gameplay; scope gates are still missing |
| G7-domain-slices | Domain slices | Concrete requirements, boundaries, lifecycle and applicable clients per family | Does not represent unexecuted modules/profiles |
| G8-scoped-preport-audit | Pre-port audit | Declared finite scope, reconciled catalog, fresh corpus, controls/replay and zero required gaps | A/A is not Java/Rust |
| G9-future-port-TDD | Future port | Sealed Java expected results; semantic mismatch of the supported candidate; minimal implementation and regression | Does not prove the entire mod |

G3 is initial functional smoke and does not replace G4/G5. G6 qualifies the pilot in the declared profile; it does not enable Rust gameplay by itself. G8 requires closure of the intended scope and its requirements/scenarios/topologies/controls closure. The published graph represents the broad baseline; a smaller projection needs an explicit manifest and review without erasing requirements from the baseline denominator. G9 follows the applicable gate and requires new Java/Rust evidence.

## Contract for each microdelivery

Before implementation, pin the requirement, independent fixture or scenario, assertion and expected failure class. Record RED for the intended reason; `infra_error`, missing capability and invalid environment are not gameplay RED. Implement the minimum real behavior, run acceptance and its controls, and refactor while repeating relevant regressions. The complete per-item fields are in JSON: `red_contract`, `expected_failure_class`, `minimal_green_deliverable`, `functional_acceptance`, `negative_control`, `evidence_required`, `refactor_exit` and `target_gate`. Broken compilation alone does not prove assertion sensitivity.

Infrastructure support does not automatically close a family's requirements. For each child, materialize its own objects, domains, transitions, observables and scenario after discovery. An Enrichment Chamber does not replace a factory; a tank does not replace a tube; a final result does not replace the trajectory of resources and progress.

## Scope, versions and reference

| ID and microdelivery | Depends on | Topology/gate | Discriminating RED → minimum GREEN | Functional acceptance and negative control |
| --- | --- | --- | --- | --- |
| RF-SCOPE-01 — Separate reference, candidate and module profiles | — | LAB-UNIT / G0-reference | Complete artifact-lock before spawn; a draft effective-lock does not qualify → Two locks and explicit diagnostic bootstrap | No game starts with unhashed artifacts; bootstrap has no qualification credit; negative control: omit a hash or try promoting a bootstrap draft |
| RF-SCOPE-02 — Resolve the identity of the four official jars | RF-SCOPE-01 | LAB-UNIT / G0-reference | A source tag does not replace the binary hash → Per-artifact and per-module SHA-256 manifest | Replacing one jar invalidates the fingerprint; negative control: Jar with the same name but different bytes |
| RF-SCOPE-03 — Pin JDK and effective arguments | RF-SCOPE-01 | LAB-UNIT / G0-reference | Same Java major with a different vendor/build does not reuse evidence → JDK identity and JVM args after resolution | Runner proves the executable selected in the lock; negative control: PATH points to a different java |
| RF-SCOPE-04 — Resolve loader and dependency closure | RF-SCOPE-02; RF-SCOPE-03 | LAB-UNIT / G0-reference | An unverified loader candidate blocks sealing → Transitive closure with hashes and order | Loaded environment reconciles exactly with the lock; negative control: Unexpected transitive library |
| RF-SCOPE-05 — Pin configuration after load | RF-SCOPE-04; RF-INSTANCE-06; RF-BRIDGE-03 | LAB-JS / G0-reference | Bootstrap is not qualification → Export config/catalog and review effective-lock | A new attempt confirms effective-lock after load; negative control: mismatching config or promotion of a bootstrap draft |
| RF-SCOPE-06 — Define execution profiles and exclusions | RF-SCOPE-01 | LAB-UNIT / G0-reference | An excluded module cannot count as pass → Scope by Core and official combinations | Result reports required, excluded and unknown separately; negative control: Silently exclude Generators |
| RF-SCOPE-07 — Pin expected-result provenance and policy | RF-SCOPE-02 | LAB-UNIT / G0-reference | Candidate results must not overwrite the reference → Immutable expected-result manifest and separate review | Java provenance and fingerprint accompany every golden; negative control: Auto-update triggered on mismatch |
| RF-SCOPE-08 — Freeze the initial ledger and traceability | RF-SCOPE-06; RF-SCOPE-07 | LAB-UNIT / G0-reference | A requirement without a scenario does not count as covered → Family, requirement, scenario and evidence IDs | A new object increases the gap before approval; negative control: Discovered object without a family |

## Rust model, types and diff

| ID and microdelivery | Depends on | Topology/gate | Discriminating RED → minimum GREEN | Functional acceptance and negative control |
| --- | --- | --- | --- | --- |
| RF-MODEL-01 — Strict scenario and capability parsing | RF-SCOPE-01 | LAB-UNIT / G1-engine-contracts | An unknown action produces invalid_spec → Rust parser rejecting unknown required fields | Error identifies the JSON path before spawn; negative control: Required field omitted |
| RF-MODEL-02 — Verdicts without success through emptiness | RF-MODEL-01 | LAB-UNIT / G1-engine-contracts | Zero checks does not produce pass → Run states separate from verdict | Cancellation and unsupported keep the requirement open; negative control: Compare two empty lists |
| RF-MODEL-03 — Integers with width and sign | RF-MODEL-01 | LAB-UNIT / G1-engine-contracts | 2^53+1 must not lose one unit → Typed decimal codec with range checks | Roundtrip of extremes preserves semantic bytes; negative control: u64 interpreted as f64 |
| RF-MODEL-04 — Float by bits and reviewed policy | RF-MODEL-03 | LAB-UNIT / G1-engine-contracts | Negative zero differs from positive zero in exact mode → f32/f64 with explicit bits, NaN and infinities | Independent cases distinguish payload and serialization; negative control: Global epsilon hides divergence |
| RF-MODEL-05 — Typed NBT and absence | RF-MODEL-03 | LAB-UNIT / G1-engine-contracts | Absent, empty and a tag of another type differ → Typed tree, arrays and homogeneous lists | Parser rejects a list with inconsistent types; negative control: Int array replaced by a list |
| RF-MODEL-06 — Identity and bijective aliases | RF-MODEL-05 | LAB-UNIT / G1-engine-contracts | Two actors do not share an alias → Resolver by session, dimension and birth | Respawn does not merge the old entity with the new one; negative control: Owner changed under the same alias |
| RF-MODEL-07 — Events and causal ordering | RF-MODEL-06 | LAB-UNIT / G1-engine-contracts | The same set in a different order may diverge → Sequence with tick, phase, actor and cause | Transient mismatch appears even with an identical final state; negative control: Reversed events |
| RF-MODEL-08 — Per-field units and normalization | RF-MODEL-04; RF-MODEL-05 | LAB-UNIT / G1-engine-contracts | J and FE are not implicitly converted → Versioned policy with raw data preserved | Transformation is valid only for the declared field and version; negative control: Broad filter removes the minecraft namespace |
| RF-MODEL-09 — First-divergence diff | RF-MODEL-07; RF-MODEL-08 | LAB-UNIT / G1-engine-contracts | An identical final state does not erase an intermediate difference → Typed-tree diff and first cause | Diagnosis includes tick, phase, object and field; negative control: Remove an intermediate snapshot |
| RF-MODEL-10 — Public CLI contract and codes | RF-MODEL-02; RF-MODEL-09 | LAB-UNIT / G1-engine-contracts | Infra_error does not use an approval exit code → Rust CLI without a shell for compare/validate | Offline execution does not start a game or change a golden; negative control: Compare command writes the expected result |

## Instances and supervision

| ID and microdelivery | Depends on | Topology/gate | Discriminating RED → minimum GREEN | Functional acceptance and negative control |
| --- | --- | --- | --- | --- |
| RF-INSTANCE-01 — Allocate scratch workspace per attempt | RF-SCOPE-04; RF-MODEL-02 | LAB-JS / G2-server-lifecycle | Two attempts do not share saves/configs → Isolated directories and read-only artifact store | Paths with spaces and Unicode remain isolated; negative control: Reuse a previous run directory |
| RF-INSTANCE-02 — Build argv and environment without a shell | RF-INSTANCE-01; RF-SCOPE-03 | LAB-JS / G2-server-lifecycle | An argument with spaces does not become a command → Rust Command with explicit env and current_dir | Probe program receives argv exactly as declared; negative control: Text with metacharacters |
| RF-INSTANCE-03 — Reserve ports and local identity | RF-INSTANCE-01 | LAB-JS / G2-server-lifecycle | An occupied port is not readiness of our server → Port reservation and association with the run's process | An unrelated server does not satisfy the adapter connection; negative control: Fake service on the expected port |
| RF-INSTANCE-04 — Supervise the owned process tree | RF-INSTANCE-02 | LAB-JS / G2-server-lifecycle | Timeout leaves no orphan JVM/client → Tree/Job Object or process group per attempt | Shutdown respects process ownership; negative control: Child remains after parent exits |
| RF-INSTANCE-05 — Boot limits and diagnostic errors | RF-INSTANCE-03; RF-INSTANCE-04 | LAB-JS / G2-server-lifecycle | Early exit is infra_error and never world-ready → Boot deadlines and failure classification | Invalid Java/missing JAR terminates without approval; negative control: Log contains ready but the JVM died |
| RF-INSTANCE-06 — First real Java server boot | RF-INSTANCE-05; RF-BRIDGE-01 | LAB-JS / G2-server-lifecycle | A healthcheck alone does not qualify a scenario → D0 starts and terminates with a proven lock | Real server loads the world and lists expected modules; negative control: Loader starts without Mekanism |
| RF-INSTANCE-07 — New fixtures or pristine clones | RF-INSTANCE-01; RF-SCOPE-05 | LAB-JS / G2-server-lifecycle | Cloning an already modified world contaminates the test → Seed/fixture manifest and verified clone | Running case B does not change case A's starting point; negative control: Previous save leaks into the next run |
| RF-INSTANCE-08 — Transactional cancellation and shutdown | RF-INSTANCE-04; RF-INSTANCE-05 | LAB-JS / G2-server-lifecycle | Cancellation during flush does not produce complete → Cancellation with final collection and partial attempt | Interrupted run preserves logs and loses approval; negative control: Cancel between trace and manifest |
| RF-INSTANCE-09 — RAM, CPU and GPU admission | RF-INSTANCE-04 | LAB-JS / G2-server-lifecycle | A saturated queue does not start clients without resources → Scheduler with measured costs and leases | Two jobs do not contend for the same display/session; negative control: Force a batch above available memory |
| RF-INSTANCE-10 — Scratch cleanup with failure preservation | RF-INSTANCE-07; RF-INSTANCE-08 | LAB-JS / G2-server-lifecycle | A link outside scratch prevents removal → Cleanup by canonical path with evidence retained | Only the tree owned by the run is removed; negative control: Symlink/junction points outside |

## Bridge, session and application

| ID and microdelivery | Depends on | Topology/gate | Discriminating RED → minimum GREEN | Functional acceptance and negative control |
| --- | --- | --- | --- | --- |
| RF-BRIDGE-01 — Dedicated bridge with minimal hello | RF-INSTANCE-02; RF-MODEL-01 | LAB-JS / G2-server-lifecycle | Incompatible protocol is rejected → Versioned hello at NeoForge startup | Real server declares build, lock and resources; negative control: Old-protocol client |
| RF-BRIDGE-02 — Separate logs and framing | RF-BRIDGE-01 | LAB-JS / G2-server-lifecycle | A log line cannot be a control frame → Local channel with explicit size and schema | Log messages do not change the sequence; negative control: JSON text mixed into stdout |
| RF-BRIDGE-03 — Negotiate effective capabilities | RF-BRIDGE-02 | LAB-JS / G2-server-lifecycle | A missing required observer blocks the case → Capabilities by action, phase and field | Scenario selects a compatible backend before execution; negative control: Bridge advertises energy but omits the unit |
| RF-BRIDGE-04 — Sequence, correlation and session | RF-BRIDGE-02 | LAB-JS / G2-server-lifecycle | A frame from a previous session is rejected → Per-session IDs and monotonicity | Replay uses the correct action without cross-talk; negative control: Resend an ACK from another run |
| RF-BRIDGE-05 — Apply actions on the owning thread | RF-BRIDGE-03; RF-BRIDGE-04 | LAB-JS / G2-server-lifecycle | Mutation outside the server thread violates the contract → Action queue executed on the owner thread | Mutation and reading belong to the declared phase; negative control: Handler executes on the socket thread |
| RF-BRIDGE-06 — ACK does not equal effect | RF-BRIDGE-05 | LAB-JS / G2-server-lifecycle | ACK without a change leaves the action unproven → Two milestones, accepted/applied, and a separate witness | World state proves observable application; negative control: Discard the action after ACK |
| RF-BRIDGE-07 — Repetition and lost ACK | RF-BRIDGE-06 | LAB-JS / G2-server-lifecycle | Retransmitting an insert duplicates resources and must be prevented → Idempotence or inconclusive failure without blind retry | ACK loss does not insert the stack twice; negative control: Timeout after the action is already applied |
| RF-BRIDGE-08 — Local channel and secure shutdown | RF-BRIDGE-04; RF-INSTANCE-04 | LAB-JS / G2-server-lifecycle | An external session does not control the instance → Loopback bind and unpersisted ephemeral token | Control accepts only a process belonging to the run; negative control: Credential from another run |

## Early real client

| ID and microdelivery | Depends on | Topology/gate | Discriminating RED → minimum GREEN | Functional acceptance and negative control |
| --- | --- | --- | --- | --- |
| RF-CLIENT-01 — Launch a real Java client and negotiate readiness | RF-INSTANCE-06; RF-BRIDGE-03 | LAB-JD1 / G3-early-real-client | An open process does not mean a connected player → D1 with server and one rendered client | Boot, resource reload, login and world-ready are separate; negative control: Client stuck on the loading screen |
| RF-CLIENT-02 — Per-client player identity | RF-CLIENT-01; RF-MODEL-06 | LAB-JD1 / G3-early-real-client | Client1 cannot act as client2 → Isolated local test profiles | Server and client agree on the controlled actor; negative control: State queried on the wrong player |
| RF-CLIENT-03 — Input through the normal client path | RF-CLIENT-02; RF-BRIDGE-06 | LAB-JD1 / G3-early-real-client | Key ACK without an action does not qualify input → Input adapter in known focus/context | Press/release produces a confirmed in-game effect; negative control: Window/menu consumes the input |
| RF-CLIENT-04 — Move and aim with a server witness | RF-CLIENT-03 | LAB-JD1 / G3-early-real-client | Setup teleport does not prove movement → Movement and aiming steps through the client | Authoritative position changes and client view converges; negative control: Only camera rotates without player movement |
| RF-CLIENT-05 — Player places and breaks a block | RF-CLIENT-04 | LAB-JD1 / G3-early-real-client | Administrative SetBlock does not prove placement → Held-item, use/mining and release flow | Inventory, block and drops corroborate the action; negative control: Block placed through a hidden command |
| RF-CLIENT-06 — Open a container and move an item through slots | RF-CLIENT-05 | LAB-JD1 / G3-early-real-client | Editing inventory does not prove a menu click → Real opening and menu clicks | Server, carried stack and screen show the transfer; negative control: Input applied while on the wrong screen |
| RF-CLIENT-07 — First vanilla vertical functional slice | RF-CLIENT-06; RF-MODEL-09 | LAB-JD1 / G3-early-real-client | An isolated screenshot does not approve craft/place/break → Short action scenario with an A/A reference | Three fresh runs reproduce the effects of the first vanilla slice; this qualifies observed control, not Mekanism logic.; negative control: Mutate the transfer in the trace |
| RF-CLIENT-08 — Record integration level without promotion | RF-CLIENT-07 | LAB-JD1 / G3-early-real-client | Fake player does not count as a real_client pass → Component/bot/real_client/UI taxonomy | Report retains the input path used; negative control: Command receipt labelled as end-to-end evidence |

## Clock and observation

| ID and microdelivery | Depends on | Topology/gate | Discriminating RED → minimum GREEN | Functional acceptance and negative control |
| --- | --- | --- | --- | --- |
| RF-CLOCK-01 — Separate ticks, wall-clock time and frames | RF-BRIDGE-05; RF-CLIENT-01 | LAB-JD1 / G4-observation-clock | Sleep cannot count simulated ticks → Independent clocks in the protocol | The same N budget counts N effective ticks; negative control: Frame rate changes tick advancement |
| RF-CLOCK-02 — Discrete advancement on the complete server | RF-CLOCK-01 | LAB-JD1 / G4-observation-clock | Direct machine tick is not a server step → Barrier with phases in the authoritative loop | Normal entities and callbacks participate in the step; negative control: Only the selected block entity ticks |
| RF-CLOCK-03 — Before/after action and end of tick | RF-CLOCK-02; RF-BRIDGE-06 | LAB-JD1 / G4-observation-clock | A snapshot with mixed phases is invalid → Tick/phase envelope and atomicity | Reading corresponds to the same logical instant; negative control: Field read on the next tick |
| RF-CLOCK-04 — Client causal barrier | RF-CLOCK-03; RF-CLIENT-06 | LAB-JD1 / G4-observation-clock | Server-ready does not mean menu-updated → client_sent/server_applied/client_applied | Client receives the update before visual comparison; negative control: Capture before a packet is applied |
| RF-CLOCK-05 — Observable RNG and setup order | RF-CLOCK-03; RF-INSTANCE-07 | LAB-JD1 / G4-observation-clock | Same seed without same setup does not prove repeatability → Declared seed, relevant calls and order | Three fresh executions preserve the controlled domain; negative control: Setup consumes RNG before the scenario |
| RF-CLOCK-06 — Observation without loading chunks or mutating caches | RF-CLOCK-03 | LAB-JD1 / G4-observation-clock | A getter that changes evolution fails the control → Safe observer plus external sentinels | Enabling capture does not add tickets or ticks; negative control: Reading forces chunk loading |
| RF-CLOCK-07 — Bridge absent/disabled/enabled controls | RF-CLOCK-06; RF-CLIENT-07 | LAB-JD1 / G4-observation-clock | Two identical observers are insufficient as a control → Equivalent schedule and independent witness | Save and real action corroborate observed state; negative control: Observer changes inventory after reading |
| RF-CLOCK-08 — Fresh repetition and flaky classification | RF-CLOCK-05; RF-CLOCK-07 | LAB-JD1 / G4-observation-clock | Retry-until-green does not qualify determinism → At least three runs with all results retained | Divergence blocks the exact profile until explained; negative control: Select only the best attempt |

## Discovery and reconciliation

| ID and microdelivery | Depends on | Topology/gate | Discriminating RED → minimum GREEN | Functional acceptance and negative control |
| --- | --- | --- | --- | --- |
| RF-DISCOVER-01 — Enumerate actually loaded modules | RF-INSTANCE-06; RF-BRIDGE-03 | LAB-JS / G4-observation-clock | A jar on disk does not prove an active mod → Per-profile runtime catalog | A missing or extra module appears as a lock deviation; negative control: Dependency disabled during load |
| RF-DISCOVER-02 — Enumerate registries and state spaces | RF-DISCOVER-01 | LAB-JS / G4-observation-clock | A translation file does not close the census → Effective IDs and states exported | Each ID receives a family or explicit gap; negative control: Unclassified registry entry |
| RF-DISCOVER-03 — Post-load recipes and resolved ingredients | RF-DISCOVER-02 | LAB-JS / G4-observation-clock | JSON on disk does not prove a loaded recipe → Catalog of types, ingredients and components | Config/datapack changes the set and fingerprint; negative control: Recipe removed after reload |
| RF-DISCOVER-04 — Configurations, tiers and modes | RF-DISCOVER-02; RF-SCOPE-05 | LAB-JS / G4-observation-clock | A static list does not confirm effective options → Accessible values and domains exported | An unknown option prevents a closed domain; negative control: Available upgrade not enumerated |
| RF-DISCOVER-05 — Chemical types, fluids and units | RF-DISCOVER-02; RF-MODEL-08 | LAB-JS / G4-observation-clock | Quantity without a unit is not a sufficient observable → Representations and attributes by type | Incompatible mixtures remain distinguishable; negative control: Normalize two chemicals to the same alias |
| RF-DISCOVER-06 — Menus, slots and item components | RF-DISCOVER-02; RF-CLIENT-06 | LAB-JS / G4-observation-clock | A visual slot does not imply an authoritative slot → Menu mapping and slot semantics | Slot IDs and complete stacks reconciled; negative control: Shift-click without an identified destination |
| RF-DISCOVER-07 — Family and child-requirement reconciliation | RF-DISCOVER-03; RF-DISCOVER-04; RF-DISCOVER-05; RF-DISCOVER-06 | LAB-JS / G4-observation-clock | A complete census does not prove all behaviors → Object→requirement→scenario ledger | Nonregistrable behavior receives an explicit requirement; negative control: Persisted frequency outside a registry ignored |
| RF-DISCOVER-08 — Discovery and reload diff | RF-DISCOVER-07 | LAB-JS / G4-observation-clock | An update must not retain the old denominator → Comparison of versioned catalogs | New item/recipe automatically opens a gap; negative control: Remove an entry to increase the percentage |

## Evidence, mutants and TDD

| ID and microdelivery | Depends on | Topology/gate | Discriminating RED → minimum GREEN | Functional acceptance and negative control |
| --- | --- | --- | --- | --- |
| RF-EVIDENCE-01 — Complete trace per attempt | RF-MODEL-09; RF-BRIDGE-04 | LAB-UNIT, LAB-JD1 / G5-sensitive-validator | Truncation must not become an approved prefix → Sequence and manifest with counts/hash | Offline reading detects EOF before complete; negative control: Delete the last frame |
| RF-EVIDENCE-02 — Atomic result commit | RF-EVIDENCE-01; RF-INSTANCE-08 | LAB-UNIT, LAB-JD1 / G5-sensitive-validator | A previous green does not survive a failed attempt → Transactional publication by attempt ID | A write crash leaves an incomplete result; negative control: Reuse old result.json |
| RF-EVIDENCE-03 — Cache by all effective inputs | RF-EVIDENCE-02; RF-SCOPE-05 | LAB-UNIT, LAB-JD1 / G5-sensitive-validator | Bridge changes invalidate cache → Content-based key and verifiable fresh mode | Fresh implies a new process and a new trace; negative control: Cache labeled as fresh execution |
| RF-EVIDENCE-04 — Type and value mutants | RF-MODEL-03; RF-MODEL-04; RF-MODEL-05; RF-EVIDENCE-01 | LAB-UNIT, LAB-JD1 / G5-sensitive-validator | Comparator must kill a known difference → Mutant matrix by field family | One bit/unit/type/presence changes the correct verdict; negative control: Never-observed mutant counted as killed |
| RF-EVIDENCE-05 — Temporal and causal mutants | RF-MODEL-07; RF-CLOCK-04; RF-EVIDENCE-04 | LAB-UNIT, LAB-JD1 / G5-sensitive-validator | An identical final state does not rescue an incorrect transient state → Perturb phase, order, tick and actor | First divergence identifies the correct boundary; negative control: Remove intermediate consumption |
| RF-EVIDENCE-06 — Session and transport failures | RF-BRIDGE-07; RF-EVIDENCE-01 | LAB-UNIT, LAB-JD1 / G5-sensitive-validator | Duplication and gaps are not equivalent to silence → Invalid-frame and deadline campaign | Each failure ends without pass and with complete logs; negative control: Frame from the wrong session |
| RF-EVIDENCE-07 — Action proof independent of the trace | RF-CLIENT-07; RF-CLOCK-07; RF-EVIDENCE-04 | LAB-UNIT, LAB-JD1 / G5-sensitive-validator | Mutating JSON does not prove correct game capture → Real action and second save/client witness | Effective change is detected through both paths; negative control: Observer returns a constant snapshot |
| RF-EVIDENCE-08 — TDD contract and RED/GREEN trail | RF-MODEL-10; RF-EVIDENCE-02 | LAB-UNIT, LAB-JD1 / G5-sensitive-validator | Infra_error does not count as semantic RED → Distinct engine-contract/guard/port records | Prior test fails for the expected cause and refactor reruns it; negative control: Mark failed boot as a gameplay mismatch |

## Pilot machine in slices

| ID and microdelivery | Depends on | Topology/gate | Discriminating RED → minimum GREEN | Functional acceptance and negative control |
| --- | --- | --- | --- | --- |
| RF-PILOT-01 — Resolve a concrete Enrichment Chamber recipe | RF-DISCOVER-03; RF-DISCOVER-05; RF-EVIDENCE-08 | LAB-JD1 / G6-pilot-functional | Without a resolved recipe, the result is blocked, not port RED → Real bindings without assumed numbers | The same input and config select a sealable reference; negative control: Selected recipe changed on reload |
| RF-PILOT-02 — Build a minimal fixture and trace setup | RF-PILOT-01; RF-CLIENT-05 | LAB-JD1 / G6-pilot-functional | Injecting energy does not prove generator/cable behavior → Declared fixture and initial-state witness | Position, orientation, slots and resources verified; negative control: Machine preloaded with the result |
| RF-PILOT-03 — Process a recipe with a per-tick trace | RF-PILOT-02; RF-CLOCK-08; RF-EVIDENCE-07 | LAB-JD1 / G6-pilot-functional | The final item alone does not characterize consumption/progress → Input/output/energy/progress capture | Three complete Java runs produce a repeatable reference; negative control: Extra consumption hidden between snapshots |
| RF-PILOT-04 — Invalid input and rejection without effects | RF-PILOT-03 | LAB-JD1 / G6-pilot-functional | Accepting invalid input produces future divergence → Rejection scenario through the real path | Wrong resource does not silently disappear; negative control: Insert an item with an incompatible component |
| RF-PILOT-05 — Energy absence and resumption | RF-PILOT-03 | LAB-JD1 / G6-pilot-functional | Lack of energy is not a runner timeout → Scenario with a controlled source and restoration | Paused state and resumption characterized per tick; negative control: Inject energy without a declared action |
| RF-PILOT-06 — Full output and unblocking | RF-PILOT-03; RF-CLIENT-06 | LAB-JD1 / G6-pilot-functional | Insertion failure must not duplicate output → Occupancy blocking and player extraction | Consumption and progress follow Java observations; negative control: Discard the result on resumption |
| RF-PILOT-07 — Insufficient input and partial batches | RF-PILOT-03 | LAB-JD1 / G6-pilot-functional | Batch boundaries must not be inferred from the port → Quantity variants resolved from the recipe | Consumption and remainders recorded at valid boundaries; negative control: Greater quantity converted with loss |
| RF-PILOT-08 — Real menu tracks the machine | RF-PILOT-03; RF-CLOCK-04; RF-CLIENT-06 | LAB-JD1 / G6-pilot-functional | A correct backend field does not prove the gauge/text → Scenario with the menu open during processing | Slots, energy and progress converge on the client; negative control: Screen shows old progress |
| RF-PILOT-09 — Persist mid-process | RF-PILOT-03; RF-INSTANCE-07; RF-EVIDENCE-02 | LAB-JD1 / G6-pilot-functional | Terminal NBT alone does not prove continuation → Checkpoint with reopen and subsequent ticks | Next transition after load matches the reference; negative control: Progress disappears on reload |
| RF-PILOT-10 — Pilot slice gate per profile | RF-PILOT-04; RF-PILOT-05; RF-PILOT-06; RF-PILOT-07; RF-PILOT-08; RF-PILOT-09 | LAB-JD1 / G6-pilot-functional | Pending observation leaves the slice unqualified → Slice report with independent evidence | No required item unobserved/unsupported/flaky; negative control: Approve based only on the case average |

## Persistence and lifecycle

| ID and microdelivery | Depends on | Topology/gate | Discriminating RED → minimum GREEN | Functional acceptance and negative control |
| --- | --- | --- | --- | --- |
| RF-LIFE-01 — Durable flush and close with ACK | RF-PILOT-09; RF-BRIDGE-06 | LAB-JD1, LAB-JD2 / G7-domain-slices | Request ACK does not mean flush completed → Save/flush/close barrier | Opening the saved copy confirms the declared checkpoint; negative control: Terminate JVM before flush |
| RF-LIFE-02 — Restart in a new process | RF-LIFE-01; RF-INSTANCE-07 | LAB-JD1, LAB-JD2 / G7-domain-slices | Reopening a screen in the same process is not restart → New JVM loads the same lifecycle world | Persisted state and subsequent transitions observed; negative control: Reuse the process's static cache |
| RF-LIFE-03 — Real chunk unload/reload | RF-LIFE-02; RF-CLIENT-04 | LAB-JD1, LAB-JD2 / G7-domain-slices | Hiding rendering does not mean a chunk is unloaded → Move actor/tickets and prove lifecycle | Block entities and networks undergo observed unload; negative control: Observer maintains an artificial ticket |
| RF-LIFE-04 — Dimension change and return | RF-LIFE-03 | LAB-JD1, LAB-JD2 / G7-domain-slices | Changing coordinates without dimension does not cover travel → Travel actions with epochs and aliases | Consistent inventory, frequencies and global objects; negative control: Alias reused across dimensions |
| RF-LIFE-05 — Death, respawn and inventory | RF-CLIENT-05; RF-MODEL-06; RF-LIFE-02 | LAB-JD1, LAB-JD2 / G7-domain-slices | A new entity must not inherit an old snapshot → Death/respawn scenarios with a pinned rule | Equipment, drops and components observed; negative control: KeepInventory assumed without configuration |
| RF-LIFE-06 — Crash before and after durability | RF-LIFE-01; RF-EVIDENCE-02 | LAB-JD1, LAB-JD2 / G7-domain-slices | Requiring persistence of a nondurable operation is a false expected result → Declared crash points in a disposable instance | Survival is characterized in Java at each point; negative control: Compare a save still being written |
| RF-LIFE-07 — Data and resource reload | RF-DISCOVER-08; RF-LIFE-02 | LAB-JD1, LAB-JD2 / G7-domain-slices | Reload may invalidate recipe binding → Explicit reload steps and new observation | New recipes/configs open a new baseline; negative control: Retain a golden from the previous configuration |
| RF-LIFE-08 — Rerun after failure without contamination | RF-LIFE-06; RF-INSTANCE-10 | LAB-JD1, LAB-JD2 / G7-domain-slices | Diagnostic run must not change the original fixture → New attempt from the correct starting point | Reproduction retains the fingerprint and defined limits; negative control: Use a broken save as pristine |

## Configuration, security and resources

| ID and microdelivery | Depends on | Topology/gate | Discriminating RED → minimum GREEN | Functional acceptance and negative control |
| --- | --- | --- | --- | --- |
| RF-CONFIG-01 — Faces and rotation through the Configurator | RF-PILOT-10; RF-DISCOVER-04 | LAB-JD1 / G7-domain-slices | Rotation does not authorize sorting network priorities → Logical/physical face and reconnection scenarios | Observed transfer changes with the real configuration; negative control: Change only the block in setup |
| RF-CONFIG-02 — Redstone at boundaries and during processing | RF-PILOT-10 | LAB-JD1 / G7-domain-slices | Unapplied toggle leaves the precondition invalid → Signal actions with phases and witness | Paused/resumed state observed on the server; negative control: Signal changes between reading and action |
| RF-CONFIG-03 — Install and remove upgrades during operation | RF-CONFIG-02; RF-CLIENT-06 | LAB-JD1 / G7-domain-slices | Nominal count does not prove effect → Scenarios per upgrade and discovered limits | Trace characterizes cost, rate and removal behavior; negative control: Assumed maximum instead of discovered maximum |
| RF-CONFIG-04 — Tier installers preserve contents | RF-CONFIG-03; RF-DISCOVER-04 | LAB-JD1 / G7-domain-slices | Tier upgrade must not erase components → Scenarios with occupied buffers and slots | Before/after state and invalid attempt recorded; negative control: Contents artificially restored after upgrade |
| RF-CONFIG-05 — Tanks and cubes at boundaries | RF-DISCOVER-05; RF-CONFIG-01 | LAB-JD1 / G7-domain-slices | Mixing incompatible types must not be normalized → Valid limits, partial insertion/extraction and modes | All buffers and units have an independent witness; negative control: A terminal receives a resource without a debit |
| RF-CONFIG-06 — Bins and chests/barrels with components | RF-CLIENT-06; RF-CONFIG-04 | LAB-JD1 / G7-domain-slices | Same ID with different components is not the same stack → Stacking, extraction, security and breaking scenarios | Real inventory and drops preserve complete stacks; negative control: Item NBT discarded in compare |
| RF-CONFIG-07 — Security Desk and authorization profiles | RF-CLIENT-02; RF-CONFIG-06 | LAB-JD1 / G7-domain-slices | Owner and visitor must not merge identities → Two actors, modes and allowed/denied operations | A denied action does not change world/inventory; negative control: UI blocks but server accepts the operation |
| RF-CONFIG-08 — Configuration Card and persisted frequencies | RF-CONFIG-07; RF-LIFE-02 | LAB-JD1 / G7-domain-slices | Copying config does not authorize copying items or owner → Source/destination, incompatibility and restart | Copied and preserved data characterized; negative control: Public frequency treated as private |

## Networks, filters and QIO

| ID and microdelivery | Depends on | Topology/gate | Discriminating RED → minimum GREEN | Functional acceptance and negative control |
| --- | --- | --- | --- | --- |
| RF-NET-01 — Energy cable merge/split | RF-CONFIG-05; RF-LIFE-03 | LAB-JD1, LAB-JD2 / G7-domain-slices | Aggregate result does not prove per-node distribution → Small topologies with controlled demand | Capture buffers/terminals before and after reconnection; negative control: Duplicate energy on split |
| RF-NET-02 — Chemical tubes and fluid pipes | RF-NET-01; RF-DISCOVER-05 | LAB-JD1, LAB-JD2 / G7-domain-slices | Different resources must not share an alias → Line/branch/cycle and incompatible types | Per-terminal balance and rejections observed; negative control: Contamination hidden in the global buffer |
| RF-NET-03 — Transporters with destinations and colors | RF-CLIENT-05; RF-CONFIG-06; RF-LIFE-03 | LAB-JD1, LAB-JD2 / G7-domain-slices | A delivered item does not prove route/priority → Topologies with concurrency and blocked paths | Entities/stacks in transit and destinations preserved; negative control: Stack disappears during chunk unload |
| RF-NET-04 — Sorter filters and fallback | RF-NET-03; RF-DISCOVER-04 | LAB-JD1, LAB-JD2 / G7-domain-slices | A filter name does not prove its predicate → Concrete filters, order, fallback and round robin | Each filter type finds valid/invalid witnesses; negative control: Wildcard filter configured as exact |
| RF-NET-05 — Entangloporter across dimensions and owners | RF-NET-01; RF-NET-02; RF-CONFIG-08; RF-LIFE-04 | LAB-JD1, LAB-JD2 / G7-domain-slices | Distance and frequency do not erase authorization → Local/remote pairs with supported resources | Two terminals and global store corroborate transfer; negative control: Another owner's frequency selected |
| RF-NET-06 — QIO drives and dashboards | RF-CONFIG-06; RF-CONFIG-08; RF-LIFE-02 | LAB-JD1, LAB-JD2 / G7-domain-slices | A correct total can hide a lost component → Type/quantity limits, query and extraction | Two clients and persisted store preserve stacks; negative control: Concurrent extraction exceeds stock |
| RF-NET-07 — QIO import/export and redstone | RF-NET-06; RF-CONFIG-02 | LAB-JD1, LAB-JD2 / G7-domain-slices | Importing one item does not cover loops and rejection → Filters, boundaries and return circuit | Debits/credits and signal monitored causally; negative control: Import and export duplicate in the same tick |
| RF-NET-08 — Topology and chunk stress | RF-NET-01; RF-NET-02; RF-NET-03; RF-NET-05; RF-NET-07 | LAB-JD1, LAB-JD2 / G7-domain-slices | Soak without declared coverage does not close networks → Reproducible editing and restart sequences | Reduced reproduction retains the divergence point; negative control: Merge/split ordered differently without a record |

## Multiblocks, heat and radiation

| ID and microdelivery | Depends on | Topology/gate | Discriminating RED → minimum GREEN | Functional acceptance and negative control |
| --- | --- | --- | --- | --- |
| RF-DEEP-01 — Multiblock formation and invalidation | RF-CONFIG-05; RF-CLIENT-05 | LAB-JD1, LAB-JD2 / G7-domain-slices | An injected ready assembly does not prove formation → Construction through actions and separate fixtures | Formed state, ports and identity observed; negative control: Ignore a missing structural block |
| RF-DEEP-02 — Dynamic Tank and Induction Matrix | RF-DEEP-01; RF-NET-01 | LAB-JD1, LAB-JD2 / G7-domain-slices | Persisting only the controller does not cover the structure → Ports/limits/rupture and reconstruction | Buffers and throughput follow the Java reference; negative control: Rupture duplicates contents across multiple ports |
| RF-DEEP-03 — Heat, conductors and heaters | RF-NET-01; RF-MODEL-04 | LAB-JD1, LAB-JD2 / G7-domain-slices | Global temperature rounding hides errors → Gradients, environment and thermal inputs | Thermal trace with discovered units/costs; negative control: Neighboring block temperature omitted |
| RF-DEEP-04 — Boiler and thermal evaporation | RF-DEEP-01; RF-DEEP-03 | LAB-JD1, LAB-JD2 / G7-domain-slices | Final temperature does not cover intermediate consumption → Water/steam/chemical/environment states | Per-phase resources, heat and formation observed; negative control: Input runs out but production continues |
| RF-DEEP-05 — Turbine and steam chain | RF-DEEP-04; RF-NET-02 | LAB-JD1, LAB-JD2 / G7-domain-slices | A generator with infinite input does not cover the chain → Turbine by return modes and load | Complete chain and isolated component kept separate; negative control: Condensate removed from the balance |
| RF-DEEP-06 — Fission, safety and waste | RF-DEEP-01; RF-DEEP-04 | LAB-JD1, LAB-JD2 / G7-domain-slices | Signaling on does not prove generation/heat → Normal, cooling, SCRAM and failure states | Results and risks characterized in disposable Java instances; negative control: Delayed alarm or missing waste |
| RF-DEEP-07 — Fusion, laser and D-T chain | RF-DEEP-03; RF-NET-02 | LAB-JD1, LAB-JD2 / G7-domain-slices | An already ignited fixture does not cover ignition → Fuel, ignition, shutdown and return | Causal chain and resources measured per tick; negative control: Laser energy not debited |
| RF-DEEP-08 — SPS, waste and radiation | RF-DEEP-06; RF-MODEL-04; RF-LIFE-02 | LAB-JD1, LAB-JD2 / G7-domain-slices | Machine NBT does not cover global radiation → Processing, leakage and persistence scenarios | Environmental store and player exposure corroborate effects; negative control: Source persists in only one store |

## World, equipment and modules

| ID and microdelivery | Depends on | Topology/gate | Discriminating RED → minimum GREEN | Functional acceptance and negative control |
| --- | --- | --- | --- | --- |
| RF-MODS-01 — Digital Miner filters and replacement | RF-NET-04; RF-CLIENT-05 | LAB-JD1, LAB-JI / G7-domain-slices | Breaking a block through a command does not prove miner behavior → Area, filters, protection and replacement defined | World/drops/energy and area boundary observed; negative control: Miner crosses a boundary or protection |
| RF-MODS-02 — Pump and Plenisher change the world | RF-NET-02; RF-CLIENT-05 | LAB-JD1, LAB-JI / G7-domain-slices | A changed tank does not prove fluid source/destination → Sources, obstruction and placement mode | Blocks and buffers show the corresponding operation; negative control: Water generated without a valid source |
| RF-MODS-03 — Robit inventory, AI and return | RF-LIFE-05; RF-CONFIG-07 | LAB-JD1, LAB-JI / G7-domain-slices | Fixture teleport does not prove Robit return → Player actions and robot transitions | Entity, owner, inventory and charge observed; negative control: Despawn loses inventory without a drop |
| RF-MODS-04 — Meka equipment and modules | RF-CONFIG-03; RF-LIFE-05 | LAB-JD1, LAB-JI / G7-domain-slices | A changed attribute does not prove a player effect → Installation, modes, conflicts and resources | Movement/damage/mining and consumption through a real action; negative control: Disabled module continues applying its effect |
| RF-MODS-05 — Jetpack, Scuba and Free Runners | RF-MODS-04; RF-CLIENT-04 | LAB-JD1, LAB-JI / G7-domain-slices | Directly changing position does not prove equipment behavior → Input, fuel, environment and exhaustion | Server and client see effect and consumption; negative control: No fuel but movement continues |
| RF-MODS-06 — Tools, paxels, armor and shields | RF-CLIENT-05; RF-LIFE-05 | LAB-JD1, LAB-JI / G7-domain-slices | A stat value does not replace mining/combat → Targets and actions by discovered material/mode | Real durability, damage, defense and transformation; negative control: Equivalent item with the wrong material |
| RF-MODS-07 — Simple generators and environment | RF-CONFIG-05; RF-DEEP-03 | LAB-JD1, LAB-JI / G7-domain-slices | Pre-injected charge does not prove generation → Fuel, sun, wind and configuration | Per-tick energy, costs and environmental condition; negative control: Environmental condition absent but generation maintained |
| RF-MODS-08 — Worldgen and drops by seed | RF-DISCOVER-02; RF-LIFE-03 | LAB-JD1, LAB-JI / G7-domain-slices | An ore registry entry does not prove generation → New chunks, seeds and frozen parameters | Real blocks/features and drops kept separate; negative control: Compare chunks saved under another config |
| RF-MODS-09 — Additions blocks, mobs, TNT and balloons | RF-CLIENT-05; RF-LIFE-05 | LAB-JD1, LAB-JI / G7-domain-slices | Model presence does not prove a physical property → Separate scenarios per family and actor | Light, collision, explosion, loot and attachment observed; negative control: Entity only drawn without server state |
| RF-MODS-10 — Walkie-Talkie and external integrations | RF-MODS-09; RF-CLIENT-02 | LAB-JD1, LAB-JI / G7-domain-slices | An audio event does not prove the communication path → Dedicated profiles and explicit support/availability | Allowed/denied channel and correct actors recorded; negative control: Message arrives on an unauthorized channel |

## Multiplayer, integrated, UI and audio

| ID and microdelivery | Depends on | Topology/gate | Discriminating RED → minimum GREEN | Functional acceptance and negative control |
| --- | --- | --- | --- | --- |
| RF-UX-01 — Two real simultaneous clients | RF-CLIENT-08; RF-CLOCK-04; RF-CONFIG-07 | LAB-JD2, LAB-JI / G7-domain-slices | Two bots do not qualify two GUIs → D2 with isolated actors and input | One actor's action does not change the other's focus/menu; negative control: Both controlled through the same identity |
| RF-UX-02 — Slot and tank concurrency | RF-UX-01; RF-PILOT-08; RF-CONFIG-05 | LAB-JD2, LAB-JI / G7-domain-slices | The last snapshot can hide transient duplication → Causal schedule of two real operations | Individual inventories and server state close the balance; negative control: Two withdrawals receive the same last item |
| RF-UX-03 — Disconnect, reconnect and late join | RF-UX-01; RF-LIFE-02 | LAB-JD2, LAB-JI / G7-domain-slices | Old readiness does not prove the current session → New session/epoch and resynchronization | New client receives current machine/network state; negative control: Previous packets contaminate reconnection |
| RF-UX-04 — Integrated server with local player | RF-CLIENT-08; RF-CLOCK-04; RF-PILOT-10 | LAB-JD2, LAB-JI / G7-domain-slices | Dedicated pass does not imply integrated pass → I1 with observed integrated lifecycle | Same actions and effects on the integrated server thread; negative control: Menu pause changes the clock without a record |
| RF-UX-05 — Widgets, keys, search and accessibility | RF-CLIENT-06; RF-PILOT-08 | LAB-JD2, LAB-JI / G7-domain-slices | An internal menu method does not prove input → Semantic interaction and mapped coordinates where necessary | Focus, search, tooltip and carried stack observed; negative control: Key captured by another widget |
| RF-UX-06 — Machine, network and particle rendering | RF-UX-01; RF-NET-08; RF-DEEP-08 | LAB-JD2, LAB-JI / G7-domain-slices | A similar overall image does not guarantee gauge/model correctness → Per-region captures and semantic state | Controls detectably change LED, liquid, connection and text; negative control: Missing model hidden by tolerance |
| RF-UX-07 — Audio, alarms and client resources | RF-UX-06; RF-DEEP-06 | LAB-JD2, LAB-JI / G7-domain-slices | A sound event does not prove audible playback → Separate event and device profiles | Start/stop/relative volume and actor made explicit; negative control: Sound continues after the machine stops |
| RF-UX-08 — Locale, GUI scale and resource reload | RF-UX-05; RF-UX-06; RF-LIFE-07 | LAB-JD2, LAB-JI / G7-domain-slices | Capture in the wrong locale does not legitimately compare text → Client matrix and visual baseline per profile | Widgets and states persist after reload; negative control: Tolerance masks truncated text |

## Generation, platforms and closure

| ID and microdelivery | Depends on | Topology/gate | Discriminating RED → minimum GREEN | Functional acceptance and negative control |
| --- | --- | --- | --- | --- |
| RF-AUDIT-01 — Generate valid boundaries and combinations | RF-DISCOVER-07; RF-PILOT-10 | LAB-UNIT, LAB-JD1, LAB-JD2, LAB-JI / G8-scoped-preport-audit | Generating invalid states does not cover valid transitions → Generator by domain with explicit constraints | Seed and expansion reproduce the same scenario; negative control: Pairwise omits a known critical combination |
| RF-AUDIT-02 — Stateful sequences and metamorphic hypotheses | RF-AUDIT-01; RF-NET-08 | LAB-UNIT, LAB-JD1, LAB-JD2, LAB-JI / G8-scoped-preport-audit | A relationship unproven in Java does not become an oracle → Relationships approved by profile and assumptions | Rerun N ticks in parts only where equivalence is proven; negative control: Rotation changes real network priority |
| RF-AUDIT-03 — Shrinking preserves reproduction | RF-AUDIT-02; RF-EVIDENCE-05 | LAB-UNIT, LAB-JD1, LAB-JD2, LAB-JI / G8-scoped-preport-audit | Reducing by removing a precondition does not reproduce the original failure → Shrinker retains causal class/tick and seed | Reduced case fails in a new instance for the same reason; negative control: Minimization turns mismatch into infra_error |
| RF-AUDIT-04 — Selection by requirements and dependencies | RF-SCOPE-08; RF-DISCOVER-08; RF-AUDIT-01 | LAB-UNIT, LAB-JD1, LAB-JD2, LAB-JI / G8-scoped-preport-audit | Text matching in docs does not select behavior → Versioned field/action/requirement/scenario graph | A codec change selects actually affected scenarios; negative control: New requirement omitted from incremental selection |
| RF-AUDIT-05 — Native Windows and Linux matrix | RF-INSTANCE-10; RF-UX-04; RF-AUDIT-03 | LAB-UNIT, LAB-JD1, LAB-JD2, LAB-JI / G8-scoped-preport-audit | Compiling Rust does not prove Java/GL launch → Real suites by OS, JDK, display and paths | Same readiness/cleanup/evidence contract per platform; negative control: Unix args used as universal |
| RF-AUDIT-06 — Budgets, soak and resource pressure | RF-INSTANCE-09; RF-NET-08; RF-UX-03 | LAB-UNIT, LAB-JD1, LAB-JD2, LAB-JI / G8-scoped-preport-audit | Timeout is not a semantic mismatch → Bounded load and causality observation | Pressure produces diagnostics and no empty approval; negative control: Batch exhausts GPU and old run stays green |
| RF-AUDIT-07 — Multidimensional coverage without misleading averages | RF-DISCOVER-08; RF-EVIDENCE-08; RF-AUDIT-04 | LAB-UNIT, LAB-JD1, LAB-JD2, LAB-JI / G8-scoped-preport-audit | Skip and unsupported do not increase completion → Profile/requirement/transition/observable/topology report | Each pending required item blocks its scope; negative control: Hide an empty domain through a weighted average |
| RF-AUDIT-08 — Independent-operator replay | RF-AUDIT-03; RF-AUDIT-05; RF-AUDIT-07 | LAB-UNIT, LAB-JD1, LAB-JD2, LAB-JI / G8-scoped-preport-audit | Replay without an identical artifact does not certify evidence → Reproduction bundle with hashes and steps | Another machine supplying jars recreates the case and diagnosis; negative control: Missing private fixture silently substituted |
| RF-AUDIT-09 — Pre-port audit by closed scope | RF-AUDIT-06; RF-AUDIT-08; RF-UX-08; RF-MODS-10; RF-DEEP-08 | LAB-UNIT, LAB-JD1, LAB-JD2, LAB-JI / G8-scoped-preport-audit | Java/Java must not be labeled Java/Rust → M13 gate by domain, module, platform and fingerprint | All slice requirements have controls and fresh evidence; negative control: Promote incomplete scope as complete Mek |
| RF-AUDIT-10 — Contract for the future first Java/Rust RED | RF-AUDIT-09; RF-SCOPE-07 | LAB-JR / G9-future-port-TDD | Unsupported/failed boot blocks; it is not gameplay RED → M14 plan with a valid fixture and sealed Java golden | Only a proven behavior mismatch starts the port cycle; negative control: Generate expected results from the Rust implementation |

## Traceability of previous epics

The links below preserve IDs used in the previous planning. They are not open issues or implementation evidence. The table by M is only thematic origin; RF-CLIENT still depends on boot/bridge, not on completing all of M10.

| Epic | Microdeliveries |
| --- | --- |
| MP-001 | RF-SCOPE-01; RF-SCOPE-02; RF-SCOPE-03; RF-SCOPE-04; RF-SCOPE-05; RF-SCOPE-06; RF-SCOPE-07; RF-SCOPE-08 |
| MP-002 | RF-MODEL-01; RF-MODEL-02; RF-MODEL-03; RF-MODEL-04; RF-MODEL-05; RF-MODEL-06; RF-MODEL-07; RF-MODEL-08; RF-MODEL-09; RF-MODEL-10 |
| MP-003 | RF-MODEL-01; RF-MODEL-02; RF-MODEL-03; RF-MODEL-04; RF-MODEL-05; RF-MODEL-06; RF-MODEL-07; RF-MODEL-08; RF-MODEL-09; RF-MODEL-10 |
| MP-004 | RF-MODEL-01; RF-MODEL-02; RF-MODEL-03; RF-MODEL-04; RF-MODEL-05; RF-MODEL-06; RF-MODEL-07; RF-MODEL-08; RF-MODEL-09; RF-MODEL-10 |
| MP-005 | RF-MODEL-01; RF-MODEL-02; RF-MODEL-03; RF-MODEL-04; RF-MODEL-05; RF-MODEL-06; RF-MODEL-07; RF-MODEL-08; RF-MODEL-09; RF-MODEL-10 |
| MP-006 | RF-INSTANCE-01; RF-INSTANCE-02; RF-INSTANCE-03; RF-INSTANCE-04; RF-INSTANCE-05; RF-INSTANCE-06; RF-INSTANCE-07; RF-INSTANCE-08; RF-INSTANCE-09; RF-INSTANCE-10 |
| MP-007 | RF-INSTANCE-01; RF-INSTANCE-02; RF-INSTANCE-03; RF-INSTANCE-04; RF-INSTANCE-05; RF-INSTANCE-06; RF-INSTANCE-07; RF-INSTANCE-08; RF-INSTANCE-09; RF-INSTANCE-10; RF-EVIDENCE-01; RF-EVIDENCE-02; RF-EVIDENCE-03; RF-EVIDENCE-04; RF-EVIDENCE-05; RF-EVIDENCE-06; RF-EVIDENCE-07; RF-EVIDENCE-08 |
| MP-008 | RF-BRIDGE-01; RF-BRIDGE-02; RF-BRIDGE-03; RF-BRIDGE-04; RF-BRIDGE-05; RF-BRIDGE-06; RF-BRIDGE-07; RF-BRIDGE-08 |
| MP-009 | RF-CLOCK-01; RF-CLOCK-02; RF-CLOCK-03; RF-CLOCK-04; RF-CLOCK-05; RF-CLOCK-06; RF-CLOCK-07; RF-CLOCK-08 |
| MP-010 | RF-DISCOVER-01; RF-DISCOVER-02; RF-DISCOVER-03; RF-DISCOVER-04; RF-DISCOVER-05; RF-DISCOVER-06; RF-DISCOVER-07; RF-DISCOVER-08 |
| MP-011 | RF-CLIENT-01; RF-CLIENT-02; RF-CLIENT-03; RF-CLIENT-04; RF-CLIENT-05; RF-CLIENT-06; RF-CLIENT-07; RF-CLIENT-08 |
| MP-012 | RF-PILOT-01; RF-PILOT-02; RF-PILOT-03; RF-PILOT-04; RF-PILOT-05; RF-PILOT-06; RF-PILOT-07; RF-PILOT-08; RF-PILOT-09; RF-PILOT-10 |
| MP-013 | RF-CLOCK-01; RF-CLOCK-02; RF-CLOCK-03; RF-CLOCK-04; RF-CLOCK-05; RF-CLOCK-06; RF-CLOCK-07; RF-CLOCK-08 |
| MP-014 | RF-EVIDENCE-01; RF-EVIDENCE-02; RF-EVIDENCE-03; RF-EVIDENCE-04; RF-EVIDENCE-05; RF-EVIDENCE-06; RF-EVIDENCE-07; RF-EVIDENCE-08 |
| MP-015 | RF-LIFE-01; RF-LIFE-02; RF-LIFE-03; RF-LIFE-04; RF-LIFE-05; RF-LIFE-06; RF-LIFE-07; RF-LIFE-08 |
| MP-016 | RF-PILOT-01; RF-PILOT-02; RF-PILOT-03; RF-PILOT-04; RF-PILOT-05; RF-PILOT-06; RF-PILOT-07; RF-PILOT-08; RF-PILOT-09; RF-PILOT-10 |
| MP-017 | RF-CONFIG-01; RF-CONFIG-02; RF-CONFIG-03; RF-CONFIG-04; RF-CONFIG-05; RF-CONFIG-06; RF-CONFIG-07; RF-CONFIG-08 |
| MP-018 | RF-NET-01; RF-NET-02; RF-NET-03; RF-NET-04; RF-NET-05; RF-NET-06; RF-NET-07; RF-NET-08 |
| MP-019 | RF-DEEP-01; RF-DEEP-02; RF-DEEP-03; RF-DEEP-04; RF-DEEP-05; RF-DEEP-06; RF-DEEP-07; RF-DEEP-08 |
| MP-020 | RF-MODS-01; RF-MODS-02; RF-MODS-03; RF-MODS-04; RF-MODS-05; RF-MODS-06; RF-MODS-07; RF-MODS-08; RF-MODS-09; RF-MODS-10 |
| MP-021 | RF-CLIENT-01; RF-CLIENT-02; RF-CLIENT-03; RF-CLIENT-04; RF-CLIENT-05; RF-CLIENT-06; RF-CLIENT-07; RF-CLIENT-08; RF-UX-01; RF-UX-02; RF-UX-03; RF-UX-04; RF-UX-05; RF-UX-06; RF-UX-07; RF-UX-08 |
| MP-022 | RF-UX-01; RF-UX-02; RF-UX-03; RF-UX-04; RF-UX-05; RF-UX-06; RF-UX-07; RF-UX-08 |
| MP-023 | RF-AUDIT-01; RF-AUDIT-02; RF-AUDIT-03; RF-AUDIT-04; RF-AUDIT-05; RF-AUDIT-06; RF-AUDIT-07; RF-AUDIT-08; RF-AUDIT-09; RF-AUDIT-10 |
| MP-024 | RF-INSTANCE-01; RF-INSTANCE-02; RF-INSTANCE-03; RF-INSTANCE-04; RF-INSTANCE-05; RF-INSTANCE-06; RF-INSTANCE-07; RF-INSTANCE-08; RF-INSTANCE-09; RF-INSTANCE-10; RF-AUDIT-01; RF-AUDIT-02; RF-AUDIT-03; RF-AUDIT-04; RF-AUDIT-05; RF-AUDIT-06; RF-AUDIT-07; RF-AUDIT-08; RF-AUDIT-09; RF-AUDIT-10 |
| MP-025 | RF-AUDIT-01; RF-AUDIT-02; RF-AUDIT-03; RF-AUDIT-04; RF-AUDIT-05; RF-AUDIT-06; RF-AUDIT-07; RF-AUDIT-08; RF-AUDIT-09; RF-AUDIT-10 |

| Historical phase | Microdeliveries |
| --- | --- |
| M00 | RF-SCOPE-01; RF-SCOPE-02; RF-SCOPE-03; RF-SCOPE-04; RF-SCOPE-05; RF-SCOPE-06; RF-SCOPE-07; RF-SCOPE-08 |
| M01 | RF-MODEL-01; RF-MODEL-02; RF-MODEL-03; RF-MODEL-04; RF-MODEL-05; RF-MODEL-06; RF-MODEL-07; RF-MODEL-08; RF-MODEL-09; RF-MODEL-10 |
| M02 | RF-INSTANCE-01; RF-INSTANCE-02; RF-INSTANCE-03; RF-INSTANCE-04; RF-INSTANCE-05; RF-INSTANCE-06; RF-INSTANCE-07; RF-INSTANCE-08; RF-INSTANCE-09; RF-INSTANCE-10 |
| M03 | RF-BRIDGE-01; RF-BRIDGE-02; RF-BRIDGE-03; RF-BRIDGE-04; RF-BRIDGE-05; RF-BRIDGE-06; RF-BRIDGE-07; RF-BRIDGE-08; RF-CLOCK-01; RF-CLOCK-02; RF-CLOCK-03; RF-CLOCK-04; RF-CLOCK-05; RF-CLOCK-06; RF-CLOCK-07; RF-CLOCK-08 |
| M04 | RF-DISCOVER-01; RF-DISCOVER-02; RF-DISCOVER-04; RF-DISCOVER-05; RF-DISCOVER-06; RF-DISCOVER-07; RF-DISCOVER-08 |
| M05 | RF-LIFE-01; RF-LIFE-02; RF-LIFE-03; RF-LIFE-04; RF-LIFE-05; RF-LIFE-06; RF-LIFE-07; RF-LIFE-08 |
| M06 | RF-EVIDENCE-01; RF-EVIDENCE-02; RF-EVIDENCE-03; RF-EVIDENCE-04; RF-EVIDENCE-05; RF-EVIDENCE-06; RF-EVIDENCE-07; RF-EVIDENCE-08 |
| M07 | RF-DISCOVER-03 |
| M08 | RF-PILOT-01; RF-PILOT-02; RF-PILOT-03; RF-PILOT-04; RF-PILOT-05; RF-PILOT-06; RF-PILOT-07; RF-PILOT-08; RF-PILOT-09; RF-PILOT-10; RF-CONFIG-01; RF-CONFIG-02; RF-CONFIG-03; RF-CONFIG-04; RF-CONFIG-05; RF-CONFIG-06; RF-CONFIG-07; RF-CONFIG-08 |
| M09 | RF-NET-01; RF-NET-02; RF-NET-03; RF-NET-04; RF-NET-05; RF-NET-06; RF-NET-07; RF-NET-08 |
| M10 | RF-DEEP-01; RF-DEEP-02; RF-DEEP-03; RF-DEEP-04; RF-DEEP-05; RF-DEEP-06; RF-DEEP-07; RF-DEEP-08; RF-MODS-01; RF-MODS-02; RF-MODS-03; RF-MODS-04; RF-MODS-05; RF-MODS-06; RF-MODS-07; RF-MODS-08; RF-MODS-09; RF-MODS-10 |
| M11 | RF-CLIENT-01; RF-CLIENT-02; RF-CLIENT-03; RF-CLIENT-04; RF-CLIENT-05; RF-CLIENT-06; RF-CLIENT-07; RF-CLIENT-08; RF-UX-01; RF-UX-02; RF-UX-03; RF-UX-04; RF-UX-05; RF-UX-06; RF-UX-07; RF-UX-08 |
| M12 | RF-AUDIT-01; RF-AUDIT-02; RF-AUDIT-03; RF-AUDIT-04 |
| M13 | RF-AUDIT-05; RF-AUDIT-06; RF-AUDIT-07; RF-AUDIT-08; RF-AUDIT-09 |
| M14 | RF-AUDIT-10 |

## Operation, regressions and closure condition

Run local contract tests for every core change; a real server for runner/bridge changes; a real client for input/clock/observation changes; two clients and integrated mode for authority/menu/sync changes. Incremental selection uses real relationships between action/field/requirement/scenario, never word searches in documents. Before a gate, run the complete mandatory scope corpus in fresh attempts; incremental testing does not replace the audit.

Each failure preserves manifest, logs, inputs and first divergence. Mandatory flaky cases block qualification; the campaign may continue for diagnosis without choosing the best attempt. Quarantine does not remove a requirement from the denominator. Soak, load, boundaries and pairwise have recorded finite domains; high-risk interactions require directed sequences. Shrinking preserves seed, provenance, preconditions, ingress and failure class.

Mutants need classification as valid/discriminating, equivalent under the declared projection, out of domain, invalid/not applied or unresolved. Only executed valid/discriminating mutants measure sensitivity; equivalent/invalid mutants have separate rationales. An isolated score does not qualify collection. Controls must also cause real, independent effects in the world, clients or saves. No mutation score was measured.

The broad gate closes modules, registries and non-registerable behaviors, child requirements, transitions, topologies/ingresses, platforms and observers. An invisible field, unsupported case, partial/stale capture or unknown count remains a gap. There is no universal equivalence guarantee beyond the declared finite domain.

## Proposed next increment

Implement the first RF-SCOPE/RF-MODEL slices and contract tests in Rust, resolve reference artifacts and build real supervision. Then obtain the first Java boot and RF-CLIENT path with input and slots, without waiting for the rest of the mod. The bridge is dev-only Java; engine, supervision, comparison and reports are Rust. The design adds no operational Python dependency.

Issue/PR statuses cited in the previous revision are snapshots of that 2026-11-05 check, not new integration verification. Review owners/existing work before implementing; this package opens no issues, claims or PRs. The [sources](SOURCES.md) record this boundary.

## Diagnostic bootstrap and qualifying attempt

The artifact-lock must resolve hashes for jars, loader/dependencies, JDK and tooling before any game process. A scratch bootstrap run may generate configs/catalog while the effective-lock is still draft, but publishes only a diagnostic record: no sealed expected results, pass or qualification credit. Review seals the effective-reference-lock and checkpoint; a new qualifying attempt starts from those inputs and confirms the effective state after load. RF-SCOPE-05 now depends on RF-INSTANCE-06 and RF-BRIDGE-03, without preventing the first diagnostic boot. A null artifact hash still prevents spawn. G0 describes complete closure, not a barrier prohibiting its own bootstrap.

