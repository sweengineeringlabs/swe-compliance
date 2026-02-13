use rsc_compat::prelude::*;
use crate::features::ai::store::{self, AiStore};
use super::chat_panel::chat_panel;
use super::audit_view::audit_view;
use super::command_gen::command_gen;
use super::ai_status::ai_status_badge;

/// AI compliance landing page (FR-800..805).
/// Provides tabbed access to chat, audit, and command generation features.
#[component]
pub fn ai_landing() -> View {
    let s = use_context::<AiStore>();

    // Load AI status on mount.
    {
        let s2 = s.clone();
        effect(move || { store::load_status(&s2); });
    }

    view! {
        style {
            .ai-landing {
                display: flex;
                flex-direction: column;
                gap: var(--space-4);
            }

            .ai-landing__tabs {
                display: flex;
                gap: var(--space-2);
                border-bottom: 1px solid var(--color-border);
                margin-bottom: var(--space-4);
            }

            .ai-landing__tab {
                padding: var(--space-2) var(--space-4);
                cursor: pointer;
                border: none;
                background: none;
                font-size: var(--font-size-sm);
                color: var(--color-text-secondary);
                border-bottom: 2px solid transparent;
            }

            .ai-landing__tab--active {
                color: var(--color-primary);
                border-bottom-color: var(--color-primary);
                font-weight: 600;
            }
        }
        div(class="ai-landing", data-testid="ai-landing") {
            (ai_status_badge(s.status.clone()))

            div(class="ai-landing__tabs", data-testid="ai-tabs") {
                button(
                    class=format!("ai-landing__tab{}", if s.active_tab.get().as_str() == "chat" { " ai-landing__tab--active" } else { "" }),
                    on:click={
                        let tab = s.active_tab.clone();
                        move || tab.set("chat".into())
                    },
                ) { "Chat" }
                button(
                    class=format!("ai-landing__tab{}", if s.active_tab.get().as_str() == "audit" { " ai-landing__tab--active" } else { "" }),
                    on:click={
                        let tab = s.active_tab.clone();
                        move || tab.set("audit".into())
                    },
                ) { "Audit" }
                button(
                    class=format!("ai-landing__tab{}", if s.active_tab.get().as_str() == "commands" { " ai-landing__tab--active" } else { "" }),
                    on:click={
                        let tab = s.active_tab.clone();
                        move || tab.set("commands".into())
                    },
                ) { "Commands" }
            }

            (match s.active_tab.get().as_str() {
                "chat" => chat_panel(
                    s.messages.clone(),
                    s.current_input.clone(),
                    Some(Box::new({
                        let s2 = s.clone();
                        move || store::send_message(&s2)
                    })),
                    s.loading.get(),
                ),
                "audit" => audit_view(
                    s.audit_result.clone(),
                    s.loading.get(),
                    Some(Box::new({
                        let s2 = s.clone();
                        move |path: String, scope: String| store::run_audit(&s2, &path, &scope)
                    })),
                ),
                "commands" => command_gen(
                    s.command_result.clone(),
                    s.loading.get(),
                    Some(Box::new({
                        let s2 = s.clone();
                        move |reqs: String, ctx: String| store::generate_commands(&s2, &reqs, &ctx)
                    })),
                ),
                _ => view! {},
            })
        }
    }
}
