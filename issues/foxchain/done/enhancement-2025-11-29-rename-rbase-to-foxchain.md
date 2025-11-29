# Rename project from rbase to foxchain

**Type:** enhancement  
**Status:** done  
**Branch:** refactor/rename-rbase-to-foxchain  
**Linked roadmap section:** Initial setup

---

## 🧠 Context
The project was initialized from a template called "rbase" but the actual project name is "foxchain". Currently, the package name in Cargo.toml is "rbase", and there are multiple references to "rbase" in README.md (badges, links) and CHANGELOG.md (URLs). These need to be updated to reflect the correct project name "foxchain".

## 🎯 Goal
Rename all references from "rbase" to "foxchain" to accurately reflect the project identity and ensure consistency across all project files.

## 📏 Success Metrics
- [ ] Package name in Cargo.toml changed to "foxchain"
- [ ] All README.md references updated (badges, links)
- [ ] All CHANGELOG.md references updated
- [ ] No remaining "rbase" references in codebase (case-insensitive)

## 🧩 Acceptance Criteria
- [ ] Cargo.toml package name is "foxchain"
- [ ] README.md badges and links reference "foxchain" or correct repository
- [ ] CHANGELOG.md URLs reference correct repository
- [ ] Project builds successfully with new name
- [ ] No regressions in functionality
- [ ] CHANGELOG entry added

## 🛠️ Implementation Outline
1. Create/switch to branch `refactor/rename-rbase-to-foxchain`
2. Update Cargo.toml: change package name from "rbase" to "foxchain"
3. Update README.md: replace all "rbase" references in badges and links with "foxchain" or correct repository paths
4. Update CHANGELOG.md: replace "rbase" references in URLs with "foxchain" or correct repository paths
5. Verify no other files contain "rbase" references
6. Test that project builds successfully
7. Move this file to `in_progress/` then `done/`
8. Create PR referencing this issue

## 🔍 Alternatives Considered
- Keep "rbase" name → Rejected: Project name should match actual purpose and identity
- Partial rename → Rejected: Inconsistent naming causes confusion

## ⚠️ Risks / Mitigations
- Breaking changes for existing users → Mitigation: This is early stage, no published releases yet
- CI/CD may need updates → Mitigation: Verify GitHub Actions workflows after rename

## 🔗 Discussion Notes
Standard project initialization task to align project name with repository and purpose.

