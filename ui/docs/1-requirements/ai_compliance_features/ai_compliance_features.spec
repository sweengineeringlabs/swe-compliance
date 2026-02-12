# Feature Spec: AI Compliance Features

**Version:** 1.0
**Status:** Draft
**Section:** 4.8

## Requirements

| ID | Source | Title | Priority | Verification | Acceptance |
|-----|--------|-------|----------|--------------|------------|
| REQ-001 | FR-800 | Chat interface | Should | Demonstration | The UI provides a chat panel; sending a message via `POST /api/v1/ai/chat` with `{"message": "What ISO standards apply to my testing docs?"}` returns the LLM response; the chat panel displays the conversation history with user and assistant messages |
| REQ-002 | FR-801 | Chat streaming | Could | Demonstration | Connecting to `WS /api/v1/ai/chat/stream` and sending a message receives incremental token-by-token responses; the UI renders tokens as they arrive, providing a real-time typing experience |
| REQ-003 | FR-802 | AI audit execution | Should | Test | `POST /api/v1/ai/audit` with `{"project_id": "uuid", "scope": "medium"}` invokes `ComplianceAuditor::audit()`, returns an `AuditResponse` JSON (summary, scan_results, recommendations) |
| REQ-004 | FR-803 | AI audit results display | Should | Demonstration | The audit view displays: the LLM-generated summary, a list of prioritized recommendations, and a collapsible panel showing the raw scan results JSON; recommendations link to related violations in the violation browser |
| REQ-005 | FR-804 | Command generation | Could | Test | `POST /api/v1/ai/generate-commands` with `{"srs_content": "...", "project_context": "..."}` invokes `CommandGenerator::generate_commands()` and returns a `GenerateCommandsResponse` JSON (commands map of requirement ID to CLI command, skipped array) |
| REQ-006 | FR-805 | AI feature availability check | Must | Test | `GET /api/v1/ai/status` returns `{"enabled": true\|false, "provider": "anthropic\|openai\|gemini\|null"}`; when disabled, AI UI sections display a "Not configured" message and AI endpoints return 503 |

## Acceptance Criteria

- **REQ-001** (FR-800): The UI provides a chat panel; sending a message via `POST /api/v1/ai/chat` with `{"message": "What ISO standards apply to my testing docs?"}` returns the LLM response; the chat panel displays the conversation history with user and assistant messages
- **REQ-002** (FR-801): Connecting to `WS /api/v1/ai/chat/stream` and sending a message receives incremental token-by-token responses; the UI renders tokens as they arrive, providing a real-time typing experience
- **REQ-003** (FR-802): `POST /api/v1/ai/audit` with `{"project_id": "uuid", "scope": "medium"}` invokes `ComplianceAuditor::audit()`, returns an `AuditResponse` JSON (summary, scan_results, recommendations)
- **REQ-004** (FR-803): The audit view displays: the LLM-generated summary, a list of prioritized recommendations, and a collapsible panel showing the raw scan results JSON; recommendations link to related violations in the violation browser
- **REQ-005** (FR-804): `POST /api/v1/ai/generate-commands` with `{"srs_content": "...", "project_context": "..."}` invokes `CommandGenerator::generate_commands()` and returns a `GenerateCommandsResponse` JSON (commands map of requirement ID to CLI command, skipped array)
- **REQ-006** (FR-805): `GET /api/v1/ai/status` returns `{"enabled": true|false, "provider": "anthropic|openai|gemini|null"}`; when disabled, AI UI sections display a "Not configured" message and AI endpoints return 503

