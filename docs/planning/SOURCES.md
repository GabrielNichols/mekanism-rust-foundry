# Sources and evidence limits

Research consolidated on 2026-11-05. This package contains specification and planning, without running games, jars or parity tests. No Mekanism Java was read/decompiled; metadata, data resources, public documentation and Horizon's own tools were used. Feature names confirm a nominal surface; semantics, rates, defaults, internal types and active registrations require observation in the pinned runtime.

## Mekanism reference

| ID | Source | What it supports | Limit |
|---|---|---|---|
| `mek-tag` | [Official tag](https://api.github.com/repos/mekanism/Mekanism/git/ref/tags/v1.21.1-10.7.19.85) | Tag and commit `a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80` | A commit is not the executed jar's hash |
| `mek-release` | [Release 10.7.19.85](https://github.com/mekanism/Mekanism/releases/tag/v1.21.1-10.7.19.85) | Existence of the release reference | Does not prove local execution |
| `mek-metadata` | [Pinned gradle.properties](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/gradle.properties) | MC 1.21.1, Java 21, build NeoForge 21.1.200, minimum 21.1.194 and integration metadata | Build loader alone does not determine the selected runtime; a build dependency does not prove an active contract |
| `mek-modules` | [Pinned README](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/README.md) | Core and Additions/Generators/Tools modules | README examples may use another version |
| `mek-core-data` | [Pinned Core translations](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json) | Core names and surfaces used in the inventory | Does not prove active IDs, behavior, observability or completeness |
| `mek-generators-data` | [Pinned Generators translations](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanismgenerators/assets/mekanismgenerators/lang/en_us.json) | Generators names and surfaces | Same nominal-evidence limit |
| `mek-tools-data` | [Pinned Tools translations](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanismtools/assets/mekanismtools/lang/en_us.json) | Tools names and surfaces | Same nominal-evidence limit |
| `mek-additions-data` | [Pinned Additions translations](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanismadditions/assets/mekanismadditions/lang/en_us.json) | Additions names and surfaces | Same nominal-evidence limit |
| `mek-license` | [Pinned LICENSE](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/LICENSE) | Repository MIT license at the reference | Artifacts/dependencies and differently licensed files need their own classification |
| `mek-wiki` | [General wiki](https://wiki.aidancbrady.com/wiki/Mekanism) | Usage context and outdated-information warning | Does not supply expected values for 10.7.19.85 |
| `mek-network-doc` | [Logistical Transporter](https://wiki.aidancbrady.com/wiki/Logistical_Transporter) | Interaction families to characterize | Revalidate all semantics in the pinned jar |
| `mek-fission-doc` | [Fission Reactor](https://wiki.aidancbrady.com/wiki/Fission_Reactor) | Thermal and failure scenario families | Do not import limits/defaults without measuring |

The package's observations, scenarios, priorities, domain dependencies, phases and acceptance criteria are proposed engineering decisions. They are not results declared by Mekanism's author. The revised catalog of 117 families and 253 proposed requirements must be reconciled with runtime discovery and expanded when unclassified objects or behaviors appear.

## Horizon infrastructure and coordination

| Source | Observed fact or relevant rule |
|---|---|
| [neoforge_oracle.py](https://github.com/horizon-rust/horizon/blob/main/tools/neoforge_oracle.py) | Static export; default 21.1.259; Java exporter release 21; current flow uses unix_args.txt |
| [Horizon FactsExporter](https://github.com/horizon-rust/horizon/blob/main/mods/horizon/src/neoforge-oracle/java/dev/horizonlab/neoforge/FactsExporter.java) | Exports at ServerStartedEvent and calls halt; does not constitute generic per-tick scenario control |
| [mod_port_harness.py](https://github.com/horizon-rust/horizon/blob/main/tools/mod_port_harness.py) | Ten static comparison families; shapes on a 32 grid and attributes to four decimal places |
| [oracle.rs](https://github.com/horizon-rust/horizon/blob/main/crates/horizon-port-pack/src/oracle.rs) | Rust types for facts/carry-over, without a dynamic runner |
| [Modpack plan](https://github.com/horizon-rust/horizon/blob/main/docs/modpacks.md) | Tiers, implemented data harness and machines/deep as expanding scope |
| [Integration workflow](https://github.com/horizon-rust/horizon/blob/main/docs/parity-workflow.md#player-facing-integration-and-deterministic-fixtures) | Distinction between deterministic fixtures and real connected paths |
| [AGENTS](https://github.com/horizon-rust/horizon/blob/main/AGENTS.md) | Fresh scratch homes, no touching real data, independent outcome tests |
| [CONTRIBUTING](https://github.com/horizon-rust/horizon/blob/main/CONTRIBUTING.md) | Permissive licenses and small deliveries |
| [Tracker #771](https://github.com/arismoko/horizon/issues/771) | Current modding API direction, open when checked |
| [Ledger #806](https://github.com/arismoko/horizon/issues/806) | Create/Mekanism coverage by capability, open when checked |
| [Tick access #795](https://github.com/arismoko/horizon/issues/795) | Access to objects during tick/callback, open when checked |
| [Stale evidence #9444](https://github.com/horizon-rust/horizon/issues/9444) | Evidence reuse problem after mutation/failure |
| [PR #9453](https://github.com/horizon-rust/horizon/pull/9453) | Freshness patch checked while open, with no merge confirmed in that check |

Horizon `main` links and issues are mutable sources, checked on 2026-11-05. The local snapshot contains no usable Git revision. When implementing, record full commits and hashes of the infrastructure actually used; do not treat these links as an execution lock. Public research found no ready universal Rust runner in the repositories checked, which does not prove absence from other projects.

`CONTRIBUTING` permits reading/adapting MIT, while the modpack pipeline maintains guidance to port by observed behavior. This specification follows the behavioral path, preserving metadata and original tools and keeping jars, facts, saves and modded builds in a private local store outside the public tree. Publishing a port is separate from this planning.

## Technical reference contracts

| Primary source | Use in the specification |
|---|---|
| [NeoForge 1.21.1 GameTests](https://docs.neoforged.net/docs/1.21.1/misc/gametest/) | Fixture in a real world as a bridge option, without replacing comparison |
| [NeoForge 1.21.1 NBT](https://docs.neoforged.net/docs/1.21.1/datastorage/nbt/) | Types/tags/lists/arrays and preserved representation |
| [JLS 21 types](https://docs.oracle.com/javase/specs/jls/se21/html/jls-4.html) | Java numeric semantics |
| [Rust overflow](https://doc.rust-lang.org/reference/expressions/operator-expr.html#overflow) | Need for explicit behavior independent of build profile |
| [Rust Command](https://doc.rust-lang.org/std/process/struct.Command.html) | Structured spawn/args/env |
| [Windows Job Objects](https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects) | Process-group supervision as a proposal to qualify |

These APIs support technical choices, not proof that the exact combination already works. The clock mechanism, non-mutating getters, bridge-free observation, transport and operational limits remain spike decisions requiring real evidence. This package produced no performance measurements, jar hashes, effective settings or parity results.

## Functional revision 0.2

The revision preserves 106 IDs and adds 11 families from the nominal audit. The [inventory](INVENTORY.md) table and its source keys do not assert runtime registration/behavior. The [roadmap](ROADMAP.md) and its 138 items are engineering decisions, as are 22 scenarios and 9 topologies; their counts are not results from the mod's authors or evidence of execution.

| Primary source | Use and limit |
|---|---|
| [NeoForge 1.21.1 sides](https://docs.neoforged.net/docs/1.21.1/concepts/sides/) | Physical client/logical sides; dedicated alone does not qualify GUI |
| [NeoForge 1.21.1 networking](https://docs.neoforged.net/docs/1.21.1/networking/) | Distinguish a sent action from updated state; the proposed bridge still needs implementation |
| [GameTests 1.21.1](https://docs.neoforged.net/docs/1.21.1/misc/gametest/) | Scenes/ticks as a component; do not certify real client input |
| [Rust test organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html) | Unit/integration layers; does not prove TDD executed in this package |
| [JDK 21 ProcessHandle](https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/lang/ProcessHandle.html) | Process identity/lifecycle; supervision still needs its own proof |
| [Horizon agent_control_e2e](https://github.com/horizon-rust/horizon/blob/main/tools/agent_control_e2e.py) | Example of actions and effect verification; not an existing Java/Mekanism adapter |
| [Horizon lan_e2e](https://github.com/horizon-rust/horizon/blob/main/tools/lan_e2e.py) | Example of connected games and separate states; does not prove the new scenarios |

Issues/PRs from the previous revision were not checked again in this consolidation. Their status remains the dated snapshot indicated above. Moving to implementation requires checking ownership and current state, without inferring publication or execution authorization from this planning.



