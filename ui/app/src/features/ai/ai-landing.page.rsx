use rsc_ui::prelude::*;
use crate::features::ai::store::{self, AiStore};
use crate::features::ai::chat_panel::ChatPanel;
use crate::features::ai::audit_view::AuditView;
use crate::features::ai::command_gen::CommandGen;
use crate::features::ai::ai_status::AiStatusBadge;

/// AI compliance landing page (FR-800..805).
/// Provides tabbed access to chat, audit, and command generation features.
component AiLanding() {
    let s = use_context::<AiStore>();

    // Load AI status on mount.
    { let s2 = s.clone(); effect(move || { store::load_status(&s2); }); }

    style {
        .ai-landing {
            display: flex;
            flex-direction: column;
            gap: var(--space-4);
        }
    }

    render {
        <div class="ai-landing" data-testid="ai-landing">
            <AiStatusBadge status={s.status.clone()} />

            <Tabs
                active={s.active_tab.clone()}
                on:change={{ let tab = s.active_tab.clone(); move |v: String| tab.set(v) }}
                data-testid="ai-tabs"
            >
                <Tab id="chat" label="Chat">
                    <ChatPanel
                        messages={s.messages.clone()}
                        input={s.current_input.clone()}
                        on_send={Some(Box::new({ let s2 = s.clone(); move || store::send_message(&s2) }))}
                        loading={s.loading.get()}
                    />
                </Tab>
                <Tab id="audit" label="Audit">
                    <AuditView
                        result={s.audit_result.clone()}
                        loading={s.loading.get()}
                        on_run={Some(Box::new({ let s2 = s.clone(); move |path: String, scope: String| store::run_audit(&s2, &path, &scope) }))}
                    />
                </Tab>
                <Tab id="commands" label="Commands">
                    <CommandGen
                        result={s.command_result.clone()}
                        loading={s.loading.get()}
                        on_generate={Some(Box::new({ let s2 = s.clone(); move |reqs: String, ctx: String| store::generate_commands(&s2, &reqs, &ctx) }))}
                    />
                </Tab>
            </Tabs>
        </div>
    }
}
