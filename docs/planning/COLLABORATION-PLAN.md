# Collaboration and release plan

## Repository decision

| Repository | Purpose | Decision |
| --- | --- | --- |
| Independent parity lab | Rust validator, Java reference profiles, inventory, scenarios, evidence contracts, contribution queue | Publish the reviewed source baseline here. Keep campaign tracking, triage, and all six workstreams here. |
| Original Mekanism | Pinned Java behavioral reference | Link and verify the precise release and archive. Keep the upstream Java project as the reference. |
| Foundry native mod phase | Native Horizon mod using the reviewed Java behavior as its specification | Begin only after the applicable pre-port gate. Preserve provenance for any separately reviewed reused material. |
| Mekanism fork | Optional route if a concrete need emerges for upstream history or reusable files | Revisit after checking the exact revision and each reused file. A fork preserves lineage and notices but carries Java history, build files, and assets into a Rust-focused review. |
| Horizon fork | Temporary integration bench if a concrete game change requires one | Keep Foundry ownership separate from Horizon. A scoped integration change can go upstream after coordination. |

Recruit inventory, Java scenario and infrastructure contributors in Horizon Discord after publication. Use the lab for tasks and results. Discuss future mod integration with Horizon modding maintainers. Route ideas through the Horizon hub or `/suggest`. Reserve an upstream issue or PR for a concrete problem or a bounded integration change after alignment. This is an independent community effort. [Horizon contribution policy](https://github.com/horizon-rust/horizon/blob/main/CONTRIBUTING.md) · [Horizon mod formats](https://github.com/horizon-rust/horizon/blob/main/docs/mods.md)

## Licensing and redistribution

| Boundary | Treatment |
| --- | --- |
| Lab source | The lab's existing MIT license applies to its original contributions. Preserve human and third-party notices where they apply. |
| Mekanism reference | The exact `v1.21.1-10.7.19.85` tag is MIT. Retain its copyright and permission notice for copied or substantial portions. Check file-specific notices and third-party provenance before reusing art, sounds, documentation, code, or assets. The first release references the original distribution by source and hash. |
| Horizon integration | Horizon's current repository license is MIT. Check the pinned adapter revision and dependencies when integration starts. Confirm the native interface, target game version and ownership of any proposed upstream change with maintainers. |
| Runtime dependencies | Resolve Minecraft through authorized sources and obtain the NeoForge/JDK dependencies locally. The pinned NeoForge 21.1.200 Maven metadata declares LGPL 2.1. Review dependency-specific terms and preserve required notices before any later dependency distribution. The lab release contains its own code and sanitized evidence summaries. |
| Branding and outreach | Use descriptive names and an independence statement. Confirm any intended use of project logos, official project positioning, curated listings, or project-owned distribution channels with the relevant maintainers. |

The Minecraft EULA restricts redistribution of game files and modded game versions. The repository excludes game and mod JARs, worlds, screenshots, copied resources, private profiles, and raw logs. A source license does not settle every asset's provenance or endorsement. [Tagged Mekanism license](https://github.com/mekanism/Mekanism/blob/v1.21.1-10.7.19.85/LICENSE) · [Horizon license](https://github.com/horizon-rust/horizon/blob/main/LICENSE) · [Minecraft EULA](https://www.minecraft.net/en-us/eula) · [Minecraft Usage Guidelines](https://www.minecraft.net/en-us/usage-guidelines)

## Initial baseline and first PR

Use a separately staged public tree. Include original lab source, build and CI recipes, scenario definitions, the English contributor guide, planning, licenses and reviewed metadata summaries. Inspect the complete staged tree and every reachable commit before pushing. Preserve full local evidence in the private checkout. CI exercises Rust contracts. Game reproduction uses the pinned local Java setup.

After the baseline is versioned, the small onboarding PR should guide a contributor through the existing `cases/server-inventory.json` headless smoke test, `verify-reference`, and one claimable inventory reconciliation task. The barrel fixture exercises setup and sealed evidence. Preserve the Enrichment Chamber pilot as the Mekanism behavior regression. Keep the new runtime catalog and remaining workstreams in separately reviewable slices.

1. Version a reproducible independent lab and small contributor tasks.
2. Invite contributors in Horizon Discord with a link to the lab and its task queue.
3. Review and qualify finite Java reference scopes through the declared gates.
4. Start corresponding gameplay work in a dedicated native mod repository and review the versioned Horizon adapter.

[First-wave workstreams](FIRST-WAVE-WORKSTREAMS.md) · [Initial inventory](FIRST-WAVE-INVENTORY.md) · [English drafts](CONTRIBUTION-DRAFTS.md)
