<!-- Repo-Specific Agent Rules Template -->
# Repository Agent Guide

## Overview

- **Repository Name**: foxchain-id
- **Purpose**: Multi-chain blockchain address identification library. Provides functionality to identify which blockchain(s) an input string (address, public key, or private key) belongs to.
- **Primary Owners**: LSH <github@lsh.tech>
- **Tech Stack**: Rust (2021 edition), single crate structure

## Alignment With Global Rules

- This repository inherits the global rules from `~/.cursor/AGENT_GLOBAL.md`.
- Deviations or extensions MUST be documented below with rationale.
- Artifact storage is discovered via `~/.flowmates/config.json` → `{flowmates_repo}/projects/{repo-identifier}/` (centralized in flowmates repository).

## Repository Layout

| Path | Description | Notes |
| --- | --- | --- |
| `src/` | Core identification logic | Metadata-driven detection pipeline |
| `docs/` | Format documentation | Detailed docs for each address format |
| `metadata/` | Chain and pipeline metadata | JSON definitions for chains, curves, pipelines |
| `issues/foxchain/` | Issue workflow | proposal/, todo/, in_progress/, done/ |
| `issues/shared/templates/` | Issue templates | Bug, enhancement, feature, proposal templates |

## Domain Conventions

- **Metadata-driven architecture**: All chain definitions and format logic are in JSON metadata files, not hardcoded
- **Multiple candidates**: The `identify()` function returns all valid candidates sorted by confidence (supports ambiguous inputs)
- **Pipeline-based derivation**: Address derivation from public keys uses pipeline definitions in `metadata/pipelines/`
- **Registry system**: Automatic grouping and matching via `CategorySignature` and `Registry`
- **Naming**: Chain IDs use lowercase strings (e.g., "ethereum", "bitcoin", "cosmos")
- **Error handling**: Uses `foxchain_id::Error` enum with `NotImplemented` and `InvalidInput` variants

## Build & Test Commands

- `cargo test` - Run all tests
- `cargo fmt --all` - Format all code
- `cargo clippy --all-targets --all-features -- -D warnings` - Lint with clippy
- `cargo build` - Build the crate
- `cargo check` - Check compilation without building

## Documentation Touchpoints

- `README.md`: Project overview, quick start, usage examples
- `docs/`: Format-specific documentation (EVM, Bitcoin, Cosmos, etc.)
- `docs/supported-chains.md`: Complete reference of all 29 supported chains
- `docs/examples.md`: Usage examples
- `CHANGELOG.md`: Release notes and changes

## Repo-Specific Agents

No additional agents defined. Uses global agent configuration.

## Exceptions to Global Rules

- **Test coverage**: Currently at 85.71% (target may vary by module)
- **Documentation**: Format documentation is comprehensive but examples may need validation against implementation (see proposal: `enhancement-2025-12-01-validate-documentation-examples-against-implementation.md`)

## Onboarding Checklist

- [ ] Clone repository and run `cargo build`
- [ ] Run `cargo test` to verify all tests pass
- [ ] Read `README.md`
- [ ] Review `docs/supported-chains.md` for supported formats
- [ ] Check `issues/foxchain/proposal/` for active proposals
- [ ] Review metadata structure in `metadata/`

## Appendix

- **GitHub**: https://github.com/librehunt/foxchain-id
- **Crates.io**: https://crates.io/crates/foxchain-id
- **Documentation**: https://docs.rs/foxchain-id
- **CI/CD**: GitHub Actions (see `.github/workflows/`)
- **Publishing**: Automated via GitHub Actions on release creation
- **Current Branch**: `refactor/only-foxchain-id`
