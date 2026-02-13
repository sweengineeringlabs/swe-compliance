use rsc_ui::prelude::*;
use crate::page::app::AppShell;
use crate::util::auth::{AuthProvider, AuthState};

/// Root component — wraps AppShell with authentication context.
component App {
    let auth_state = signal(AuthState::default());

    render {
        <AuthProvider state={auth_state}>
            <AppShell />
        </AuthProvider>
    }
}
