# Software Requirements Specification: swe-compliance-frontend

**Audience**: Developers, architects, project stakeholders, compliance officers

## TLDR

This SRS defines requirements for swe-compliance-frontend, a web-based dashboard and management interface for the swe-compliance ecosystem. It unifies doc-engine (128 documentation compliance checks, spec scaffolding, AI-powered audit/chat/command generation), template-engine (W3H documentation framework, 56-point compliance checklist, SDLC phase templates), and struct-engine (44 Rust structure compliance checks) under a single browser-based UI. The frontend exposes project management, real-time compliance scanning, violation browsing, scaffolding, template previewing, report generation, AI-powered compliance analysis, SRS editing, spec file viewing, and struct-engine integration through a REST/WebSocket API layer. It covers stakeholder needs, functional requirements across 12 domains, non-functional requirements for performance, accessibility, security, and maintainability, and traceability from stakeholder goals to UI components and API endpoints.

**Version**: 1.0
**Date**: 2026-02-12
**Standard**: ISO/IEC/IEEE 29148:2018

---

## 1. Introduction

### 1.1 Purpose

This SRS defines the stakeholder, system, and software requirements for **swe-compliance-frontend**, a web application that provides a unified browser-based interface to the swe-compliance ecosystem. The frontend consumes the library APIs of doc-engine (`scan_with_config`, `scaffold_from_srs`, `ComplianceChat`, `ComplianceAuditor`, `CommandGenerator`), struct-engine (`scan_with_config`), and template-engine (static template assets) through a backend API layer, presenting compliance results, scaffolding controls, AI-powered analysis, and report generation in an interactive dashboard.

### 1.2 Scope

swe-compliance-frontend is a web application within the `swe-compliance` workspace. It:

- Provides a browser-based dashboard for managing compliance projects
- Triggers doc-engine scans (128 checks across 18 categories) and struct-engine scans (44 checks across 7 categories) from the UI
- Displays scan results with filtering, sorting, and drill-down by category, severity, and module
- Provides a scaffolding interface for generating SDLC spec files from SRS documents
- Integrates template-engine templates for browsing, previewing, and applying W3H documentation patterns
- Exposes AI-powered compliance chat, audit analysis, and command generation through interactive UI components
- Generates and exports compliance reports in JSON, markdown, and PDF formats
- Provides an SRS markdown editor with live preview and doc-engine format validation
- Exposes a REST/WebSocket API for all backend operations

swe-compliance-frontend does **not**:

- Replace the doc-engine or struct-engine CLI tools (those remain first-class interfaces)
- Implement its own compliance checking logic (all checks are delegated to the engine libraries)
- Require direct file system access from the browser (all file operations go through the API layer)

### 1.3 Definitions and Acronyms

| Term | Definition |
|------|-----------|
| **doc-engine** | Rust CLI and library that audits documentation against 128 compliance checks; includes scan, scaffold, and AI subsystems |
| **struct-engine** | Rust CLI and library that audits Rust project structure against 44 compliance checks |
| **template-engine** | Documentation template framework providing W3H structure, 56-point checklist, and SDLC phase templates |
| **W3H** | WHO-WHAT-WHY-HOW -- documentation structure pattern from template-engine |
| **ScanReport** | JSON structure returned by `scan_with_config()` containing per-check results, summary, and ISO 15289 metadata |
| **ScaffoldResult** | JSON structure returned by `scaffold_from_srs()` containing created/skipped files and domain/requirement counts |
| **AuditResponse** | JSON structure from compliance-audit containing LLM summary, raw scan results, and recommendations |
| **SDLC** | Software Development Life Cycle -- phases 0-7 mapped to directory names |
| **SEA** | Stratified Encapsulation Architecture -- layered module pattern used by the engines |
| **BRD** | Business Requirements Document -- master inventory of domain specs |
| **Spec file** | Documentation artifact using domain-specific extensions (`.spec`, `.arch`, `.test`, `.deploy`) in YAML or markdown format |
| **Project scope** | Tier that determines which rule subset applies: Small, Medium, Large |
| **WebSocket** | Full-duplex communication protocol for real-time scan progress and AI chat streaming |
| **REST** | Representational State Transfer -- HTTP-based API style for CRUD operations |
| **JWT** | JSON Web Token -- stateless authentication token for API access |
| **SSE** | Server-Sent Events -- one-way server-to-client streaming for scan progress |

### 1.4 References

| Document | Location |
|----------|----------|
| doc-engine SRS | `doc-engine/docs/1-requirements/srs.md` |
| struct-engine SRS | `struct-engine/docs/srs.md` |
| doc-engine Architecture | `doc-engine/docs/3-design/architecture.md` |
| ISO/IEC/IEEE 29148:2018 | Requirements engineering standard (this document conforms to) |
| ISO/IEC/IEEE 15289:2019 | Content of life-cycle information items (audit status report format) |
| ISO/IEC/IEEE 29119-3:2021 | Software testing -- Part 3: Test documentation |
| Documentation Framework | `swe-labs/template-engine/templates/framework.md` |
| Compliance Checklist | `swe-labs/template-engine/templates/compliance-checklist.md` |
| doc-engine Scan API Types | `doc-engine/scan/main/src/api/types.rs` |
| doc-engine Scaffold API Types | `doc-engine/scaffold/src/api/types.rs` |
| Compliance Audit API Types | `doc-engine/compliance-audit/src/api/types.rs` |
| Compliance Chat API Types | `doc-engine/compliance-chat/src/api/types.rs` |
| Command Generator API Types | `doc-engine/command-generator/src/api/types.rs` |

---

## 2. Stakeholder Requirements (StRS)

### 2.1 Stakeholders

| Stakeholder | Role | Needs |
|-------------|------|-------|
| Developer | Runs scans and views results in browser | Visual compliance feedback, quick violation navigation, one-click scan triggers |
| Architect | Audits projects, reviews compliance posture | Cross-project dashboards, trend analysis, category breakdowns, ISO mapping |
| Compliance officer | Generates audit reports for external review | PDF/markdown report export, historical comparison, ISO 15289 compliance evidence |
| Documentation maintainer | Creates and manages SRS documents, templates | SRS editor with live preview, template browser, scaffold controls |
| AI user | Leverages LLM for compliance interpretation | Chat interface, audit summaries, command generation UI |
| CI/CD integration | Automated scan triggering and result consumption | REST API for programmatic access, webhook notifications, JSON responses |
| Project manager | Tracks compliance progress across projects | Multi-project dashboard, trend charts, status summary |

### 2.2 Operational Scenarios

#### OS-1: Developer reviews scan results

A developer opens the dashboard, selects their project, and clicks "Run Scan" with scope "medium". The UI shows a progress indicator during the scan. When complete, the dashboard displays 128 check results grouped by category with pass/fail/skip counts. The developer filters to "failed" checks, clicks a violation to see the file path and fix guidance, and resolves issues.

#### OS-2: Architect generates compliance report

An architect selects a project, reviews the current compliance status on the dashboard, clicks "Generate Report", selects PDF format with ISO 15289 metadata, and downloads the report. They compare it against last month's report using the historical comparison view.

#### OS-3: Developer scaffolds spec files from SRS

A developer pastes or uploads their SRS markdown into the SRS editor. The editor validates the document structure in real-time. They click "Preview Scaffold" to see the list of files that will be generated. After selecting phases and file types, they click "Generate" and monitor the scaffold progress. The generated files appear in the spec file viewer.

#### OS-4: Compliance officer uses AI audit

A compliance officer selects a project, navigates to the AI section, and clicks "Run AI Audit". The system executes a compliance scan, sends results to the LLM, and displays a prioritized summary with actionable recommendations. The officer exports the AI audit report alongside the raw scan results.

#### OS-5: Documentation maintainer browses templates

A documentation maintainer opens the template browser, filters by SDLC phase, and previews a test plan template with W3H structure. They copy the template to their project, then review the 56-point compliance checklist to track which items are addressed.

#### OS-6: CI system triggers scan via API

A CI pipeline sends `POST /api/v1/scans` with the project path, scope, and project type. The API queues the scan, returns a scan ID, and the CI system polls `GET /api/v1/scans/{id}` until the status is "completed". It then retrieves the JSON results and the audit status report.

#### OS-7: Architect reviews struct-engine results

An architect views a Rust project's structural compliance alongside its documentation compliance. The struct-engine panel shows 44 checks with crate layout visualization, while the doc-engine panel shows 128 documentation checks. Both feed into a unified compliance score.

#### OS-8: Developer uses AI chat for compliance questions

A developer opens the AI chat panel, types "What ISO standards apply to my test documentation?", and receives a streaming response from the compliance-auditor agent. The response includes specific check references and remediation suggestions.

### 2.3 Stakeholder Requirements

| ID | Requirement | Source | Priority | Rationale |
|----|-------------|--------|----------|-----------|
| STK-01 | The frontend shall provide a visual dashboard showing compliance status across projects | Architect, Project manager needs | Must | Replaces command-line-only compliance review |
| STK-02 | The frontend shall trigger doc-engine and struct-engine scans from the browser | Developer needs | Must | Eliminates need to switch to terminal for scans |
| STK-03 | The frontend shall display violations with filtering, sorting, and navigation to source context | Developer needs | Must | Developers need visual tools to locate and prioritize fixes |
| STK-04 | The frontend shall provide scaffolding controls with preview before generation | Architect needs | Should | Prevents accidental file generation; enables selective scaffolding |
| STK-05 | The frontend shall expose AI-powered compliance chat, audit, and command generation | AI user needs | Should | Natural-language interface for compliance interpretation |
| STK-06 | The frontend shall generate exportable compliance reports in multiple formats | Compliance officer needs | Must | Compliance evidence must be distributable outside the tool |
| STK-07 | The frontend shall provide a REST/WebSocket API for all operations | CI/CD integration needs | Must | Enables automation and external system integration |
| STK-08 | The frontend shall include an SRS markdown editor with live validation | Documentation maintainer needs | Should | In-browser editing reduces context switching |
| STK-09 | The frontend shall display template-engine templates with W3H structure preview | Documentation maintainer needs | Should | Visual template browsing accelerates documentation creation |
| STK-10 | The frontend shall provide struct-engine integration for Rust projects | Architect needs | Should | Unified view of documentation and structural compliance |
| STK-11 | The frontend shall support multi-project management | Project manager needs | Must | Organizations manage compliance across multiple codebases |
| STK-12 | The frontend shall show historical compliance trends | Architect, Compliance officer needs | Should | Trend analysis demonstrates compliance improvement over time |

---

## 3. System Requirements (SyRS)

### 3.1 System Context

```
doc-engine (Rust library)                 struct-engine (Rust library)
├── scan_with_config()                    ├── scan_with_config()
├── scaffold_from_srs()                   └── format_report_json()
├── ComplianceChat::chat()
├── ComplianceAuditor::audit()
├── CommandGenerator::generate_commands()
└── format_report_json()
         │                                         │
         ▼                                         ▼
┌─────────────────────────────────────────────────────────┐
│           swe-compliance-frontend API layer              │
│  REST endpoints + WebSocket channels                     │
│  Authentication (JWT) + Session management                │
└──────────────────────┬──────────────────────────────────┘
                       │
                       ▼
┌──────────────────────────────────────────────────────────┐
│           swe-compliance-frontend Web UI                  │
│  Dashboard │ Scan │ Violations │ Scaffold │ AI │ Reports  │
└──────────────────────────────────────────────────────────┘
                       │
                       ▼
              Browser (user)

template-engine/templates/
├── framework.md
├── compliance-checklist.md
└── phase templates
         │
         ▼
    Static assets served by API layer
```

### 3.2 System Functions

| ID | Function | Description |
|----|----------|-------------|
| SYS-01 | Project management | Create, configure, list, and delete compliance projects |
| SYS-02 | Scan execution | Trigger doc-engine and struct-engine scans via library APIs |
| SYS-03 | Result display | Present scan results with grouping, filtering, and drill-down |
| SYS-04 | Scaffolding | Parse SRS documents and generate SDLC spec files via scaffold library |
| SYS-05 | Template serving | Serve template-engine templates for browsing and preview |
| SYS-06 | Report generation | Export compliance results as JSON, markdown, and PDF |
| SYS-07 | AI integration | Proxy compliance chat, audit, and command generation to AI crates |
| SYS-08 | SRS editing | Provide markdown editor with real-time validation and preview |
| SYS-09 | Spec file browsing | Display YAML and markdown spec files with cross-references |
| SYS-10 | API layer | Expose all functions as REST/WebSocket endpoints with authentication |
| SYS-11 | Historical tracking | Store scan results over time for trend analysis |

### 3.3 System Constraints

- **Backend language**: Rust (2021 edition) for API server, leveraging existing engine crates as library dependencies
- **Frontend framework**: Web standards (HTML/CSS/JS); framework choice deferred to design phase
- **Transport**: HTTPS for REST, WSS for WebSocket
- **Authentication**: JWT-based stateless authentication
- **No direct file system access from browser**: All file operations proxied through API
- **Engine crates as dependencies**: The API server links against `doc-engine-scan`, `doc-engine-scaffold`, `doc-engine-compliance-chat`, `doc-engine-compliance-audit`, `doc-engine-command-generator`, and `struct-engine` as Cargo dependencies
- **Platform**: Linux, macOS, Windows (server); any modern browser (client)

### 3.4 Assumptions and Dependencies

- doc-engine crates expose stable public APIs via SAF layer (`scan_with_config`, `scaffold_from_srs`, `parse_srs`, `load_command_map`)
- struct-engine exposes `scan_with_config` via SAF layer
- template-engine templates are available as static files on the server file system
- AI features require LLM API keys configured on the server (environment variables)
- External crate dependencies (server): `axum` or `actix-web`, `tokio`, `serde`, `serde_json`, `jsonwebtoken`
- External crate dependencies (client): framework-specific (deferred to design)

---

## 4. Software Requirements (SRS)

### Requirement Attributes

Each requirement includes:

| Attribute | Description |
|-----------|-------------|
| **ID** | Unique identifier (FR-nnn for functional, NFR-nnn for non-functional) |
| **Priority** | Must / Should / Could / Won't (MoSCoW) |
| **State** | Proposed / Approved / Implemented / Verified |
| **Verification** | Test / Inspection / Analysis / Demonstration |
| **Traces to** | Stakeholder requirement (STK-nn), architecture component |
| **Acceptance criteria** | Condition(s) that prove the requirement is met |

### 4.1 Project Management

#### FR-100: Create project

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-11 -> `api/projects.rs`, `ui/projects/` |
| **Acceptance** | `POST /api/v1/projects` with `{"name": "my-project", "root_path": "/path/to/project", "scope": "medium", "project_type": "open_source"}` returns 201 with a project ID; the project appears in `GET /api/v1/projects` |

The frontend shall allow users to create compliance projects by specifying a project name, root file system path on the server, project scope (small/medium/large), and project type (open_source/internal).

#### FR-101: List projects

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-11 -> `api/projects.rs`, `ui/projects/` |
| **Acceptance** | `GET /api/v1/projects` returns a JSON array of project objects with id, name, root_path, scope, project_type, last_scan_timestamp, and compliance_summary fields |

#### FR-102: Update project configuration

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-11 -> `api/projects.rs` |
| **Acceptance** | `PATCH /api/v1/projects/{id}` with `{"scope": "large"}` returns 200; subsequent `GET /api/v1/projects/{id}` reflects the updated scope |

#### FR-103: Delete project

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-11 -> `api/projects.rs` |
| **Acceptance** | `DELETE /api/v1/projects/{id}` returns 204; subsequent `GET /api/v1/projects/{id}` returns 404; associated scan history is retained for audit trail |

#### FR-104: Project configuration validation

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-11 -> `api/projects.rs` |
| **Acceptance** | `POST /api/v1/projects` with `{"scope": "invalid"}` returns 422 with a validation error message; `root_path` that does not exist on the server returns 422 |

### 4.2 Compliance Dashboard

#### FR-200: Compliance overview

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Demonstration |
| **Traces to** | STK-01 -> `ui/dashboard/` |
| **Acceptance** | The dashboard page displays for each project: project name, last scan date, total/passed/failed/skipped counts, and a compliance percentage (passed / total * 100); the data is sourced from `GET /api/v1/projects` |

The dashboard shall display a summary card for each project showing its most recent scan results, allowing at-a-glance compliance assessment across all managed projects.

#### FR-201: Category breakdown chart

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Demonstration |
| **Traces to** | STK-01 -> `ui/dashboard/` |
| **Acceptance** | Selecting a project displays a bar or stacked chart with pass/fail/skip counts per category (structure, naming, root_files, content, navigation, cross_ref, adr, traceability, ideation, requirements, planning, design, development, testing, deployment, operations, backlog, module) |

#### FR-202: Trend over time

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Demonstration |
| **Traces to** | STK-12 -> `ui/dashboard/`, `api/scans.rs` |
| **Acceptance** | `GET /api/v1/projects/{id}/trends?period=30d` returns an array of `{timestamp, passed, failed, skipped}` objects; the UI renders a line chart showing compliance score over the selected period |

#### FR-203: Multi-engine summary

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Demonstration |
| **Traces to** | STK-01, STK-10 -> `ui/dashboard/` |
| **Acceptance** | For Rust projects, the dashboard shows both doc-engine (128 checks) and struct-engine (44 checks) results side by side with separate compliance scores and a combined total |

### 4.3 Scan Execution

#### FR-300: Trigger doc-engine scan

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-02 -> `api/scans.rs` |
| **Acceptance** | `POST /api/v1/scans` with `{"project_id": "uuid", "engine": "doc-engine", "scope": "medium"}` returns 202 with `{"scan_id": "uuid", "status": "queued"}`; the scan runs asynchronously using `doc_engine_scan::scan_with_config()` |

The API shall accept scan requests and execute them asynchronously, invoking the doc-engine library's `scan_with_config()` function with the project's configured root path and the requested scope.

#### FR-301: Trigger struct-engine scan

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-02, STK-10 -> `api/scans.rs` |
| **Acceptance** | `POST /api/v1/scans` with `{"project_id": "uuid", "engine": "struct-engine"}` returns 202; the scan runs using `struct_engine::scan_with_config()` |

#### FR-302: Real-time scan progress

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Demonstration |
| **Traces to** | STK-02 -> `api/ws.rs`, `ui/scans/` |
| **Acceptance** | Connecting to `WS /api/v1/scans/{id}/progress` receives JSON messages with `{"check_id": N, "category": "...", "status": "pass|fail|skip"}` as each check completes; the UI updates a progress bar showing completed/total checks |

#### FR-303: Scan result retrieval

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-02, STK-03 -> `api/scans.rs` |
| **Acceptance** | `GET /api/v1/scans/{id}` returns the full `ScanReport` JSON (matching doc-engine's `ScanReport` struct: standard, clause, tool, tool_version, timestamp, project_root, results, summary, project_type, project_scope) with HTTP 200 when complete, or `{"status": "in_progress"}` with HTTP 200 when still running |

#### FR-304: Scan filtering options

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-02 -> `api/scans.rs` |
| **Acceptance** | `POST /api/v1/scans` accepts optional `checks` (e.g., "1-13"), `phase` (e.g., "testing,module"), and `module` (e.g., "scan,cli") fields; these are passed through to `ScanConfig` |

#### FR-305: Scan history

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-12 -> `api/scans.rs` |
| **Acceptance** | `GET /api/v1/projects/{id}/scans` returns an array of past scan summaries ordered by timestamp descending, each containing scan_id, timestamp, engine, summary (passed/failed/skipped), and scope |

### 4.4 Violation Browser

#### FR-400: Violation list

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Demonstration |
| **Traces to** | STK-03 -> `ui/violations/` |
| **Acceptance** | Given a completed scan, the violation browser displays all failed checks with: check ID, category, description, severity (error/warning/info), and each violation's file path and message; the data is derived from `CheckEntry` objects where `result.status == "fail"` |

#### FR-401: Violation filtering

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-03 -> `ui/violations/` |
| **Acceptance** | The violation browser supports filtering by: category (dropdown of 18 categories), severity (error/warning/info checkboxes), and free-text search on check description and violation message; filters are applied client-side and update the view without a new API call |

#### FR-402: Violation sorting

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Demonstration |
| **Traces to** | STK-03 -> `ui/violations/` |
| **Acceptance** | Violations can be sorted by: check ID (ascending/descending), severity (error first), category (alphabetical), and file path (alphabetical); the default sort is severity descending then check ID ascending |

#### FR-403: Fix guidance display

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Demonstration |
| **Traces to** | STK-03 -> `ui/violations/` |
| **Acceptance** | Clicking a violation expands a detail panel showing: the check's full description from `rules.toml`, the violation file path, the expected condition (from the rule definition), and a suggested fix action |

#### FR-404: Violation export

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-03, STK-06 -> `api/scans.rs` |
| **Acceptance** | `GET /api/v1/scans/{id}/violations?format=csv` returns a CSV file with columns: check_id, category, severity, file_path, message; `format=json` returns the violations array as JSON |

### 4.5 Scaffolding Interface

#### FR-500: Upload SRS document

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-04 -> `api/scaffold.rs` |
| **Acceptance** | `POST /api/v1/scaffold/parse` with the SRS markdown content in the request body returns a JSON representation of extracted domains and requirements (matching `SrsDomain` and `SrsRequirement` structures: section, title, slug, requirements with id, title, kind, priority, state, verification, traces_to, acceptance, description) |

The API shall accept an SRS markdown document, parse it using `doc_engine_scaffold::parse_srs()`, and return the extracted domain/requirement structure for preview.

#### FR-501: Scaffold preview

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Demonstration |
| **Traces to** | STK-04 -> `ui/scaffold/` |
| **Acceptance** | After uploading an SRS, the UI displays a tree of files that will be generated: per-domain `.spec.yaml`, `.spec`, `.arch.yaml`, `.arch`, `.test.yaml`, `.test`, `.manual.exec`, `.auto.exec`, `.deploy.yaml`, `.deploy`, plus BRD files; each file shows its target path under the output directory |

#### FR-502: Execute scaffold

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-04 -> `api/scaffold.rs` |
| **Acceptance** | `POST /api/v1/scaffold/execute` with `{"project_id": "uuid", "srs_content": "...", "phases": ["requirements", "testing"], "force": false}` invokes `scaffold_from_srs()` and returns a `ScaffoldResult` JSON (standard, clause, tool, tool_version, timestamp, srs_source, phases, force, domain_count, requirement_count, created, skipped) |

#### FR-503: Phase and type filters

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-04 -> `ui/scaffold/`, `api/scaffold.rs` |
| **Acceptance** | The scaffold UI provides checkboxes for SDLC phases (requirements, design, testing, deployment) and file types (yaml, spec, arch, test, exec, deploy); selections are passed to `ScaffoldConfig.phases` and `ScaffoldConfig.file_types` |

#### FR-504: Scaffold progress monitoring

| Attribute | Value |
|-----------|-------|
| **Priority** | Could |
| **State** | Proposed |
| **Verification** | Demonstration |
| **Traces to** | STK-04 -> `ui/scaffold/` |
| **Acceptance** | During scaffold execution, the UI displays which files are being created (`+` prefix) and which are skipped (`~` prefix) in real-time; upon completion, a summary shows domain_count, requirement_count, created count, and skipped count |

### 4.6 Template Browser

#### FR-600: List templates

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-09 -> `api/templates.rs` |
| **Acceptance** | `GET /api/v1/templates` returns a JSON array of template objects with name, sdlc_phase, description, and path fields; the list includes all templates from the configured template-engine templates directory |

#### FR-601: Template preview with W3H structure

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Demonstration |
| **Traces to** | STK-09 -> `ui/templates/` |
| **Acceptance** | Selecting a template displays its rendered markdown with W3H sections (WHO, WHAT, WHY, HOW) visually highlighted; the raw markdown is also available in a code view |

#### FR-602: Template copy to project

| Attribute | Value |
|-----------|-------|
| **Priority** | Could |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-09 -> `api/templates.rs` |
| **Acceptance** | `POST /api/v1/templates/{name}/copy` with `{"project_id": "uuid", "target_path": "docs/2-planning/quality_plan.md"}` copies the template to the project's directory; returns 201 on success or 409 if file already exists |

#### FR-603: Compliance checklist display

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Demonstration |
| **Traces to** | STK-09 -> `ui/templates/` |
| **Acceptance** | The template browser includes a dedicated checklist view that displays the 56-point compliance checklist from template-engine with each item's status (pass/fail/skip) mapped to the most recent doc-engine scan results for the selected project |

### 4.7 Report Generation

#### FR-700: JSON report export

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-06 -> `api/reports.rs` |
| **Acceptance** | `GET /api/v1/scans/{id}/report?format=json` returns the `ScanReport` JSON conforming to ISO/IEC/IEEE 15289:2019 clause 9.2 (containing standard, clause, tool, tool_version, timestamp, project_root, results, summary, project_type, project_scope) |

#### FR-701: Markdown report export

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-06 -> `api/reports.rs` |
| **Acceptance** | `GET /api/v1/scans/{id}/report?format=markdown` returns a structured markdown document with sections for: executive summary, category breakdown, violation details, and ISO standard mapping; the Content-Type header is `text/markdown` |

#### FR-702: PDF report export

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-06 -> `api/reports.rs` |
| **Acceptance** | `GET /api/v1/scans/{id}/report?format=pdf` returns a downloadable PDF with the same content as the markdown report rendered with headers, tables, and charts; the Content-Type header is `application/pdf` |

#### FR-703: Historical report comparison

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Demonstration |
| **Traces to** | STK-12 -> `ui/reports/` |
| **Acceptance** | The UI allows selecting two scan results for the same project; a diff view highlights checks that changed status between scans (e.g., fail-to-pass, pass-to-fail); new and removed checks are flagged |

#### FR-704: Audit status report (ISO 15289)

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-06 -> `api/reports.rs` |
| **Acceptance** | `GET /api/v1/scans/{id}/audit-report` returns the full ISO/IEC/IEEE 15289:2019 clause 9.2 audit status report JSON, matching the structure produced by doc-engine's `--output` flag (standard, clause, tool, tool_version, timestamp, project_root, project_type, project_scope, results, summary) |

### 4.8 AI Compliance Features

#### FR-800: Chat interface

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Demonstration |
| **Traces to** | STK-05 -> `api/ai.rs`, `ui/ai/chat/` |
| **Acceptance** | The UI provides a chat panel; sending a message via `POST /api/v1/ai/chat` with `{"message": "What ISO standards apply to my testing docs?"}` returns the LLM response; the chat panel displays the conversation history with user and assistant messages |

The API proxies chat requests to `ComplianceChat::chat()` from the `doc-engine-compliance-chat` crate.

#### FR-801: Chat streaming

| Attribute | Value |
|-----------|-------|
| **Priority** | Could |
| **State** | Proposed |
| **Verification** | Demonstration |
| **Traces to** | STK-05 -> `api/ws.rs`, `ui/ai/chat/` |
| **Acceptance** | Connecting to `WS /api/v1/ai/chat/stream` and sending a message receives incremental token-by-token responses; the UI renders tokens as they arrive, providing a real-time typing experience |

#### FR-802: AI audit execution

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-05 -> `api/ai.rs` |
| **Acceptance** | `POST /api/v1/ai/audit` with `{"project_id": "uuid", "scope": "medium"}` invokes `ComplianceAuditor::audit()`, returns an `AuditResponse` JSON (summary, scan_results, recommendations) |

#### FR-803: AI audit results display

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Demonstration |
| **Traces to** | STK-05 -> `ui/ai/audit/` |
| **Acceptance** | The audit view displays: the LLM-generated summary, a list of prioritized recommendations, and a collapsible panel showing the raw scan results JSON; recommendations link to related violations in the violation browser |

#### FR-804: Command generation

| Attribute | Value |
|-----------|-------|
| **Priority** | Could |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-05 -> `api/ai.rs` |
| **Acceptance** | `POST /api/v1/ai/generate-commands` with `{"srs_content": "...", "project_context": "..."}` invokes `CommandGenerator::generate_commands()` and returns a `GenerateCommandsResponse` JSON (commands map of requirement ID to CLI command, skipped array) |

#### FR-805: AI feature availability check

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-05 -> `api/ai.rs`, `ui/ai/` |
| **Acceptance** | `GET /api/v1/ai/status` returns `{"enabled": true|false, "provider": "anthropic|openai|gemini|null"}`; when disabled, AI UI sections display a "Not configured" message and AI endpoints return 503 |

### 4.9 SRS Editor

#### FR-900: Markdown editor

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Demonstration |
| **Traces to** | STK-08 -> `ui/editor/` |
| **Acceptance** | The SRS editor provides a split-pane view: a code editor with markdown syntax highlighting on the left, and a rendered preview on the right; editing the markdown updates the preview within 500ms |

#### FR-901: Doc-engine format validation

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-08 -> `api/editor.rs`, `ui/editor/` |
| **Acceptance** | `POST /api/v1/editor/validate` with SRS markdown content invokes `parse_srs()` and returns validation results: `{"valid": true|false, "domains": N, "requirements": N, "errors": [...]}` where errors include missing attribute table fields, duplicate FR IDs, and malformed section headings |

#### FR-902: Requirement ID auto-generation

| Attribute | Value |
|-----------|-------|
| **Priority** | Could |
| **State** | Proposed |
| **Verification** | Demonstration |
| **Traces to** | STK-08 -> `ui/editor/` |
| **Acceptance** | When the user types `#### FR-` in a new domain section, the editor suggests the next sequential FR ID based on the domain's numbering range (e.g., FR-201 if the domain starts at FR-200 and FR-200 exists); accepting the suggestion inserts the complete requirement header with an empty attribute table |

#### FR-903: SRS save and load

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-08 -> `api/editor.rs` |
| **Acceptance** | `PUT /api/v1/projects/{id}/srs` with SRS markdown content saves the document to the project's `docs/1-requirements/srs.md` path; `GET /api/v1/projects/{id}/srs` returns the current SRS content with 200, or 404 if no SRS exists |

### 4.10 Spec File Viewer

#### FR-1000: Browse spec files

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-04 -> `api/specs.rs`, `ui/specs/` |
| **Acceptance** | `GET /api/v1/projects/{id}/specs` returns a JSON array of discovered spec files with path, format (yaml/markdown), kind (brd/spec/architecture/test_plan/deployment), and domain_slug fields; the UI displays them in a tree grouped by domain |

#### FR-1001: Spec file content view

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Demonstration |
| **Traces to** | STK-04 -> `ui/specs/` |
| **Acceptance** | Selecting a spec file displays its content: YAML files are rendered with syntax highlighting; markdown files are rendered as formatted HTML; both views show the file's metadata (Version, Status, Spec ID, Related) in a summary header |

#### FR-1002: Cross-reference display

| Attribute | Value |
|-----------|-------|
| **Priority** | Could |
| **State** | Proposed |
| **Verification** | Demonstration |
| **Traces to** | STK-04 -> `ui/specs/` |
| **Acceptance** | The spec viewer shows cross-references as navigable links: clicking a dependency reference navigates to the referenced spec file; test plan `verifies` fields link to the corresponding requirement spec; unresolved references are highlighted in red |

#### FR-1003: BRD inventory display

| Attribute | Value |
|-----------|-------|
| **Priority** | Could |
| **State** | Proposed |
| **Verification** | Demonstration |
| **Traces to** | STK-04 -> `ui/specs/` |
| **Acceptance** | The spec viewer includes a BRD overview page showing the master inventory: domain count, specs per domain, and coverage indicators for each SDLC phase (requirements, design, testing, deployment) |

### 4.11 Struct-Engine Integration

#### FR-1100: Struct-engine scan results

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-10 -> `api/scans.rs`, `ui/struct-engine/` |
| **Acceptance** | `GET /api/v1/scans/{id}` for a struct-engine scan returns the struct-engine `ScanReport` JSON (containing results with check_id, category, description, result per check across 7 categories: structure, cargo_metadata, cargo_targets, naming, test_org, documentation, hygiene) |

#### FR-1101: Crate layout visualization

| Attribute | Value |
|-----------|-------|
| **Priority** | Could |
| **State** | Proposed |
| **Verification** | Demonstration |
| **Traces to** | STK-10 -> `ui/struct-engine/` |
| **Acceptance** | The struct-engine results page includes a tree diagram showing the project's actual crate layout (src/, main/, tests/, benches/, examples/) with pass/fail indicators overlaid on directories that correspond to checked paths |

#### FR-1102: Project kind display

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Demonstration |
| **Traces to** | STK-10 -> `ui/struct-engine/` |
| **Acceptance** | The struct-engine results display the detected project kind (Library, Binary, Both, Workspace) and indicate which checks were skipped due to kind filtering |

### 4.12 API Layer

#### FR-1200: REST API versioning

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Inspection |
| **Traces to** | STK-07 -> `api/router.rs` |
| **Acceptance** | All API endpoints are prefixed with `/api/v1/`; the version is part of the URL path; no endpoints exist outside the versioned prefix (except `/health`) |

#### FR-1201: Authentication

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-07 -> `api/auth.rs` |
| **Acceptance** | `POST /api/v1/auth/login` with `{"username": "...", "password": "..."}` returns a JWT token; all other endpoints (except `/health` and `/api/v1/auth/login`) require a valid `Authorization: Bearer <token>` header; expired or invalid tokens return 401 |

#### FR-1202: Health check

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-07 -> `api/router.rs` |
| **Acceptance** | `GET /health` returns 200 with `{"status": "ok", "version": "x.y.z", "engines": {"doc-engine": "x.y.z", "struct-engine": "x.y.z"}}` |

#### FR-1203: WebSocket connection management

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-07 -> `api/ws.rs` |
| **Acceptance** | WebSocket connections at `/api/v1/ws` require a valid JWT token as a query parameter (`?token=...`); connections without a valid token are rejected with 401; the server sends a ping every 30 seconds and closes idle connections after 120 seconds |

#### FR-1204: Error response format

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Inspection |
| **Traces to** | STK-07 -> `api/error.rs` |
| **Acceptance** | All error responses follow the format `{"error": {"code": "ERROR_CODE", "message": "Human-readable description", "details": {...}}}` with appropriate HTTP status codes: 400 (bad request), 401 (unauthorized), 404 (not found), 422 (validation error), 500 (internal error), 503 (service unavailable) |

#### FR-1205: Request rate limiting

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-07 -> `api/middleware.rs` |
| **Acceptance** | API endpoints enforce rate limiting of 100 requests per minute per authenticated user; scan execution endpoints are limited to 10 concurrent scans per user; exceeding limits returns 429 with a `Retry-After` header |

---

## 5. Non-Functional Requirements

### 5.1 Performance

#### NFR-100: Dashboard load time

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-01 -> `ui/dashboard/` |
| **Acceptance** | The dashboard page loads and renders the project list within 2 seconds on a standard broadband connection when there are fewer than 50 projects |

#### NFR-101: Scan result rendering

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-03 -> `ui/violations/` |
| **Acceptance** | A scan result with 128 check entries and up to 500 violations renders in the violation browser within 1 second; filtering and sorting operations complete within 200ms |

#### NFR-102: API response time

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-07 -> `api/` |
| **Acceptance** | Non-scan API endpoints (project CRUD, report retrieval, template listing) respond within 500ms at the 95th percentile under normal load (10 concurrent users) |

#### NFR-103: Concurrent scan execution

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-02 -> `api/scans.rs` |
| **Acceptance** | The server supports at least 5 concurrent scan executions without degradation; scans are queued when the concurrency limit is exceeded |

### 5.2 Security

#### NFR-200: Authentication required

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-07 -> `api/auth.rs` |
| **Acceptance** | Every API endpoint (except `/health` and `/api/v1/auth/login`) returns 401 when no Authorization header is provided; JWT tokens expire after a configurable period (default: 24 hours) |

#### NFR-201: Path traversal prevention

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-07 -> `api/projects.rs` |
| **Acceptance** | Project root_path values containing `..`, symbolic links outside allowed directories, or paths above a configurable base directory are rejected with 422; the server resolves and validates all paths before passing to engine libraries |

#### NFR-202: API key protection

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Inspection |
| **Traces to** | STK-05 -> `api/ai.rs` |
| **Acceptance** | LLM API keys are read from server environment variables and never exposed in API responses, logs, or error messages; the `/api/v1/ai/status` endpoint reports only whether a key is configured (true/false), never the key value |

#### NFR-203: CORS configuration

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Inspection |
| **Traces to** | STK-07 -> `api/middleware.rs` |
| **Acceptance** | CORS origins are configurable via environment variable; the default allows only `localhost` origins; production deployments must explicitly configure allowed origins |

### 5.3 Accessibility

#### NFR-300: Keyboard navigation

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Demonstration |
| **Traces to** | STK-01 -> `ui/` |
| **Acceptance** | All interactive elements (buttons, links, filters, tree nodes) are reachable via keyboard Tab navigation; Enter/Space activates focused elements; Escape closes modals and panels |

#### NFR-301: Screen reader compatibility

| Attribute | Value |
|-----------|-------|
| **Priority** | Should |
| **State** | Proposed |
| **Verification** | Inspection |
| **Traces to** | STK-01 -> `ui/` |
| **Acceptance** | All UI components use semantic HTML elements and ARIA attributes; charts provide text alternatives; the violation count and compliance percentage are announced by screen readers |

### 5.4 Maintainability

#### NFR-400: Component-based architecture

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Inspection |
| **Traces to** | STK-07 -> `ui/`, `api/` |
| **Acceptance** | The frontend UI is organized into self-contained components (dashboard, violations, scaffold, editor, templates, ai, specs, struct-engine); the backend API is organized into route modules matching the domain structure; no circular dependencies exist between components |

#### NFR-401: API-first design

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Inspection |
| **Traces to** | STK-07 -> `api/` |
| **Acceptance** | Every UI operation is backed by a documented REST or WebSocket API endpoint; no functionality is available only through the UI; the API can be consumed by external clients (CI systems, scripts) independently of the web UI |

#### NFR-402: Engine version compatibility

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-02 -> `Cargo.toml` |
| **Acceptance** | The frontend server declares doc-engine and struct-engine crates as Cargo dependencies with semver-compatible version constraints; the `/health` endpoint reports the linked engine versions; incompatible engine API changes are detected at compile time |

### 5.5 Reliability

#### NFR-500: Graceful engine errors

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-02 -> `api/scans.rs` |
| **Acceptance** | When `scan_with_config()` returns `ScanError::Path` or `ScanError::Config`, the API returns a structured error response (422 or 500) with the engine's error message; the server does not panic or crash |

#### NFR-501: AI graceful degradation

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-05 -> `api/ai.rs` |
| **Acceptance** | When AI features are not configured (missing API key or disabled), all AI endpoints return 503 with `{"error": {"code": "AI_NOT_CONFIGURED", "message": "..."}}` and the UI disables AI panels; LLM API failures return 502 with the provider's error message |

#### NFR-502: Data persistence

| Attribute | Value |
|-----------|-------|
| **Priority** | Must |
| **State** | Proposed |
| **Verification** | Test |
| **Traces to** | STK-12 -> `api/storage.rs` |
| **Acceptance** | Project configurations and scan results survive server restarts; the storage backend (SQLite or PostgreSQL, deferred to design) persists all data to disk; no in-memory-only state is required for correctness |

---

## 6. External Interface Requirements

### 6.1 Browser Interface

| Direction | Data | Format |
|-----------|------|--------|
| Output | Dashboard, violations, reports | HTML/CSS/JS rendered in browser |
| Input | User interactions | HTTP requests via fetch/WebSocket |
| Constraint | Browser support | Latest 2 versions of Chrome, Firefox, Safari, Edge |

### 6.2 REST API Interface

| Direction | Data | Format |
|-----------|------|--------|
| Input | Project CRUD, scan triggers, scaffold requests | JSON over HTTPS |
| Output | Scan results, project data, reports | JSON (application/json), markdown (text/markdown), PDF (application/pdf) |
| Auth | JWT token | Authorization: Bearer header |

### 6.3 WebSocket Interface

| Direction | Data | Format |
|-----------|------|--------|
| Bidirectional | Scan progress, AI chat streaming | JSON messages over WSS |
| Auth | JWT token | Query parameter (?token=...) |

### 6.4 Engine Library Interface

| Direction | Data | Type |
|-----------|------|------|
| Input (doc-engine scan) | Project root, config | `&Path`, `&ScanConfig` |
| Output (doc-engine scan) | Report | `Result<ScanReport, ScanError>` |
| Input (doc-engine scaffold) | SRS content, config | `&ScaffoldConfig` |
| Output (doc-engine scaffold) | Result | `Result<ScaffoldResult, ScaffoldError>` |
| Input (compliance chat) | Message | `&str` |
| Output (compliance chat) | Response | `Result<String, ChatError>` |
| Input (compliance audit) | Path, scope | `&str`, `&str` |
| Output (compliance audit) | Response | `Result<AuditResponse, AuditError>` |
| Input (command gen) | Request | `&GenerateCommandsRequest` |
| Output (command gen) | Response | `Result<GenerateCommandsResponse, CommandGeneratorError>` |
| Input (struct-engine scan) | Project root, config | `&Path`, `&ScanConfig` |
| Output (struct-engine scan) | Report | `Result<ScanReport, ScanError>` |

### 6.5 File System Interface

| Direction | Data | Format |
|-----------|------|--------|
| Input | Project directories (read-only scan) | File system paths on server |
| Input | Template-engine templates | Static markdown files |
| Output | Scaffold files | Generated spec files via scaffold library |
| Output | Scan reports | JSON files via `--output` mechanism |

### 6.6 LLM API Interface (proxied)

| Direction | Data | Detail |
|-----------|------|--------|
| Input | API key | Server environment variable (never exposed to client) |
| Output | LLM completions | Via compliance-chat, compliance-audit, command-generator crates over HTTPS |

---

## 7. Risk Analysis

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Engine library API changes break the frontend server at compile time | High | Medium | Pin engine crate versions with semver; run integration tests on engine updates; `/health` reports versions |
| Scan execution blocks the API server thread pool under load | High | Medium | Execute scans in a dedicated `tokio::spawn_blocking` pool; enforce concurrency limits (NFR-103); queue excess requests |
| Path traversal allows access to arbitrary server files | Critical | Low | Validate and canonicalize all paths; reject `..` components; enforce configurable base directory (NFR-201) |
| LLM API keys leaked through API responses or browser DevTools | Critical | Low | Keys read from env vars only; never serialized to JSON; `/ai/status` reports presence not value (NFR-202) |
| Large scan results (500+ violations) cause browser performance degradation | Medium | Medium | Paginate violation list; virtualize long lists; limit initial render to visible viewport (NFR-101) |
| WebSocket connections exhaust server resources | Medium | Low | Enforce connection limits per user; ping/pong keepalive; 120s idle timeout (FR-1203) |
| Template-engine templates not available on server | Low | Low | Fail gracefully with empty template list; document template directory configuration |
| AI feature not configured but users expect it | Medium | Medium | `/ai/status` endpoint; UI shows clear "not configured" state; AI endpoints return 503 (FR-805, NFR-501) |
| Concurrent scaffold operations on the same project corrupt files | Medium | Low | Serialize scaffold operations per project; use file locks or queue |
| JWT token theft enables unauthorized access | High | Low | Short token expiry (configurable, default 24h); HTTPS-only; secure/httpOnly cookie option; token refresh flow |

---

## Appendix A: Traceability Matrix

### Stakeholder -> System

| STK | SYS |
|-----|-----|
| STK-01 | SYS-03, SYS-11 |
| STK-02 | SYS-02 |
| STK-03 | SYS-03 |
| STK-04 | SYS-04, SYS-09 |
| STK-05 | SYS-07 |
| STK-06 | SYS-06, SYS-11 |
| STK-07 | SYS-10 |
| STK-08 | SYS-08 |
| STK-09 | SYS-05 |
| STK-10 | SYS-02, SYS-03 |
| STK-11 | SYS-01 |
| STK-12 | SYS-11 |

### Stakeholder -> Software

| STK | FR / NFR |
|-----|----------|
| STK-01 | FR-200, FR-201, FR-202, FR-203, NFR-100 |
| STK-02 | FR-300, FR-301, FR-302, FR-303, FR-304, FR-305, NFR-103 |
| STK-03 | FR-400, FR-401, FR-402, FR-403, FR-404, NFR-101 |
| STK-04 | FR-500, FR-501, FR-502, FR-503, FR-504 |
| STK-05 | FR-800, FR-801, FR-802, FR-803, FR-804, FR-805, NFR-501 |
| STK-06 | FR-700, FR-701, FR-702, FR-703, FR-704 |
| STK-07 | FR-1200, FR-1201, FR-1202, FR-1203, FR-1204, FR-1205, NFR-200, NFR-201, NFR-202, NFR-203, NFR-401 |
| STK-08 | FR-900, FR-901, FR-902, FR-903 |
| STK-09 | FR-600, FR-601, FR-602, FR-603 |
| STK-10 | FR-1100, FR-1101, FR-1102 |
| STK-11 | FR-100, FR-101, FR-102, FR-103, FR-104 |
| STK-12 | FR-202, FR-305, FR-703, NFR-502 |

### Software -> Architecture Component

| FR / NFR | Architecture Component |
|----------|----------------------|
| FR-100 -- FR-104 | `api/projects.rs`, `ui/projects/` |
| FR-200 -- FR-203 | `ui/dashboard/`, `api/scans.rs` |
| FR-300 -- FR-305 | `api/scans.rs`, `api/ws.rs`, `ui/scans/` |
| FR-400 -- FR-404 | `ui/violations/`, `api/scans.rs` |
| FR-500 -- FR-504 | `api/scaffold.rs`, `ui/scaffold/` |
| FR-600 -- FR-603 | `api/templates.rs`, `ui/templates/` |
| FR-700 -- FR-704 | `api/reports.rs`, `ui/reports/` |
| FR-800 -- FR-805 | `api/ai.rs`, `api/ws.rs`, `ui/ai/` |
| FR-900 -- FR-903 | `api/editor.rs`, `ui/editor/` |
| FR-1000 -- FR-1003 | `api/specs.rs`, `ui/specs/` |
| FR-1100 -- FR-1102 | `api/scans.rs`, `ui/struct-engine/` |
| FR-1200 -- FR-1205 | `api/router.rs`, `api/auth.rs`, `api/ws.rs`, `api/middleware.rs`, `api/error.rs` |
| NFR-100 -- NFR-103 | `ui/`, `api/` |
| NFR-200 -- NFR-203 | `api/auth.rs`, `api/middleware.rs`, `api/projects.rs`, `api/ai.rs` |
| NFR-300 -- NFR-301 | `ui/` (all components) |
| NFR-400 -- NFR-402 | `ui/`, `api/`, `Cargo.toml` |
| NFR-500 -- NFR-502 | `api/scans.rs`, `api/ai.rs`, `api/storage.rs` |

---

## Appendix B: API Endpoint Summary

| Method | Endpoint | Domain | FR |
|--------|----------|--------|----|
| POST | `/api/v1/auth/login` | Auth | FR-1201 |
| GET | `/health` | System | FR-1202 |
| GET | `/api/v1/projects` | Projects | FR-101 |
| POST | `/api/v1/projects` | Projects | FR-100 |
| GET | `/api/v1/projects/{id}` | Projects | FR-101 |
| PATCH | `/api/v1/projects/{id}` | Projects | FR-102 |
| DELETE | `/api/v1/projects/{id}` | Projects | FR-103 |
| POST | `/api/v1/scans` | Scans | FR-300, FR-301 |
| GET | `/api/v1/scans/{id}` | Scans | FR-303, FR-1100 |
| WS | `/api/v1/scans/{id}/progress` | Scans | FR-302 |
| GET | `/api/v1/projects/{id}/scans` | Scans | FR-305 |
| GET | `/api/v1/projects/{id}/trends` | Dashboard | FR-202 |
| GET | `/api/v1/scans/{id}/violations` | Violations | FR-404 |
| GET | `/api/v1/scans/{id}/report` | Reports | FR-700, FR-701, FR-702 |
| GET | `/api/v1/scans/{id}/audit-report` | Reports | FR-704 |
| POST | `/api/v1/scaffold/parse` | Scaffold | FR-500 |
| POST | `/api/v1/scaffold/execute` | Scaffold | FR-502 |
| GET | `/api/v1/templates` | Templates | FR-600 |
| POST | `/api/v1/templates/{name}/copy` | Templates | FR-602 |
| POST | `/api/v1/ai/chat` | AI | FR-800 |
| WS | `/api/v1/ai/chat/stream` | AI | FR-801 |
| POST | `/api/v1/ai/audit` | AI | FR-802 |
| POST | `/api/v1/ai/generate-commands` | AI | FR-804 |
| GET | `/api/v1/ai/status` | AI | FR-805 |
| POST | `/api/v1/editor/validate` | Editor | FR-901 |
| GET | `/api/v1/projects/{id}/srs` | Editor | FR-903 |
| PUT | `/api/v1/projects/{id}/srs` | Editor | FR-903 |
| GET | `/api/v1/projects/{id}/specs` | Specs | FR-1000 |

---

## Summary Statistics

| Metric | Count |
|--------|-------|
| Stakeholder requirements (STK) | 12 |
| Operational scenarios (OS) | 8 |
| System functions (SYS) | 11 |
| Functional requirement domains | 12 |
| Functional requirements (FR) | 57 |
| Non-functional requirement domains | 5 |
| Non-functional requirements (NFR) | 16 |
| Total requirements | 73 |
| API endpoints | 28 |

---
