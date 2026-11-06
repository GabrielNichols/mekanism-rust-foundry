# Basic Enriching Factory with one blocked lane

`W1-PROC-01` is a proposed dedicated-server fixture for [issue #2](https://github.com/GabrielNichols/mekanism-rust-foundry/issues/2). It covers one administrative blocked-output case within [MK-MAC-010.R01/R02](INVENTORY.md#mk-mac-010--factories). Its two observation points describe hypotheses for Java characterization.

## Reference and setup

Use a fresh isolated Core world on Minecraft 1.21.1, NeoForge 21.1.200 and Mekanism 10.7.19.85. Verify the [pinned artifacts](../../cases/SOURCES.md) and record the effective config, enabled packs and loaded recipe. The [diagnostic catalog](../../evidence/core-catalog-bootstrap.json) records `mekanism:basic_enriching_factory` and the candidate processing recipe `mekanism:processing/iron/dust/from_ore`. Verify that the loaded factory accepts this recipe and that `minecraft:iron_ore` resolves through `c:ores/iron` to two `mekanism:dust_iron`. The factory crafting recipe is a separate binding.

Place one Basic Enriching Factory in a verified ticking chunk on the server. Keep auto-sort disabled and verify that only the two selected lanes contain inputs. Call the lanes A and B in this specification and keep their discovered runtime identities fixed throughout the run.

- **A, blocked.** Seed one iron ore and a full stack of the recipe output in its output slot. Record the verified stack limit and fixture-added count separately from production.
- **B, open.** Seed one iron ore and leave its output slot empty.
- **Shared FE.** Precharge through a verified NeoForge energy capability with enough stored FE for both lanes to finish under the effective factory config. Record the applied charge and starting buffer. Exclude external energy transfers, item transfers and any further fixture mutations during observation.

While frozen, verify the placed block, recipe, sorting state, lane stacks, zero progress in both lanes and the shared buffer. Require an independent inventory read through the mapped public handlers to corroborate the two output conditions. Advance the server at the serial 20 ticks/s baseline within a bounded tick budget agreed before the run.

## Two observation points

Capture each tick on the authoritative server thread. Record the world tick and phase, recipe identity, typed input and output stacks and progress for each lane, and the typed shared energy value in FE. Keep the two boundary observations below alongside the full trajectory.

| Boundary | Expected lane A | Expected lane B | Shared energy and witness |
| --- | --- | --- | --- |
| Blocked while B is active, at the first captured positive progress for B before completion | Input and full output unchanged. Progress remains at baseline. | Positive progress with one ore still present and output empty. | Record the buffer and debit since setup. Corroborate the blocked and open output states from the mapped handlers. |
| B completes, at its first observed output change | Input, full output and baseline progress unchanged. | One ore consumed and two dust produced. Record completion progress and its reset behavior as observed. | Record the ending buffer and total debit as starting FE minus ending FE. Preserve each per-tick buffer change. Check final lane stacks against independently decoded saved NBT after flushing. |

Stop as incomplete if the preconditions, typed per-lane progress, independent output witnesses or either boundary cannot be established. Retain the first observed divergence if Java contradicts a hypothesis. A shared buffer measures the total debit and cannot attribute it to individual lanes.

## Matched positive control

Repeat from a fresh world with the same pinned profile, bindings, lane inputs, starting FE and tick budget. The only setup difference is that lane A starts with an empty output. Independently verify that A advances and produces the expected output. This control distinguishes output blockage from an invalid lane binding, configuration or energy budget. Keep its evidence separate from the blocked run.

## Negative control

Use a copy of a complete trace and transfer B's positive progress to A at the first boundary. Keep tick, phase, inventories, total progress, shared FE and the entire completion observation unchanged. An unchanged copy should compare equal. The mutated copy should be rejected at the first boundary on the progress field for lane A or B, according to the comparator's field order. Record the exact path once the trace fields are bound. This is an offline control.

## Bindings required before execution

| Owner | Resolve and record |
| --- | --- |
| W1-CAT | Confirm the loaded factory and recipe, resolved input and output components, effective sorting and side configuration, stack limit, and the lane-to-handler and lane-to-NBT slot map. |
| W1-ENG | Bind stable lane identities, typed per-lane progress, the post-tick sampling phase, the independent inventory witness, and the FE receive side, capacity and buffer measurement. Establish the charge and tick budget from the pinned profile. |

The [factory observer](../../adapters/neoforge/src/dev/meklab/FactoryProbe.java) reads the typed `progress` int array and reconciles every inventory slot with the unsided NeoForge handler. The pinned layout maps lanes A, B and C to input/output pairs 0/1, 2/3 and 4/5. The fixture uses the upper FE capability and a shared starting buffer of 12,000 FE. The third lane stays empty. The runner independently reads the saved region after shutdown and compares the typed factory state with the last live snapshot.

## Run the paired cases

Prepare the pinned runtime with the current runner and bridge as described in the [contributor guide](../../CONTRIBUTING.md#setup). Choose new private output paths for each run.

```powershell
$runner = './target/debug/mek-lab.exe'
& $runner run-reference .lab/environment.json cases/factory-blocked.json .lab/evidence/factory-blocked-001
& $runner run-reference .lab/environment.json cases/factory-open.json .lab/evidence/factory-open-001
./adapters/neoforge/verify-factory.ps1 -Runner $runner -BlockedRun .lab/evidence/factory-blocked-001 -OpenRun .lab/evidence/factory-open-001 -ControlOutput .lab/evidence/factory-controls-001
```

The verification script checks the seals and matched reference configuration, reports both observation points and the total FE debit, and runs the original and progress-swap copies through the comparator. The cases cover administrative placement and output blocking. Menu interaction, player-driven blockage, energy contention, auto-sort behavior, upgrades, reload and parity are outside this slice.
