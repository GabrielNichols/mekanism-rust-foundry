# Revised Mekanism functional inventory

Revision 0.2, 2026-11-05. **117 families and 253 proposed behavioral requirements**, preserving the 106 IDs from the initial revision. These are characterization targets, not a registry census or executable tests. Zero observed behavior and zero runtime qualification. Expected values remain pending the pinned Java reference.

Each leaf requirement specifies an action, observables, boundary and proof limit. The [detailed JSON](requirements.json) also contains Arrange, actor, witnesses, TDD contract and axes to resolve. [JSON families](inventory.json), [vertical scenarios](SCENARIOS.md), [laboratory](FUNCTIONAL-LAB.md) and [roadmap](ROADMAP.md) form the same plan.

## Taxonomy changes

| Module | Families | Requirements |
| --- | --- | --- |
| Core | 94 | 204 |
| Generators | 11 | 25 |
| Tools | 6 | 12 |
| Additions | 6 | 12 |

The 11 additions below correct nominal surfaces without an explicit family. Names/keys from pinned resources support their identification; the proposed action and its effects still need discovery and execution. Do not assume that a translation key proves active registration or an AI rule.

| ID | New family | Reason for distinction |
| --- | --- | --- |
| MK-MAC-025 | Antiprotonic Nucleosynthesizer | Missing machine; SPS produces antimatter but does not replace nucleosynthesis |
| MK-WOR-007 | Cardboard Box | Transporting a block with data is more than item transport |
| MK-ENE-006 | Chargepad | Charging a player/Robit through a dedicated block was not explicit in ENE-003 |
| MK-UI-005 | Dictionary | Querying tags/registries through a tool is more than a generic HUD |
| MK-EQP-010 | HDPE Reinforced Elytra | Dedicated flight item distinct from MekaSuit/jetpack modules |
| MK-REG-005 | Player advancements and criteria | Progression triggered by real actions requires a dedicated ledger for each module |
| MK-REG-006 | Commands and permissions | Commands extend beyond the radiation commands already mentioned |
| MK-WOR-008 | Retrogen in existing chunks | Retrogen is not initial generation; risk of changes to existing data |
| MK-TOL-005 | Tools equipment in mob spawning | Tool/armor attributes do not cover natural-spawn gear |
| MK-TOL-006 | Tools material reactions and effects | Nominal source indicates a reaction; the exact implementation remains a hypothesis |
| MK-REG-007 | Effective configuration and scoped reload | New cross-cutting ledger distinguishes startup/server/client and real effect |

## Interpreting scenario links

A scenario associated with a family is a vertical investigation seed, not automatic coverage of every child. The 253 requirements still need concrete IDs, finite domains, checkpoints, typed steps, predicates and budgets. Manual partial design links are in `scenario_design_links`, with exercised and remaining aspects; `executable_scenario_ids` remains empty. Requirements without a reviewed match retain their gap. The 22 scenarios are not the complete corpus.

Closure distinguishes discovered objects, states/transitions, applied actions, observability, integration and fresh evidence. Requirement counts or percentages of passed tests do not certify the mod. An inaccessible observable, unsupported case or missing artifact keeps the gap open.

## Core module

### MK-REG-001 — Static registries

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-REG-001.R01 — Effective census and registry identity | Load each official profile and export registries with resolved aliases | Namespaced IDs, states, serializers, components and owning module | An entry present in resources but absent at runtime remains a gap | LAB-JS |
| MK-REG-001.R02 — Reconcile objects with behavior | Instantiate representatives of each registered type and query available surfaces | Created object, supported actions and actually observable fields | An unknown registry does not automatically become passive data | LAB-JS |

Proof limit: A runtime census does not qualify in-game use; link objects to functional families. A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-REG-002 — Recipes and data processing

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-REG-002.R01 — Recipes and ingredients after reload | Export loaded recipes before/after datapack reload | Type, resolved ingredient, components, order and result | A removed recipe or empty tag does not leave the old recipe valid | LAB-JS |
| MK-REG-002.R02 — Real recipe application | Insert ingredients through the player path and extract the product through the corresponding path | Actual consumption, remainders, complete result and selected recipe | A recipe valid in data but not executing in-game remains uncovered | LAB-JD1 |
| MK-REG-002.R03 — Competing recipes and invalidation | Replace input and reload recipes during controlled operation | Recipe selection and progress/resource continuity or reset | Two recipes accepting the same input require priority characterization | LAB-JS |

Proof limit: A runtime census does not qualify in-game use; link objects to functional families. Do not use commands/admin to perform the Act intended to qualify as interaction. A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-REG-003 — Tiers and installers

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-REG-003.R01 — Install a tier on a machine with contents | Apply the installer to the block with inventory, energy, upgrades and progress | Tier, capacity, contents, progress, configuration and installer consumption | Incompatible or repeated installer; creative separate from survival | LAB-JD1 |
| MK-REG-003.R02 — Upgrade a connected transmitter | Apply alloy/installer to the transmitter while a second client observes | Topology, buffers, connections, tier and player inventory | Partial network upgrade and an output without space must not hide loss | LAB-JD2 |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. At least two clients; use three for owner/trusted/stranger where required.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-REG-004 — Upgrades

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-REG-004.R01 — Install and remove active upgrades | Insert/remove each discovered upgrade through the active machine's interface | Applied count, limits, consumption and progression before/after | An upgrade above the limit or not accepted must have its effect characterized | LAB-JD1 |
| MK-REG-004.R02 — Relevant upgrade combinations | Execute speed/energy/chemical combinations and anchor/filter/stone-generator cases | Actual rates, resource use, tickets and specific enabled effect | Combination with blocked output and a config change requires its own case | LAB-JS |
| MK-REG-004.R03 — Upgrades and configuration reload | Save and restart with installed upgrades and an alternative valid configuration | Preserved upgrades and observed recalculated effects | Do not assume every config supports hot reload | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: SC-LAB-006. No link constitutes approval.

### MK-IO-001 — Side configuration

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-IO-001.R01 — Physical and logical faces after rotation | Configure input/output per face and rotate the machine with a tool | Orientation, side mapping, accepted transfers and block state | All six faces and discovered disabled/input/output sides | LAB-JD1 |
| MK-IO-001.R02 — Auto-eject and simultaneous automation | Player changes auto-eject while a hopper or tube transfers through another face | Delta of each inventory and both clients' views | Full destination and face change in the same tick | LAB-JD2 |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. At least two clients; use three for owner/trusted/stranger where required.

Related vertical seeds: SC-LAB-007, SC-LAB-022. No link constitutes approval.

### MK-IO-002 — Redstone control

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-IO-002.R01 — Signal edges and modes | Toggle the signal at tick boundaries in each supported redstone mode | Received signal, active state, progress, consumption and comparator output | Short pulse and sustained signal characterized separately | LAB-JS |
| MK-IO-002.R02 — Redstone pause with pending work | Player changes mode and removes the signal during processing | Progress, inputs, visual states and resumption | Do not assume pause/reset or zero consumption without observing Java | LAB-JD1 |

Proof limit: A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player. Do not use commands/admin to perform the Act intended to qualify as interaction.

Related vertical seeds: SC-LAB-005, SC-LAB-022. No link constitutes approval.

### MK-IO-003 — Slots and automation

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-IO-003.R01 — Slot and carried-stack transactions | Click, shift-click, drag and split a stack in correct/incorrect slots | All slots, carried stack, limits and server confirmation | Full inventory, stack with components and rejected clicks | LAB-JD1 |
| MK-IO-003.R02 — Hopper and two viewers on the same inventory | Two clients operate the menu while a hopper inserts/extracts | Application order, per-slot deltas and client convergence | Concurrency with block removal and menu closing | LAB-JD2 |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. At least two clients; use three for owner/trusted/stranger where required.

Related vertical seeds: SC-LAB-002, SC-LAB-004, SC-LAB-018. No link constitutes approval.

### MK-IO-004 — Copied configuration

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-IO-004.R01 — Copy only supported configuration | Capture a Configuration Card and apply it to a compatible, distinct destination | Source/destination config, card payload and nonconfiguration contents | Missing source and application across incompatible types | LAB-JD1 |
| MK-IO-004.R02 — Copy with permissions and old data | Owner and third party attempt to apply a card after security changes | Effective authorization, configuration change and feedback | A card saved before reload or rename does not imply current access | LAB-JD2 |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. At least two clients; use three for owner/trusted/stranger where required.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-ENE-001 — Energy Cubes

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-ENE-001.R01 — Cube limits and per-face transfer | Insert/extract at discovered limits and configured sides | Energy before/after, capacity, accepted rate and rejection | Zero, one unit, limit and beyond-limit attempt with actual representation | LAB-JS |
| MK-ENE-001.R02 — Portability of an energized cube | Charge, break, collect and replace the cube through the player | Energy in block and item, drops, owner and configuration | Creative and survival tiers with contents differ and require profiles | LAB-JD1 |

Proof limit: A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player. Do not use commands/admin to perform the Act intended to qualify as interaction.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-ENE-002 — Universal Cable

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-ENE-002.R01 — Network distribution with competing demands | Build a line and branch through player actions, connect sources and consumers | Energy at each terminal, buffers, topology and causal order | Two consumers with unequal demand and limited capacity | LAB-JD2 |
| MK-ENE-002.R02 — Merge and split with occupied buffers | Remove/connect a cable between charged networks | Network identity, redistribution, observed losses and terminals | Ring, disconnected network and merge at a chunk boundary | LAB-JS |
| MK-ENE-002.R03 — Network crossing chunks | Unload a section, save, restart and reconnect the network | Buffers, connections and per-node resumption | Residual chunk ticket or duplicate transfer after reload | LAB-JREC |

Proof limit: At least two clients; use three for owner/trusted/stranger where required. A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: SC-LAB-008, SC-LAB-019, SC-LAB-021. No link constitutes approval.

### MK-ENE-003 — Charge and discharge items

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-ENE-003.R01 — Flow between a chargeable item and storage | Move an empty/partial/full item between charge and discharge slots | Item/block energy, stack quantity and slots | Wrong item or an item with the same ID and different components | LAB-JD1 |
| MK-ENE-003.R02 — Equipment charging during use | Player uses equipment while another path charges it | Accepted energy, consumption and actual player effect | Remove the item on the transfer tick and inventory without space | LAB-JD2 |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. At least two clients; use three for owner/trusted/stranger where required.

Related vertical seeds: SC-LAB-003. No link constitutes approval.

### MK-ENE-004 — Induction Matrix

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-ENE-004.R01 — Matrix formation and capacity | Construct valid/invalid shapes and vary cells/providers | Formed state, recognized components, capacity and throughput | Distinct ports and providers; discovered dimension limits | LAB-JD1 |
| MK-ENE-004.R02 — Energized breakdown and reconstruction | Charge the matrix and remove casing/port; reconstruct | Stored contents, identity, port access and remnants | Separate parts or two adjacent structures do not silently merge | LAB-JS |
| MK-ENE-004.R03 — Identity and storage after restart | Save a charged matrix, restart and use it through different ports | Persisted energy, limit and transfer results | A partially loaded structure must not report ready capacity | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-ENE-005 — Lasers

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-ENE-005.R01 — Beam and physical obstruction | Activate a laser with declared targets, obstructions and distance | Beam, impact, affected blocks/entities and energy | Object crossing the beam and a chunk not yet loaded | LAB-JS |
| MK-ENE-005.R02 — Amplifier and tractor beam | Player configures thresholds and collects beam-caused drops | Accumulation, emission, loot, item destination and inventories | Full output and a shield in the path need separate results | LAB-JD1 |

Proof limit: Do not replace natural spawn/generation with summon or injected blocks. Do not use commands/admin to perform the Act intended to qualify as interaction.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-CHE-001 — Chemical types and attributes

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-CHE-001.R01 — Chemical identity and attributes | Enumerate types and query public representations and attributes | ID, unit, quantity representation and accessible attributes | Never merge chemical/slurry/pigment/radioactive types by approximate name | LAB-JS |
| MK-CHE-001.R02 — Type compatibility in receivers | Offer valid/incompatible types to selected receivers | Stored type, accepted quantity and special effects | The same visible name does not imply the same identity or attribute | LAB-JS |

Proof limit: A runtime census does not qualify in-game use; link objects to functional families. A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-CHE-002 — Chemical Tanks

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-CHE-002.R01 — Fill, empty and dump through the interface | Configure a tank and move chemical through the player path | Type/quantity, dumping mode and environmental effects | Full, partial and type change without emptying | LAB-JD1 |
| MK-CHE-002.R02 — Break and reload an occupied tank | Save/reopen and break a tank with contents in a controlled environment | Item contents, loss/emission and world state | Radioactive contents require a radiation witness in addition to the tank | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: SC-LAB-009. No link constitutes approval.

### MK-CHE-003 — Pressurized Tubes

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-CHE-003.R01 — Connect distinct chemical networks | Connect/disconnect tubes with the same and incompatible chemicals | Topology, segment types and per-terminal transfers | Merge occupied networks and a source without resources | LAB-JD2 |
| MK-CHE-003.R02 — Residue on split and unload | Split a full network, unload chunks and reopen | Per-segment quantities and demand continuity | Do not count only the global total; observe every terminal | LAB-JREC |

Proof limit: At least two clients; use three for owner/trusted/stranger where required. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: SC-LAB-009. No link constitutes approval.

### MK-CHE-004 — Gauge Dropper

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-CHE-004.R01 — Gauge Dropper in the real interface | Use an empty/partial dropper on a gauge with both applicable buttons | Item contents, gauge, carried stack and transferred quantity | Incompatible type and a full dropper must not hide an improper swap | LAB-JD1 |
| MK-CHE-004.R02 — Synchronized gauge and changed tank | Two viewers interact with a dropper while the machine consumes | Authoritative state, per-player item and per-client gauges | An old client gauge does not authorize duplication | LAB-JD2 |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. At least two clients; use three for owner/trusted/stranger where required.

Related vertical seeds: SC-LAB-009. No link constitutes approval.

### MK-FLU-001 — Fluid Tanks

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-FLU-001.R01 — Buckets and partial filling | Use a bucket and container item on tanks of each tier | Fluid and components, volumes, remainders and tank state | Capacity below the attempted amount and a container with another fluid | LAB-JD1 |
| MK-FLU-001.R02 — Portable tank and creative mode | Break/replace a tank with contents; repeat in a creative profile | Contents, capacity, mode and container effects | Creative is excluded from survival conservation invariants | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-FLU-002 — Mechanical Pipes

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-FLU-002.R01 — Fluid topology and distribution | Connect sources and destinations through a line/ring/branch | Per-node/terminal volume and fluid compatibility | Full destinations, disconnected sides and connection order | LAB-JS |
| MK-FLU-002.R02 — Split and resumption of an occupied network | Separate the network, unload a segment and restart | Persisted fluids, buffers and resumption deltas | No mixing between types after a network change | LAB-JREC |

Proof limit: A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-FLU-003 — Electric Pump

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-FLU-003.R01 — Collection from finite sources and water | Position a pump with declared liquid types and surroundings | Fluid blocks, captured volume and energy per tick | Incomplete source, waterlogging and range measured in Java | LAB-JS |
| MK-FLU-003.R02 — Filter and heavy-water production | Player installs/removes the relevant upgrade and configures the destination | Produced type, volumes, world resource and consumption | Full output and interrupted energy with a search in progress | LAB-JD1 |

Proof limit: Do not replace natural spawn/generation with summon or injected blocks. Do not use commands/admin to perform the Act intended to qualify as interaction.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-FLU-004 — Fluidic Plenisher

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-FLU-004.R01 — Liquid expansion in the world | Supply a plenisher with a bounded region and obstacles | Filled positions, order, spent volume and energy | Height boundaries and a nonreplaceable block | LAB-JS |
| MK-FLU-004.R02 — Resumption of the pending region | Interrupt supply, save/reopen and resupply | Filling frontier and new block changes | Do not repeat consumption for an already placed block | LAB-JREC |

Proof limit: Do not replace natural spawn/generation with summon or injected blocks. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-HEA-001 — Thermal conduction

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-HEA-001.R01 — Thermal transfer between nodes | Connect nodes with controlled temperatures and environment | Temperature, heat capacity, transfer and observed losses | Valid extremes, disabled face and distance from ambient temperature | LAB-JS |
| MK-HEA-001.R02 — Thermal topology change | Disconnect a hot node and restore it after unload/restart | Per-node thermal states and subsequent evolution | Do not impose conservation if the model includes environmental losses | LAB-JREC |

Proof limit: A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-HEA-002 — Heaters

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-HEA-002.R01 — Resistive Heater control | Player adjusts consumption while the source has partial energy | Effective config, consumed energy, temperature and interface | Out-of-range value and insufficient source characterized | LAB-JD1 |
| MK-HEA-002.R02 — Fuelwood Heater and remainders | Supply accepted fuels and observe depletion | Fuel slot, remainder, heat and observed duration | Invalid fuel and output/remainder without space | LAB-JS |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-HEA-003 — Thermoelectric Boiler

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-HEA-003.R01 — Form a boiler with functional components | Build a boiler and vary dispersers/elements/valves | Formation, capacity, water/coolant, heat and outputs | Missing required component and invalid structure | LAB-JD1 |
| MK-HEA-003.R02 — Thermal exchange and saturated outputs | Supply discovered heat and fluids; block steam/coolant | Per-tick inputs/outputs, temperatures and limits | Water/sodium in distinct profiles and rejected mixtures | LAB-JS |
| MK-HEA-003.R03 — Hot rupture and recovery | Remove a valve/casing during operation and restart | Remaining state, fluids and evolution after reconstruction | Do not assume content retention on breaking | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-HEA-004 — Thermal Evaporation

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-HEA-004.R01 — Evaporation under environmental conditions | Vary day/night, biome, environment and external heating | Temperature, recipe and actual conversion rates | The same layout in distinct environments requires its own baseline | LAB-JS |
| MK-HEA-004.R02 — Structure and valve access | Assemble an evaporator and insert/extract through ports | Formation, capacity and contents at each port | Missing controller/valve or full output | LAB-JD1 |

Proof limit: Do not replace natural spawn/generation with summon or injected blocks. Do not use commands/admin to perform the Act intended to qualify as interaction.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-MAC-001 — Enrichment Chamber

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-MAC-001.R01 — Complete enriching cycle | Place the machine, open the menu, insert recipe inputs and extract the product | Complete inputs/outputs, progress, energy and per-tick active state | Invalid input, lack of energy and full output are separate cases | LAB-JD1 |
| MK-MAC-001.R02 — Resumption during enriching | Save/restart at start/middle/end boundaries | Progress, consumption and production before/after loading | An identical final product does not hide different consumption/timing | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: SC-LAB-001, SC-LAB-002, SC-LAB-003, SC-LAB-004, SC-LAB-005, SC-LAB-006, SC-LAB-007, SC-LAB-018, SC-LAB-021. No link constitutes approval.

### MK-MAC-002 — Crusher

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-MAC-002.R01 — Crushing with distinct products | Insert recipes with distinct ingredients/components through the menu | Selected recipe, output and consumed resources | Stack with components and incompatible container | LAB-JD1 |
| MK-MAC-002.R02 — Input change and scarce resource | Alternate input or energy during crushing | Selection, progress, partial consumption and resumption | Full output on the last tick and input removed before completion | LAB-JS |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-MAC-003 — Energized Smelter

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-MAC-003.R01 — Smelting through the real inventory path | Process loaded smelting recipes and extract the result | Output, attributes/components, energy and observed time | Unsupported recipe and incompatible result in the output | LAB-JD1 |
| MK-MAC-003.R02 — Recipe change before and after restart | Change input during processing and save/reopen | Per-recipe progress and continuation result | Do not assume vanilla furnace behavior is an oracle for the mod | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-MAC-004 — Metallurgic Infuser

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-MAC-004.R01 — Combined infusion resource and item | Supply infusion reagent and input through real slots | Infusion type/quantity, item and energy consumption | Wrong or insufficient reagent and residual type change | LAB-JD1 |
| MK-MAC-004.R02 — Clear and refill pending infusion | Empty/refill the tank during a controlled cycle | Progress reset/continuity, residues and output | Enriched versus unenriched input with equivalence to be measured | LAB-JS |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-MAC-005 — Osmium Compressor

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-MAC-005.R01 — Compression with an auxiliary chemical | Supply item and chemical accepted by an effective recipe | Auxiliary quantity, consumption and output | Incompatible type and discovered quantity threshold | LAB-JD1 |
| MK-MAC-005.R02 — Compression blocking at completion | Fill output and interrupt chemical during processing | Progress, consumption and production on resumption | Do not consume a component merely because a recipe was found | LAB-JS |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-MAC-006 — Combiner

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-MAC-006.R01 — Combination with two material inputs | Fill primary/auxiliary slots separately and execute | Both inputs, observed ratio, result and energy | One missing input or items swapped between slots | LAB-JD1 |
| MK-MAC-006.R02 — Combiner components and tags | Execute recipes with tag-based ingredients and partially occupied outputs | Matching, per-slot consumption and output composition | Tag members and reload changing availability | LAB-JS |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-MAC-007 — Purification Chamber

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-MAC-007.R01 — Purification with a valid chemical | Insert ore/input and supply the required chemical | Chemical per tick, progress and product | Wrong chemical type and insufficient quantity | LAB-JD1 |
| MK-MAC-007.R02 — Purification interruption and upgrades | Vary energy and upgrades during operation | Consumption, time and continuity under each configuration | Speed×chemical-upgrade×output-full is a risky combination | LAB-JS |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-MAC-008 — Chemical Injection Chamber

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-MAC-008.R01 — Chemical injection in the ore chain | Process recognized input with the required chemical | Chain items, chemical consumption and progress | Incompatible chemical residue and depleted input | LAB-JD1 |
| MK-MAC-008.R02 — Refill and restart during injection | Refill sequentially and restart with pending work | Persisted contents and next transition after loading | Production does not duplicate at the save/completion boundary | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-MAC-009 — Precision Sawmill

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-MAC-009.R01 — Probabilistic output with controlled RNG | Process a batch with a qualified RNG source | Primary/secondary product per execution and observable draws | Do not impose a wiki probability as the numeric expected result | LAB-JS |
| MK-MAC-009.R02 — Blocking only the secondary output | Player occupies the secondary output and triggers relevant recipes | Both outputs, progress and consumption | Recipe without a secondary product and incompatible output stack | LAB-JD1 |

Proof limit: A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player. Do not use commands/admin to perform the Act intended to qualify as interaction.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-MAC-010 — Factories

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-MAC-010.R01 — Concurrent factory processes | Distribute inputs across lanes through the menu and enable sorting | Recipe/progress/consumption of each lane and inventories | Heterogeneous lanes and one blocked output | LAB-JD1 |
| MK-MAC-010.R02 — Shared energy budget | Run more processes than the energy budget permits | Progress order, starvation and total expenditure per tick | Input sorting must not hide execution priority | LAB-JS |
| MK-MAC-010.R03 — Upgrade and reload with multiple lanes | Apply a tier installer and save during simultaneous processes | Lanes, contents, upgrades, sorting and resumption | Tier and factory type treated as discovered parameters | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-MAC-011 — Chemical Oxidizer

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-MAC-011.R01 — Item-to-chemical oxidation | Insert a valid item and extract chemical through the automation path | Item consumption, produced chemical and energy | Full chemical output and an incompatible pre-existing type | LAB-JD1 |
| MK-MAC-011.R02 — Atomic conversion with partial output space | Offer less space than one cycle's product | Item, tank and progress state at every tick | Do not assume partial batches are allowed without observation | LAB-JS |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-MAC-012 — Chemical Infuser

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-MAC-012.R01 — Infusion of two chemicals | Connect two tanks of the required types and open the menu | Consumption of each reagent, product and progress | Reversed inputs, one absent input and boundary quantity | LAB-JD1 |
| MK-MAC-012.R02 — Restore reagent with blocked output | Deplete one input, fill the output and resume | Reserves and progress at blocking transitions | Residual types preventing a recipe change | LAB-JS |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-MAC-013 — Electrolytic Separator

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-MAC-013.R01 — Separation with two outputs | Supply fluid and extract products through different sides | Consumed fluid, both chemicals and energy | One full output while the other remains available | LAB-JD1 |
| MK-MAC-013.R02 — Independent output dumping | Toggle dumping modes per output during processing | Produced/discarded quantity and effective expenditure | Dump-excess and dump-all characterized separately | LAB-JS |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-MAC-014 — Rotary Condensentrator

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-MAC-014.R01 — Toggle condensing and decondensing | Player reverses mode with residual contents | Mode, liquid/chemical input/output and progress | An absent recipe in one direction does not inherit validity from the other | LAB-JD1 |
| MK-MAC-014.R02 — Mode and buffers after restart | Save each direction with partially filled tanks | Types, quantities and next conversion | Mode changes must not silently reinterpret units | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-MAC-015 — Chemical Dissolution Chamber

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-MAC-015.R01 — Dissolution and slurry per recipe | Supply item and reagent to the dissolution chamber | Consumed item/reagent, slurry and energy per tick | Full output and intermittent reagent | LAB-JD1 |
| MK-MAC-015.R02 — RNG and reagent-use policy | Execute a batch with declared RNG and upgrades | Consumption, progress and output sequence | Do not round stochastic consumption or assume constant expenditure | LAB-JS |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-MAC-016 — Chemical Washer

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-MAC-016.R01 — Slurry washing with fluid | Connect correct slurry and fluid and extract the product | Dirty/clean slurry, spent fluid and result | Insufficient water and slurry from another ore | LAB-JD1 |
| MK-MAC-016.R02 — Residues and refill after restart | Save before refill and resume the cycle | Residues, progress and continuity per recipe | Do not merge different IDs because they have the same visible name | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-MAC-017 — Chemical Crystallizer

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-MAC-017.R01 — Crystallization with residual quantity | Supply chemical and extract the item through the interface | Residual chemical, product and energy consumption | Less than the minimum recipe amount and an incompatible output type | LAB-JD1 |
| MK-MAC-017.R02 — Recipe reload during crystallization | Reload data with a changed/removed recipe | Valid selection, progress and preserved or consumed resources | No use of a stale recipe cache | LAB-JS |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-MAC-018 — Pressurized Reaction Chamber

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-MAC-018.R01 — Reaction with three input classes | Offer item, fluid and chemical required by the recipe | Each input, item/chemical outputs and progress | All combinations with one missing input | LAB-JD1 |
| MK-MAC-018.R02 — Independent output blocking | Alternately block item output and chemical output | Reserves and consumption of all resources per tick | One free output does not authorize assumed partial completion | LAB-JS |
| MK-MAC-018.R03 — Recipe change with multiple residues | Save/reopen after changing one input's type | Matching, residues and next transition | Chemical identities and item components must persist | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-MAC-019 — Solar Neutron Activator

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-MAC-019.R01 — Solar condition and obstructed sky | Vary day/night, weather and the block above the activator | Effective environment, chemical, progress and rate | Do not treat artificial light as sunlight without reference evidence | LAB-JS |
| MK-MAC-019.R02 — Activator input and output under blocking | Player connects supply and removes output during the solar window | Transfers, progress and active state | Nightfall on the completion tick and a full destination | LAB-JD1 |

Proof limit: Do not replace natural spawn/generation with summon or injected blocks. Do not use commands/admin to perform the Act intended to qualify as interaction.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-MAC-020 — Isotopic Centrifuge

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-MAC-020.R01 — Centrifuging and radioactive products | Supply an accepted chemical and extract the product | Chemical/energy per tick, output and relevant attributes | Incompatible output tank and insufficient quantity | LAB-JD1 |
| MK-MAC-020.R02 — Nuclear residue persistence | Save/reopen and break the container in a scratch world | Contents and associated environmental effects | Qualify radiation beyond the tank delta alone | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-MAC-021 — Pigments and painting

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-MAC-021.R01 — Pigment extraction and mixing | Use extractor and mixer with discovered recipes/colors | Input/output pigments, quantities and energy | Asymmetric mixing and output with a different pigment | LAB-JD1 |
| MK-MAC-021.R02 — Item painting and final identity | Insert a colorable item and pigment into the painting machine | Final item with components, spent pigment and progress | Noncolorable item and output containing another color | LAB-JD1 |
| MK-MAC-021.R03 — Color and gauge after resource reload | Reload the resource pack while retaining painting state | Semantic pigment ID and client visual representation | A correct image does not replace server quantity/ID | LAB-JD1 |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. An isolated visual comparison does not prove game logic.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-MAC-022 — Nutritional Liquifier

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-MAC-022.R01 — Food liquefaction and containers | Process foods with different effective properties | Nutritional paste, remainders, energy and inputs | Unaccepted food and full output | LAB-JD1 |
| MK-MAC-022.R02 — Player consumption from a canteen | Partially fill a canteen and consume at distinct hunger levels | Paste in the item, hunger/saturation and use animation | Full hunger, interrupted use and empty tank | LAB-JD1 |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-MAC-023 — Formulaic Assemblicator

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-MAC-023.R01 — Formula and grid in the Assemblicator | Encode a real formula and supply the grid manually/automatically | Selected recipe, per-slot consumption, output and remainders | Invalid formula, ingredient with components and full slot | LAB-JD1 |
| MK-MAC-023.R02 — Automatic crafting under a recipe change | Change formula/reload while another viewer observes | Grid, buffer, carried stack and synchronization | Craft executes once even with a change at the boundary | LAB-JD2 |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. At least two clients; use three for owner/trusted/stranger where required.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-MAC-024 — Oredictionificator

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-MAC-024.R01 — Tag filters and result selection | Configure the Oredictionificator and insert an item from a valid tag | Selected filter, output member and consumption | Tag without members and an item eligible for multiple tags | LAB-JD1 |
| MK-MAC-024.R02 — Reload and filter priority | Reorder filters and change a tag through datapack reload | Order, effective selection and subsequent results | A result removed from the tag must not remain cached | LAB-JS |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-NET-001 — Logistical Transporter

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-NET-001.R01 — Routing with color and priorities | Build normal/restrictive/diversion routes and send stacks | Items in transit, route, destination and per-terminal deltas | No route, loop and two equally eligible outputs | LAB-JD2 |
| MK-NET-001.R02 — Destination disappears during transport | Break/fill the destination while a stack is in transit | Return, reroute, drop and item accounting | Do not test only the final item; transient state matters | LAB-JS |
| MK-NET-001.R03 — Items in transit after reload | Save/reopen or unload an intermediate chunk | Cargo identity/quantity and subsequent delivery | No duplication from transit retransmission | LAB-JREC |

Proof limit: At least two clients; use three for owner/trusted/stranger where required. A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: SC-LAB-010. No link constitutes approval.

### MK-NET-002 — Logistical Sorter

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-NET-002.R01 — Overlapping filters and stack size | Create item/tag/mod-id filters with distinct priorities | Match, extracted quantity and observed destination | Inverted, fuzzy, min/max limits and default route | LAB-JD1 |
| MK-NET-002.R02 — Sorter changes during transfer | Player edits filters with extraction and viewers active | Selection sequence, inventories and server confirmation | A filter removed on the extraction tick does not use old client state | LAB-JD2 |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. At least two clients; use three for owner/trusted/stranger where required.

Related vertical seeds: SC-LAB-010. No link constitutes approval.

### MK-NET-003 — Quantum Entangloporter

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-NET-003.R01 — Frequency across dimensions and owners | Link entangloporters to the same/different frequency across dimensions | Shared buffers, access and transfer of each resource | Same name with different owner and private frequency | LAB-JD2 |
| MK-NET-003.R02 — Multiresource concurrency and sides | Offer items/fluids/chemicals/energy/heat according to discovered support | State per resource, face and endpoint | One blocked resource class does not define the others' behavior | LAB-JS |
| MK-NET-003.R03 — Endpoint and frequency removal | Unload an endpoint, delete the frequency and restart | Remaining resources, association and resumption | Last member removed and reconnect with a distinct UUID | LAB-JREC |

Proof limit: At least two clients; use three for owner/trusted/stranger where required. A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: SC-LAB-020. No link constitutes approval.

### MK-NET-004 — Network Reader

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-NET-004.R01 — Reader against independent observation | Query an empty/occupied network and compare endpoints | Displayed text/values and authoritative state | The reader is not the only oracle for its own network | LAB-JD1 |
| MK-NET-004.R02 — Reader update on split/merge | Split/merge the network with the screen/tooltip open | Refresh frequency, name, contents and stale values | Locale and displayed unit without changing semantic value | LAB-JD1 |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. An isolated visual comparison does not prove game logic.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-NET-005 — Configure connections

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-NET-005.R01 — Configurator on each face and mode | Use normal/sneak interaction and toggle the tool mode | Color, connection type, orientation and tool energy | Action on the opposite face and incompatible block | LAB-JD1 |
| MK-NET-005.R02 — Reconfigure an active network with restricted access | Owner/third party changes a transmitter while resources circulate | Authorization, topology and subsequent transfer | Visual prediction without server confirmation does not count | LAB-JD2 |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. At least two clients; use three for owner/trusted/stranger where required.

Related vertical seeds: SC-LAB-008, SC-LAB-019. No link constitutes approval.

### MK-STO-001 — Bins

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-STO-001.R01 — Bin with lock and stack identity | Insert/extract through interaction and compare items with components | Count, type, lock and item held by the player | Full bin, empty lock and same-ID item with different components | LAB-JD1 |
| MK-STO-001.R02 — Full bin on break and replacement | Break/replace and restart with contents | Stored count, drops and item attribute | Creative explicitly separate from survival consumption | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-STO-002 — Personal Chest/Barrel

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-STO-002.R01 — Personal storage and permissions | Owner/trusted/stranger attempt to open and automate a chest/barrel | Access, contents, owner and per-client feedback | Portable blocks and items may have different surfaces | LAB-JD2 |
| MK-STO-002.R02 — Personal contents outside the block | Remove chest/barrel, open if supported, save and replace | Slots, owner, identity and global persistence | Do not lose contents when the physical block ceases to exist | LAB-JREC |

Proof limit: At least two clients; use three for owner/trusted/stranger where required. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: SC-LAB-011. No link constitutes approval.

### MK-STO-003 — Dynamic Tank

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-STO-003.R01 — Dynamic Tank with valves and shapes | Construct valid/invalid structures and supply them | Formation, capacity, type and per-valve volumes | Two adjacent structures and boundary dimensions | LAB-JD1 |
| MK-STO-003.R02 — Break and reconstruct an occupied tank | Remove a component and restore it after restart | Contents and identity retained/lost according to Java | Two reconstructed segments do not receive the same contents | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: SC-LAB-013. No link constitutes approval.

### MK-QIO-001 — Drive Array and drives

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-QIO-001.R01 — Drive types and capacity | Insert same-ID stacks with distinct components into QIO | Total quantity, type count and per-drive limit | Type limit and quantity limit are separate dimensions | LAB-JD1 |
| MK-QIO-001.R02 — Remove a drive during concurrent storage operations | Player removes a drive while another inserts/withdraws | Per-drive contents and frequency total, applied transactions | Array without energy/config where applicable: discovered, not assumed | LAB-JD2 |
| MK-QIO-001.R03 — Drive identity outside the array | Save the world with removed drives and reinstall them | Drive ID, association, contents and access | A cloned reference/ID must not merge distinct drives | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. At least two clients; use three for owner/trusted/stranger where required. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: SC-LAB-012. No link constitutes approval.

### MK-QIO-002 — Dashboards

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-QIO-002.R01 — Dashboard search and transaction | Search for an item and click/shift-click with a real carried stack | Visible items, filters, slots and server inventory | Full inventory and search without results | LAB-JD1 |
| MK-QIO-002.R02 — Crafting windows and simultaneous viewers | Two clients use crafting/storage on the same frequency | Ingredients, outputs/remainders, tabs and synchronization | Recipe competes with withdrawal of the last ingredient | LAB-JD2 |
| MK-QIO-002.R03 — Portable dashboard and remote access | Use the device across dimensions/distances and security modes | Availability, effective cost and frequency access | Unavailable destination and frequency deleted during the session | LAB-JD1 |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. At least two clients; use three for owner/trusted/stranger where required.

Related vertical seeds: SC-LAB-012. No link constitutes approval.

### MK-QIO-003 — Import/Export

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-QIO-003.R01 — Filtered import and export | Configure importer/exporter against inventories with distinct stacks | Quantity accepted per operation, filters and state on both sides | Fuzzy, min/max, full output and partial input | LAB-JS |
| MK-QIO-003.R02 — Transfer during unload and reload | Unload an endpoint and restart the frequency with pending work | Contents, filters and resumption without duplicate deltas | Opposing import/export on the same inventory require a declared order | LAB-JREC |

Proof limit: A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: SC-LAB-012. No link constitutes approval.

### MK-QIO-004 — Redstone Adapter

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-QIO-004.R01 — Signal around the threshold | Configure the adapter and vary stock below/equal to/above the limit | Considered quantity, filter and redstone signal | Equal/different criterion and item with components | LAB-JD1 |
| MK-QIO-004.R02 — Update after a frequency change | Remove a drive or change frequency while another client observes | Authoritative signal and display updates | Cached stock must not maintain the old signal indefinitely | LAB-JD2 |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. At least two clients; use three for owner/trusted/stranger where required.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-SEC-001 — Security Desk

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-SEC-001.R01 — Owner/trusted/stranger matrix | Owner configures the desk and three identities attempt real actions | Per-action permission, feedback and affected state | Opening a menu, extracting resources, breaking and configuring are different access operations | LAB-JD2 |
| MK-SEC-001.R02 — Revocation during an open session | Revoke trusted status and restart with previously open menus | List, owner, current permissions and action blocking | Do not rely on authorization granted only at menu opening | LAB-JREC |

Proof limit: At least two clients; use three for owner/trusted/stranger where required. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: SC-LAB-011. No link constitutes approval.

### MK-SEC-002 — Frequencies

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-SEC-002.R01 — Name scope and visibility | Create public/private frequencies with the same name and distinct owners | Identity, listing and actually associated endpoints | A visible name does not replace the real composite key | LAB-JD2 |
| MK-SEC-002.R02 — Delete and recreate a frequency | Remove the last member, delete and create the same name after restart | Resources, references and new identity | An old endpoint must not accidentally access a new frequency | LAB-JREC |

Proof limit: At least two clients; use three for owner/trusted/stranger where required. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: SC-LAB-011, SC-LAB-020. No link constitutes approval.

### MK-WOR-001 — Digital Miner

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-WOR-001.R01 — Filtered mining and replacement | Configure filters and start the Digital Miner on a known fixture | Targets, changed blocks, drops, energy and replacement | Inverse filter, silk/fortune and unavailable replacement | LAB-JD1 |
| MK-WOR-001.R02 — Miner pause and full destination | Fill output and interrupt a resource during search/mining | Counter, current target, consumption and world | Pending search and manual target removal in the same tick | LAB-JS |
| MK-WOR-001.R03 — Miner restart and boundaries | Save an active miner and reopen with partial chunks | Search frontier, target and subsequent changes | Do not repeat drops from an already mined target | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: SC-LAB-022. No link constitutes approval.

### MK-WOR-002 — Dimensional Stabilizer

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-WOR-002.R01 — Chunk tickets and energy budget | Select the stabilizer area and vary energy/activity | Owned tickets, actually ticking chunks and expenditure | Area limit, offline owner and overlapping stabilizers | LAB-JS |
| MK-WOR-002.R02 — Release tickets on removal/reopen | Remove/deactivate the block and restart the server | Remaining tickets and actual chunk state | A ticket from another system must not be removed along with it | LAB-JREC |

Proof limit: A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: SC-LAB-019. No link constitutes approval.

### MK-WOR-003 — Teleporter and portable teleporter

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-WOR-003.R01 — Player travel through portal and portable device | Build the frame, select the frequency and cross/use the item | Position/dimension, energy, cooldown and feedback | Invalid frame, uncharged destination and obstructed space | LAB-JD1 |
| MK-WOR-003.R02 — Entities and concurrent transport | Transport players/entities with passengers where supported | Identity, inventory, vehicle/passenger and authoritative side | Two travelers and a frequency change during the action | LAB-JD2 |
| MK-WOR-003.R03 — Destination in an unloaded chunk | Reopen the world and travel to a nonresident endpoint | Tickets, confirmed arrival and consumed resources | A timeout must not pass based on client-predicted position | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. At least two clients; use three for owner/trusted/stranger where required. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-WOR-004 — Seismic Vibrator/Reader

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-WOR-004.R01 — Seismic Reader against a known world | Turn on the vibrator and query a column with known blocks and heights | Per-layer result, energy and displayed values | Min/max height, air and a block with the same name but different state | LAB-JD1 |
| MK-WOR-004.R02 — Update after modifying a column | Change a block in the column and repeat the reading | Data source, refresh and presentation | An unloaded chunk or unpowered machine does not invent data | LAB-JD1 |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. An isolated visual comparison does not prove game logic.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-WOR-005 — Ores and generation

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-WOR-005.R01 — Deterministic per-chunk generation | Generate fresh chunks with pinned seed and configs | Blocks/states and mod resource placements | Generation order, chunk boundary and dimension without the feature | LAB-JS |
| MK-WOR-005.R02 — Generation profiles by configuration | Generate the same region in profiles with features enabled/disabled | Effective features and mod-caused differences | An old region cannot count as a newly generated chunk | LAB-JS |

Proof limit: Do not replace natural spawn/generation with summon or injected blocks.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-WOR-006 — Materials and drops

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-WOR-006.R01 — Material drops and processing | Mine blocks with declared tools/enchantments | Loot, components, wear and block changes | Unsuitable tool, silk and fortune have their own cases | LAB-JD1 |
| MK-WOR-006.R02 — Intermediate material tags and recipes | Reconcile ingots/dusts/clumps/shards/crystals/slurry and conversions | Tag members and effective recipe paths | Nothing is interchangeable merely because it appears in the same wiki category | LAB-JS |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. A runtime census does not qualify in-game use; link objects to functional families.

Related vertical seeds: SC-LAB-016. No link constitutes approval.

### MK-NUC-001 — Waste Barrels

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-NUC-001.R01 — Waste barrel decay and saturation | Supply the barrel and advance controlled time | Type/quantity, per-tick decay and emissions | Full barrel, unaccepted type and lack of ticking | LAB-JS |
| MK-NUC-001.R02 — Radioactive barrel removal and reload | Break/restart with contents and measure the environment | Persisted contents, radiation sources and subsequent damage | Do not assume offline wall-clock decay | LAB-JREC |

Proof limit: A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-NUC-002 — Environmental and personal radiation

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-NUC-002.R01 — Radiation field and personal dose | Create a source in scratch and vary distance/dimension/exposure | Environmental source, entity dose and per-tick damage | Removing a source does not imply instantaneous dose removal | LAB-JS |
| MK-NUC-002.R02 — Protection and independent instruments | Equip protection pieces and query dosimeter/geiger | Authoritative dose and damage plus instrument readings | Partial protection, broken equipment and disabled instrument | LAB-JD1 |
| MK-NUC-002.R03 — Radiation after death and restart | Kill/respawn and restart in the presence of sources | Environmental state, player data and subsequent evolution | Separate source persistence from dose persistence | LAB-JREC |

Proof limit: Do not replace natural spawn/generation with summon or injected blocks. Do not use commands/admin to perform the Act intended to qualify as interaction. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: SC-LAB-014. No link constitutes approval.

### MK-NUC-003 — SPS

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-NUC-003.R01 — Form SPS and convert resources | Build SPS and supply specified energy/chemical | Formation, progress, chemical and antimatter | Energy threshold and insufficient chemical | LAB-JD1 |
| MK-NUC-003.R02 — Entity effects within the SPS volume | Position discovered entities and activate the field under fixed conditions | Transformations, damage and entity states | Do not limit the oracle to antimatter; nominally indicated secondary effects | LAB-JS |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. Do not replace natural spawn/generation with summon or injected blocks.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-EQP-001 — MekaSuit

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-EQP-001.R01 — MekaSuit protection across damage types | Equip subsets and apply damage through real paths | Health, energy, mitigation and wear | Multiple sources in the same tick and energy depleting on impact | LAB-JD1 |
| MK-EQP-001.R02 — Equipment state visible to others | Change pieces/modes during movement and combat | Server state, local HUD and rendering of another player | Predicted invulnerability without energy does not count | LAB-JD2 |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. At least two clients; use three for owner/trusted/stranger where required.

Related vertical seeds: SC-LAB-015. No link constitutes approval.

### MK-EQP-002 — Meka-Tool

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-EQP-002.R01 — Meka-Tool modes and areas | Toggle mode and mine/attack in the pinned layout | Affected blocks, loot, energy and speed | Protected block and energy sufficient for only part of the area | LAB-JD1 |
| MK-EQP-002.R02 — Tool components after a usage change | Save the item, change hand/slot and resume the action | Mode, modules, charge and effective selection | Same-ID items with different modules must not share state | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-EQP-003 — Installable modules

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-EQP-003.R01 — Compatibility by module and host | Install/remove each discovered module in accepted/rejected equipment pieces | Count, limits, configurations and concrete effects | Exclusive modules, prerequisites and distinct host types | LAB-JD1 |
| MK-EQP-003.R02 — Combined effects of active modules | Exercise relevant movement, mining, vision, feeding and protection | Player/world effects, energy and configurations | Remove a module during use; disabling does not imply uninstalling | LAB-JD1 |
| MK-EQP-003.R03 — Individual configuration and conservation in the station | Modify options and save/reload equipment | Modules, consumed/returned items, settings and HUD | Do not validate only the installed module icon | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: SC-LAB-015. No link constitutes approval.

### MK-EQP-004 — Atomic Disassembler

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-EQP-004.R01 — Atomic Disassembler in available modes | Toggle modes and mine/attack with controlled charge | Target set, duration, energy and drops | Area mode with a protected block and empty tool | LAB-JD1 |
| MK-EQP-004.R02 — Mode change and authorized action | Player changes mode while the server processes an action | Applied mode, authoritative targets and client confirmation | A stale client mode does not select the server effect | LAB-JD2 |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. At least two clients; use three for owner/trusted/stranger where required.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-EQP-005 — Jetpacks

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-EQP-005.R01 — Flight and hover with real fuel | Activate the jetpack in supported modes and vary input | Per-tick position/velocity, fuel, mode and fall damage | Deplete fuel in flight and deactivate on landing | LAB-JD1 |
| MK-EQP-005.R02 — Flight and server correction | Observe flight with a second client and causal barriers | Authoritative state, pose and remote convergence | Reconnection in flight and equipment change during propulsion | LAB-JD2 |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. At least two clients; use three for owner/trusted/stranger where required.

Related vertical seeds: SC-LAB-015. No link constitutes approval.

### MK-EQP-006 — Scuba and Free Runners

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-EQP-006.R01 — Scuba and air while submerged | Equip mask/tank and stay submerged with partial gas | Air, chemical, damage and filtered effects | Mask without tank, wrong gas and mode interruption | LAB-JD1 |
| MK-EQP-006.R02 — Free Runners and real falling | Fall from controlled heights with full/partial charge and without equipment | Fall damage, energy and landing position | Partial armor and damage from a source unrelated to falling | LAB-JD1 |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-EQP-007 — Hazmat

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-EQP-007.R01 — Partial and complete Hazmat under exposure | Equip each relevant combination during radiation exposure | Dose, damage and actually present pieces | Break/remove a piece during exposure | LAB-JD1 |
| MK-EQP-007.R02 — Protection after the item is saved and reequipped | Save damaged armor and resume exposure | Integrity, protection and dose evolution | Do not turn a nominal description into a fixed protection rate | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: SC-LAB-014. No link constitutes approval.

### MK-EQP-008 — Weapons and projectiles

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-EQP-008.R01 — Electric Bow and real projectile | Draw/fire at targets and obstacles with limited energy | Spawn, trajectory, hit, damage and charge | Canceled shot, invulnerable target and no energy | LAB-JD1 |
| MK-EQP-008.R02 — Flamethrower and environmental effects | Activate modes on entities and blocks in scratch | Fuel, flame, world fire and damage | Water, obstruction, griefing rule and out-of-range target | LAB-JS |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. Do not replace natural spawn/generation with summon or injected blocks.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-EQP-009 — Robit

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-EQP-009.R01 — Robit follow, pickup and return home | Place robit/chargepad and toggle modes through the player | Position, owner, inventory, energy and navigation target | Removed home, distant owner and full inventory | LAB-JD1 |
| MK-EQP-009.R02 — Robit and inventory after despawn/restart | Unload the chunk, restart and collect the entity where supported | Identity, home, skins, items and persisted data | Death and item collection may have distinct contracts | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-UI-001 — Machine menus

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-UI-001.R01 — Menus as server transactions | Execute normal/shift/drag clicks in the real menu | Slots, carried stack, data values and world result | Invalid click, closed menu and removed block | LAB-JD1 |
| MK-UI-001.R02 — Two interfaces on the same machine | Open the machine with two viewers and change settings/process | Both views, authority and update sequence | Client receives old data or opens during unload | LAB-JD2 |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. At least two clients; use three for owner/trusted/stranger where required.

Related vertical seeds: SC-LAB-001, SC-LAB-002, SC-LAB-013, SC-LAB-018. No link constitutes approval.

### MK-UI-002 — HUD, tooltips and modes

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-UI-002.R01 — State-dependent HUD and tooltips | Vary real energy/module state and render HUD/tooltips | Semantic value, text/localization and drawing | Locale and scale without truncating required information | LAB-JD1 |
| MK-UI-002.R02 — Keys and radial menu change the real mode | Trigger keybind/radial/sneak-scroll according to support | Selection, feedback and subsequent server effect | Input conflict and text focus in a menu | LAB-JD1 |

Proof limit: An isolated visual comparison does not prove game logic. Do not use commands/admin to perform the Act intended to qualify as interaction.

Related vertical seeds: SC-LAB-015. No link constitutes approval.

### MK-UI-003 — Rendering and particles

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-UI-003.R01 — Machine geometry and visual state | Render active/inactive states and moving parts in the real client | Models, layers, gauges, light and transformations | Different camera/occlusion does not hide a missing model | LAB-JD1 |
| MK-UI-003.R02 — Particles without simulation effects | Enable/disable effects and compare rendering/simulation | Per-region events and particles with equivalent world state | Visual reduction does not change machine consumption or RNG | LAB-JD1 |

Proof limit: An isolated visual comparison does not prove game logic.

Related vertical seeds: SC-LAB-017. No link constitutes approval.

### MK-UI-004 — Audio and alarms

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-UI-004.R01 — Sound start, end and position | Activate machine/alarm and change the activity condition | Events, position, observed duration and captured audio | The loop must follow lifecycle including unload and restart | LAB-JD2 |
| MK-UI-004.R02 — Muffling and audio options | Player installs the upgrade and changes volume configuration | Events and audible amplitude/route according to profile | No device means audio is not qualified; HORIZON_NO_AUDIO separates the no-audio scenario | LAB-JD1 |

Proof limit: Audio requires an explicit profile with isolated test output; no-audio does not qualify sound. Do not use commands/admin to perform the Act intended to qualify as interaction.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-MAC-025 — Antiprotonic Nucleosynthesizer

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`. Audit key: `block.mekanism.antiprotonic_nucleosynthesizer`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-MAC-025.R01 — Nucleosynthesis with an effective recipe | Supply item, chemical and energy accepted by the nucleosynthesizer | Inputs, progress, output and consumption | Absent recipe, insufficient reagent and full output | LAB-JD1 |
| MK-MAC-025.R02 — Nucleosynthesis continuity | Save/restart during the cycle and resume | Progress, reserves and next production | Do not reuse SPS as proof of this different machine | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-WOR-007 — Cardboard Box

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`. Audit key: `block.mekanism.cardboard_box`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-WOR-007.R01 — Box and restore a block with data | Use a Cardboard Box on a block with a block entity and reopen | Original block/state, data, inventory and box consumption | Unboxable target, owner and special component | LAB-JD1 |
| MK-WOR-007.R02 — Transport an occupied box | Collect the box, save as an item, replace and unbox | Payload preserved, identity and contents without duplication | The same box referenced in two slots does not imply two entities | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-ENE-006 — Chargepad

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`. Audit key: `block.mekanism.chargepad`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-ENE-006.R01 — Chargepad with player and equipment | Player steps onto a chargepad with chargeable items | Pad energy, items actually charged and expenditure | Inventory full of eligible items and limited charge | LAB-JD1 |
| MK-ENE-006.R02 — Chargepad and Robit home | Place a robit on the pad and interrupt/restore supply | Home, energy, availability and subsequent behavior | Home removed while the robit returns | LAB-JS |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-UI-005 — Dictionary

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`. Audit key: `item.mekanism.dictionary`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-UI-005.R01 — Dictionary on a block, item and entity | Use the Dictionary on supported targets and query tags | Target type, IDs/tags and displayed text | Target without tags and resource reload changing members | LAB-JD1 |
| MK-UI-005.R02 — Dictionary and namespace after locale change | Change locale and repeat the query on the same target | Semantic identity and localized presentation | A translated name must not replace a tag ID | LAB-JD1 |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. An isolated visual comparison does not prove game logic.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-EQP-010 — HDPE Reinforced Elytra

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`. Audit key: `item.mekanism.hdpe_elytra`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-EQP-010.R01 — Flight with HDPE Reinforced Elytra | Equip the item and perform real glide/takeoff/landing | Movement, flight state, durability and damage | No item, broken item and change during flight | LAB-JD1 |
| MK-EQP-010.R02 — Repair, components and flight after reload | Repair/save the item and repeat flight | Durability, components and real effects | Do not inherit vanilla elytra rules without comparison | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-REG-005 — Player advancements and criteria

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`. Audit key: `advancements.mekanism.root.title`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-REG-005.R01 — Advancements through a concrete action | Perform discovered trigger actions with a new player | Granted advancements, criteria and client effect | Creating an item through a command does not prove a craft/use trigger | LAB-JD1 |
| MK-REG-005.R02 — Criteria and progress after restart | Save a player with partial criteria and complete them | Persisted progress and one-time grant | Another player does not inherit criteria through a confused alias | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-REG-006 — Commands and permissions

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`. Audit key: `command.mekanism.build.built`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-REG-006.R01 — Registered commands and permissions | Enumerate the command tree and execute allowed/rejected commands | Tree, arguments, feedback and world/player effects | Console versus player and op-level in separate profiles | LAB-JS |
| MK-REG-006.R02 — Debug tools do not mask the scenario | Use build/testrules commands only in declared setup | Gamerules, world and recorded effective fixture | Admin setup does not prove functional player construction | LAB-JD1 |

Proof limit: A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player. Do not use commands/admin to perform the Act intended to qualify as interaction.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-WOR-008 — Retrogen in existing chunks

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`. Audit key: `configuration.mekanism.world.retrogen`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-WOR-008.R01 — Retrogen in existing chunks | Generate an old region, change profile and request supported retrogen | Queue, changed chunks and added resources | Disabled config, unavailable chunk and repeated execution of the same request | LAB-JS |
| MK-WOR-008.R02 — Retrogen interrupted and restarted | Save/restart while declared regeneration work is pending | Progress, queue and final changes | Do not assume idempotence or safety from destructive changes without observation | LAB-JREC |

Proof limit: Do not replace natural spawn/generation with summon or injected blocks. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-REG-007 — Effective configuration and scoped reload

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanism/assets/mekanism/lang/en_us.json). Status: `not_observed`; runtime registration `pending`. Audit key: `configuration.mekanism.general.cardboard.strict_unboxing`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-REG-007.R01 — Effective configuration and reload policy | Export startup/server/client configs and change a valid option | Post-load value, scope and application time | Startup-only not treated as hot reload | LAB-JS |
| MK-REG-007.R02 — Server-synchronized configuration | Connect clients with differing local configs and observe the affected action | Authoritative configuration, effect and client information | A client does not choose a server value through its local file | LAB-JD2 |

Proof limit: A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player. At least two clients; use three for owner/trusted/stranger where required.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

## Generators module

### MK-GEN-001 — Heat Generator

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanismgenerators/assets/mekanismgenerators/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-GEN-001.R01 — Generation with fuel and thermal environment | Supply fuel and vary declared lava/environment | Fuel, heat, production, buffer and delivered energy | Full output and unaccepted fuel/remainder | LAB-JS |
| MK-GEN-001.R02 — Charge a consumer through the generator | Build generator-cable-cube through the player and operate it | Consumption and production at terminals and UI | Fixture-injected energy does not replace this case | LAB-JD1 |

Proof limit: Do not replace natural spawn/generation with summon or injected blocks. Do not use commands/admin to perform the Act intended to qualify as interaction.

Related vertical seeds: SC-LAB-021. No link constitutes approval.

### MK-GEN-002 — Solar generators

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanismgenerators/assets/mekanismgenerators/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-GEN-002.R01 — Solar production by condition and variant | Compare solar/advanced with day/night/weather/obstruction | Solar condition, production and visual state | Alternative environment/dimension and a block covering the panel | LAB-JS |
| MK-GEN-002.R02 — Solar buffer and resumption | Save during a solar-condition change and reopen | Energy, effective environment and next production | Do not produce offline ticks merely from wall-clock time | LAB-JREC |

Proof limit: Do not replace natural spawn/generation with summon or injected blocks. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-GEN-003 — Wind Generator

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanismgenerators/assets/mekanismgenerators/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-GEN-003.R01 — Wind by height and dimension | Position generators at selected heights/environments | Production, conditions and rotor states | Min/max height and skyless dimension according to the reference | LAB-JS |
| MK-GEN-003.R02 — Rotor and construction area | Player attempts to place/remove blocks in the occupied area | Placement validity, bounding blocks and production | Do not assume an obstruction rule solely from rendered geometry | LAB-JD1 |

Proof limit: Do not replace natural spawn/generation with summon or injected blocks. Do not use commands/admin to perform the Act intended to qualify as interaction.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-GEN-004 — Bio-Generator

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanismgenerators/assets/mekanismgenerators/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-GEN-004.R01 — Accepted biofuel and partial consumption | Insert loader-declared fuels into the generator | Fuel, buffer, remainders and energy | Full output and unaccepted input | LAB-JD1 |
| MK-GEN-004.R02 — Pending fuel after restart | Interrupt during burning and reopen | Reserved fuel, subsequent generation and inventory | Do not assume fluid fuel from old bioethanol text | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-GEN-005 — Gas-Burning Generator

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanismgenerators/assets/mekanismgenerators/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-GEN-005.R01 — Chemical fuel and variable load | Supply effective fuels and vary output demand | Consumed fuel, production and per-tick buffer | Incompatible fuel, residual quantity and full output | LAB-JD1 |
| MK-GEN-005.R02 — Gas depletion and refill | Save near depletion and refill | Remainder, expenditure and activation | Fuel type changed in a tank with residue | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-GEN-006 — Industrial Turbine

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanismgenerators/assets/mekanismgenerators/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-GEN-006.R01 — Functional turbine assembly | Build rotor/coils/vents/condensers and inject steam through a declared process | Formation, limits, flow rate, water and energy | Invalid height/rotor/components and saturated outputs | LAB-JD1 |
| MK-GEN-006.R02 — Turbine bottlenecks and dumping | Vary steam supply, condensation and dumping modes | Effective flow rate, observed efficiency and discarded resources | Coil/vent limit distinct from storage limit | LAB-JS |
| MK-GEN-006.R03 — Turbine rupture and rebuild | Remove an active part and restore after reload | Contents, identity and next rotation/production | Visual formation alone does not prove server validity | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-GEN-007 — Fission structure/control

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanismgenerators/assets/mekanismgenerators/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-GEN-007.R01 — Fission structure and controls | Assemble a reactor and change activation/burn through GUI and logic adapter | Formation, setpoints, active state and feedback | Invalid structure, out-of-range value and SCRAM during a tick | LAB-JD1 |
| MK-GEN-007.R02 — Concurrent reactor control | Two operators change configuration and signal in the same scenario | Applied order, current control and state on both clients | Permission and GUI opened before revocation | LAB-JD2 |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. At least two clients; use three for owner/trusted/stranger where required.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-GEN-008 — Fission thermal behavior/waste/failure

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanismgenerators/assets/mekanismgenerators/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-GEN-008.R01 — Fission coolant and waste | Supply fuel/coolant and block outputs in a controlled manner | Temperature, fuel/waste/coolant and per-tick damage | No coolant, full waste buffer and independent input losses | LAB-JS |
| MK-GEN-008.R02 — Failure and environmental effects | Induce failure conditions in a disposable world | Observed damage, structural change, explosion and radiation | Failure config enabled/disabled in separate profiles | LAB-JS |
| MK-GEN-008.R03 — Restart in a degraded reactor | Save under subcritical/near-failure conditions and continue | Thermal state, damage, resources and subsequent consequence | A checkpoint does not eliminate risk or duplicate waste | LAB-JREC |

Proof limit: Do not replace natural spawn/generation with summon or injected blocks. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-GEN-009 — Fusion Reactor

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanismgenerators/assets/mekanismgenerators/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-GEN-009.R01 — Fusion Reactor ignition | Build the structure and execute the ignition sequence with real resources | Ignition, plasma/casing heat, fuel and production | Incomplete ignition, insufficient laser and absent resource | LAB-JD1 |
| MK-GEN-009.R02 — Intermittent fuel and heat extraction | Alternate supply and the ability to extract energy/heat | Sustaining conditions and thermal evolution | Do not assume removing fuel causes an instantaneous stop | LAB-JS |
| MK-GEN-009.R03 — Hot fusion after restart | Save while active and resume with limited supply | Temperatures, active states and subsequent conversions | Formation and global state cannot come only from the client | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. A controlled fixture qualifies a rule; use actions require a companion in-game scenario with a player. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-GEN-010 — D-T chain

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanismgenerators/assets/mekanismgenerators/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-GEN-010.R01 — D-T chain with real conversions | Build processing for heavy water/brine and discovered fuels | All intermediates, recipes, losses and final product | Block an intermediate node and measure backpressure | LAB-JD1 |
| MK-GEN-010.R02 — D-T fuel and recipe identity | Reconcile recipes between Core and Generators | IDs, modes, ingredients and units | Missing modules and the same chemical name do not create a fictitious recipe | LAB-JS |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. A runtime census does not qualify in-game use; link objects to functional families.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-GEN-011 — Equipment generation modules

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanismgenerators/assets/mekanismgenerators/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-GEN-011.R01 — Generation modules by environment | Equip solar/geothermal units and vary exposure/submersion | Equipment energy and environment per tick | Night, partial depth and full equipment charge | LAB-JD1 |
| MK-GEN-011.R02 — Simultaneous suit generation and expenditure | Use consuming equipment with generator modules active | Charge, consumption, effects and synchronization | Combination with charge distribution does not permit inferring order | LAB-JD2 |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. At least two clients; use three for owner/trusted/stranger where required.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

## Tools module

### MK-TOL-001 — Materials and tools

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanismtools/assets/mekanismtools/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-TOL-001.R01 — Tools by material and block | Mine/attack with each discovered class and material | Tool rules, loot, speed, damage and durability | Incorrect tool, enchantment and configured attribute | LAB-JD1 |
| MK-TOL-001.R02 — Repair and identity after crafting | Repair/combine the tool through supported paths | Durability, cost, components and enchantments | Wrong repair material and excessive cost | LAB-JD1 |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction.

Related vertical seeds: SC-LAB-016. No link constitutes approval.

### MK-TOL-002 — Paxels

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanismtools/assets/mekanismtools/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-TOL-002.R01 — Paxel mining and alternative actions | Use on actionable blocks with normal/sneak interaction | Supported strip/path/till transformation, loot and durability | Do not impose all axe/shovel/hoe actions without characterization | LAB-JD1 |
| MK-TOL-002.R02 — Enchanted and configured paxel | Save/reopen a paxel with attributes/components and use it | Correct actions and cost with the persisted item | Same ID with a different enchantment treated as a distinct item | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: SC-LAB-016. No link constitutes approval.

### MK-TOL-003 — Armor

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanismtools/assets/mekanismtools/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-TOL-003.R01 — Mitigation and wear by piece/material | Equip pieces and receive damage types through real combat | Health, defense, toughness and durability | Partial set and a piece breaking on the same impact | LAB-JD1 |
| MK-TOL-003.R02 — Effective attributes by material config | Compare an item created under a distinct startup configuration | Loaded components and attributes | Do not confuse nominal stats with configured runtime stats | LAB-JS |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. A runtime census does not qualify in-game use; link objects to functional families.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-TOL-004 — Shields

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanismtools/assets/mekanismtools/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-TOL-004.R01 — Shield against attacks and projectiles | Defend in combat with controlled sources and angles | Blocking, damage, cooldown and wear | Unblockable attack and disabled shield | LAB-JD1 |
| MK-TOL-004.R02 — Shield decoration, repair and enchantment | Apply a banner and repair/enchant through real paths | Components, texture and durability | Invalid banner/recipe and incompatible material | LAB-JD1 |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-TOL-005 — Tools equipment in mob spawning

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanismtools/assets/mekanismtools/lang/en_us.json). Status: `not_observed`; runtime registration `pending`. Audit key: `configuration.mekanismtools.server.gear_spawn_chance`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-TOL-005.R01 — Equipment in natural mob spawning | Generate spawns with a configured profile and qualified RNG | Species, material, slot, enchantments and distribution | Summon with injected gear does not prove natural spawning | LAB-JS |
| MK-TOL-005.R02 — Spawn settings and difficulty | Compare chance/material/difficulty profiles | Effective equipment, frequency and applicable order | Zero/boundary values and explicit statistical hypotheses | LAB-JS |

Proof limit: Do not replace natural spawn/generation with summon or injected blocks.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-TOL-006 — Tools material reactions and effects

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanismtools/assets/mekanismtools/lang/en_us.json). Status: `not_observed`; runtime registration `pending`. Audit key: `advancements.mekanismtools.loved_by_piglins.title`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-TOL-006.R01 — Piglin reaction to equipped material | Approach piglins with a player wearing selected armor | AI state, hostility and effective piece | Removed piece and aggressive action changing the reaction | LAB-JD1 |
| MK-TOL-006.R02 — Other material-specific effects | Enumerate exposed properties and execute discovered effects | Light/protection/actions actually supported | An advancement key does not prove effect implementation | LAB-JS |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. Do not replace natural spawn/generation with summon or injected blocks.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

## Additions module

### MK-ADD-001 — Plastic families

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanismadditions/assets/mekanismadditions/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-ADD-001.R01 — Plastic blocks by color and variant | Place, walk on and mine discovered variants | States, collision, friction, light, loot and time | Normal/slick/road/glow are not equivalent merely because of color | LAB-JD1 |
| MK-ADD-001.R02 — Variant transparency and color | Render a fixed plastic layout under distinct lighting and faces | Layer, texture, occlusion and lighting | A visible block does not prove collision or friction | LAB-JD1 |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. An isolated visual comparison does not prove game logic.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-ADD-002 — Glow Panels and construction

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanismadditions/assets/mekanismadditions/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-ADD-002.R01 — Neighbor-dependent construction connections | Place fences/gates/stairs/slabs/panels and change neighbors | States, shape, supported waterlogging and light | Unsupported panel, reversed face and chunk boundary | LAB-JD1 |
| MK-ADD-002.R02 — Constructed states after reload | Save combined variants and reopen | Orientation, connection, waterlogging and subsequent loot | No state restored solely through an image | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-ADD-003 — Balloons

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanismadditions/assets/mekanismadditions/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-ADD-003.R01 — Tie, release and pop a balloon | Create balloons and attach to supported targets | Attachment, position, physics and item/entity lifecycle | Removed target, rupture and concurrent interaction | LAB-JD1 |
| MK-ADD-003.R02 — Balloon with an attachment outside the chunk | Unload target/balloon and reopen | Identity, attachment and subsequent movement | An old alias must not attach to a new target through a reused ID | LAB-JREC |

Proof limit: Do not use commands/admin to perform the Act intended to qualify as interaction. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: SC-LAB-017. No link constitutes approval.

### MK-ADD-004 — Baby mobs

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanismadditions/assets/mekanismadditions/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-ADD-004.R01 — Baby mobs by species and difficulty | Create/spawn effective species and observe real interaction | Hitbox, attributes, AI, attack and drops | Spawn rules and direct invocation are different levels | LAB-JS |
| MK-ADD-004.R02 — Baby mobs and goals after reload | Save an entity in pursuit/relevant state and continue | Identity, position, state and next action | A fixed seed does not replace per-process RNG qualification | LAB-JREC |

Proof limit: Do not replace natural spawn/generation with summon or injected blocks. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-ADD-005 — Obsidian TNT

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanismadditions/assets/mekanismadditions/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-ADD-005.R01 — Real Obsidian TNT detonation | Ignite through player/redstone and observe the explosion | Fuse, blocks, entities, damage and drops | Water, chain reaction and griefing in separate profiles | LAB-JS |
| MK-ADD-005.R02 — Primed TNT in a checkpoint | Save/reopen during the fuse and continue | Remaining fuse, position and a single explosion | Crash recovery separate from clean save | LAB-JREC |

Proof limit: Do not replace natural spawn/generation with summon or injected blocks. Distinguish clean save, crash recovery and chunk unload; compare continuity.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

### MK-ADD-006 — Walkie-Talkie

Nominal source: [pinned data](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/datagen/generated/mekanismadditions/assets/mekanismadditions/lang/en_us.json). Status: `not_observed`; runtime registration `pending`.

| Requirement | Characterization action | Required observables | Targeted boundary/failure | Minimum topology |
| --- | --- | --- | --- | --- |
| MK-ADD-006.R01 — Walkie-Talkie channel delivery | Real clients activate radios and send a test audio signal | Channel, transmission state and received audio | Same/other channel, receiver off and voice disabled | LAB-JD2 |
| MK-ADD-006.R02 — Radio reconnection and privacy | Disconnect/rejoin and change channel during transmission | Actual recipients and absence of improper delivery | No real user audio; known synthetic signal | LAB-JD2 |

Proof limit: Audio requires an explicit profile with isolated test output; no-audio does not qualify sound. At least two clients; use three for owner/trusted/stranger where required.

Related vertical seeds: none of the 22 seeds; its own expansion remains pending. No link constitutes approval.

## Optional integrations and census closure

CC:Tweaked, recipe viewers, scripts, HUDs, alternative slots and integrations with other systems use separate profiles. Build metadata only identifies audit targets; it does not prove an integration is loaded or works. Contracts must check APIs/capabilities, permissions, data changes and observed effects; external addons do not silently enter the 117 families. ([Pinned metadata](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/gradle.properties)).

Post-load/reload discovery will reconcile all registrations, recipes, modes, tiers, settings and non-registerable behaviors. Each orphan opens a requirement or justified exclusion. Progression and configuration span all four modules; a Core origin family does not attribute Generators, Tools or Additions data to Core.

