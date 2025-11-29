# Setup automated publishing workflow for all crates

**Type:** enhancement  
**Status:** in_progress  
**Branch:** feat/crates-io-publishing  
**Linked roadmap section:** v0.x - Publishing and distribution

---

## 🧠 Context
The foxchain workspace contains multiple crates (foxchain, foxchain-id, foxchain-analysis) that need to be published to crates.io. Currently, there's no automated workflow for publishing. We need a GitHub Actions workflow that can publish all crates to crates.io when a release is created or when manually triggered.

## 🎯 Goal
Create a GitHub Actions workflow that automatically publishes all workspace crates to crates.io with proper versioning, dependency management, and error handling.

## 📏 Success Metrics
- [ ] GitHub Actions workflow created for crate publishing
- [ ] Workflow supports manual trigger (workflow_dispatch)
- [ ] Workflow supports automatic trigger on release/tag
- [ ] All workspace crates are published in correct order (dependencies first)
- [ ] Proper error handling and rollback if publishing fails
- [ ] Version validation before publishing
- [ ] Documentation on how to use the workflow

## 🧩 Acceptance Criteria
- [ ] Workflow file created in `.github/workflows/`
- [ ] Workflow publishes crates in dependency order (foxchain-id, foxchain-analysis, foxchain)
- [ ] Uses CARGO_REGISTRY_TOKEN from GitHub secrets
- [ ] Validates crate versions before publishing
- [ ] Handles publishing failures gracefully
- [ ] Supports both manual and automatic triggers
- [ ] Documentation added on publishing process
- [ ] CHANGELOG entry added

## 🛠️ Implementation Outline
1. Create/switch to branch `feat/crates-io-publishing`
2. Create `.github/workflows/publish.yml` workflow
3. Configure workflow to:
   - Trigger on release creation and manual dispatch
   - Extract version from tag or use version from Cargo.toml
   - Publish crates in dependency order
   - Use CARGO_REGISTRY_TOKEN secret
   - Validate versions before publishing
   - Handle errors and provide clear feedback
4. Add workflow documentation
5. Update main README with publishing instructions
6. Move this file to `in_progress/` then `done/`
7. Create PR referencing this issue

## 🔍 Alternatives Considered
- Manual publishing → Rejected: Error-prone, time-consuming
- Single workflow for all crates → Accepted: Simpler, ensures correct order
- Separate workflows per crate → Rejected: More complex, harder to coordinate

## ⚠️ Risks / Mitigations
- Publishing wrong version → Mitigation: Version validation before publishing
- Dependency order issues → Mitigation: Explicit order in workflow (dependencies first)
- Token security → Mitigation: Use GitHub secrets, never expose in logs
- Partial publishing failure → Mitigation: Error handling and clear reporting

## 🔗 Discussion Notes
The workflow should publish crates in dependency order:
1. foxchain-id (no dependencies on other workspace crates)
2. foxchain-analysis (depends on foxchain-id)
3. foxchain (depends on both)

This ensures dependencies are available when dependent crates are published.

