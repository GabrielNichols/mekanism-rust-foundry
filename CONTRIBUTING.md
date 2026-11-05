# Contributing

This project builds a Rust compatibility validator before implementing a Mekanism-derived native mod in a separate repository for Horizon. The two required outcomes are equivalent behavior and measured lower execution cost. A language choice, a passing smoke test or reduced functionality cannot establish optimization.

Read [AGENTS.md](AGENTS.md), [STATUS.md](STATUS.md), [VALIDATOR-CONTRACT.md](VALIDATOR-CONTRACT.md) and [BENCHMARKS.md](BENCHMARKS.md) before changing a contract. Keep authored documentation, descriptive text, source comments and review descriptions in English. Preserve raw evidence exactly, including localized runtime or operating-system messages.

## Repository and runtime boundaries

- This is its own Cargo workspace and local repository. Do not add Horizon dependencies or modify Horizon's source to implement the laboratory.
- The future native mod targets a compatible, pinned Horizon runtime through a versioned adapter. Repository independence does not remove that runtime dependency.
- The Java bridge is a development oracle adapter for the original runtime. It is not the player-facing Rust port.
- Do not start gameplay porting before the applicable reference/validator qualification gate. A functional pilot does not qualify all Mekanism families.
- Never commit game jars, prepared resources, native libraries, player data or generated worlds. Keep them under ignored `.lab/`.
- Preserve license/copyright notices and genuine human credits. Do not add automatic tool attribution, generated footers or agent coauthor trailers.

## Setup

The exercised profile is Windows x64 with Rust 1.95.0, PowerShell 7.6.5, JDK 21.0.13, Minecraft 1.21.1, NeoForge 21.1.200 and Mekanism 10.7.19.85. PowerShell 7 is required by the scripts. Other platforms and the declared MSRV are not yet qualified.

```powershell
cargo build --locked
$labJdk = 'C:\Tools\jdk-21'
./adapters/neoforge/prepare-reference.ps1 -Jdk $labJdk
```

Use an explicit JDK 21 directory; do not rely on the machine's default Java. Preparation verifies pinned downloads, builds the bridge and creates the private runtime manifest. Review a new lock when the runner, bridge or dependencies change. Never bypass a hash mismatch or treat a new lock as an expected-result update.

Boot defaults to a 300-second budget. A documented slow environment may use an explicit budget up to 900 seconds:

```powershell
./adapters/neoforge/prepare-reference.ps1 -Jdk $labJdk -BootTimeoutSeconds 900
```

## Choose a reviewable scope

Use the repository-local [planning bundle](docs/planning/README.md), which is canonical for contributors, to choose a finite [roadmap item](docs/planning/ROADMAP.md), behavior requirement or explicitly declared subset. Identify its runtime objects, initial state, actors, ingress level, phases, expected origin, negative control and required witnesses. Keep IDs and requirement links stable. Updating infrastructure does not automatically cover a parent family or dependent profile. Historical workspace reports are not required to use a standalone clone.

Useful entry points:

| Area | Files |
| --- | --- |
| Runner, scenarios, process coordination | `src/lab.rs`, `src/process.rs`, `src/main.rs` |
| Typed values and exact trace comparison | `src/lib.rs` |
| Tick-capture contracts | `src/capture.rs` |
| Independent NBT/region reading and inventory witnesses | `src/nbt.rs`, `src/witness.rs` |
| Capture-frequency comparison | `src/observer.rs` |
| Measurements and completed-work accounting | `src/measurement.rs`, `src/benchmark.rs`, `src/work.rs`, `src/bin/mek-work.rs` |
| Original-runtime bridge and setup | `adapters/neoforge/` |
| Functional case definitions and binding provenance | `cases/`, `cases/SOURCES.md` |
| Contract tests and TDD evidence | `tests/`, `evidence/tdd/` |

## Meaningful TDD

1. Specify observable behavior using independent values, pinned recipe data or a characterized original-runtime oracle.
2. Write the test first and record a failing assertion for the intended reason. A compile error, missing runtime, failed boot, unsupported capability or unapplied input is not a semantic port RED.
3. Make the smallest implementation that satisfies that behavior. Keep the expected result independent of the candidate implementation.
4. Run the affected regressions and required checks. Preserve the original RED/GREEN records and classify setup failures separately.

Test outcomes rather than documentation wording, source structure or calls to stubs. Use actual process/transport behavior where that is the contract. Add discriminating controls for missing fields, wrong units/types, reordered events, gaps, stale state and absent effects. Do not manufacture a green result by changing a golden, dropping a case, retrying a mutation or accepting a lower ingress level.

Java characterization discovers the reference behavior; it does not prove a future Rust port. A candidate mismatch counts as a port RED only after valid preconditions, proven application and a reviewed, sealed Java expected result.

## Required Rust checks

```powershell
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

Socket tests require local loopback access. Once dependencies are cached, `--offline` can make dependency resolution explicit.

The [contributor CI workflow](.github/workflows/ci.yml) is configured to run these Rust unit/contract checks on Windows x64 with Rust 1.95.0, rustfmt and Clippy. It installs the explicit toolchain through the hosted image's Rustup and pins [official checkout](https://github.com/actions/checkout/commit/3d3c42e5aac5ba805825da76410c181273ba90b1) to an immutable commit, with read-only repository permissions and no persisted Git credentials. It does not download Minecraft, launch game instances, use repository secrets or publish anything. A green CI run does not qualify in-game parity, future-port equivalence or optimization. Real-runtime campaigns and their applicable gates remain separate requirements.

While a Windows campaign is running, its binary and all pinned jars must remain unchanged. Use a separate target directory for development checks:

```powershell
$env:CARGO_INCREMENTAL = '0'
cargo clippy --locked --offline --target-dir .lab/test-target --all-targets -- -D warnings
cargo test --locked --offline --target-dir .lab/test-target
```

Do not rebuild the active runner, rebuild its bridge jar, regenerate its lock or edit its archived case. Wait for the attempt to finish, then prepare a new fingerprint and run new instances.

## Actual game validation

Use only new destinations under the laboratory's private evidence directory:

```powershell
./target/debug/mek-lab.exe run-reference .lab/environment.json cases/enrichment-lifecycle.json "$PWD/.lab/evidence/lifecycle-new"
./target/debug/mek-lab.exe run-reference .lab/environment.json cases/client-processing-menu.json "$PWD/.lab/evidence/client-new"
./adapters/neoforge/test-processing.ps1 -OutputRoot "$PWD/.lab/evidence/campaign-new"
```

Each game must have private gameDirs, saves, configs, logs, ports, temporary directories and data homes. Never use personal worlds, launcher state, credentials or account tokens. Client-native libraries are copied into the private instance. The runner owns and terminates the processes it creates.

The current client path uses keybinding and Screen handlers, not OS input. Verify effects on both server and client. A command acknowledgment, opened socket or screenshot alone cannot establish an applied action. Keep administrative setup explicit: forced chunk anchoring, initial items, FE supply and output filling have different proof scopes from gameplay input, generators, cables or player interaction.

If a run fails, retain its directory, inspect and classify the failure, fix the cause and start a new attempt. Do not rerun into the same output or select only successful attempts. Inspect the final campaign record before claiming that the cohort completed.

For observer or recovery changes, run `run-lifecycle` with a new private output directory. Preserve failed attempts and provide the result, completed treatments, exact divergence if present, and evidence paths. A graceful stop, an abrupt stop before flush, and an abrupt stop after flush have distinct expectations. See the [lifecycle command](README.md#bridge-free-and-recovery-controls) and [measurement boundaries](BENCHMARKS.md#lifecycle-measurements).

For an observer fault, start with `simulate-observer`. Add an accepted control and record the first rejected field, including whether the independent world agrees with the observer. Use `run-observer-controls` to exercise the three Java modes and report the requested tick rate, worker count, and comparison against the serial reference. [Observer commands](README.md#observer-controls-and-faster-test-runs)

Verify fresh reference evidence with `verify-reference` before comparing or reporting it. The verifier checks the trace seal and its binding to the attempt, case and locked execution. The audit command launches disposable game worlds and deliberately injects its named faults there. Keep the full attempted evidence private and report each outcome. [Validator contract](VALIDATOR-CONTRACT.md)

Use the pinned offscreen bridge for every real-client run. For a phase change, archive the pre-action server state, client ingress, independent server application, fresh client revision, and later render. For a chunk probe, record the selected coordinate, immediate and delayed load witnesses, and persisted target record. [Client and chunk commands](README.md#client-phases-and-chunk-observation)

## Evidence and review

- Preserve raw observations, typed traces, artifact/case/runner/config hashes, measurements, work ledgers and process logs.
- Keep screenshots and worlds private in `.lab/`; publish only appropriate source and metadata summaries.
- Keep supported, excluded, unknown, blocked and qualified scope explicit. A requirement without a case is not covered.
- Preserve the strict `keepPacked Byte(0)` storage projection and all raw data. Additional normalizations need a field-specific, versioned policy and independent tests.
- Distinguish functional pass, validator qualification, Java/Rust equivalence and optimization. Current broad qualification flags remain false.

A review description should state the concrete behavior change, its requirement/case scope, the actual RED/GREEN and functional evidence, measurement boundaries and remaining limitations. Do not claim an upstream Horizon issue for independent laboratory work. Opening remote issues, forks, pushes or publication is separate from local implementation authorization.

## Performance claims

Follow [BENCHMARKS.md](BENCHMARKS.md). Compare equivalent completed work and outputs under pinned conditions. Preserve all attempts, define warmup and ordering, measure distributions and uncertainty, and account for instrumentation, native memory and the full process set.

The current Pre/Post and snapshot timers have explicit boundaries. A 20 TPS cap, Java heap alone or a single run cannot prove lower total cost. Do not compare candidate speed while its behavior differs from the reference, and do not report a missing feature as an optimization.




## Isolate your workspace

Clone the lab into a directory separate from Horizon and your ordinary Minecraft installation. Use a new branch and a new private output for each attempt.

```powershell
git switch -c docs/w1-cat-first-row
./target/debug/mek-lab.exe run-reference .lab/environment.json cases/server-inventory.json .lab/evidence/w1-cat-first-run
./target/debug/mek-lab.exe verify-reference .lab/evidence/w1-cat-first-run
```

Build and prepare the runtime with the setup commands above before the first run. Contributors running in parallel use separate checkouts or worktrees with their own ignored `.lab` directories. Coordinate bridge, runner and schema changes with the integrator. [Ownership and integration](docs/planning/FIRST-WAVE-WORKSTREAMS.md)

## Task to pull request

1. Pick a stable ID from the [first-wave workstreams](docs/planning/FIRST-WAVE-WORKSTREAMS.md). Once the issue exists, check that it is available, ask to take it, and wait for assignment if another contributor is working on it. Before issues are published, use the ID in your local draft.
2. Reproduce the [dedicated smoke case](README.md#quickstart-on-windows-x64) in a fresh directory. For an inventory-only PR, keep the change to the row, source and gap. Runtime or runner changes need the relevant [Rust checks](#required-rust-checks), a precise negative control and independent evidence.
3. Record the exact artifact profile, command, result, seal and limitations. Link a sanitized evidence summary and keep raw runtime files local.
4. Open a small PR with the task ID and linked issue, using the [PR template](.github/PULL_REQUEST_TEMPLATE.md). Review checks that the result proves the declared scope and that any unknown bindings remain explicit.

### Preview an issue batch

The [versioned issue queue](docs/planning/issue-queue.json) contains the umbrella and six first-wave tasks. To prepare only the selected IDs, run the read-only preview with GitHub CLI or supply a JSON array of observed issues with number, title, body and state.

```powershell
./tools/preview-issues.ps1 -Repository 'OWNER/mekanism-rust-lab' -Ids W1-CAT,W1-PROC
./tools/preview-issues.ps1 -ObservedIssues 'private-issues.json' -Ids W1-CAT,W1-PROC
./tools/test-issue-preview.ps1
```

Review `prepare_create` entries before posting. Keep the stable HTML marker on any posted issue. A later preview uses that marker to preserve existing issues, including closed issues and maintainer edits. Resolve `conflict` entries manually. The preview reads the full issue list when using GitHub and never creates or updates issues.
