# Windows ZIP packaging dry run

Use this proof for a reviewed packaging change and again at the exact revision proposed for a release. It shares the Windows packager's build, license notices, archive layout, and verification. The output name contains `dry-run` and the complete source SHA. The report records `publication_candidate=false` and `qualified=false`. Release preparation still requires the dedicated Java smoke, sealed reference trace, processing evidence, and publication review in the [release policy](README.md#rebuild-and-review).

## Local proof

Use Windows x64, PowerShell 7, Rust 1.95.0, and a disposable clean checkout at the expected revision. Use a new output directory on every attempt. Point `CARGO_HOME` at a cache containing the locked crates and index. If the cache needs preparation, run `cargo fetch --locked --target x86_64-pc-windows-msvc` with the pinned toolchain in a separate step. The dry run uses `--frozen` for build and metadata. It requires neither Java nor an authenticated GitHub session.

```powershell
./tools/test-preview-zip.ps1 -OutputDirectory .lab/packaging/zip-controls-01
./tools/package-preview.ps1 -DryRun -ExpectedCommit $reviewedSha `
  -OutputDirectory .lab/packaging/dry-run-01
```

Set `$reviewedSha` to the full 40-character source SHA selected by review or CI. The dry run rejects a different HEAD and tracked or untracked working-tree changes outside ignored paths. It checks again before writing the report. Keep the checkout reserved for that attempt.

The verifier compares the ZIP member names with the pre-archive staging inventory before extraction. The inventory consists of the two executables, the dedicated smoke case, install and release notes, the project license, locked dependency notices, and the manifest. The verifier rejects unexpected, missing, duplicate, case-aliased, or non-regular entries. It checks the fresh install against the trusted staging manifest digest, file lengths, and file hashes. `SHA256SUMS` covers the output ZIP. The test script verifies a clean fixture and exercises checksum, allowlist, manifest, and payload failures.

The extracted `mek-lab.exe` runs the offline observer simulation. The extracted `mek-work.exe` must start and return its expected missing-argument diagnostic and exit code. This probe covers the analyzer's CLI startup. Full analyzer processing stays in the [Java-backed release check](README.md#rebuild-and-review).

## CI integration contract

- Route packaging scripts, packaging tests, and changes to packaged inputs through the repository's existing CI routing. The release maintainer can run the same command at a reviewed candidate SHA. Leave scheduled and unrelated builds out of this proof.
- On a GitHub-hosted Windows x64 runner, pin the toolchain and checkout Action, disable persisted checkout credentials, and grant `contents: read`. Pass the exact checkout SHA selected by the caller. Supply no release credentials, private runtime files, or game downloads. The CI owner decides where the job is called and how its result is reported.
- Run the archive controls and the dry run in fresh ignored directories. Keep the resulting `verification.json` and `SHA256SUMS` as compact job evidence if needed. The ZIP and scratch output can remain on the ephemeral runner. A passing job must report the expected source SHA and target, a verified extraction, passing simulation, analyzer usage exit 1, and both qualification flags false.
- Keep the required contributor check and the release publication gates in place. The offline proof grants no release authority.
