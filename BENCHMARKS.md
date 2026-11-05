# Demonstrating optimization while preserving behavior

The intended result is an independent Mekanism fork for Horizon that preserves an explicit behavior scope while reducing its execution cost. Current measurements do not establish that improvement. They exercise automation of the original Java runtime and collect an initial functional baseline.

## Required comparisons

| Study | Reference | Candidate | Question |
| --- | --- | --- | --- |
| Game without mods | Pinned Minecraft Java | Pinned Horizon | What does the same work and result cost in each game runtime? |
| Game with Mekanism | Pinned NeoForge + Java Mekanism | Pinned Horizon + future Rust port | Does the complete stack preserve behavior at lower cost? |
| Incremental mod cost | Paired Java runs without/with Mekanism | Paired Horizon runs without/with the port | How much cost does the mod add in each engine? |
| Optional optimized Java stack | Java + an explicitly pinned optimization set | The same Horizon candidate | Does the improvement remain against an optimized Java reference? |

The last reference is a separate study; do not silently replace the baseline. Minecraft 1.21.1 and Horizon 26.3 have vanilla differences that need a compatibility ledger before attributing a difference to the mod or comparing work. An absent feature cannot count as reduced resource use.

## Reproducible functional workloads

1. Single machine: the same recipe, processed quantity, upgrades, energy supplied/consumed and result.
2. Machine group: fixed inputs and consumers, blocking and resumption, with unchanged production.
3. Networks: the same nodes, topology, resources transferred, connection changes and active chunks.
4. Multiblocks: equivalent size and environmental conditions, production and observed hazards.
5. Client: equivalent camera, resolution, UI scale, distance, visible entities and interface state.
6. Lifecycle: boot, fresh world, saved world and save/load as separate studies.

These studies depend on a characterized Java reference and a functional future candidate. A barrel fixture or an unpowered machine is not a Mekanism processing workload. The current processing pilot advances the single-machine baseline; it does not represent networks, multiblocks or the entire game.

## Measurement and controls

- Pin artifacts, toolchains, flags, build profiles, hardware, drivers, packs, world, scheduler and power/thermal conditions. Compare suitable production builds; debug tools need their own study.
- Fix completed work and its semantic results. Fewer ticks, chunks or machines, shorter distance, discarded items or omitted rendering cannot count as improvement.
- Separate unlimited throughput from controlled tick cost. A 20 TPS limit alone cannot distinguish two engines that both meet it.
- Measure total batch time, tick/frame p50/p95/p99 distributions, RAM/working set for server and clients, GC pauses, CPU, boot and save time. Java heap is not total process memory.
- Distinguish fresh processes, fresh worlds, application caches and the operating-system cache. Do not call disk cache cold without controlling it.
- Measure and state instrumentation cost, including observation controls. Do not omit pauses or slow attempts to favor a candidate.
- Run repeated pairs with recorded alternating/randomized order and defined warmup. Five pairs are an initial operational floor, not statistical proof; review variance, uncertainty and sampling before drawing conclusions.
- Preserve every attempt, including failures. Functional equivalence is a prerequisite for each workload; divergent results block its speed comparison.

`benchmark-summary` checks completed quantities, declared digests, warmup and duration, and uses the lower median for an even number of pairs. Its ratio is descriptive. It does not collect measurements, establish digest provenance or declare optimization; it always returns `optimization_proven=false`.

## Performance implementation milestones

The runner collects `measurements.json` from actual instances and produces `performance.json`: percentiles for the bridge Pre/Post interval and snapshot cost, batch time, JVM CPU, GC and sampled heap. [VALIDATOR-CONTRACT.md](VALIDATOR-CONTRACT.md) defines the units and interval boundaries. The functional processing campaign records these metrics per run and alternates capture-mode launch order. This partially advances PERF-02/03. Warmup, native memory and equivalent-work Java/Rust comparison remain open.

### Lifecycle measurements

`run-lifecycle` records fresh-process boot, controller wall time for verified tick batches, Windows JVM process CPU, acknowledged flush, exit, and fixture-copy time. Durations are integer nanoseconds. Startup includes server readiness and the frozen-clock probe. Controller wall time includes command transport and polling. Enabled runs also include receipt archival and validation. Flush timing ends at the console barrier. Exit timing includes shutdown log verification. The every-tick bridge additionally records its own timing, heap, and GC counters. Keep those measurement intervals separate.

The comparison fixes the seed, machine, recipe, inputs and outputs, and the scheduled 200 processing ticks plus 128 dry ticks. The pre-flush crash treatment records 137 discarded ticks and the replay separately from the completed recipe. Recovery uses explicit save and JVM exit boundaries. It measures process termination and reopen on the same host and filesystem, with no claim about power-loss durability. Application startup uses fresh JVMs while operating-system caches remain uncontrolled. The default tick rate and console overhead limit the interpretation of these Java reference measurements.

The three continuous runs in each mode completed the same 328 commanded ticks and matched the same checkpoints and terminal disk witness. The table reports medians of the sums over verified batches per run. The campaign file includes the individual runs and separate boot, flush, exit, and replay timings. [Lifecycle measurements](evidence/lifecycle-campaign.json)

| Java observation mode | Controller wall time | JVM process CPU |
| --- | ---: | ---: |
| Bridge absent | 17.40 s | 0.92 s |
| Bridge enabled, every tick | 18.40 s | 1.59 s |

These describe the complete observation modes, which use different control paths. The performance comparison with an equivalent Rust candidate remains open.

| Milestone | Evidence required before use |
| --- | --- |
| PERF-01 Workload manifest and measurement protocol | An independent test rejects different work/results; the summary implements part of this contract |
| PERF-02 Measurement clocks and equivalent phases | Controls detect wrong timers, units and work outside the measured interval |
| PERF-03 Process and frame collection | Actual server/client measurements, including descendants and native memory |
| PERF-04 Warmup and pair ordering | Reproducible seeds/order, without retry-until-green or best-run selection |
| PERF-05 Representative corpus | The same outputs and transitions as characterized Java; declared domain workloads and limits |
| PERF-06 Horizon/port candidate | Only after the applicable functional gate; fresh Java/Rust evidence with equivalent results |
| PERF-07 Auditable report | Raw metrics, dispersion/uncertainty, instrumentation costs and a conclusion limited to the measured scope |

The validator and fork remain independent of Horizon's source tree. The execution adapter will require a compatible Horizon runtime/API version and form the explicit integration boundary. Repository independence does not remove that runtime dependency.


### Tick rate and worker scheduling

The observer controls record the requested tick rate, bounded worker count, per-job start and finish offsets, per-batch controller wall time, Windows JVM CPU, and boot, flush and exit intervals. Use the 20 ticks/s serial profile as the semantic baseline. Accelerated and parallel results must match the same live checkpoints and terminal disk witness before comparing elapsed test time. Every continuous treatment commands 328 world ticks and checks one recipe with the same inputs and outputs.

Compare rates in serial first. Compare worker counts at the same rate, using the span from the first job starting to the last job finishing within a three-mode cohort. That span includes fixture copy, JVM startup, tick execution, flush and exit. Seed setup and artifact verification are outside the cohort span. Overlapping worker windows measure scheduler behavior on this host. JVM CPU and batch wall time remain separate from campaign elapsed time. Operating-system cache state and contention vary, so these runs are descriptive test-harness measurements.

The simulated observer report measures actual Rust validation over 200 checks per case. Its synthetic bridge timing values are fixture inputs and are excluded from the validation latency. [Simulated observer measurements](evidence/simulated-observer-faults.json)


| Live profile | Cohorts | Tick-batch wall time per trial | Three-mode cohort span |
| --- | ---: | ---: | ---: |
| 20 ticks/s, one worker | 1 | 17.39–18.39 s | 110.93 s |
| 100 ticks/s, one worker | 1 | 3.64–4.68 s | 117.76 s |
| 100 ticks/s, three workers | 3 | 3.52–10.15 s | 39.86–116.27 s |

All 15 treatments matched the serial baseline's live checkpoints and terminal disk NBT. At 100 ticks/s, the serial tick batches were about four to five times shorter. Startup and contention limited the end-to-end gains. The parallel cohort median was 102.74 s at the same target rate, with substantial variation across the three cohorts. The simulation's per-case p95 validation latency was 6.6–24.3 microseconds in this debug runner. [Live observer results](evidence/observer-controls-campaign.json)


### Live observer fault audit

Each treatment commanded 137 ticks, comprising 73 powered ticks and 64 dry ticks. The three-worker campaign at a target of 100 ticks/s took 156.54 s including the seed and excluding artifact verification. Tick-batch wall time per treatment was 1.69–1.72 s. Median bridge round-trip time was 8.74 ms for the clean read, 7.67 ms for the stale replay, and 15.32 ms for the read with the inventory mutation. Each median represents three trials. The report retains console witness, JVM startup, batch CPU, flush, exit and classification timings separately. [Live audit timings](evidence/live-observer-audit.json)


### Client phase and chunk controls

The three hidden client runs took 79.55–118.48 ms from the armed capture request to the render receipt. That interval includes transport, scheduling and writing the PNG. The render occurred 3–7 frames after arming. The timings record real frame capture and preserve each client's own clock.

The six dedicated chunk trials ran in 82.79 s with two workers at a target of 100 ticks/s. The lookup itself took 14.3–23.2 microseconds for the resident-only path and 59.41–84.79 ms for the deliberate loading control. The latter generated a full chunk. These are different workloads used to verify the side-effect detector. The report separates lookup, controller round trip, follow-up tick batch, startup, flush and exit. [Phase and chunk measurements](evidence/phase-chunk-controls.json)
