# Foundry characterization work packages

The six packages give one primary characterization package to all **117 mapped families** and their **253 requirement outlines**. They extend the [first-wave crosswalk](FIRST-WAVE-INVENTORY.md) without replacing [the canonical inventory](inventory.json), [requirements](requirements.json), [scenario designs](functional-scenarios.json), or the [roadmap](ROADMAP.md). Keep queue state and claims in the linked GitHub issues. The [first-wave workstreams](FIRST-WAVE-WORKSTREAMS.md#published-first-wave-issues) identify the existing issue numbers and shared ownership rules.

A family appears under one primary package. Cross-package dependencies still apply. Menu evidence for a factory, for example, belongs with the player interaction witness even when factory processing owns the recipe transition. The package IDs are stable grouping keys. Keep the canonical family, requirement and scenario IDs when opening or linking tasks.

## Evidence and promotion

- **Mapped** means the canonical inventory identifies a nominal family. Its entry is the starting point for the exact Java profile, object bindings and requirement split.
- **Runtime registration** means the diagnostic identified a loaded module or selected registry or recipe entry. This can narrow the fixture without establishing behavior.
- **Scenario design** means the family is associated with a proposed `SC-LAB` design. The [requirement mapping](requirements.json) specifies whether that design has reviewed partial links to individual requirements. An associated design alone gives no credit for those requirements.
- **Observed Java** means a linked executable case or campaign witnessed the stated action on the original runtime. The current table credits only the bounded Enrichment Chamber and associated slot/menu actions.
- **Simulation and controls** exercise fault detection, trace seals, observer effects, or lifecycle supervision. They support the evidence contract and carry no separate gameplay-family credit.
- **Reviewed and sealed reference** requires an explicit scoped review, pinned inputs, fresh observed outcomes, an independent witness, the expected negative control, and an intact seal. A sealed execution trace alone does not promote the reference. **Every package remains open at this gate.**
- **Future Rust parity** requires a qualifying Java scope and a separately exercised Rust candidate. It remains open for every family.

Core is the only mod profile in the catalog below. Generators, Tools and Additions retain nominal rows and require separately pinned runtime profiles. A catalog-backed ID narrows discovery for its object only. Unknown scope, recipe inputs, state domains, resources, hooks and configurations remain gaps. Use the serial 20 ticks/s profile as the reference. Admit accelerated or parallel evidence only when it reproduces the required scenario states and transitions and keep timing separate. [Profile templates](profiles.reference.template.json) · [Qualification status](../../STATUS.md) · [Evidence contract](../../VALIDATOR-CONTRACT.md)

| Key | Evidence | Boundary |
| --- | --- | --- |
| C | [Core runtime catalog](../../evidence/core-catalog-bootstrap.json) | Registration and selected recipe bindings. Diagnostic only. |
| P | [Enrichment processing case](../../cases/enrichment-processing.json) and [processing campaign](../../evidence/processing-campaign.json) | Bounded machine processing, FE supply, slot transfer and output. |
| U | [Causal client case](../../cases/client-causal-barrier.json) and [phase campaign](../../evidence/phase-chunk-controls.json) | Bounded hidden-client pickup and render barrier. |
| L | [Lifecycle campaign](../../evidence/lifecycle-campaign.json) | Bounded Enrichment Chamber persistence and next tick after reopen. |
| V | [Simulated observer faults](../../evidence/simulated-observer-faults.json), [live observer audit](../../evidence/live-observer-audit.json) and [trace seal controls](../../evidence/trace-seal-controls.json) | Validator/observer controls. The live audit runs Java and demonstrates fault detection, not additional machine coverage. |

A dash in a runtime or behavior cell means this crosswalk has no direct evidence for that family at that level. All rows are mapped. The gap column gives a first unresolved witness, not the full family acceptance criteria.

## Package index

| ID | Package | Families | Requirement outlines |
| --- | --- | ---: | ---: |
| `FP-PROC` | [Processing and factories](#fp-proc--processing-and-factories) | 29 | 62 |
| `FP-ENERGY` | [Energy and generation](#fp-energy--energy-and-generation) | 13 | 27 |
| `FP-TRANS` | [Transport and chemicals](#fp-trans--transport-and-chemicals) | 17 | 37 |
| `FP-LIFE` | [Multiblocks and lifecycle](#fp-life--multiblocks-and-lifecycle) | 16 | 38 |
| `FP-UI` | [Menus and player interactions](#fp-ui--menus-and-player-interactions) | 28 | 58 |
| `FP-RUNTIME` | [Shared runtime capabilities](#fp-runtime--shared-runtime-capabilities) | 14 | 31 |

## FP-PROC · Processing and factories

- **Scope.** Single-machine input/recipe/energy/output transitions, then shared-budget and blocked-lane factory behavior. Separate recipe selection from recipe execution.
- **Dependencies.** Core post-load registry and recipe bindings, typed energy and slot witnesses from FP-RUNTIME.
- **Acceptance.** One independently witnessed state trajectory with input, progress, output and energy conservation. Preserve finite energy and administrative output controls from the pilot.
- **Negative control.** An incompatible input or a blocked lane cannot advance progress or silently consume input.

Independent starter contributions

- `FP-PROC-S01` — Compare one Crusher recipe key and its runtime type/serializer with the static archive. Record the remaining input-tag gap.
- `FP-PROC-S02` — Draft a two-lane Basic Enriching Factory fixture and one blocked lane boundary without changing the runner.

| Family | Profile | Requirement IDs | Scenario designs | Runtime registration | Observed Java | First gap |
| --- | --- | --- | --- | --- | --- | --- |
| `MK-REG-003` Tiers and installers | Core | `MK-REG-003.R01`, `MK-REG-003.R02` | — | — | — | Bind and observe tier, slots, capacity. |
| `MK-REG-004` Upgrades | Core | `MK-REG-004.R01`, `MK-REG-004.R02`, `MK-REG-004.R03` | `SC-LAB-006` | — | — | Bind and observe count, limit, measured effects. |
| `MK-IO-002` Redstone control | Core | `MK-IO-002.R01`, `MK-IO-002.R02` | `SC-LAB-005`, `SC-LAB-022` | — | — | Bind and observe input/output signal, activation tick. |
| `MK-MAC-001` Enrichment Chamber | Core | `MK-MAC-001.R01`, `MK-MAC-001.R02` | `SC-LAB-001`, `SC-LAB-002`, `SC-LAB-003`, `SC-LAB-004`, `SC-LAB-005`, `SC-LAB-006`, `SC-LAB-007`, `SC-LAB-018`, `SC-LAB-021` | C · enrichment_chamber | P · processing and FE; U · placement and pickup; L · reopen | Broaden recipes, invalid input, redstone and player-driven output blockage. |
| `MK-MAC-002` Crusher | Core | `MK-MAC-002.R01`, `MK-MAC-002.R02` | — | C · crusher and one crushing recipe | — | Observe recipe selection, work cost and boundary rejection. |
| `MK-MAC-003` Energized Smelter | Core | `MK-MAC-003.R01`, `MK-MAC-003.R02` | — | — | — | Bind and observe recipe and time/tick. |
| `MK-MAC-004` Metallurgic Infuser | Core | `MK-MAC-004.R01`, `MK-MAC-004.R02` | — | C · metallurgic_infuser | — | Bind infusion type and observe input, infusion debit and output. |
| `MK-MAC-005` Osmium Compressor | Core | `MK-MAC-005.R01`, `MK-MAC-005.R02` | — | — | — | Bind and observe chemical consumption and output. |
| `MK-MAC-006` Combiner | Core | `MK-MAC-006.R01`, `MK-MAC-006.R02` | — | — | — | Bind and observe two inputs and result. |
| `MK-MAC-007` Purification Chamber | Core | `MK-MAC-007.R01`, `MK-MAC-007.R02` | — | — | — | Bind and observe chemical, progress and output. |
| `MK-MAC-008` Chemical Injection Chamber | Core | `MK-MAC-008.R01`, `MK-MAC-008.R02` | — | — | — | Bind and observe chemical, progress and output. |
| `MK-MAC-009` Precision Sawmill | Core | `MK-MAC-009.R01`, `MK-MAC-009.R02` | — | — | — | Bind and observe primary/secondary product, observed RNG. |
| `MK-MAC-010` Factories | Core | `MK-MAC-010.R01`, `MK-MAC-010.R02`, `MK-MAC-010.R03` | — | C · basic_enriching_factory | — | Resolve lane slots, sorting mode and per-lane consumption. |
| `MK-MAC-011` Chemical Oxidizer | Core | `MK-MAC-011.R01`, `MK-MAC-011.R02` | — | — | — | Bind and observe item→chemical, energy/progress. |
| `MK-MAC-012` Chemical Infuser | Core | `MK-MAC-012.R01`, `MK-MAC-012.R02` | — | — | — | Bind and observe two chemical inputs and output. |
| `MK-MAC-013` Electrolytic Separator | Core | `MK-MAC-013.R01`, `MK-MAC-013.R02` | — | — | — | Bind and observe fluid, two chemical outputs, energy. |
| `MK-MAC-014` Rotary Condensentrator | Core | `MK-MAC-014.R01`, `MK-MAC-014.R02` | — | — | — | Bind and observe direction, fluids/chemicals and energy. |
| `MK-MAC-015` Chemical Dissolution Chamber | Core | `MK-MAC-015.R01`, `MK-MAC-015.R02` | — | — | — | Bind and observe item/reagent/slurry and energy. |
| `MK-MAC-016` Chemical Washer | Core | `MK-MAC-016.R01`, `MK-MAC-016.R02` | — | — | — | Bind and observe fluid, input and output slurry. |
| `MK-MAC-017` Chemical Crystallizer | Core | `MK-MAC-017.R01`, `MK-MAC-017.R02` | — | — | — | Bind and observe chemical→item and progress. |
| `MK-MAC-018` Pressurized Reaction Chamber | Core | `MK-MAC-018.R01`, `MK-MAC-018.R02`, `MK-MAC-018.R03` | — | — | — | Bind and observe item/fluid/chemical, two output classes. |
| `MK-MAC-019` Solar Neutron Activator | Core | `MK-MAC-019.R01`, `MK-MAC-019.R02` | — | — | — | Bind and observe chemical, light/time, progress. |
| `MK-MAC-020` Isotopic Centrifuge | Core | `MK-MAC-020.R01`, `MK-MAC-020.R02` | — | — | — | Bind and observe chemical→chemical and energy. |
| `MK-MAC-021` Pigments and painting | Core | `MK-MAC-021.R01`, `MK-MAC-021.R02`, `MK-MAC-021.R03` | — | — | — | Bind and observe color IDs, inputs/outputs, rates. |
| `MK-MAC-022` Nutritional Liquifier | Core | `MK-MAC-022.R01`, `MK-MAC-022.R02` | — | — | — | Bind and observe food, paste, energy and saturation. |
| `MK-MAC-023` Formulaic Assemblicator | Core | `MK-MAC-023.R01`, `MK-MAC-023.R02` | — | — | — | Bind and observe grid, formula, consumption and result. |
| `MK-MAC-024` Oredictionificator | Core | `MK-MAC-024.R01`, `MK-MAC-024.R02` | — | — | — | Bind and observe filters/tags and selected result. |
| `MK-ADD-001` Plastic families | Additions pending | `MK-ADD-001.R01`, `MK-ADD-001.R02` | — | — | — | Bind and observe states, shape, friction. |
| `MK-MAC-025` Antiprotonic Nucleosynthesizer | Core | `MK-MAC-025.R01`, `MK-MAC-025.R02` | — | — | — | Bind and observe Inputs, progress, output and consumption, Progress, reserves and next production. |

## FP-ENERGY · Energy and generation

- **Scope.** Capability accounting and bounded generator behavior. Start with direct Core energy transfer and a separate fuel-driven Generators profile.
- **Dependencies.** FP-RUNTIME profile fingerprint and typed energy accounting. Pin Generators before any generation claim.
- **Acceptance.** Independently account for generation, storage, accepted transfer and sink debit with exact units and sided constraints.
- **Negative control.** Remove fuel under the same conditions. Fuel-free or passive energy cannot count as fuel-driven production.

Independent starter contributions

- `FP-ENERGY-S01` — Draft a Core Basic Energy Cube side matrix using one confirmed registry ID and exact FE units.
- `FP-ENERGY-S02` — Draft the Heat Generator fueled and unfueled fixture, explicitly flagging the Generators artifact and environmental dependencies.

| Family | Profile | Requirement IDs | Scenario designs | Runtime registration | Observed Java | First gap |
| --- | --- | --- | --- | --- | --- | --- |
| `MK-ENE-001` Energy Cubes | Core | `MK-ENE-001.R01`, `MK-ENE-001.R02` | — | C · basic_energy_cube | — | Observe sided transfer, limits and accepted delivery. |
| `MK-ENE-002` Universal Cable | Core | `MK-ENE-002.R01`, `MK-ENE-002.R02`, `MK-ENE-002.R03` | `SC-LAB-008`, `SC-LAB-019`, `SC-LAB-021` | C · basic_universal_cable | — | Observe topology and source/sink balance before unload or split. |
| `MK-ENE-003` Charge and discharge items | Core | `MK-ENE-003.R01`, `MK-ENE-003.R02` | `SC-LAB-003` | — | — | Bind and observe item/block energy and observed loss. |
| `MK-ENE-005` Lasers | Core | `MK-ENE-005.R01`, `MK-ENE-005.R02` | — | — | — | Bind and observe energy, beam, interaction and drops. |
| `MK-HEA-001` Thermal conduction | Core | `MK-HEA-001.R01`, `MK-HEA-001.R02` | — | — | — | Bind and observe temperature, thermal energy, environment. |
| `MK-HEA-002` Heaters | Core | `MK-HEA-002.R01`, `MK-HEA-002.R02` | — | — | — | Bind and observe fuel, energy and temperature. |
| `MK-GEN-001` Heat Generator | Generators pending | `MK-GEN-001.R01`, `MK-GEN-001.R02` | `SC-LAB-021` | — | — | Bind and observe fuel/lava, heat and energy. |
| `MK-GEN-002` Solar generators | Generators pending | `MK-GEN-002.R01`, `MK-GEN-002.R02` | — | — | — | Bind and observe production and exposure. |
| `MK-GEN-003` Wind Generator | Generators pending | `MK-GEN-003.R01`, `MK-GEN-003.R02` | — | — | — | Bind and observe production, height and obstruction. |
| `MK-GEN-004` Bio-Generator | Generators pending | `MK-GEN-004.R01`, `MK-GEN-004.R02` | — | — | — | Bind and observe fuel, buffer and production. |
| `MK-GEN-005` Gas-Burning Generator | Generators pending | `MK-GEN-005.R01`, `MK-GEN-005.R02` | — | — | — | Bind and observe chemical fuel and energy/tick. |
| `MK-GEN-010` D-T chain | Generators pending | `MK-GEN-010.R01`, `MK-GEN-010.R02` | — | — | — | Bind and observe IDs, recipes, gases/fluids and quantities. |
| `MK-ENE-006` Chargepad | Core | `MK-ENE-006.R01`, `MK-ENE-006.R02` | — | — | — | Bind and observe Pad energy, items actually charged and expenditure, Home, energy, availability and subsequent behavior. |

## FP-TRANS · Transport and chemicals

- **Scope.** Chemical/fluid and item transport on small, isolated topologies. Establish identity and conservation before branch, merge, split or cross-dimensional claims.
- **Dependencies.** FP-RUNTIME chemical identity and endpoint bindings. Coordinate cable and power transport with FP-ENERGY.
- **Acceptance.** Account for source, in-flight state and sink using one topology and one typed resource. Record routing and incompatibility witnesses.
- **Negative control.** An incompatible sink leaves the amount and type conserved and exposes a distinct no-transfer outcome.

Independent starter contributions

- `FP-TRANS-S01` — Draft a straight one-chemical Basic Pressurized Tube line and incompatible endpoint using the confirmed block ID.
- `FP-TRANS-S02` — Specify an item sorter with one full destination and two independent route observations.

| Family | Profile | Requirement IDs | Scenario designs | Runtime registration | Observed Java | First gap |
| --- | --- | --- | --- | --- | --- | --- |
| `MK-IO-001` Side configuration | Core | `MK-IO-001.R01`, `MK-IO-001.R02` | `SC-LAB-007`, `SC-LAB-022` | — | — | Bind and observe logical/physical face, permissions, transfers. |
| `MK-CHE-001` Chemical types and attributes | Core | `MK-CHE-001.R01`, `MK-CHE-001.R02` | — | C · registry count only | — | Bind chemical IDs and verify attributes, units and conversions. |
| `MK-CHE-002` Chemical Tanks | Core | `MK-CHE-002.R01`, `MK-CHE-002.R02` | `SC-LAB-009` | — | — | Bind and observe contents, capacity, side. |
| `MK-CHE-003` Pressurized Tubes | Core | `MK-CHE-003.R01`, `MK-CHE-003.R02` | `SC-LAB-009` | C · basic_pressurized_tube and crafting recipe | — | Observe route, incompatible endpoint and per-terminal conservation. |
| `MK-CHE-004` Gauge Dropper | Core | `MK-CHE-004.R01`, `MK-CHE-004.R02` | `SC-LAB-009` | — | — | Bind and observe item/tank contents; transferred quantity. |
| `MK-FLU-001` Fluid Tanks | Core | `MK-FLU-001.R01`, `MK-FLU-001.R02` | — | — | — | Bind and observe fluid, quantity, components. |
| `MK-FLU-002` Mechanical Pipes | Core | `MK-FLU-002.R01`, `MK-FLU-002.R02` | — | — | — | Bind and observe topology and fluid balance. |
| `MK-FLU-003` Electric Pump | Core | `MK-FLU-003.R01`, `MK-FLU-003.R02` | — | — | — | Bind and observe affected liquid blocks, volume, energy. |
| `MK-FLU-004` Fluidic Plenisher | Core | `MK-FLU-004.R01`, `MK-FLU-004.R02` | — | — | — | Bind and observe placed blocks, volume and energy. |
| `MK-NET-001` Logistical Transporter | Core | `MK-NET-001.R01`, `MK-NET-001.R02`, `MK-NET-001.R03` | `SC-LAB-010` | C · basic_logistical_transporter and crafting recipe | — | Observe path, item state and full-destination handling. |
| `MK-NET-002` Logistical Sorter | Core | `MK-NET-002.R01`, `MK-NET-002.R02` | `SC-LAB-010` | — | — | Bind and observe filter order, quantities and destinations. |
| `MK-NET-003` Quantum Entangloporter | Core | `MK-NET-003.R01`, `MK-NET-003.R02`, `MK-NET-003.R03` | `SC-LAB-020` | — | — | Bind and observe frequency, buffers and per-resource transfer. |
| `MK-NET-004` Network Reader | Core | `MK-NET-004.R01`, `MK-NET-004.R02` | — | — | — | Bind and observe displayed information and authoritative state. |
| `MK-NET-005` Configure connections | Core | `MK-NET-005.R01`, `MK-NET-005.R02` | `SC-LAB-008`, `SC-LAB-019` | — | — | Bind and observe faces, color and topology after interaction. |
| `MK-QIO-001` Drive Array and drives | Core | `MK-QIO-001.R01`, `MK-QIO-001.R02`, `MK-QIO-001.R03` | `SC-LAB-012` | — | — | Bind and observe types/counts, limit, serialization. |
| `MK-QIO-003` Import/Export | Core | `MK-QIO-003.R01`, `MK-QIO-003.R02` | `SC-LAB-012` | — | — | Bind and observe filters and transferred quantities. |
| `MK-QIO-004` Redstone Adapter | Core | `MK-QIO-004.R01`, `MK-QIO-004.R02` | — | — | — | Bind and observe count and signal. |

## FP-LIFE · Multiblocks and lifecycle

- **Scope.** Formation and invalidation, saved membership and contents, and restart/recovery semantics. Keep generic chunk controls separate from machine-specific lifecycle evidence.
- **Dependencies.** FP-RUNTIME process isolation, ownership and persistence witnesses. Energy or chemical hazards need separate typed ledgers.
- **Acceptance.** Witness formation, one invalidation, save/close/reopen and one post-reopen behavior step through independent typed evidence.
- **Negative control.** Remove one required block or terminate before flush. Unformed or unsaved state must remain distinguishable from restored state.

Independent starter contributions

- `FP-LIFE-S01` — Draft a small Dynamic Tank geometry with one invalid boundary block and named witness points.
- `FP-LIFE-S02` — Compare the recorded Enrichment Chamber restart sequence with one requirement and document the wider lifecycle gap.

| Family | Profile | Requirement IDs | Scenario designs | Runtime registration | Observed Java | First gap |
| --- | --- | --- | --- | --- | --- | --- |
| `MK-ENE-004` Induction Matrix | Core | `MK-ENE-004.R01`, `MK-ENE-004.R02`, `MK-ENE-004.R03` | — | — | — | Bind and observe formation, storage, throughput. |
| `MK-HEA-003` Thermoelectric Boiler | Core | `MK-HEA-003.R01`, `MK-HEA-003.R02`, `MK-HEA-003.R03` | — | — | — | Bind and observe formation, fluids/chemicals, heat. |
| `MK-HEA-004` Thermal Evaporation | Core | `MK-HEA-004.R01`, `MK-HEA-004.R02` | — | — | — | Bind and observe temperature, input/output, formation. |
| `MK-STO-001` Bins | Core | `MK-STO-001.R01`, `MK-STO-001.R02` | — | — | — | Bind and observe type, count, capacity. |
| `MK-STO-002` Personal Chest/Barrel | Core | `MK-STO-002.R01`, `MK-STO-002.R02` | `SC-LAB-011` | — | — | Bind and observe slots, owner, permissions and contents. |
| `MK-STO-003` Dynamic Tank | Core | `MK-STO-003.R01`, `MK-STO-003.R02` | `SC-LAB-013` | C · dynamic_tank and dynamic_valve | — | Observe formation, contents, invalidation and reopen. |
| `MK-WOR-002` Dimensional Stabilizer | Core | `MK-WOR-002.R01`, `MK-WOR-002.R02` | `SC-LAB-019` | — | — | Bind and observe chunks tickets, energy and activity. |
| `MK-NUC-001` Waste Barrels | Core | `MK-NUC-001.R01`, `MK-NUC-001.R02` | — | — | — | Bind and observe quantity, observed decay and emissions. |
| `MK-NUC-002` Environmental and personal radiation | Core | `MK-NUC-002.R01`, `MK-NUC-002.R02`, `MK-NUC-002.R03` | `SC-LAB-014` | — | — | Bind and observe sources, dose, damage. |
| `MK-NUC-003` SPS | Core | `MK-NUC-003.R01`, `MK-NUC-003.R02` | — | — | — | Bind and observe formation, energy/chemical and antimatter. |
| `MK-GEN-006` Industrial Turbine | Generators pending | `MK-GEN-006.R01`, `MK-GEN-006.R02`, `MK-GEN-006.R03` | — | — | — | Bind and observe formation, flow rate, energy and water. |
| `MK-GEN-007` Fission structure/control | Generators pending | `MK-GEN-007.R01`, `MK-GEN-007.R02` | — | — | — | Bind and observe formation, burn rate and active state. |
| `MK-GEN-008` Fission thermal behavior/waste/failure | Generators pending | `MK-GEN-008.R01`, `MK-GEN-008.R02`, `MK-GEN-008.R03` | — | — | — | Bind and observe heat, coolant, waste. |
| `MK-GEN-009` Fusion Reactor | Generators pending | `MK-GEN-009.R01`, `MK-GEN-009.R02`, `MK-GEN-009.R03` | — | — | — | Bind and observe ignition, fuel, plasma/casing heat. |
| `MK-WOR-007` Cardboard Box | Core | `MK-WOR-007.R01`, `MK-WOR-007.R02` | — | — | — | Bind and observe Original block/state, data, inventory and box consumption, Payload preserved, identity and contents without duplication. |
| `MK-WOR-008` Retrogen in existing chunks | Core | `MK-WOR-008.R01`, `MK-WOR-008.R02` | — | — | — | Bind and observe Queue, changed chunks and added resources, Progress, queue and final changes. |

## FP-UI · Menus and player interactions

- **Scope.** Client/server action boundaries, real hidden-client menu and inventory effects, equipment/tool inputs and rendering. Check authoritative outcomes before presentation.
- **Dependencies.** FP-RUNTIME phase ownership and hidden-window launch. Load extra modules only in their own pinned profiles.
- **Acceptance.** Observe action ingress, authoritative server state, applied client revision and rendered barrier where relevant.
- **Negative control.** Capture before authoritative apply or use the wrong actor or screen. A predicted result must not satisfy the barrier.

Independent starter contributions

- `FP-UI-S01` — Annotate one real Enrichment Chamber menu action with sent, server-applied, client-applied and rendered witnesses.
- `FP-UI-S02` — Draft a separate shift/drag or late-join scenario without modifying existing causal fixtures.

| Family | Profile | Requirement IDs | Scenario designs | Runtime registration | Observed Java | First gap |
| --- | --- | --- | --- | --- | --- | --- |
| `MK-IO-003` Slots and automation | Core | `MK-IO-003.R01`, `MK-IO-003.R02` | `SC-LAB-002`, `SC-LAB-004`, `SC-LAB-018` | — | P · machine inventory; U · real pickup | Observe broader slot rejection, automated sides and late join. |
| `MK-IO-004` Copied configuration | Core | `MK-IO-004.R01`, `MK-IO-004.R02` | — | — | — | Bind and observe configuration before/after; contents unchanged. |
| `MK-QIO-002` Dashboards | Core | `MK-QIO-002.R01`, `MK-QIO-002.R02`, `MK-QIO-002.R03` | `SC-LAB-012` | — | — | Bind and observe query, sorting, crafting. |
| `MK-EQP-001` MekaSuit | Core | `MK-EQP-001.R01`, `MK-EQP-001.R02` | `SC-LAB-015` | — | — | Bind and observe energy, damage, movement. |
| `MK-EQP-002` Meka-Tool | Core | `MK-EQP-002.R01`, `MK-EQP-002.R02` | — | — | — | Bind and observe mode, area, speed. |
| `MK-EQP-003` Installable modules | Core | `MK-EQP-003.R01`, `MK-EQP-003.R02`, `MK-EQP-003.R03` | `SC-LAB-015` | — | — | Bind and observe installation, limit, conflicts and options. |
| `MK-EQP-004` Atomic Disassembler | Core | `MK-EQP-004.R01`, `MK-EQP-004.R02` | — | — | — | Bind and observe mode, energy, mining and attack. |
| `MK-EQP-005` Jetpacks | Core | `MK-EQP-005.R01`, `MK-EQP-005.R02` | `SC-LAB-015` | — | — | Bind and observe fuel, mode, position and velocity. |
| `MK-EQP-006` Scuba and Free Runners | Core | `MK-EQP-006.R01`, `MK-EQP-006.R02` | — | — | — | Bind and observe air, chemical, fall damage and energy. |
| `MK-EQP-007` Hazmat | Core | `MK-EQP-007.R01`, `MK-EQP-007.R02` | `SC-LAB-014` | — | — | Bind and observe protection and dose/damage. |
| `MK-EQP-008` Weapons and projectiles | Core | `MK-EQP-008.R01`, `MK-EQP-008.R02` | — | — | — | Bind and observe energy/fuel, projectile, hit and damage. |
| `MK-EQP-009` Robit | Core | `MK-EQP-009.R01`, `MK-EQP-009.R02` | — | — | — | Bind and observe inventory, owner, position. |
| `MK-UI-001` Machine menus | Core | `MK-UI-001.R01`, `MK-UI-001.R02` | `SC-LAB-001`, `SC-LAB-002`, `SC-LAB-013`, `SC-LAB-018` | C · selected menu types | U · bounded open, pickup and revision barrier | Observe shift/drag, multiple clients, gauges and comparable frames. |
| `MK-UI-002` HUD, tooltips and modes | Core | `MK-UI-002.R01`, `MK-UI-002.R02` | `SC-LAB-015` | — | — | Bind and observe text/keys, values, selection and actions. |
| `MK-UI-003` Rendering and particles | Core | `MK-UI-003.R01`, `MK-UI-003.R02` | `SC-LAB-017` | — | — | Bind and observe capture, events and active state. |
| `MK-UI-004` Audio and alarms | Core | `MK-UI-004.R01`, `MK-UI-004.R02` | — | — | — | Bind and observe events/position, activation and defined volume. |
| `MK-GEN-011` Equipment generation modules | Generators pending | `MK-GEN-011.R01`, `MK-GEN-011.R02` | — | — | — | Bind and observe equipment energy, heat/environment. |
| `MK-TOL-001` Materials and tools | Tools pending | `MK-TOL-001.R01`, `MK-TOL-001.R02` | `SC-LAB-016` | — | — | Bind and observe attributes, durability, tool rules and loot. |
| `MK-TOL-002` Paxels | Tools pending | `MK-TOL-002.R01`, `MK-TOL-002.R02` | `SC-LAB-016` | — | — | Bind and observe mining, alternative actions, durability. |
| `MK-TOL-003` Armor | Tools pending | `MK-TOL-003.R01`, `MK-TOL-003.R02` | — | — | — | Bind and observe defense/toughness/attributes/durability. |
| `MK-TOL-004` Shields | Tools pending | `MK-TOL-004.R01`, `MK-TOL-004.R02` | — | — | — | Bind and observe blocking, damage, durability and decoration. |
| `MK-ADD-003` Balloons | Additions pending | `MK-ADD-003.R01`, `MK-ADD-003.R02` | `SC-LAB-017` | — | — | Bind and observe attachment, position, physics and persistence. |
| `MK-ADD-005` Obsidian TNT | Additions pending | `MK-ADD-005.R01`, `MK-ADD-005.R02` | — | — | — | Bind and observe fuse, explosion, world and damage. |
| `MK-ADD-006` Walkie-Talkie | Additions pending | `MK-ADD-006.R01`, `MK-ADD-006.R02` | — | — | — | Bind and observe channel, activation and audio delivery. |
| `MK-UI-005` Dictionary | Core | `MK-UI-005.R01`, `MK-UI-005.R02` | — | — | — | Bind and observe Target type, IDs/tags and displayed text, Semantic identity and localized presentation. |
| `MK-EQP-010` HDPE Reinforced Elytra | Core | `MK-EQP-010.R01`, `MK-EQP-010.R02` | — | — | — | Bind and observe Movement, flight state, durability and damage, Durability, components and real effects. |
| `MK-TOL-005` Tools equipment in mob spawning | Tools pending | `MK-TOL-005.R01`, `MK-TOL-005.R02` | — | — | — | Bind and observe Species, material, slot, enchantments and distribution, Effective equipment, frequency and applicable order. |
| `MK-TOL-006` Tools material reactions and effects | Tools pending | `MK-TOL-006.R01`, `MK-TOL-006.R02` | — | — | — | Bind and observe AI state, hostility and effective piece, Light/protection/actions actually supported. |

## FP-RUNTIME · Shared runtime capabilities

- **Scope.** Registry/config/recipe discovery, reusable observation, security/frequency identity, world integration and evidence infrastructure. Verify input and runtime profile boundaries.
- **Dependencies.** Artifact locks, distinct profile directories, serial baseline and independent review of bridge/runner/schema changes.
- **Acceptance.** Produce a bounded effective observation or deterministic validator control with exact profiles, first rejection and retained evidence.
- **Negative control.** Remove a registry binding, drift a config hash, or omit a worker record and reject the first unsupported claim.

Independent starter contributions

- `FP-RUNTIME-S01` — Reconcile one existing Core registry row with the archived registration and its requirement.
- `FP-RUNTIME-S02` — Add one deterministic missing-worker or stale-evidence test without changing a machine fixture.

| Family | Profile | Requirement IDs | Scenario designs | Runtime registration | Observed Java | First gap |
| --- | --- | --- | --- | --- | --- | --- |
| `MK-REG-001` Static registries | Core | `MK-REG-001.R01`, `MK-REG-001.R02` | — | C · selected registries and modules | — | Bind block states, components, effective tags and non-Core modules. |
| `MK-REG-002` Recipes and data processing | Core | `MK-REG-002.R01`, `MK-REG-002.R02`, `MK-REG-002.R03` | — | C · selected recipe IDs, types and serializers | — | Resolve inputs and outputs, tag members and reload behavior. |
| `MK-SEC-001` Security Desk | Core | `MK-SEC-001.R01`, `MK-SEC-001.R02` | `SC-LAB-011` | — | — | Bind and observe owner, trusted list, mode and authorizations. |
| `MK-SEC-002` Frequencies | Core | `MK-SEC-002.R01`, `MK-SEC-002.R02` | `SC-LAB-011`, `SC-LAB-020` | — | — | Bind and observe identity, public/private and association. |
| `MK-WOR-001` Digital Miner | Core | `MK-WOR-001.R01`, `MK-WOR-001.R02`, `MK-WOR-001.R03` | `SC-LAB-022` | — | — | Bind and observe targets, filters, energy. |
| `MK-WOR-003` Teleporter and portable teleporter | Core | `MK-WOR-003.R01`, `MK-WOR-003.R02`, `MK-WOR-003.R03` | — | — | — | Bind and observe destination, energy, entities and cooldown. |
| `MK-WOR-004` Seismic Vibrator/Reader | Core | `MK-WOR-004.R01`, `MK-WOR-004.R02` | — | — | — | Bind and observe map/query, energy and displayed data. |
| `MK-WOR-005` Ores and generation | Core | `MK-WOR-005.R01`, `MK-WOR-005.R02` | — | — | — | Bind and observe registered features, blocks and distribution. |
| `MK-WOR-006` Materials and drops | Core | `MK-WOR-006.R01`, `MK-WOR-006.R02` | `SC-LAB-016` | — | — | Bind and observe tags, recipes and loot. |
| `MK-ADD-002` Glow Panels and construction | Additions pending | `MK-ADD-002.R01`, `MK-ADD-002.R02` | — | — | — | Bind and observe orientation, connection, light. |
| `MK-ADD-004` Baby mobs | Additions pending | `MK-ADD-004.R01`, `MK-ADD-004.R02` | — | — | — | Bind and observe spawn, AI, attributes. |
| `MK-REG-005` Player advancements and criteria | Core | `MK-REG-005.R01`, `MK-REG-005.R02` | — | — | — | Bind and observe Granted advancements, criteria and client effect, Persisted progress and one-time grant. |
| `MK-REG-006` Commands and permissions | Core | `MK-REG-006.R01`, `MK-REG-006.R02` | — | — | — | Bind and observe Tree, arguments, feedback and world/player effects, Gamerules, world and recorded effective fixture. |
| `MK-REG-007` Effective configuration and scoped reload | Core | `MK-REG-007.R01`, `MK-REG-007.R02` | — | C · generated config file hashes | — | Read effective values, scopes and client synchronization across reload. |

## Shared integration gates

Keep per-package fixtures and outputs isolated. Coordinate edits to the bridge, runner, scheduler and evidence schema with the integrator. Design work can proceed while bindings are open. Real Java execution requires the exact module and applicable topology, a typed witness and the negative control. Preserve every attempt and retain private artifacts outside the public tree. A finite reference review decides whether the pre-port gate for that scope is satisfied.

[Contributor guide](../../CONTRIBUTING.md) · [Scenario designs and topologies](SCENARIOS.md) · [First-wave issue links](FIRST-WAVE-WORKSTREAMS.md#published-first-wave-issues)
