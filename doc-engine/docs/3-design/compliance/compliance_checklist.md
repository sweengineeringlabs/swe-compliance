# Compliance Checklist

**Audience**: Developers, architects

This checklist tracks doc-engine's compliance against its own documentation framework rules. See [architecture.md](../architecture.md) for the system design.

## Structure (Checks 1-13)

- [ ] Check 1: Root docs/ folder exists
- [ ] Check 2: docs/README.md hub document exists
- [ ] Check 3: docs/glossary.md exists
- [ ] Check 4: All module doc folders use docs/ (plural)
- [ ] Check 5: No module has both doc/ and docs/
- [ ] Check 6: docs/3-design/compliance/compliance_checklist.md exists
- [ ] Check 7: Compliance checklist references architecture.md
- [ ] Check 8: Every enforceable rule has a checkbox
- [ ] Check 9: SDLC phase numbering correct
- [ ] Check 10: SDLC phases in correct order
- [ ] Check 11: ADR directory exists
- [ ] Check 12: Developer guides in guide/ (singular)
- [ ] Check 13: UX/UI assets in uxui/

## Naming (Checks 14-25)

- [ ] Check 14: README.md exists (uppercase)
- [ ] Check 15: CONTRIBUTING.md exists (uppercase)
- [ ] Check 16: CHANGELOG.md exists (uppercase)
- [ ] Check 17: SECURITY.md exists (uppercase)
- [ ] Check 18: LICENSE file exists (uppercase)
- [ ] Check 19: .gitignore exists
- [ ] Check 20: .editorconfig exists
- [ ] Check 21: All filenames in docs/ are lowercase
- [ ] Check 22: All filenames use underscores
- [ ] Check 23: No spaces in filenames
- [ ] Check 24: Guide files follow naming convention
- [ ] Check 25: No testing files outside 5-testing/

## Root Files (Checks 26-32)

- [ ] Check 26: README.md exists
- [ ] Check 27: CONTRIBUTING.md exists
- [ ] Check 28: CHANGELOG.md exists
- [ ] Check 29: SECURITY.md exists
- [ ] Check 30: LICENSE exists
- [ ] Check 31: CODE_OF_CONDUCT.md and SUPPORT.md exist
- [ ] Check 32: GitHub issue/PR templates exist

## Content (Checks 33-39)

- [ ] Check 33: Every .md file has Audience declaration
- [ ] Check 34: Module README.md has Audience declaration
- [ ] Check 35: Long docs have TLDR section
- [ ] Check 36: Short docs do not need TLDR
- [ ] Check 37: Glossary uses correct format
- [ ] Check 38: Glossary terms alphabetized
- [ ] Check 39: Glossary acronyms expanded

## Navigation (Checks 40-43)

- [ ] Check 40: Root README links to docs/README.md
- [ ] Check 41: docs/README.md uses W3H structure
- [ ] Check 42: Hub links to all SDLC phase directories
- [ ] Check 43: Root README no deep-links into docs/

## Cross-References (Checks 44-47)

- [ ] Check 44: All internal markdown links resolve
- [ ] Check 45: All relative links are valid
- [ ] Check 46: All references use docs/ (plural)
- [ ] Check 47: All references use guide/ (singular)

## ADR (Checks 48-50)

- [ ] Check 48: ADR index file exists
- [ ] Check 49: ADR files follow NNN-title.md convention
- [ ] Check 50: ADR index references all ADR files
