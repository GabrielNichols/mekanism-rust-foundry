# Mekanism Rust Foundry

Foundry owns the Rust Parity Lab and its pinned Java reference. It is an independent Cargo workspace. Horizon is the future native runtime target. Gameplay work starts only after the relevant scope passes the [pre-port qualification gate](docs/planning/ROADMAP.md).

Read [CONTRIBUTING.md](CONTRIBUTING.md), [STATUS.md](STATUS.md), and the relevant [roadmap item](docs/planning/GITHUB-ROADMAP.md) before changing behavior. Read [SECURITY.md](SECURITY.md) and [security operations](docs/SECURITY-OPERATIONS.md) before touching credentials, CI, distribution, or data boundaries.

## Repository map

| Path | Responsibility |
| --- | --- |
| `src/`, `tests/` | Rust runner, reference capture and comparison, simulation controls, measurement, and contract tests |
| `adapters/neoforge/` | Pinned Java preparation, instrumentation and instance adapters |
| `cases/` | Executable scenario contracts and their preconditions |
| `evidence/` | Reviewed English summaries, evidence index, and public hashes |
| `docs/planning/` | Canonical inventory, requirements, roadmap and work packages |
| `tools/`, `.github/` | Local proofs, release and metadata tooling, and the protected contributor CI |
| `docs/releases/` | Release inputs, installation and archive verification |

## Issue to pull request

1. Start with one tracked, reviewable scope. Confirm existing work and use the [Project](https://github.com/users/GabrielNichols/projects/2), [issue index](docs/planning/GITHUB-ROADMAP.md), and [taxonomy](docs/planning/GITHUB-TAXONOMY.md). Avoid claiming a Horizon issue for independent Foundry work.
2. Specify a pinned profile, expected behavior, witness, negative control and coverage boundary. Add a behavior test and record the intended assertion failing before implementing the change. Keep setup failures and compilation errors separate from the semantic RED.
3. Implement the smallest change that preserves existing callers and trace semantics. Record the GREEN and relevant regressions. Never weaken expectations, rewrite a golden to fit a candidate, bypass an artifact lock, or substitute a lower evidence level.
4. Submit a small PR using the [template](.github/PULL_REQUEST_TEMPLATE.md). Link the issue, commands, test counts, sanitized evidence and limitations. Use `Refs #N` for partial work and `Closes #N` only when reviewed evidence satisfies the linked acceptance.
5. Route CI, publishing, security, dependency, runner, bridge, scenario and evidence-schema changes to the maintainer listed in [CODEOWNERS](.github/CODEOWNERS). The same PR gate applies to maintainers. Report check skips, cancellation, stale heads and setup failures explicitly.

## Local checks

Use Windows x64, PowerShell 7 and Rust 1.95.0 for the exercised baseline. Use a fresh evidence output for each attempt. Socket tests need local loopback access.

| Change | Smallest useful local proof |
| --- | --- |
| Markdown or planning | Review rendered links and scope. Run `git diff --check` and `./tools/check-public-tree.ps1` after staging. |
| Issue map, labels, or contributor tooling | Run `./tools/test-issue-preview.ps1`, `./tools/test-roadmap-sync.ps1`, and `./tools/test-github-labels.ps1` and `./tools/test-roadmap-milestones.ps1` as affected. Use the read-only preview before a live metadata operation. |
| Rust contract or comparison | Start with `cargo test --locked --test contracts` or the matching integration target under `tests/`. Run `cargo fmt --check`, `cargo clippy --locked --all-targets -- -D warnings`, and `cargo test --locked` before review. |
| Observer or evidence handling | Run the affected Rust test target and `cargo run --locked --bin mek-lab -- simulate-observer .lab/evidence/my-new-attempt`. Use the real reference when a change claims Java observation or world effects. |
| Bridge, scenarios, or runtime pins | Build and prepare the private runtime, run the dedicated smoke case, then the relevant targeted characterization and its negative control. See the commands below. |
| Performance measurement | Keep equivalent work, warmup, repeated trials and native memory visible. Follow [BENCHMARKS.md](BENCHMARKS.md) and report tool overhead separately. |
| Packaging or distribution | Use a reviewed clean commit and the [release packaging commands](docs/releases/README.md). Validate the extracted tool and checksums. Publication requires explicit authorization. |

```powershell
cargo build --locked
./adapters/neoforge/prepare-reference.ps1 -Jdk '<JDK21-directory>'
./target/debug/mek-lab.exe run-reference .lab/environment.json cases/server-inventory.json .lab/evidence/my-new-attempt
./target/debug/mek-lab.exe verify-reference .lab/evidence/my-new-attempt
```

## Evidence and instance boundaries

- Use the pinned Java jars, loader, reference profile and artifact hashes. A proposal, registration snapshot, simulated fault, real run, sealed trace, reviewed reference and qualified scope are separate evidence levels. Report the lowest demonstrated level and exclusions.
- Give every process its own game directory, config, save, logs, ports and temp directory under ignored `.lab/`. Use dedicated Java headless. Only cases that need a client use a real client created hidden from startup. Stop on a visible window or a missing isolation witness. Keep player data, normal installations, credentials, logs, captures, game files and prepared resources outside the tracked tree.
- Preserve original evidence and failed attempts privately. Public evidence is sanitized and reviewed. Change a reference only through a newly witnessed run and explicit review. Never change a pinned executable or JAR while a Windows campaign holds it open.
- Treat server setup, client keybinding and Screen handling, and OS input as separate ingress levels. Check independent effects and the relevant causal phase. Report timings and semantic results separately.

## CI and publication

[Contributor CI](.github/workflows/pr-gate.yml) selects relevant checks for pull requests and main pushes. Selected Rust checks use standard ephemeral GitHub-hosted Windows runners. Manual dispatch can run all supported Rust platforms. It has read-only permissions, pinned Actions, no checkout credentials, and no published artifacts. Older runs for the same PR or ref are cancelled when superseded. Inspect the exact head, event and executed steps before citing a pass.

A contributor can run the same commands locally. To use their own fork's Actions, open its Actions tab, enable workflows if GitHub asks, then push to the fork's `main` or run **Rust contracts** manually from the workflow page. The workflow must be present on the fork's default branch for manual dispatch. That result belongs to the fork and does not satisfy Foundry's protected branch check. Keep the fork public when relying on standard public-runner compute pricing.

Run heavy Java campaigns locally or on isolated dedicated infrastructure for reviewed commits. Keep untrusted fork PRs off self-hosted and personal machines. Avoid privileged PR triggers, repository secrets, paid runners and uploaded game evidence. Run authenticated metadata or release commands as separate maintainer operations after review. [Operations](docs/OPERATIONS.md)

Preserve human and legal credits. Omit automatic tool or agent attribution, signatures, footers, and coauthor trailers. Upstream Horizon claims, public posts, GitHub changes, and publication require scope-specific authorization.
