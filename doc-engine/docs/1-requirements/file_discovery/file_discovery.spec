# Feature Spec: File Discovery

**Version:** 1.0
**Status:** Draft
**Section:** 4.2

## Requirements

| ID | Source | Title | Priority | Verification | Acceptance |
|-----|--------|-------|----------|--------------|------------|
| REQ-001 | FR-200 | Recursive scanning | Must | Test | Given a project with nested directories 5 levels deep, all files are discovered |
| REQ-002 | FR-201 | Directory exclusions | Must | Test | Files inside `.git/`, `target/`, and `node_modules/` are not included in the file list |
| REQ-003 | FR-202 | Relative paths | Must | Test | All paths in the file list are relative (no leading `/` or absolute prefix) |

## Acceptance Criteria

- **REQ-001** (FR-200): Given a project with nested directories 5 levels deep, all files are discovered
- **REQ-002** (FR-201): Files inside `.git/`, `target/`, and `node_modules/` are not included in the file list
- **REQ-003** (FR-202): All paths in the file list are relative (no leading `/` or absolute prefix)

