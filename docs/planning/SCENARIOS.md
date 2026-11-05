# Functional scenarios and in-game reproduction

Revision 0.2. **22 scenario designs**, all non-executable and unobserved. No expected trace has been captured, instance started or RED/GREEN recorded. The [JSON catalog](functional-scenarios.json), [instances](instance-profiles.template.json) and [TDD](TDD.md) define future promotion to real tests.

Natural-language steps must be materialized in a strict DSL: step IDs, effective objects/recipes, quantities/units, seed, phase, tick limit, deadlines, predicates and input/observation capabilities. Unavailable data blocks execution; it receives no arbitrary defaults.

| ID | Scenario | Topology | Families in context |
| --- | --- | --- | --- |
| SC-LAB-001 | Player places a machine and opens its menu | LAB-JD1 | MK-MAC-001; MK-UI-001 |
| SC-LAB-002 | Input transfer through real slots | LAB-JD1 | MK-IO-003; MK-MAC-001; MK-UI-001 |
| SC-LAB-003 | Processing without energy and resumption | LAB-JD1 | MK-MAC-001; MK-ENE-003 |
| SC-LAB-004 | Full output blocks processing and clearing it releases processing | LAB-JD1 | MK-MAC-001; MK-IO-003 |
| SC-LAB-005 | Player-controlled redstone modes | LAB-JD1 | MK-IO-002; MK-MAC-001 |
| SC-LAB-006 | Upgrade installation and removal | LAB-JD1 | MK-REG-004; MK-MAC-001 |
| SC-LAB-007 | Save and reopen during processing | LAB-JREC | MK-MAC-001; MK-IO-001 |
| SC-LAB-008 | Placing and removing a cable splits the network | LAB-JD2 | MK-ENE-002; MK-NET-005 |
| SC-LAB-009 | Chemical tank with an incompatible resource | LAB-JD1 | MK-CHE-002; MK-CHE-003; MK-CHE-004 |
| SC-LAB-010 | Sorter applies filters with a full destination | LAB-JD1 | MK-NET-001; MK-NET-002 |
| SC-LAB-011 | Security between the owner and a second player | LAB-JD2 | MK-SEC-001; MK-SEC-002; MK-STO-002 |
| SC-LAB-012 | QIO stores and returns items with components | LAB-JD2 | MK-QIO-001; MK-QIO-002; MK-QIO-003 |
| SC-LAB-013 | Multiblock forms and invalidates through real construction | LAB-JD2 | MK-STO-003; MK-UI-001 |
| SC-LAB-014 | Radiation and protection in an isolated area | LAB-JD2 | MK-NUC-002; MK-EQP-007 |
| SC-LAB-015 | Equipment consumes a resource and switches mode | LAB-JI | MK-EQP-001; MK-EQP-003; MK-EQP-005; MK-UI-002 |
| SC-LAB-016 | Tool from Mekanism Tools breaks a block and loses durability | LAB-JD1 | MK-TOL-001; MK-TOL-002; MK-WOR-006 |
| SC-LAB-017 | Balloons attach to and detach from an entity | LAB-JD2 | MK-ADD-003; MK-UI-003 |
| SC-LAB-018 | Late join and reconnection to a machine in use | LAB-JD2 | MK-UI-001; MK-IO-003; MK-MAC-001 |
| SC-LAB-019 | Network crosses a chunk boundary and resumes after unload | LAB-JD1 | MK-ENE-002; MK-WOR-002; MK-NET-005 |
| SC-LAB-020 | Quantum Entangloporter connects a frequency across dimensions | LAB-JD2 | MK-NET-003; MK-SEC-002 |
| SC-LAB-021 | Real generator powers a machine through the network | LAB-JD1 | MK-GEN-001; MK-ENE-002; MK-MAC-001 |
| SC-LAB-022 | Digital Miner filters, mines and replaces a block | LAB-JD1 | MK-WOR-001; MK-IO-001; MK-IO-002 |

## SC-LAB-001 — Player places a machine and opens its menu

Topology `LAB-JD1`; ingress `client_event_queue`; level `real_client_input_server_and_gui`. Families in context: MK-MAC-001, MK-UI-001. The links below are a partial design review, without execution or qualification credit; context is not coverage.

**Arrange and provenance.** Administratively set up only terrain, game mode and starting items; the machine under test does not exist in the world before the click.

**Required bindings.** machine_item_id, target_block_face, player_hotbar_slot, menu_identity, tick_deadline. Also resolve seed, checkpoint, phases, budgets and deadlines from the common contract.

**Act and barriers.**

1. p1: wait for chunks to be applied and select the hotbar item
2. p1: aim at the surface and press/release use through the real queue
3. barrier: input consumed, placement applied on the server and block replicated
4. p1: aim at the placed machine and press/release use
5. barrier: menu open on the client and container associated with the same block on the server

**Assert on the server.** block_state_and_orientation; player_item_consumption; block_entity_created; container_owner_and_target.

**Assert on clients.** input_event_consumed; rendered_block; current_screen_menu_identity; fresh_menu_frame.

**Hypotheses to characterize in Java.** placement uses the action's face/orientation; opening the menu does not place a second machine. These do not constitute assumed expected results.

**Negative controls.** directing input to another instance must fail because the witness is missing; an old screenshot does not satisfy the current-frame requirement; setBlock fallback must be rejected at this level.

**Completion rule.** placement and menu have a complete causal chain, characterized inventory delta and current frame; retain the trace and post-action checkpoint. Also preserve all failures, raw trace per process, session states and an intact final manifest.

**TDD and expected-result origin.** Qualify control/observation, characterize in at least three fresh Java attempts and review/seal the trace. Only the future supported candidate can produce semantic RED against these expected results. Failed boot/invalid ingress/unsupported is a blocker. No result was captured in this revision.


**Design traceability by aspect.**

| Requirement | Aspects exercised in the design | Aspects still pending |
| --- | --- | --- |
| MK-MAC-001.R01 | Player placement and menu opening with server/client witnesses. | Input/recipe, processing, extraction and rejection/resource variants require other cases. |
| MK-UI-001.R01 | Use of the real queue and association of the menu with the server block. | Normal/shift/drag clicks, cursor and complete transactions still require additional cases. |

## SC-LAB-002 — Input transfer through real slots

Topology `LAB-JD1`; ingress `client_event_queue`; level `real_client_menu_inventory`. Families in context: MK-IO-003, MK-MAC-001, MK-UI-001. The links below are a partial design review, without execution or qualification credit; context is not coverage.

**Arrange and provenance.** Pilot machine placed by SC-LAB-001 or a labeled fixture; starting items recorded and a valid recipe discovered.

**Required bindings.** recipe, source_slot, machine_input_slot, invalid_item, amount_bound. Also resolve seed, checkpoint, phases, budgets and deadlines from the common contract.

**Act and barriers.**

1. p1: open the menu and identify slots using the effective layout
2. p1: click source and destination through the real Screen
3. barrier: server applies the transaction and client confirms slots/cursor
4. p1: attempt to insert an incompatible item into the same slot
5. barrier: record acceptance/rejection and complete states after the attempt
6. p1: close/reopen the menu and compare inventories

**Assert on the server.** inventory_before_after; carried_stack; slot_rules; menu_transaction_state.

**Assert on clients.** clicked_widget_and_coordinates; input_consumed; slot_stacks; cursor_stack; fresh_menu_frame.

**Hypotheses to characterize in Java.** an incompatible item does not cause an unaccounted change; close/reopen does not duplicate the item or cursor stack. These do not constitute assumed expected results.

**Negative controls.** ACK without server application is insufficient; changing GUI scale without recalculating the target must expose an error; direct backend insertion cannot satisfy a slot action.

**Completion rule.** server and client agree for every click, including the cursor, with the absence/presence of effects identified by Java. Also preserve all failures, raw trace per process, session states and an intact final manifest.

**TDD and expected-result origin.** Qualify control/observation, characterize in at least three fresh Java attempts and review/seal the trace. Only the future supported candidate can produce semantic RED against these expected results. Failed boot/invalid ingress/unsupported is a blocker. No result was captured in this revision.


**Design traceability by aspect.**

| Requirement | Aspects exercised in the design | Aspects still pending |
| --- | --- | --- |
| MK-MAC-001.R01 | Input insertion through the Screen and identification of recipe/slots. | Placement is composed with SC-LAB-001; processing and final extraction with SC-LAB-003; variants are not exhausted. |
| MK-IO-003.R01 | Normal click, cursor, incompatible-item attempt and close/reopen. | Shift-click, drag, split, full inventory and component combinations have not all been specified. |
| MK-UI-001.R01 | Slot/cursor transaction corroborated by server and client. | Other gestures, layouts and menus remain pending. |

## SC-LAB-003 — Processing without energy and resumption

Topology `LAB-JD1`; ingress `client_event_queue`; level `real_machine_processing_with_declared_energy_fixture`. Families in context: MK-MAC-001, MK-ENE-003. The links below are a partial design review, without execution or qualification credit; context is not coverage.

**Arrange and provenance.** Recipe/input through the real menu; isolated administrative energy source, explicitly making no claim about a cable or generator.

**Required bindings.** recipe, required_typed_input, energy_representation_unit, safe_supply, observation_ticks. Also resolve seed, checkpoint, phases, budgets and deadlines from the common contract.

**Act and barriers.**

1. p1: insert input and open the screen with known initial energy
2. clock: advance the window without supply and capture every tick
3. fixture: supply energy through the declared mechanism and record its effect
4. clock: advance to a bounded processing transition
5. p1: extract output through a slot and confirm inventory

**Assert on the server.** typed_energy_each_tick; input_output_each_tick; recipe_selection; progress; active_state.

**Assert on clients.** energy_gauge; progress_gauge; output_slot; player_received_stack.

**Hypotheses to characterize in Java.** lack of energy changes progress/consumption according to the reference; resumption must not be assumed to reset or preserve progress without observation. These do not constitute assumed expected results.

**Negative controls.** extra consumption in the trace must diverge; the same final output with a transient delay must diverge; a substituted energy unit must be rejected.

**Completion rule.** no-energy→resumption→output trajectory characterized; no invented rate/quantity. Also preserve all failures, raw trace per process, session states and an intact final manifest.

**TDD and expected-result origin.** Qualify control/observation, characterize in at least three fresh Java attempts and review/seal the trace. Only the future supported candidate can produce semantic RED against these expected results. Failed boot/invalid ingress/unsupported is a blocker. No result was captured in this revision.


**Design traceability by aspect.**

| Requirement | Aspects exercised in the design | Aspects still pending |
| --- | --- | --- |
| MK-MAC-001.R01 | No-energy trajectory, declared supply, processing and player extraction of output. | Depends on composition with placement/input and concrete recipes; full output and invalid input require their own cases. |

## SC-LAB-004 — Full output blocks processing and clearing it releases processing

Topology `LAB-JD1`; ingress `client_event_queue`; level `real_client_menu_and_machine`. Families in context: MK-MAC-001, MK-IO-003. The links below are a partial design review, without execution or qualification credit; context is not coverage.

**Arrange and provenance.** Full-output state constructed through recorded inventory actions or a declared owned checkpoint; sufficient energy observable.

**Required bindings.** output_capacity, recipe, valid_full_output_stack, bounded_blocked_ticks. Also resolve seed, checkpoint, phases, budgets and deadlines from the common contract.

**Act and barriers.**

1. p1: fill the output according to the real menu rules
2. p1: insert valid input
3. clock: capture resources and progress while blocked
4. p1: extract part of the output through a real click
5. clock: capture the unblocking and processing transition
6. p1: collect products and record the final balance

**Assert on the server.** output_capacity_and_count; energy_input_progress_trace; recipe_remaining_input.

**Assert on clients.** blocked_indicator_if_available; gauge_and_slots; withdrawal_visible.

**Hypotheses to characterize in Java.** energy behavior while blocked is observed rather than assumed; partial output follows the real limit rule. These do not constitute assumed expected results.

**Negative controls.** omitting blocked ticks must invalidate the trace; duplicating extraction must change the balance and be detected.

**Completion rule.** blocking and release have witnesses and complete balances, including transient behavior. Also preserve all failures, raw trace per process, session states and an intact final manifest.

**TDD and expected-result origin.** Qualify control/observation, characterize in at least three fresh Java attempts and review/seal the trace. Only the future supported candidate can produce semantic RED against these expected results. Failed boot/invalid ingress/unsupported is a blocker. No result was captured in this revision.


**Design traceability by aspect.**

| Requirement | Aspects exercised in the design | Aspects still pending |
| --- | --- | --- |
| MK-MAC-001.R01 | Full-output blocking, partial extraction and resumption with traces. | Does not cover all recipes, quantity boundaries, placement or invalid input. |
| MK-IO-003.R01 | Output occupancy and extraction through the real menu path. | Drag/shift/split gestures and components are not all exercised. |

## SC-LAB-005 — Player-controlled redstone modes

Topology `LAB-JD1`; ingress `client_event_queue`; level `real_player_config_and_world_signal`. Families in context: MK-IO-002, MK-MAC-001. The links below are a partial design review, without execution or qualification credit; context is not coverage.

**Arrange and provenance.** Machine with known recipe/resources; lever and redstone path built in an original fixture and identified.

**Required bindings.** discovered_redstone_modes, config_widget, lever_target, processing_window. Also resolve seed, checkpoint, phases, budgets and deadlines from the common contract.

**Act and barriers.**

1. p1: choose a redstone mode in the menu
2. p1: operate the real lever using use input
3. barrier: server signal applied and client state updated
4. clock: observe progress/consumption under the signal
5. p1: invert the signal and repeat for the discovered modes

**Assert on the server.** mode; signal_at_machine; active_progress_resources.

**Assert on clients.** selected_mode_text_or_widget; lever_state; progress_indicator.

**Hypotheses to characterize in Java.** each mode has a real signal→activity table; a mode change during processing has its own transition. These do not constitute assumed expected results.

**Negative controls.** a changed signal with the wrong mode must diverge; a local menu change without server commit does not satisfy the step.

**Completion rule.** per-mode table and signal transitions captured in Java with widget and machine identity. Also preserve all failures, raw trace per process, session states and an intact final manifest.

**TDD and expected-result origin.** Qualify control/observation, characterize in at least three fresh Java attempts and review/seal the trace. Only the future supported candidate can produce semantic RED against these expected results. Failed boot/invalid ingress/unsupported is a blocker. No result was captured in this revision.


**Design traceability by aspect.**

| Requirement | Aspects exercised in the design | Aspects still pending |
| --- | --- | --- |
| MK-IO-002.R01 | Mode/signal table and lever transitions with server/client observation. | Short pulses and precise alignment to tick boundaries require concrete variants. |
| MK-IO-002.R02 | Mode/signal change while processing is active, with resources observed. | All pause/reset/consumption states depend on the reference; persistence is outside this design. |

## SC-LAB-006 — Upgrade installation and removal

Topology `LAB-JD1`; ingress `client_event_queue`; level `real_player_upgrade_menu_processing`. Families in context: MK-REG-004, MK-MAC-001. The links below are a partial design review, without execution or qualification credit; context is not coverage.

**Arrange and provenance.** Pilot machine and valid upgrade discovered; fixed configuration profile.

**Required bindings.** upgrade_id, upgrade_count_limit, install_controls, recipe, typed_energy. Also resolve seed, checkpoint, phases, budgets and deadlines from the common contract.

**Act and barriers.**

1. clock: record baseline processing without an upgrade
2. p1: open the upgrades tab and insert the item through real slots
3. clock: wait for installation for the discovered duration and observe the count
4. clock: process a recipe under the new configuration
5. p1: remove the upgrade through the real control and record its return
6. clock: observe restoration of configuration and behavior

**Assert on the server.** upgrade_inventory_and_installed_count; player_inventory; processing_rate_and_energy_trace.

**Assert on clients.** upgrade_tab; installation_progress; installed_count; returned_item.

**Hypotheses to characterize in Java.** the upgrade effect depends on effective configuration; removal during processing follows the real rule. These do not constitute assumed expected results.

**Negative controls.** the comparator must not ignore an upgrade above the limit; removing an item without changing the observed module must diverge.

**Completion rule.** baseline/installation/effect/removal have their own traces and no formula was derived from the future port. Also preserve all failures, raw trace per process, session states and an intact final manifest.

**TDD and expected-result origin.** Qualify control/observation, characterize in at least three fresh Java attempts and review/seal the trace. Only the future supported candidate can produce semantic RED against these expected results. Failed boot/invalid ingress/unsupported is a blocker. No result was captured in this revision.


**Design traceability by aspect.**

| Requirement | Aspects exercised in the design | Aspects still pending |
| --- | --- | --- |
| MK-REG-004.R01 | Upgrade installation/removal through the interface and observation of its effect. | All upgrades/machine types, exceeded limits and configuration combinations require expansion. |

## SC-LAB-007 — Save and reopen during processing

Topology `LAB-JREC`; ingress `client_event_queue`; level `real_server_restart_with_client_rejoin`. Families in context: MK-MAC-001, MK-IO-001. The links below are a partial design review, without execution or qualification credit; context is not coverage.

**Arrange and provenance.** Machine demonstrably processing; the same world belongs exclusively to the lifecycle scenario.

**Required bindings.** mid_process_checkpoint, durability_ack, restart_deadlines, continuation_ticks. Also resolve seed, checkpoint, phases, budgets and deadlines from the common contract.

**Act and barriers.**

1. clock: pause at the safe phase after processing starts
2. server: save and confirm durable flush
3. p1/server: disconnect and close gracefully
4. runner: start new processes with the owned saved world and identical lock
5. p1: reconnect, wait for chunks/menu and observe state
6. clock: advance continuation and extract output

**Assert on the server.** inventory_energy_progress_side_config_before_after; saved_typed_state; world_epoch_change; post_load_trajectory.

**Assert on clients.** rejoin_identity; reopened_menu; progress_energy_slots; final_output.

**Hypotheses to characterize in Java.** actually persisted fields may differ from transient caches; continuation depends on the original saved state. These do not constitute assumed expected results.

**Negative controls.** loading an earlier save must fail on hash/state; restarting without flush ACK cannot satisfy the durable scenario.

**Completion rule.** restart is real, all old processes are terminated and continuation characterizes persisted state without stale evidence. Also preserve all failures, raw trace per process, session states and an intact final manifest.

**TDD and expected-result origin.** Qualify control/observation, characterize in at least three fresh Java attempts and review/seal the trace. Only the future supported candidate can produce semantic RED against these expected results. Failed boot/invalid ingress/unsupported is a blocker. No result was captured in this revision.


**Design traceability by aspect.**

| Requirement | Aspects exercised in the design | Aspects still pending |
| --- | --- | --- |
| MK-MAC-001.R02 | Mid-process checkpoint, flush, new JVM, rejoin and subsequent ticks. | Start/end boundaries and crashes before/after durability still require separate cases. |
| MK-IO-001.R01 | Observed preservation of side configuration after restart. | Does not exercise configuration/rotation on all six faces or every mode. |

## SC-LAB-008 — Placing and removing a cable splits the network

Topology `LAB-JD2`; ingress `client_event_queue`; level `two_real_clients_topology_mutation`. Families in context: MK-ENE-002, MK-NET-005. The links below are a partial design review, without execution or qualification credit; context is not coverage.

**Arrange and provenance.** Source and consumers at original positions; connecting cable initially absent; quantities/tiers discovered.

**Required bindings.** cable_tier, producer, consumer_set, link_position, rebuild_window. Also resolve seed, checkpoint, phases, budgets and deadlines from the common contract.

**Act and barriers.**

1. p1: place the real connecting cable
2. barrier: topology/connection observed on the server and both clients
3. clock: capture distribution to consumers
4. p2: break the cable through the mining path
5. clock: capture the split and buffer effects
6. p1: replace the cable and observe the merge

**Assert on the server.** nodes_connections; per_node_buffers; per_consumer_energy; drops_on_break; resource_accounting.

**Assert on clients.** cable_geometry_both_clients; break_animation; consumer_menus.

**Hypotheses to characterize in Java.** distribution and retention on split follow the real priority; reconnection does not duplicate resources. These do not constitute assumed expected results.

**Negative controls.** the same global balance with a changed per-node distribution must diverge; stale geometry on p2 must fail the client witness.

**Completion rule.** formation/split/merge produces authoritative and visual effects on both clients. Also preserve all failures, raw trace per process, session states and an intact final manifest.

**TDD and expected-result origin.** Qualify control/observation, characterize in at least three fresh Java attempts and review/seal the trace. Only the future supported candidate can produce semantic RED against these expected results. Failed boot/invalid ingress/unsupported is a blocker. No result was captured in this revision.


**Design traceability by aspect.**

| Requirement | Aspects exercised in the design | Aspects still pending |
| --- | --- | --- |
| MK-ENE-002.R02 | Place/remove a cable to separate/reunite nodes with observation from two clients. | Ring topology, merging occupied networks, per-node resources and chunk boundaries require variants. |

## SC-LAB-009 — Chemical tank with an incompatible resource

Topology `LAB-JD1`; ingress `client_event_queue`; level `real_chemical_transfer_and_gui`. Families in context: MK-CHE-002, MK-CHE-003, MK-CHE-004. Child requirement linkage remains pending; context is not coverage.

**Arrange and provenance.** Tank and source with observed chemical A; chemical B and container supplied through a declared original fixture; faces/modes identified.

**Required bindings.** chemical_A, chemical_B, tank_capacity, transfer_mode, dropper_contract. Also resolve seed, checkpoint, phases, budgets and deadlines from the common contract.

**Act and barriers.**

1. p1: connect the tube through the chosen face and transfer A
2. p1: open the tank and observe type/quantity
3. p1: attempt to transfer B through the same path
4. clock: capture rejection or conversion according to the original
5. p1: use the Gauge Dropper through the screen and record extraction/insertion

**Assert on the server.** typed_chemical_id_amount_attributes; source_destination_balances; side_configuration; container_contents.

**Assert on clients.** chemical_gauge; tooltip_unit; dropper_cursor_or_inventory_state.

**Hypotheses to characterize in Java.** mixing/rejection depends on the tank rule; the dropper preserves identity and quantity. These do not constitute assumed expected results.

**Negative controls.** changing the ID while retaining quantity must diverge; an empty tank and an absent field must not compare equal.

**Completion rule.** each attempt has type/unit/quantity and an observed reaction, without normalizing chemicals by display name. Also preserve all failures, raw trace per process, session states and an intact final manifest.

**TDD and expected-result origin.** Qualify control/observation, characterize in at least three fresh Java attempts and review/seal the trace. Only the future supported candidate can produce semantic RED against these expected results. Failed boot/invalid ingress/unsupported is a blocker. No result was captured in this revision.

## SC-LAB-010 — Sorter applies filters with a full destination

Topology `LAB-JD1`; ingress `client_event_queue`; level `real_player_filter_configuration_and_logistics`. Families in context: MK-NET-001, MK-NET-002. The links below are a partial design review, without execution or qualification credit; context is not coverage.

**Arrange and provenance.** Layout with source, sorter, branch and two destinations; destinations built in an original fixture; filters not yet configured.

**Required bindings.** filter_modes, filter_item_components, routing_colors, destination_capacity, bounded_transit_ticks. Also resolve seed, checkpoint, phases, budgets and deadlines from the common contract.

**Act and barriers.**

1. p1: create a filter in the menu with discovered parameters
2. p1: insert a batch with matching and nonmatching items
3. clock: observe destination selection and transit
4. p1: fill the preferred destination through the real inventory
5. clock: observe Java blocking/return/alternative route
6. p1: change filter order and repeat from reset state

**Assert on the server.** ordered_filters; inflight_items; origin_destination_balances; route_decisions.

**Assert on clients.** filter_editor_values; transporter_visuals; destination_inventories.

**Hypotheses to characterize in Java.** filter order and color can change routing; item return cannot be assumed lossless without accounting. These do not constitute assumed expected results.

**Negative controls.** silently sorting filters must be detected; removing inflight state from the snapshot must invalidate completeness.

**Completion rule.** configuration, transit and balances account for all items throughout execution. Also preserve all failures, raw trace per process, session states and an intact final manifest.

**TDD and expected-result origin.** Qualify control/observation, characterize in at least three fresh Java attempts and review/seal the trace. Only the future supported candidate can produce semantic RED against these expected results. Failed boot/invalid ingress/unsupported is a blocker. No result was captured in this revision.


**Design traceability by aspect.**

| Requirement | Aspects exercised in the design | Aspects still pending |
| --- | --- | --- |
| MK-NET-002.R01 | Filters and full destination with observed route/transfer. | All item/tag/mod predicates, min/max/fuzzy/invert and overlapping priority require their own cases. |

## SC-LAB-011 — Security between the owner and a second player

Topology `LAB-JD2`; ingress `client_event_queue`; level `real_two_player_permissions`. Families in context: MK-SEC-001, MK-SEC-002, MK-STO-002. The links below are a partial design review, without execution or qualification credit; context is not coverage.

**Arrange and provenance.** p1/p2 have distinct fixture aliases and UUIDs; storage owner established through the recorded real action.

**Required bindings.** security_modes, trusted_player_controls, owned_storage, frequency_access_rules. Also resolve seed, checkpoint, phases, budgets and deadlines from the common contract.

**Act and barriers.**

1. p1: configure restricted mode in the menu
2. p2: attempt to open and transfer an item through real use
3. barrier: record server rejection/acceptance and p2's screen
4. p1: add p2 to the applicable trust mechanism
5. p2: repeat the action
6. p1: revoke access and have p2 repeat with the previously open menu

**Assert on the server.** owner_trusted_set; authorization_result; inventory_mutations; frequency_membership.

**Assert on clients.** p1_security_screen; p2_open_or_denial; menu_close_or_stale_access; both_player_inventories.

**Hypotheses to characterize in Java.** revocation with an open menu has its own rule; the same display name does not replace owner identity. These do not constitute assumed expected results.

**Negative controls.** changing owner/UUID while retaining the display name must diverge; visual success without server authorization does not pass.

**Completion rule.** three access states and balances are observed by players and server. Also preserve all failures, raw trace per process, session states and an intact final manifest.

**TDD and expected-result origin.** Qualify control/observation, characterize in at least three fresh Java attempts and review/seal the trace. Only the future supported candidate can produce semantic RED against these expected results. Failed boot/invalid ingress/unsupported is a blocker. No result was captured in this revision.


**Design traceability by aspect.**

| Requirement | Aspects exercised in the design | Aspects still pending |
| --- | --- | --- |
| MK-SEC-001.R01 | Owner and second actor switch between stranger/trusted states and attempt real actions. | Three simultaneous identities, all operations and automation require a specific variant. |
| MK-SEC-001.R02 | Revocation while the second actor had a menu open. | Persistence after restart and other access paths remain pending. |
| MK-STO-002.R01 | Opening/transfer in personal storage with allowed/denied permission. | All chests/barrels/portable items and automation access are not covered by this single design. |

## SC-LAB-012 — QIO stores and returns items with components

Topology `LAB-JD2`; ingress `client_event_queue`; level `real_qio_dashboard_storage_and_concurrent_clients`. Families in context: MK-QIO-001, MK-QIO-002, MK-QIO-003. The links below are a partial design review, without execution or qualification credit; context is not coverage.

**Arrange and provenance.** Drive Array and frequency created in a traceable fixture; distinct drives and items available to the player; no unlabeled pre-injected QIO entries.

**Required bindings.** drive_limits, frequency, item_component_variants, dashboard_widgets, withdrawal_schedule. Also resolve seed, checkpoint, phases, budgets and deadlines from the common contract.

**Act and barriers.**

1. p1: install the drive and associate the dashboard with the frequency
2. p1: deposit component variants through the menu
3. p2: open the authorized dashboard and check the distinction
4. clock: schedule p1/p2 withdrawals in explicit order
5. server: save/restart and have clients reconnect
6. p1/p2: check the balance and withdraw the remainder

**Assert on the server.** drive_item_type_counts; component_keys; frequency_owner; transaction_order; post_load_storage.

**Assert on clients.** search_results_and_counts; component_tooltips; withdrawn_inventory_both_players.

**Hypotheses to characterize in Java.** components distinguish types according to the real QIO rule; concurrent withdrawals follow authoritative order without ghost items. These do not constitute assumed expected results.

**Negative controls.** merging variants by item ID must diverge; a stale client count after withdrawal does not satisfy convergence.

**Completion rule.** drive, dashboard and inventories account for each variant before/after restart. Also preserve all failures, raw trace per process, session states and an intact final manifest.

**TDD and expected-result origin.** Qualify control/observation, characterize in at least three fresh Java attempts and review/seal the trace. Only the future supported candidate can produce semantic RED against these expected results. Failed boot/invalid ingress/unsupported is a blocker. No result was captured in this revision.


**Design traceability by aspect.**

| Requirement | Aspects exercised in the design | Aspects still pending |
| --- | --- | --- |
| MK-QIO-001.R01 | Insertion/extraction of items with distinct components in QIO. | Type/quantity limits and all drive tiers still require boundary cases. |

## SC-LAB-013 — Multiblock forms and invalidates through real construction

Topology `LAB-JD2`; ingress `client_event_queue`; level `real_player_multiblock_construction`. Families in context: MK-STO-003, MK-UI-001. The links below are a partial design review, without execution or qualification credit; context is not coverage.

**Arrange and provenance.** Original Dynamic Tank blueprint without a complete structure; player receives materials rather than an already formed controller.

**Required bindings.** valid_blueprint, last_shell_block, valve_face, multiblock_observables, structure_recheck_bound. Also resolve seed, checkpoint, phases, budgets and deadlines from the common contract.

**Act and barriers.**

1. p1: build the shell according to the blueprint through real placement
2. p1: place the final closing block
3. clock: observe the unformed→formed transition and capacity
4. p2: open the valve/menu and confirm the structure view
5. p1: break the designated structural block
6. clock: observe invalidation, contents and events on both clients

**Assert on the server.** formed_state; dimensions_capacity_controller; contents; invalidated_state; drops.

**Assert on clients.** shell_render_state; valve_menu; formation_or_failure_feedback.

**Hypotheses to characterize in Java.** capacity and contents on invalidation follow Java; final placement must not be replaced with a pre-formed structure. These do not constitute assumed expected results.

**Negative controls.** removing the formation witness must prevent a pass; changing dimensions/capacity with the same contents must diverge.

**Completion rule.** player-caused formation and invalidation have an authoritative trajectory and visual witness. Also preserve all failures, raw trace per process, session states and an intact final manifest.

**TDD and expected-result origin.** Qualify control/observation, characterize in at least three fresh Java attempts and review/seal the trace. Only the future supported candidate can produce semantic RED against these expected results. Failed boot/invalid ingress/unsupported is a blocker. No result was captured in this revision.


**Design traceability by aspect.**

| Requirement | Aspects exercised in the design | Aspects still pending |
| --- | --- | --- |
| MK-STO-003.R01 | Player construction and valid/invalid multiblock transition. | Dimensional limits and adjacent structures are not yet concrete variants. |

## SC-LAB-014 — Radiation and protection in an isolated area

Topology `LAB-JD2`; ingress `client_event_queue`; level `real_player_exposure_and_equipment`. Families in context: MK-NUC-002, MK-EQP-007. Child requirement linkage remains pending; context is not coverage.

**Arrange and provenance.** Scratch world dedicated to global radiation; declared, bounded test source; p1/p2 with distinct equipment, never a personal world.

**Required bindings.** source_creation_method, hazmat_slots, measurement_item, exposure_bound, damage_and_dose_observers. Also resolve seed, checkpoint, phases, budgets and deadlines from the common contract.

**Act and barriers.**

1. p1/p2: equip protection through real slots according to the variants
2. p1/p2: approach the area through real movement
3. clock: capture environment, dose and effects over a bounded window
4. p1: remove one piece and repeat controlled exposure
5. server: save/restart and observe environmental and personal persistence

**Assert on the server.** environmental_radiation_field; personal_dose_effects; equipment_components; health_and_damage_events; global_saved_data.

**Assert on clients.** measurement_reading; armor_slots; health_hud; warnings_effects.

**Hypotheses to characterize in Java.** partial protection may be nonlinear; persistence/decay must not be assumed. These do not constitute assumed expected results.

**Negative controls.** omitting personal dose while retaining environmental dose must invalidate the profile; normalizing health/dose with a global tolerance must be prohibited.

**Completion rule.** exposure and protection are measured with a declared RNG/tick profile and isolated global state. Also preserve all failures, raw trace per process, session states and an intact final manifest.

**TDD and expected-result origin.** Qualify control/observation, characterize in at least three fresh Java attempts and review/seal the trace. Only the future supported candidate can produce semantic RED against these expected results. Failed boot/invalid ingress/unsupported is a blocker. No result was captured in this revision.

## SC-LAB-015 — Equipment consumes a resource and switches mode

Topology `LAB-JI`; ingress `client_event_queue`; level `integrated_real_player_keybind_equipment`. Families in context: MK-EQP-001, MK-EQP-003, MK-EQP-005, MK-UI-002. Child requirement linkage remains pending; context is not coverage.

**Arrange and provenance.** Equipment/installable module and resource supplied in starting inventory; player installs/equips; mode discovered at runtime.

**Required bindings.** equipment_profile, module_controls, mode_keybinding, resource_unit, movement_window. Also resolve seed, checkpoint, phases, budgets and deadlines from the common contract.

**Act and barriers.**

1. p1: install the module and equip through the real path
2. p1: trigger the mode keybind and confirm HUD/server
3. p1: perform the bounded movement/use action
4. clock: capture position, resource and effects
5. p1: repeat without the resource and observe behavior
6. p1: open the GUI during use and observe input release

**Assert on the server.** installed_module_state; equipment_resource; movement_effects; active_mode; input_release.

**Assert on clients.** keybinding_consumed; mode_hud; resource_gauge; camera_movement; modal_screen.

**Hypotheses to characterize in Java.** opening the GUI interacts with use/flight according to the original; lack of resource has a real effect on the mode. These do not constitute assumed expected results.

**Negative controls.** a keybind consumed by another screen must prevent the change claim; a client-only mode without server confirmation does not pass.

**Completion rule.** mode, input, resources and movement are correlated in the real integrated instance. Also preserve all failures, raw trace per process, session states and an intact final manifest.

**TDD and expected-result origin.** Qualify control/observation, characterize in at least three fresh Java attempts and review/seal the trace. Only the future supported candidate can produce semantic RED against these expected results. Failed boot/invalid ingress/unsupported is a blocker. No result was captured in this revision.

## SC-LAB-016 — Tool from Mekanism Tools breaks a block and loses durability

Topology `LAB-JD1`; ingress `client_event_queue`; level `real_player_mining_and_drops`. Families in context: MK-TOL-001, MK-TOL-002, MK-WOR-006. Child requirement linkage remains pending; context is not coverage.

**Arrange and provenance.** Test blocks and tool with discovered components; survival mode; no direct calls to server loot/break methods.

**Required bindings.** paxel_material, tool_damage, block_classes, enchantments, mining_bound. Also resolve seed, checkpoint, phases, budgets and deadlines from the common contract.

**Act and barriers.**

1. p1: select the tool in the hotbar and aim at the block
2. p1: hold attack until the break is observed
3. barrier: server removes the block and creates drops
4. p1: collect drops through real movement
5. p1: repeat on an incompatible block/class and with a tool near its limit

**Assert on the server.** mining_progress; tool_damage_components; drops_and_pickup; block_removal; tool_break_event.

**Assert on clients.** break_progress_overlay; tool_durability_bar; drop_entity; inventory_after_pickup.

**Hypotheses to characterize in Java.** the paxel follows the correct abilities for each class; durability limit and drops depend on enchantments/conditions. These do not constitute assumed expected results.

**Negative controls.** correct final drops with incorrect mining/timing must diverge; omitted durability must block the requirement.

**Completion rule.** breaking, loot, collection and durability have independent effects and traces for each variant. Also preserve all failures, raw trace per process, session states and an intact final manifest.

**TDD and expected-result origin.** Qualify control/observation, characterize in at least three fresh Java attempts and review/seal the trace. Only the future supported candidate can produce semantic RED against these expected results. Failed boot/invalid ingress/unsupported is a blocker. No result was captured in this revision.

## SC-LAB-017 — Balloons attach to and detach from an entity

Topology `LAB-JD2`; ingress `client_event_queue`; level `real_player_entity_interaction_and_sync`. Families in context: MK-ADD-003, MK-UI-003. Child requirement linkage remains pending; context is not coverage.

**Arrange and provenance.** Fixture entity and balloon items in an isolated world; variant/color chosen by discovery, with no balloon already attached.

**Required bindings.** balloon_variant, supported_target_entity, attach_detach_input, physics_window. Also resolve seed, checkpoint, phases, budgets and deadlines from the common contract.

**Act and barriers.**

1. p1: use the balloon on the target with a real hit
2. barrier: server identifies the attachment and both clients apply it
3. clock: observe movement/physics over a bounded window
4. p2: perform the applicable detach/break action
5. clock: observe attachment removal, drops and client rendering

**Assert on the server.** balloon_entity_and_attachment; positions_velocities; item_consumption; detach_and_drop_events.

**Assert on clients.** color_attachment_geometry; entity_motion_both_clients; detach_visual.

**Hypotheses to characterize in Java.** target support and physics are characterized; detach must not be assumed identical to killing the entity. These do not constitute assumed expected results.

**Negative controls.** an attachment changed to another entity must diverge; a screenshot alone without authoritative attachment does not pass.

**Completion rule.** real attach/motion/detach is observed on the server and two clients without arbitrarily equating ephemeral IDs. Also preserve all failures, raw trace per process, session states and an intact final manifest.

**TDD and expected-result origin.** Qualify control/observation, characterize in at least three fresh Java attempts and review/seal the trace. Only the future supported candidate can produce semantic RED against these expected results. Failed boot/invalid ingress/unsupported is a blocker. No result was captured in this revision.

## SC-LAB-018 — Late join and reconnection to a machine in use

Topology `LAB-JD2`; ingress `client_event_queue`; level `real_client_late_join_disconnect_reconnect`. Families in context: MK-UI-001, MK-IO-003, MK-MAC-001. The links below are a partial design review, without execution or qualification credit; context is not coverage.

**Arrange and provenance.** p1 starts processing and keeps the menu open; p2 is still disconnected; separate identities/menus.

**Required bindings.** processing_checkpoint, join_deadline, menu_convergence, disconnect_action_boundary. Also resolve seed, checkpoint, phases, budgets and deadlines from the common contract.

**Act and barriers.**

1. p1: start processing and open the menu
2. runner/p2: start a new client and join the server
3. p2: open the same machine and check the initial state received
4. p2: start a transfer and disconnect at the declared boundary
5. server: record whether the action was applied or rejected
6. p2: reconnect with its identity and compare inventory/menu with the server

**Assert on the server.** players_distinct; container_viewers; action_transaction_outcome; inventory_and_progress.

**Assert on clients.** p1_menu_unaffected; p2_join_snapshot; p2_inventory_after_reconnect; reopened_menu.

**Hypotheses to characterize in Java.** an action at disconnect may commit or be rejected depending on the boundary; late-join initial state is current. These do not constitute assumed expected results.

**Negative controls.** a lost ACK does not authorize blind transfer replay; using p1's UUID for p2 must fail identity validation.

**Completion rule.** the action outcome is accounted for once and both views converge without reusing the previous menu cache. Also preserve all failures, raw trace per process, session states and an intact final manifest.

**TDD and expected-result origin.** Qualify control/observation, characterize in at least three fresh Java attempts and review/seal the trace. Only the future supported candidate can produce semantic RED against these expected results. Failed boot/invalid ingress/unsupported is a blocker. No result was captured in this revision.


**Design traceability by aspect.**

| Requirement | Aspects exercised in the design | Aspects still pending |
| --- | --- | --- |
| MK-UI-001.R02 | Two viewers of the same machine, late join and reconnect with separate menus. | All concurrent settings and process edits need a specific schedule. |
| MK-IO-003.R02 | Second-player transaction with disconnect at the boundary and a single outcome. | A concurrent hopper, block removal and all contention schedules are not exercised. |

## SC-LAB-019 — Network crosses a chunk boundary and resumes after unload

Topology `LAB-JD1`; ingress `client_event_queue`; level `real_chunk_lifecycle_and_network`. Families in context: MK-ENE-002, MK-WOR-002, MK-NET-005. The links below are a partial design review, without execution or qualification credit; context is not coverage.

**Arrange and provenance.** Original network crosses a known chunk boundary; explicit tickets and stabilizer absent/present in separate variants.

**Required bindings.** chunk_boundary, ticket_policy, player_view_simulation_distance, unload_witness, stabilizer_mode. Also resolve seed, checkpoint, phases, budgets and deadlines from the common contract.

**Act and barriers.**

1. p1: form the connection and confirm consumption on both sides
2. p1: move away to allow unload under the declared policy
3. barrier: server confirms real unload, not merely visual absence
4. clock: capture still-loaded nodes and persisted data
5. p1: return and wait for the chunk to be applied
6. clock: observe reconstruction/resumption and repeat with the stabilizer configured

**Assert on the server.** actual_loaded_chunks_and_tickets; network_membership; buffers_saved_and_live; post_load_transfer.

**Assert on clients.** chunk_presence; network_render_after_return; consumer_state.

**Hypotheses to characterize in Java.** unload may change distribution and timing; the stabilizer maintains only the defined tickets. These do not constitute assumed expected results.

**Negative controls.** visual teleport without unload does not satisfy the case; comparing only loaded nodes and deleting the others is invalid.

**Completion rule.** server-proven lifecycle with accounting across saved and live state. Also preserve all failures, raw trace per process, session states and an intact final manifest.

**TDD and expected-result origin.** Qualify control/observation, characterize in at least three fresh Java attempts and review/seal the trace. Only the future supported candidate can produce semantic RED against these expected results. Failed boot/invalid ingress/unsupported is a blocker. No result was captured in this revision.


**Design traceability by aspect.**

| Requirement | Aspects exercised in the design | Aspects still pending |
| --- | --- | --- |
| MK-ENE-002.R03 | Network crossing a chunk boundary with unload/reload and observed resumption. | Persistence on restart and all tickets/reload orders require variants where not described. |

## SC-LAB-020 — Quantum Entangloporter connects a frequency across dimensions

Topology `LAB-JD2`; ingress `client_event_queue`; level `real_frequency_configuration_cross_dimension`. Families in context: MK-NET-003, MK-SEC-002. The links below are a partial design review, without execution or qualification credit; context is not coverage.

**Arrange and provenance.** Two entangloporters in distinct dimensions, original world data and known owners; frequency not yet associated.

**Required bindings.** dimensions, frequency_type, resource_classes_supported, side_modes, transfer_window. Also resolve seed, checkpoint, phases, budgets and deadlines from the common contract.

**Act and barriers.**

1. p1/p2: open their respective real menus
2. p1: create a frequency and have p2 select the same one with permission
3. p1/p2: configure input/output faces through the GUI
4. clock: observe the supported resource being transferred and shared
5. p2: change the frequency and observe disconnection
6. server: save/restart and have clients verify the association

**Assert on the server.** frequency_identity_owner; per_dimension_block_entities; shared_resource_state; side_modes; post_load_association.

**Assert on clients.** selected_frequency_both_menus; resource_gauges; disconnect_feedback.

**Hypotheses to characterize in Java.** frequencies with the same name may depend on owner/public visibility; resource classes have their own transfer behavior. These do not constitute assumed expected results.

**Negative controls.** normalizing names and merging distinct frequencies must diverge; an omitted dimension makes identity ambiguous and must fail.

**Completion rule.** association/disassociation and persistence account for resource and identity across both dimensions. Also preserve all failures, raw trace per process, session states and an intact final manifest.

**TDD and expected-result origin.** Qualify control/observation, characterize in at least three fresh Java attempts and review/seal the trace. Only the future supported candidate can produce semantic RED against these expected results. Failed boot/invalid ingress/unsupported is a blocker. No result was captured in this revision.


**Design traceability by aspect.**

| Requirement | Aspects exercised in the design | Aspects still pending |
| --- | --- | --- |
| MK-NET-003.R01 | Frequency across dimensions and owners with real endpoints. | All same-name public/private pairs and all dimensions still require expansion. |

## SC-LAB-021 — Real generator powers a machine through the network

Topology `LAB-JD1`; ingress `client_event_queue`; level `real_player_generator_network_consumer_chain`. Families in context: MK-GEN-001, MK-ENE-002, MK-MAC-001. The links below are a partial design review, without execution or qualification credit; context is not coverage.

**Arrange and provenance.** Fuel source/environmental condition defined by discovery; generator/cable/machine placed by the player, without backend energy during measurement.

**Required bindings.** heat_generator_recipe_or_environment, fuel, cable_tier, consumer_recipe, measurement_window. Also resolve seed, checkpoint, phases, budgets and deadlines from the common contract.

**Act and barriers.**

1. p1: place the generator, cable and machine through real input
2. p1: supply fuel/resource through the applicable menu
3. p1: insert the machine input
4. clock: capture generation, buffers, transfer and consumption
5. p1: interrupt the generator supply
6. clock: observe depletion and stopping/resumption when supply is restored

**Assert on the server.** generator_input_and_generation; cable_buffers; machine_energy_progress; all_output_and_resource_balances.

**Assert on clients.** generator_and_machine_gauges; cable_connections; produced_output.

**Hypotheses to characterize in Java.** the chain has real buffers and rounding; accounting considers generation, consumption and valid losses. These do not constitute assumed expected results.

**Negative controls.** fixture-injected energy invalidates the end-to-end chain claim; equal final consumption with false generation must diverge.

**Completion rule.** the operational chain originates from real actions and each source/sink is observed per tick. Also preserve all failures, raw trace per process, session states and an intact final manifest.

**TDD and expected-result origin.** Qualify control/observation, characterize in at least three fresh Java attempts and review/seal the trace. Only the future supported candidate can produce semantic RED against these expected results. Failed boot/invalid ingress/unsupported is a blocker. No result was captured in this revision.


**Design traceability by aspect.**

| Requirement | Aspects exercised in the design | Aspects still pending |
| --- | --- | --- |
| MK-GEN-001.R02 | Real generator connects through the network to a consumer with energy observed. | All fuels/thermal environments and cubes/demand limits remain pending. |

## SC-LAB-022 — Digital Miner filters, mines and replaces a block

Topology `LAB-JD1`; ingress `client_event_queue`; level `real_player_miner_configuration_and_world_effects`. Families in context: MK-WOR-001, MK-IO-001, MK-IO-002. The links below are a partial design review, without execution or qualification credit; context is not coverage.

**Arrange and provenance.** Small original area with target/nontarget blocks and a known boundary; miner assembled and resources available; no calls to an internal mining method.

**Required bindings.** search_bounds, filter_types, matching_blocks, replacement_policy, power, bounded_job_ticks. Also resolve seed, checkpoint, phases, budgets and deadlines from the common contract.

**Act and barriers.**

1. p1: configure boundaries and filter through the menu
2. p1: start the search and record the discovered set/count
3. p1: start mining with observable resources
4. clock: record each block change and inventories
5. p1: enable the replacement policy and remove required material in a variant
6. clock: observe blocking/continuation and the area's final contents

**Assert on the server.** configured_bounds_filters; search_selection; mined_block_positions; replacement_states; energy_inventory_progress.

**Assert on clients.** miner_config_widgets; search_counter; running_status; visible_world_changes.

**Hypotheses to characterize in Java.** filter selection and replacement follow real order/semantics; lack of material is not equivalent to ignoring the policy. These do not constitute assumed expected results.

**Negative controls.** the same output with a nontarget block destroyed must diverge; a position missing from the trace invalidates the world diff.

**Completion rule.** every world selection and change has a position, reason, inventory/energy effect and client witness. Also preserve all failures, raw trace per process, session states and an intact final manifest.

**TDD and expected-result origin.** Qualify control/observation, characterize in at least three fresh Java attempts and review/seal the trace. Only the future supported candidate can produce semantic RED against these expected results. Failed boot/invalid ingress/unsupported is a blocker. No result was captured in this revision.


**Design traceability by aspect.**

| Requirement | Aspects exercised in the design | Aspects still pending |
| --- | --- | --- |
| MK-WOR-001.R01 | Digital Miner uses filters/mining/replacement on a known layout. | Inverse filter, all predicates, permission and area boundaries require variants. |

## Partial pilot composition

MK-MAC-001.R01 needs composition: SC-LAB-001 for placement/menu, SC-LAB-002 for input and transactions, SC-LAB-003 for processing/withdrawal; SC-LAB-004 adds output blocking. This planned composition is not an execution and does not exhaust recipes/boundaries. MK-MAC-001.R02 has SC-LAB-007 for restart midway; start/end boundaries and crashes remain pending. All manual links retain remaining_aspects and implicit qualified=false through zero qualification credit.

