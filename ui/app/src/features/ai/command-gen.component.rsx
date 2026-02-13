use rsc_ui::prelude::*;
use crate::features::ai::ai_type::CommandGenResult;

/// Command generation form and results view (FR-804).
///
/// Provides:
///   - Input for project context description
///   - Textarea for requirements JSON input
///   - Button to trigger generation
///   - Table mapping requirement IDs to generated shell commands
///   - List of skipped requirements with reasons
component CommandGen(
    result: Signal<Option<CommandGenResult>>,
    loading: bool,
    on_generate: Fn(String, String),
) {
    let project_context = signal(String::new());
    let requirements_json = signal(String::new());

    style {
        .command-gen {
            display: flex;
            flex-direction: column;
            gap: var(--space-4);
        }

        .command-gen__form {
            display: flex;
            flex-direction: column;
            gap: var(--space-3);
        }

        .command-gen__actions {
            display: flex;
            gap: var(--space-2);
        }

        .command-gen__skipped {
            display: flex;
            flex-direction: column;
            gap: var(--space-2);
        }

        .command-gen__skip-item {
            display: flex;
            gap: var(--space-2);
            align-items: baseline;
            padding: var(--space-1) 0;
        }

        .command-gen__empty {
            color: var(--color-text-secondary);
            font-style: italic;
            padding: var(--space-4);
            text-align: center;
        }

        .command-gen__summary {
            display: flex;
            gap: var(--space-3);
            align-items: center;
        }
    }

    render {
        <div class="command-gen" data-testid="command-gen">
            <Card data-testid="command-gen-form">
                <div class="command-gen__form">
                    <FormField label="Project Context">
                        <Input
                            value={project_context.clone()}
                            on:input={|v| project_context.set(v)}
                            placeholder="e.g. Rust compliance engine with doc-engine and struct-engine"
                            data-testid="command-gen-context-input"
                        />
                    </FormField>
                    <FormField label="Requirements JSON">
                        <Input
                            value={requirements_json.clone()}
                            on:input={|v| requirements_json.set(v)}
                            placeholder={"[{\"id\":\"REQ-001\",\"title\":\"...\",\"description\":\"...\"}]"}
                            data-testid="command-gen-reqs-input"
                        />
                    </FormField>
                    <div class="command-gen__actions">
                        <Button
                            label="Generate Commands"
                            variant="primary"
                            disabled={loading || requirements_json.get().is_empty()}
                            on:click={|| on_generate(requirements_json.get().clone(), project_context.get().clone())}
                            data-testid="command-gen-btn"
                        />
                    </div>
                </div>
            </Card>

            @if loading {
                <div class="command-gen__empty" data-testid="command-gen-loading">
                    "Generating commands..."
                </div>
            }

            @match result.get() {
                Some(gen) => {
                    <div class="command-gen__summary" data-testid="command-gen-summary">
                        <Badge variant="success" data-testid="command-gen-count">
                            {format!("{} generated", gen.generated_count())}
                        </Badge>
                        @if gen.skipped_count() > 0 {
                            <Badge variant="warning" data-testid="command-gen-skipped-count">
                                {format!("{} skipped", gen.skipped_count())}
                            </Badge>
                        }
                    </div>

                    <Table data-testid="command-gen-table">
                        <thead>
                            <tr>
                                <th>"Requirement ID"</th>
                                <th>"Generated Command"</th>
                            </tr>
                        </thead>
                        <tbody>
                            @for (req_id, command) in gen.commands.iter() {
                                <tr data-testid={format!("command-row-{}", req_id)}>
                                    <td data-testid="command-req-id">
                                        <Badge variant="info">{req_id}</Badge>
                                    </td>
                                    <td data-testid="command-value">
                                        <CodeBlock language="bash">{command}</CodeBlock>
                                    </td>
                                </tr>
                            }
                        </tbody>
                    </Table>

                    @if !gen.skipped.is_empty() {
                        <Card data-testid="command-gen-skipped">
                            <h4>"Skipped Requirements"</h4>
                            <div class="command-gen__skipped">
                                @for skip in gen.skipped.iter() {
                                    <div class="command-gen__skip-item" data-testid="command-skip-item">
                                        <Badge variant="warning" data-testid="command-skip-id">
                                            {&skip.id}
                                        </Badge>
                                        {&skip.reason}
                                    </div>
                                }
                            </div>
                        </Card>
                    }
                }
                None => {
                    @if !loading {
                        <div class="command-gen__empty" data-testid="command-gen-empty">
                            "Enter requirements and generate compliance commands."
                        </div>
                    }
                }
            }
        </div>
    }
}
