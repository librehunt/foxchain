# Fix Cargo Publish Metadata and Lock File

**Type**: Bug 🐞  
**Created**: 2025-12-03  
**Status**: in_progress  
**Priority**: High

## Summary

The cargo publish workflow is failing with two issues:
1. Missing package metadata (documentation, homepage, repository) in Cargo.toml
2. Cargo.lock has uncommitted changes

## Problem

When running `cargo publish`, it fails with:
- Warning: manifest has no documentation, homepage or repository
- Error: Cargo.lock contains uncommitted changes

## Proposed Solution

1. Add missing metadata to Cargo.toml:
   - `repository = "https://github.com/librehunt/foxchain"`
   - `homepage = "https://github.com/librehunt/foxchain"`
   - `documentation = "https://docs.rs/foxchain-id"`
   - `keywords` and `categories` for better crates.io discoverability

2. Ensure Cargo.lock is committed to the repository

## Acceptance Criteria

- [ ] Cargo.toml includes all required metadata fields
- [ ] Cargo.lock is committed
- [ ] `cargo publish --dry-run` passes without warnings
- [ ] Publishing workflow can complete successfully

