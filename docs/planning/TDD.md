# TDD and functional acceptance without false approval

Revision 0.2. This contract distinguishes Rust engine TDD, Java characterization and future Java/Rust acceptance. No tests were executed, RED/GREEN recorded or instances started in this package. The [roadmap](ROADMAP.md), [requirements](requirements.json) and [scenarios](SCENARIOS.md) specify the work to implement.

## Three cycles with different outcomes

| Cycle | Test comes from | Valid RED | Valid GREEN | Not a semantic RED |
|---|---|---|---|---|
| Rust engine/bridge | Independent original fixture and observable contract | Assertion fails for the expected cause, or a guard admits invalid input | Real contract and negatives pass; integration uses a real process/game where claimed | Broken compilation or missing environment alone |
| Java characterization | Controllable scenario and qualified observers | Do not invent RED against existing behavior; test the guard and engine sensitivity | Stable reference trace, controls, provenance and review | Unconfirmed hypothesis, unsupported, failed boot or invisible field |
| Future Rust gameplay | Java expected results reviewed and sealed before implementation | Semantic mismatch with a valid environment, applied action and complete trace | Smallest implementation passes Java/Rust and required integration/lifecycle | Missing adapter, unapplied click, control timeout or golden created by the candidate |

Project rules require independent, observable outcomes; mocks isolate parts, but stub calls do not replace the functional outcome. ([AGENTS](https://github.com/horizon-rust/horizon/blob/main/AGENTS.md#write-tests-with-signal), [Rust unit/integration](https://doc.rust-lang.org/book/ch11-03-test-organization.html)).

## Contract for an engine delivery

1. Specify a small capability and its proof: input, initial state, action, mandatory observables, completion and discriminating control. Pin the independent fixture/result before coding.
2. Write the test and run it before implementation. Record artifact hashes, command/argv, assertion and failure class. A build error can be a preparatory step; continue until demonstrating the assertion failing for the intended reason.
3. Implement the minimum real behavior. A codec uses independent fixtures and values; supervision uses a real probe process; in-game acceptance uses real NeoForge and client at the declared ingress.
4. GREEN requires the effect, correct rejections and absence of prohibited effects. Do not remove assertions, change expected results, normalize differences or hide cases to obtain green.
5. Refactor while preserving corpus/expected results and run relevant regressions. Changes to clock, input, capture, schema, normalization or provenance invalidate dependent qualifications.
6. Complete a microdelivery only after reviewing the test trail and evidence. A library pass does not qualify the game, and client smoke does not qualify every machine.

The proposed trail is `specified → independent_test_authored → intended_contract_red_recorded → minimal_green_recorded → refactor_regressions_passed → evidence_reviewed`. Today all execution records are absent and explicitly pending.

## Obtaining and sealing Java expected values

Resolve artifacts/configs, objects/recipes, units and types, checkpoint, actors, actions, phases and budgets. Confirm preconditions and action application before expecting a value. Run at least three fresh Java attempts of the same scenario, preserving all of them; compare A/A and observer on/off with an independent witness. An incorrect hypothesis must correct the characterization specification, not change Java to fit the test.

Review the finite domain and record whether a surprising behavior/original bug belongs to the reference or is a separate deliberate exception. Seal raw trace, projection/policies, fingerprint and scenario. Golden updates are a separate flow with diff and rationale; never automatic to fix a red test. Expected results extracted from the new Rust violate oracle independence.

A pilot machine needs normal processing, invalid input, missing resource, full output, resumption, a real menu and restart. Each cause has a diagnosable variant. Injected energy tests consumption under a fixture; generator/cable need another real scenario. Recipes, rates, components and durations are observed, not assumed wiki numbers.

## First functional TDD with real instances

| Increment | Intended proof before expansion | Roadmap items |
|---|---|---|
| Validate job and environment | Invalid input rejected before spawn; effective settings traceable | RF-SCOPE; RF-MODEL-01/02 |
| Supervise process | Owned child terminates on cancellation and does not publish pass | RF-INSTANCE-01..05/08/10 |
| Reference boot | Real JVM, world and modules/config checked | RF-INSTANCE-06; RF-BRIDGE-01..06 |
| First client | Current identity, login, chunks, screen and frame | RF-CLIENT-01/02 |
| Movement and placement | Effective press/release, aim and world/inventory change | RF-CLIENT-03..05 |
| Menu and slots | Screen clicks, cursor/slots and server/client state | RF-CLIENT-06..08; SC-LAB-001/002 |
| Clock and capture | Coherent phases and no observer effects | RF-CLOCK-01..08 |
| Pilot | Complete trace for no energy/full/resumption and negative control | RF-PILOT-01..10; SC-LAB-003..007 |
| Lifecycle | Save/flush/close, new JVM and real continuity | RF-LIFE; SC-LAB-007/019 |
| Two clients | Separate identities, menus, authorization and convergence | RF-UX-01..03; SC-LAB-011/018 |

Administrative setup is labeled in Arrange. In the action under test, check target, face/aim, hotbar, widget/slot, focus/modal and ingress. Then require server application/rejection, authoritative delta and each client's update; a receipt, open title or old frame does not satisfy this path. GameTest and fake-player are components, not an E2E fallback.

## Mutants and faults must prove real sensitivity

Classify each mutant as valid/discriminating, equivalent under the declared projection, out of domain, invalid/not applied or unresolved. A test failing because the mutant did not compile does not demonstrate the comparator detecting behavior. Do not count equivalent/invalid mutants as detected; retain rationales and unknowns. Only the executed valid/discriminating set can have a sensitivity denominator; no score has been measured here, nor is a score universal proof.

For pure contracts, change value, type, unit, presence, order, identity, tick and phase and require correct diagnostics. In observer tests, cause a known real action/effect and compare external inventory, another client or an independent save. Changing only JSON tests diff, not capture. For processes/transport, inject truncation, gaps, duplication, death, timeout and stale evidence and require non-pass.

A control within visual tolerance must pass only where policy permits; another outside it must fail. Do not inherit legacy grid/rounding or create a universal epsilon. A different transient with the same final state is a divergence when the contract requires the trajectory.

## Regression matrix and refactoring exit condition

| Change | Required regression |
|---|---|
| Types/schema/diff | Independent unit/property/contract tests and negative controls; affected corpus revalidated |
| Spawn/env/ownership | Real process and Java lifecycle on the affected platform |
| Input/widget/slot | Real client, effective action, authority and screen; focus and resize negatives |
| Clock/observer | Fresh A/A, on/off control and all dependent requirements |
| Config/recipe/registry | Discovery diff, new fingerprint and affected materialized scenarios |
| Menu/sync/authority | Dedicated server with two real clients and applicable integrated companion |
| Save/chunks/dimensions | Restart with a new process, continuity and durable points; separate crash cases |
| Scope gate | Complete fresh mandatory corpus for the scope; incremental testing is insufficient |

Do not use retry-until-green. A mandatory flaky case blocks its scope even if the campaign continues for diagnosis. Quarantine does not remove a case from the denominator. Preserve all attempts, limits and origin of the first divergence. Refactoring does not change expected results or reduce the proof level.

## Two-stage locks and qualification

Before any game, resolve and seal an artifact-lock with hashes of every selected artifact, JDK/loader/dependencies and tooling. Diagnostic bootstrap in scratch may start with a draft effective-lock to generate post-load settings and catalog; it produces neither expected results nor a qualification pass. After review, seal the effective-reference-lock, checkpoint and bindings; the qualifying attempt is new and confirms that state after load. Changes or omissions block approval. Do not allow a null jar hash or promote a bootstrap draft. The [reference profile](profiles.reference.template.json) defines both contracts, and RF-SCOPE-05 depends on real boot/bridge.


