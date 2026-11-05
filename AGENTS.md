# Mekanism Rust Lab

This is an independent Cargo workspace for the Rust validator and a future Mekanism port targeting Horizon. Do not modify Horizon's source or join its Cargo workspace to implement the laboratory. An eventual Horizon adapter is a versioned runtime boundary. No port gameplay starts before its applicable qualification gate.

- Preserve the user's prohibition on automatic attribution, signatures and coauthor trailers.
- Write meaningful failing tests before implementing the corresponding behavior. Keep compilation/setup failures separate from intended assertion failures. Preserve RED/GREEN evidence without updating expected values to fit new code.
- Run `cargo fmt --check`, `cargo clippy --locked --all-targets -- -D warnings`, and `cargo test --locked`. Socket tests need local loopback access. Use a separate target directory while a game campaign executes the main binary on Windows.
- Every game gets its own gameDir, configs, saves, logs, ports and temporary files. Never use player worlds, launchers or credentials. Keep downloaded jars, resources, native binaries and generated worlds under ignored `.lab/`.
- Do not claim a server fixture validates player input. Current client ingress is keybinding/Screen handling, not operating-system input. Check effects on the server and client; acknowledgments and screenshots alone are insufficient.
- Never claim performance improvement from language choice, health checks, missing work, incompatible outputs or a single run. Functional qualification and measured optimization are distinct.
- No automatic golden update, artifact-hash bypass, mutation retry, broad normalization or fallback to a lower proof level.
- Upstream Horizon claims/PRs belong to actual Horizon contributions. This local laboratory does not authorize external issues, comments, forks, pushes or publication.
