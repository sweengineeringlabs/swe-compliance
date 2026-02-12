use rsc_ui::prelude::*;
use crate::features::ai::ai_store as store;
use crate::features::ai::chat_panel::ChatPanel;
use crate::features::ai::audit_view::AuditView;
use crate::features::ai::command_gen::CommandGen;
use crate::features::ai::ai_status::AiStatusBadge;

/// AI compliance landing page (FR-800..805).
/// Provides tabbed access to chat, audit, and command generation features.
component AiLanding() {
    // Load AI status on mount.
    effect(|| { store::load_status(); });

    style {
        .ai-landing {
            display: flex;
            flex-direction: column;
            gap: var(--space-4);
        }
    }

    render {
        <div class="ai-landing" data-testid="ai-landing">
            <AiStatusBadge status={store::status.clone()} />

            <Tabs
                active={store::active_tab.clone()}
                on:change={|tab| store::active_tab.set(tab)}
                data-testid="ai-tabs"
            >
                <Tab id="chat" label="Chat">
                    <ChatPanel
                        messages={store::messages.clone()}
                        input={store::current_input.clone()}
                        on_send={|| store::send_message()}
                        loading={store::loading.get()}
                    />
                </Tab>
                <Tab id="audit" label="Audit">
                    <AuditView
                        result={store::audit_result.clone()}
                        loading={store::loading.get()}
                        on_run={|path, scope| store::run_audit(path, scope)}
                    />
                </Tab>
                <Tab id="commands" label="Commands">
                    <CommandGen
                        result={store::command_result.clone()}
                        loading={store::loading.get()}
                        on_generate={|reqs, ctx| store::generate_commands(reqs, ctx)}
                    />
                </Tab>
            </Tabs>
        </div>
    }
}
