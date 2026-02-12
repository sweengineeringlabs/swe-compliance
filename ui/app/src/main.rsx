use rsc_ui::prelude::*;
use crate::page::app::App;
use crate::util::auth::{AuthProvider, AuthState};

/// Application entry point.
/// Mounts the root App component with authentication context.
fn main() {
    let auth_state = signal(AuthState::default());

    mount(|| {
        render {
            <AuthProvider state={auth_state}>
                <App />
            </AuthProvider>
        }
    });
}
