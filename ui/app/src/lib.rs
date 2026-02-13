//! swe-compliance-frontend — compliance dashboard compiled via RFC-005 proc macros.

pub mod features;
pub mod page;
pub mod util;

#[cfg(test)]
pub mod test_common;

use rsc_compat::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// WASM entry point — mounts the root `App` component into `document.body`.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    let window = web_sys::window().expect("no global window");
    let document = window.document().expect("no document");
    let body = document.body().expect("no body element");
    let root_view = app();
    root_view.mount(&body.into());
}

use crate::page::app::app_shell;
use crate::util::auth::{auth_provider, AuthState};
use crate::features::ai::store::AiStore;
use crate::features::dashboard::store::DashboardStore;
use crate::features::editor::store::EditorStore;
use crate::features::projects::store::ProjectsStore;
use crate::features::reports::store::ReportsStore;
use crate::features::scaffold::store::ScaffoldStore;
use crate::features::scans::store::ScansStore;
use crate::features::specs::store::SpecsStore;
use crate::features::struct_engine::store::StructEngineStore;
use crate::features::templates::store::TemplatesStore;
use crate::features::violations::store::ViolationsStore;

/// Root component — wraps app_shell with authentication context.
#[component]
pub fn app() -> View {
    let auth_state = signal(AuthState::default());

    // Provide auth context BEFORE child components are evaluated,
    // since AppShell() calls use_auth() which requires this context.
    provide_context(auth_state.clone());

    // Initialize and provide all feature stores via context so that
    // child components can retrieve them with use_context::<StoreType>().
    provide_context(AiStore::new());
    provide_context(DashboardStore::new());
    provide_context(EditorStore::new());
    provide_context(ProjectsStore::new());
    provide_context(ReportsStore::new());
    provide_context(ScaffoldStore::new());
    provide_context(ScansStore::new());
    provide_context(SpecsStore::new());
    provide_context(StructEngineStore::new());
    provide_context(TemplatesStore::new());
    provide_context(ViolationsStore::new());

    // Call component functions directly (the view! macro renders PascalCase
    // tags as plain HTML elements, so we invoke them as Rust functions).
    auth_provider(auth_state, vec![app_shell()])
}
