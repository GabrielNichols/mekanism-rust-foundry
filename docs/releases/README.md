# Parity Lab Preview releases

Mekanism Rust Foundry is a community project targeting a future native Horizon mod. Parity Lab previews package tools for characterizing the pinned Java reference and validating evidence. The lab has no Rust gameplay implementation.

## Versions and artifact policy

- The first preview is tagged `v0.1.0-preview.1`. The core version tracks the Cargo workspace. Increment the preview suffix for distinct builds of that core version. Publish a new tag for fixes and leave earlier downloads and checksums intact.
- GitHub Releases carry the installation-tested Windows x64 ZIP, `SHA256SUMS`, and a sanitized `verification.json` report. The ZIP contains the two lab executables, the smoke case, the license, dependency notices, release notes, and a manifest with file hashes and the source commit.
- A registry package needs a concrete consumer, compatibility contract, release automation, and provenance review. Keep Cargo dependencies pinned in the workspace. The current consumer is the command-line contributor, served by GitHub Releases. A crates.io library package or installer package remains pending a consumer and supported API or installer contract.
- Future native Horizon mod distribution is a separate deliverable gated on pre-port qualification, a pinned Horizon integration, and review of the target mod channel. These executables are lab tools.
- Obtain Minecraft, NeoForge, Mekanism, and the JDK privately using the source quickstart. Release archives exclude their binaries and assets, worlds, captures, and raw run outputs. The `licenses/` directory reproduces Rust dependency notices for the compiled executables.

## Install and verify

The tested binary target is `x86_64-pc-windows-msvc`. Use PowerShell 7 on Windows x64. For offline verification or simulation, the extracted executable is sufficient. A dedicated Java reference scenario also needs a checkout at the matching release tag and the locally prepared Java reference.

1. Download the Windows ZIP and `SHA256SUMS` from the same release. Compare the SHA-256 shown by `Get-FileHash -Algorithm SHA256 <zip>` with the entry for the ZIP.
2. Extract to a new directory. Run `mek-lab.exe simulate-observer <new-local-output-directory>`. Confirm `result.json` says `pass` and leaves `qualified` false.
3. For a real scenario, clone the source repository at the release tag. In that checkout, use its preparation scripts and the extracted runner. Replace the absolute paths with your local paths.

```powershell
$runner = 'C:\Tools\foundry-preview\mek-lab.exe'
$jdk = 'C:\Tools\jdk-21'
./adapters/neoforge/prepare-reference.ps1 -Jdk $jdk -Runner $runner
& $runner run-reference .lab/environment.json cases/server-inventory.json .lab/evidence/preview-first-run
& $runner verify-reference .lab/evidence/preview-first-run
```

A successful reference attempt reports `functional_pass` and a nonempty trace seal. The barrel fixture proves the dedicated runner path. Use a different output directory on every attempt.

## Rebuild and review

Build from the release commit with Windows x64 PowerShell 7 and Rust 1.95.0. Prepare a private reference environment and retain a completed processing run compatible with the work ledger.

```powershell
./tools/package-preview.ps1 -OutputDirectory .lab/releases/v0.1.0-preview.1 `
  -ReferenceEnvironment .lab/environment.json `
  -WorkEvidence .lab/evidence/processing-cohort/detail-1
```

The script builds with the lockfile, creates the ZIP from an explicit allowlist, extracts it, checks its file hashes, runs observer simulation and the dedicated Java reference with the extracted runner, then runs the extracted analyzer on processing evidence. The dedicated test binds the extracted runner's exact hash while retaining every private runtime pin. Compare the report's source commit with the GitHub release tag and require `publication_candidate=true`. Use `-LocalProof` for an unpublished test of working-copy release changes. Package layout and ZIP metadata are deterministic for the same input files. Cross-host binary reproducibility is unverified.
