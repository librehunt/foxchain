# Update Repository URLs to foxchain-id

**Type**: Enhancement ✨  
**Created**: 2025-12-03  
**Status: in_progress  
**Priority**: Medium

## Summary

Repository has been renamed from `foxchain` to `foxchain-id`. Need to update all repository URLs in configuration and documentation files.

## Problem

Repository URLs still reference the old name `foxchain` instead of `foxchain-id`:
- Cargo.toml has `repository = "https://github.com/librehunt/foxchain"`
- Other files may reference the old URL

## Proposed Solution

Update all repository URLs from:
- `https://github.com/librehunt/foxchain` → `https://github.com/librehunt/foxchain-id`

Files to update:
- `Cargo.toml` - repository and homepage URLs
- `AGENT.md` - repository URLs
- `README.md` - badges and links (if any)
- Any other documentation files

## Acceptance Criteria

- [ ] All repository URLs updated to foxchain-id
- [ ] Cargo.toml has correct repository URL
- [ ] Documentation files updated
- [ ] No references to old repository name remain

