use std::rc::Rc;
use rsc_compat::prelude::*;
use super::types::CommandGenResult;

/// Command generation form and results view (FR-804).
///
/// Provides:
///   - Input for project context description
///   - Textarea for requirements JSON input
///   - Button to trigger generation
///   - Table mapping requirement IDs to generated shell commands
///   - List of skipped requirements with reasons
#[component]
pub fn command_gen(
    result: Signal<Option<CommandGenResult>>,
    loading: bool,
    on_generate: Option<Box<dyn Fn(String, String)>>,
) -> View {
    let on_generate: Option<Rc<dyn Fn(String, String)>> = on_generate.map(|f| Rc::from(f));
    let project_context = signal(String::new());
    let requirements_json = signal(String::new());

    view! {
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
        div(class="command-gen", data-testid="command-gen") {
            div(class="card", data-testid="command-gen-form") {
                div(class="command-gen__form") {
                    div(class="form-field") {
                        label { "Project Context" }
                        input(
                            type="text",
                            class="input",
                            value=project_context.clone(),
                            on:input={let pc = project_context.clone(); move |v: String| pc.set(v)},
                            placeholder="e.g. Rust compliance engine with doc-engine and struct-engine",
                            data-testid="command-gen-context-input",
                        )
                    }
                    div(class="form-field") {
                        label { "Requirements JSON" }
                        input(
                            type="text",
                            class="input",
                            value=requirements_json.clone(),
                            on:input={let rj = requirements_json.clone(); move |v: String| rj.set(v)},
                            placeholder=r#"[{"id":"REQ-001","title":"...","description":"..."}]"#,
                            data-testid="command-gen-reqs-input",
                        )
                    }
                    div(class="command-gen__actions") {
                        button(
                            class="btn btn--primary",
                            disabled=loading || requirements_json.get().is_empty(),
                            on:click={
                                let requirements_json = requirements_json.clone();
                                let project_context = project_context.clone();
                                move || { if let Some(ref cb) = on_generate { cb(requirements_json.get().clone(), project_context.get().clone()) } }
                            },
                            data-testid="command-gen-btn",
                        ) {
                            "Generate Commands"
                        }
                    }
                }
            }

            (if loading {
                view! {
                    div(class="command-gen__empty", data-testid="command-gen-loading") {
                        "Generating commands..."
                    }
                }
            } else {
                view! {}
            })

            (match result.get() {
                Some(gen) => {
                    let gen_count = format!("{} generated", gen.generated_count());
                    let skip_count = gen.skipped_count();
                    let skip_label = format!("{} skipped", skip_count);
                    let commands: Vec<(String, String)> = gen.commands.iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect();
                    let skipped = gen.skipped.clone();
                    let has_skipped = !skipped.is_empty();
                    view! {
                        div(class="command-gen__summary", data-testid="command-gen-summary") {
                            span(class="badge badge--success", data-testid="command-gen-count") {
                                (gen_count)
                            }
                            (if skip_count > 0 {
                                view! {
                                    span(class="badge badge--warning", data-testid="command-gen-skipped-count") {
                                        (skip_label)
                                    }
                                }
                            } else {
                                view! {}
                            })
                        }

                        table(class="table", data-testid="command-gen-table") {
                            thead {
                                tr {
                                    th { "Requirement ID" }
                                    th { "Generated Command" }
                                }
                            }
                            tbody {
                                Indexed(
                                    each=commands,
                                    key=|(req_id, _)| req_id.clone(),
                                    view=|(req_id, command)| {
                                        let row_testid = format!("command-row-{}", req_id);
                                        view! {
                                            tr(data-testid=row_testid) {
                                                td(data-testid="command-req-id") {
                                                    span(class="badge badge--info") { (req_id) }
                                                }
                                                td(data-testid="command-value") {
                                                    div(class="code-block", data-language="bash") {
                                                        pre {
                                                            code { (command) }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    },
                                )
                            }
                        }

                        (if has_skipped {
                            view! {
                                div(class="card", data-testid="command-gen-skipped") {
                                    h4 { "Skipped Requirements" }
                                    div(class="command-gen__skipped") {
                                        Indexed(
                                            each=skipped,
                                            key=|skip| skip.id.clone(),
                                            view=|skip| {
                                                let skip_id = skip.id.clone();
                                                let skip_reason = skip.reason.clone();
                                                view! {
                                                    div(class="command-gen__skip-item", data-testid="command-skip-item") {
                                                        span(class="badge badge--warning", data-testid="command-skip-id") {
                                                            (skip_id)
                                                        }
                                                        (skip_reason)
                                                    }
                                                }
                                            },
                                        )
                                    }
                                }
                            }
                        } else {
                            view! {}
                        })
                    }
                }
                None => {
                    if !loading {
                        view! {
                            div(class="command-gen__empty", data-testid="command-gen-empty") {
                                "Enter requirements and generate compliance commands."
                            }
                        }
                    } else {
                        view! {}
                    }
                }
            })
        }
    }
}
