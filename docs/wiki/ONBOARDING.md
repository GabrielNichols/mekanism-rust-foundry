# Onboarding

1. Read [Start Here](https://github.com/GabrielNichols/mekanism-rust-foundry/issues/8), then check [open, unassigned Ready work](https://github.com/users/GabrielNichols/projects/2/views/3?filterQuery=status%3AReady%20is%3Aissue%20is%3Aopen%20no%3Aassignee). Choose a scoped work issue, such as a catalog row, a fixture draft or a validator control.
2. Confirm availability with the maintainer on the issue. Name the stable ID, the smallest reviewable result, the negative control and what you still need. Use [Q&A](https://github.com/GabrielNichols/mekanism-rust-foundry/discussions/categories/q-a) for setup or reproduction questions. [First-wave workstreams](../planning/FIRST-WAVE-WORKSTREAMS.md)
3. Use a separate Foundry checkout with private `.lab/` directories. Follow the [Windows x64 quickstart](../../README.md#quickstart-on-windows-x64) to build with the lockfile, prepare the pinned Java reference, run the dedicated barrel inventory case in a fresh directory, and verify its trace. [Platform status](PLATFORM-STATUS.md)
4. Submit one reviewable slice. A design-only PR lists open runtime bindings and the intended witness. A runtime PR links commands, profile, evidence and controls. Preserve raw outputs and failed attempts under `.lab/`. [Contribution guide](../../CONTRIBUTING.md) · [PR template](../../.github/PULL_REQUEST_TEMPLATE.md)

Use `Refs #N` while the linked issue still needs evidence. Reserve `Closes #N` for work that satisfies the issue's reviewed acceptance. [Qualification gates](../planning/ROADMAP.md#gates-by-scope-and-proof-level)

[Wiki index](../WIKI.md)
