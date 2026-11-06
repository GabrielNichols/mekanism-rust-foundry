# Dynamic Tank formation, break and reopen

`W1-LIFE-01` is a proposed dedicated-server fixture for [issue #5](https://github.com/GabrielNichols/mekanism-rust-foundry/issues/5). It maps to `W1-INV-013` and [MK-STO-003.R01/R02](INVENTORY.md#mk-sto-003--dynamic-tank). The administrative fixture leaves the two-client construction and menu witnesses of [SC-LAB-013](SCENARIOS.md#sc-lab-013--multiblock-forms-and-invalidates-through-real-construction) to that scenario.

## Reference and blueprint

Use a fresh private Core overworld on Minecraft 1.21.1, NeoForge 21.1.200 and Mekanism 10.7.19.85. Record the [artifact lock](../../cases/SOURCES.md), enabled packs and effective tank configuration. The [Core catalog](../../evidence/core-catalog-bootstrap.json) lists `mekanism:dynamic_tank` and `mekanism:dynamic_valve` as loaded blocks. The pinned source supplies the candidate minimum dimensions and distinguishes shell, frame and valve positions. [Tank validator](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/main/java/mekanism/common/content/tank/TankValidator.java#L10-L21) · [Cuboid validator](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/main/java/mekanism/common/lib/multiblock/CuboidStructureValidator.java)

Place the 3 × 3 × 3 shell at inclusive world bounds `(4, 80, 4)` through `(6, 82, 6)` inside the verified ticking chunk `(0, 0)`. Each layer below has three rows in increasing `z` order, with columns in increasing `x` order.

| Layer | Rows |
| --- | --- |
| `y = 82` | `DDD / DDD / DDD` |
| `y = 81` | `DDD / G.V / DDD` |
| `y = 80` | `DDD / DDD / DDD` |

`D` and `G` are Dynamic Tank blocks, `V` is one Dynamic Valve and `.` is air. The closed shell has 25 tank blocks and one valve. The valve at `(6, 81, 5)` has candidate exterior face east (`+x`). `G` at `(4, 81, 5)` is the closing block and the only block removed by the control. Keep adjacent multiblocks and external fluid connections out of the fixture.

## Checkpoints and missing-block control

Use the serial 20 ticks/s profile. Fix the post-tick sampling phase and recheck budget before the run. Record the block changes separately from the multiblock observations. At each formed checkpoint capture the bounds, exact membership, valve face, live inventory UUID, fluid ID and components, amount and capacity in mB. Record applied transfers separately from stored fluid.

| Checkpoint | Required observation |
| --- | --- |
| Close the shell | Witness `G` as air and the tank as unformed. Place `G` through the declared administrative action, verify the new block state, then witness formation with the expected bounds, membership and valve. Through the verified exterior fluid handler, fill the empty tank with 1,000 mB of `minecraft:water`. Require the full amount to be accepted and observe the same stored amount before saving. |
| Save the occupied tank | Flush and stop cleanly. Decode the closed world's chunk block states and block entity NBT, and join the valve's saved inventory UUID to the matching fluid record in the manager's saved data. Require the persisted shell and typed water stack to agree with the live witness. Preserve this checkpoint and copy it for the two branches while the world is closed. |
| Reopen the intact copy | Start a fresh JVM, verify reconstruction and compare the bounds, membership, valve, capacity and 1,000 mB stack with the saved checkpoint. Record the old and new inventory UUIDs and link them through the saved cache and shell. Drain 250 mB through the valve, require exactly 250 mB out and 750 mB remaining, and flush and corroborate the new amount on disk. |
| Break and reopen the control copy | After the same initial reconstruction, replace only `G` with air. Witness the removal, then require an unformed tank and a freshly queried valve without usable fluid access. Flush and stop, corroborate the gap from the saved chunk, and reopen this broken copy in another JVM. Require the gap and unformed state again within the bound. Restore only `G` and record the next formation, identity and contents as characterization. |

The control checks the formed state after the observed removal and recheck. A still-formed tank fails at that field. An unformed tank can retain a cache, so treat the amount after repair as an observation. The pinned formation code can allocate a fresh inventory UUID when rebuilding a structure. Use the UUID to join the records within each checkpoint, and use the shell and saved cache to relate checkpoints. [Formation protocol](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/main/java/mekanism/common/lib/multiblock/FormationProtocol.java#L86-L106)

## Bindings required before execution

| Owner | Resolve and record |
| --- | --- |
| W1-CAT | Confirm the block and block entity IDs, the east-sided fluid capability, effective fluid capacity/config and loaded pack hashes. Confirm the handler accepts at least 1,000 mB of the selected water stack in the closed geometry. |
| W1-ENG | Bind the applied block-change witness, formed state, membership and post-tick recheck window. Bind the saved chunk palette and block entity UUID plus the independent typed decoder for the candidate `data/mekanism_dynamictank.dat` cache. Record save and clean-exit receipts, checkpoint digests and distinct JVM identities. [Multiblock tile persistence](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/main/java/mekanism/common/tile/prefab/TileEntityMultiblock.java#L312-L327) · [Manager cache persistence](https://github.com/mekanism/Mekanism/blob/a00109e4856fd38b9c5b3dd7f22ce4a59cd65a80/src/main/java/mekanism/common/lib/multiblock/MultiblockManager.java#L197-L228) |

Classify missing witnesses, an unproven initial formation or an unapplied removal as incomplete. Geometry, valve behavior and restart outcomes remain provisional until observed on the pinned Java runtime.
