# Architecture

A case describes the topology, preconditions, ordered actions and observations for one finite scope. The Rust runner checks the case and reference lock, creates private instance directories and supervises the processes. [Cases](../../cases) · [Lab runner](../../src/lab.rs) · [Process supervision](../../src/process.rs)

The NeoForge bridge exposes the control and observation paths for the pinned Java runtime. The runner checks the relevant phase and correlates applied actions with observations. Client cases use a hidden real client. Server administration, client keybinding and Screen ingress, and OS input are distinct proof levels. [NeoForge adapter](../../adapters/neoforge) · [Functional lab](../planning/FUNCTIONAL-LAB.md)

The comparator works with typed traces and reports the first mismatch. For applicable cases, independent persisted NBT and inventory observations corroborate the observer. Sealed reference traces bind an attempt to its case and locked execution. [Comparison](../../src/lib.rs) · [Capture](../../src/capture.rs) · [World witness](../../src/witness.rs) · [Validator contract](../../VALIDATOR-CONTRACT.md)

The lab runs as an independent Cargo workspace. Java serves as the original-runtime reference. A future native mod targets a pinned Horizon runtime through an adapter after the applicable scope qualifies. [Roadmap gates](../planning/ROADMAP.md#gates-by-scope-and-proof-level)

[Wiki index](../WIKI.md)
