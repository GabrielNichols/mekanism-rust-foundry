# Automated instance laboratory

Revision 0.2. The proposed controller is Rust; an original dev-only Java bridge observes and controls NeoForge. The profiles below are design contracts, not executed instances. The laboratory enters the roadmap's first vertical path, before expansion to machines/networks. [JSON profiles](instance-profiles.template.json), [scenarios](SCENARIOS.md), [TDD](TDD.md) and [backlog](roadmap.json).

## Topologies and proof limits

| ID | Proposed processes | Proof limit |
| --- | --- | --- |
| LAB-UNIT | rust_test | No game; code/CLI contracts and probe processes only. |
| LAB-JS | java_dedicated_reference | Authoritative simulation without certifying client input/GUI/rendering/audio. |
| LAB-JI | java_client_with_integrated_server | Real singleplayer; does not demonstrate physical dedicated/client separation. |
| LAB-JD1 | java_dedicated_reference; java_client_p1 | One real player's flow; does not demonstrate contention between multiple actors. |
| LAB-JD2 | java_dedicated_reference; java_client_p1; java_client_p2 | Two real clients; simultaneous owner/trusted/stranger requires three real actors or an explicitly sequential variant. |
| LAB-JAA | independent_reference_topology_A; independent_reference_topology_B | Wrapper over a base topology; repeatability does not eliminate shared observation bugs. |
| LAB-JOBS | bridge_absent_when_feasible; bridge_loaded_observation_off; bridge_observation_on | Wrapper for instrumentation controls; equivalent actions and schedules need independent proof. |
| LAB-JREC | reference_server_with_declared_restart; reference_real_client_set | Wrapper for restart/checkpoint/crash; durability is observed, not assumed. |
| LAB-JR | java_reference_topology; horizon_candidate_topology | Available only after the pre-port gate; does not require protocol equality between versions. |

LAB-JAA, LAB-JOBS and LAB-JREC are wrappers: the base topology and actor count must be materialized. LAB-JD2 defines the initial two-client case; simultaneous owner/trusted/stranger tests require a variant with three real clients and its own resource contract. No actor is replaced by a bot without changing the claim. LAB-JR enters only after the pre-port gate.

NeoForge separates physical/logical sides, and the integrated server lives in the client; the dedicated server does not contain client-only surfaces. GameTests serve as world/component fixtures with tick scheduling, but do not certify mouse/keyboard input, widgets or rendering. ([NeoForge 1.21.1 sides](https://docs.neoforged.net/docs/1.21.1/concepts/sides/), [GameTests](https://docs.neoforged.net/docs/1.21.1/misc/gametest/)).

## Action ingresses have different proofs

| Ingress | Required path and confirmation | Limit |
|---|---|---|
| os_input | Window/PID, focus, viewport, capture and event received/consumed | A focused window does not prove an action; desktop jobs are serialized |
| client_event_queue | Real window/input/Screen queue, same keyboard/mouse dispatch | The spike must prove the path and focus/modal rules; enqueue is not application |
| client_action_pipeline | Normal player manager and networking | Does not certify keybinding, hit-testing or widget dispatch |
| protocol_bot | Server packets with a bot | No graphical client or real UI |
| gametest_fake_player | Action in an in-world fixture | Not a complete connected player |
| server_fixture | Administrative setup or component rule | Does not prove player placement/use/menu |

Each step records requested and effective ingress, origin, target, actor and effect. There is no silent fallback to setBlock, inventory editing or machine calls. Semantic selectors may locate a widget/slot; the click still traverses the required ingress. The event queue is a proposal to qualify, not an existing API. A screenshot/frame strengthens visual proof and does not replace server state.

The minimum flow is `requested → dispatched → consumed → server_applied_or_rejected → authoritative_delta → client_applied_each → current_frame_when_required`. Correlation by action and barrier can replace packet ACK where the protocol provides no dedicated ACK, but the limitation must be recorded. A legitimate rejection also has before/after states and a witness; ACK without effect does not satisfy a positive action.

## Job with explicit readiness, clock and closure

The lifecycle is `validated → admitted → materialized → spawning_server → server_hello → modules_verified → world_ready → spawning_clients → client_hello → login_confirmed → chunks_applied → screen_ready → fixture_verified → paused_at_barrier → running → evidence_finalizing → graceful_shutdown → processes_reaped → result_committed`.

A live process, open port or Done log is insufficient. world_ready checks dimension, seed, world epoch, effective modules/config and phase. Client readiness includes identity, session, applied chunks/teleport, screen and current frame. Each stage has a wall-clock deadline; actions/ticks also have a logical budget. Failure or cancellation at any phase preserves partial evidence and ends in non-pass. No fixed sleep is used as proof of readiness.

The clock must advance the full server, with entities/callbacks and demonstrated phases. Login/transport/GUI must keep progressing while simulation is paused; the spike must demonstrate how this happens without hidden ticks. Do not assume a tick command provides this contract. Tick, frame and wall clock have separate counters.

## Isolated materialization and faithful replication

Artifacts addressed by hash are shared read-only only. Each attempt has its own server gameDir, client gameDirs, configs/options, saves/playerdata, logs, native temp, control, ports and evidence. Post-load configuration, datapacks/resourcepacks, locale, GUI scale and graphical environment enter the fingerprint. Do not resolve through a personal launcher, .minecraft or inherited homes; do not use real credentials/servers for fixtures.

Create a new world or copy an immutable closed checkpoint, with proven hash and isolation. Do not clone a live world or use writable hardlinks to a shared save. Resetting blocks alone does not clear frequencies, radiation, tickets, players or RNG. An A/A repetition begins from its own initial checkpoint with fresh JVM/processes. Restarting the same world is allowed only within a lifecycle scenario, after save/flush/close or an explicitly recorded crash point.

Ports/control bind to loopback, with ownership by run and handles/start time; handle allocation races without relying only on check-then-use. Rust uses structured argv/env and supervision of its own group; Windows Job Objects are a candidate requiring proof. A PID can be reused. Shut down gracefully, verify durability/close, impose a deadline and terminate only children belonging to the run. Cleanup requires canonicalization/ownership and does not follow links outside. Preserve failure reproductions. ([Rust Command](https://doc.rust-lang.org/std/process/struct.Command.html), [Windows Job Objects](https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects), [ProcessHandle](https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/lang/ProcessHandle.html)).

Admission reserves RAM/CPU/GPU/display per job. Do not infer parallelism from CPU count alone. OS input requires exclusive access to the desktop session; two clients need distinct routes/identities. Warm pools help exploration but do not count as fresh qualification attempts. Budgets and soak duration will be calibrated; no values were measured now.

An offscreen client or virtual display must still execute the real client and its renderer. A headless server does not certify screens; java.awt.headless=true does not itself provide a display/OpenGL. Validate native Windows and graphical/virtual Linux separately. Unicode/space-containing paths, scaling, resizing and focus enter platform tests.

For future Horizon games provide fresh HORIZON_HOME, HORIZON_LAUNCHER_HOME, XDG_DATA_HOME and XDG_CONFIG_HOME, HORIZON_LAUNCHER_NO_SELF_UPDATE=1 and HORIZON_NO_AUDIO=1; for the launcher, HORIZON_LAUNCHER_TRASH, HORIZON_LAUNCHER_CLIPBOARD and HORIZON_LAUNCHER_DESKTOP under scratch. Audio/Walkie-Talkie require a separate profile for events versus device delivery; no-audio does not qualify audio. Do not remove flags by inference. ([Project AGENTS](https://github.com/horizon-rust/horizon/blob/main/AGENTS.md)).

## Evidence and controls that close a scenario

Per step: action ID, run/instance/world epoch, actor, requested/effective ingress, initial state, dispatch/consume where required, server application/rejection, tick/phase, authoritative delta, each client's update and current frame if visual. Preserve raw trace, logs, initial checkpoint, settings and global states, final save where relevant, expanded scenario, hashes and first divergence.

Control for input directed at the wrong client, incorrect focus/modal, changed keybind, GUI scale without a new target, player not logged in, unapplied chunk, stale frame, lost ACK, duplicated action, disconnect between dispatch/commit, dead JVM, save without durable ACK, another process's port and residue from an earlier run. Ingress failure is infra/inconclusive; it is not a Mekanism semantic mismatch. In-world controls must produce a known effect and be compared against independent observation, not just JSON editing.

A scenario closes only with every step/observable/capability and an intact manifest. Compare the whole temporal path, not just eventually item exists. Fresh Java repetitions number at least three; no selection of the best attempt. Flaky keeps the requirement blocked. Shrinking preserves preconditions, seed, ingress and failure class; replacing a click with inventory injection is not equivalent reduction.

The [22-scenario catalog](SCENARIOS.md) is a vertical seed. The 253 requirements need their own expansion after discovery, including recipes, tiers, modes, permissions, lifecycle and platforms. Rates, duration, quantities, safe getters and driver/control capabilities remain pending.

## Two-stage locks and qualification

Before any game, resolve and seal an artifact-lock with hashes of every selected artifact, JDK/loader/dependencies and tooling. Diagnostic bootstrap in scratch may start with a draft effective-lock to generate post-load settings and catalog; it produces neither expected results nor a qualification pass. After review, seal the effective-reference-lock, checkpoint and bindings; the qualifying attempt is new and confirms that state after load. Changes or omissions block approval. Do not allow a null jar hash or promote a bootstrap draft. The [reference profile](profiles.reference.template.json) defines both contracts, and RF-SCOPE-05 depends on real boot/bridge.


