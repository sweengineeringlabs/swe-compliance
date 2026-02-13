use rsc_compat::prelude::*;

/// Authentication state.
#[derive(Clone)]
pub struct AuthState {
    pub token: Option<String>,
    pub username: Option<String>,
    pub authenticated: bool,
}

impl Default for AuthState {
    fn default() -> Self {
        // Attempt to restore token from localStorage
        let token = local_storage_get("swe_auth_token");
        let username = local_storage_get("swe_auth_username");
        let authenticated = token.is_some();

        Self {
            token,
            username,
            authenticated,
        }
    }
}

/// Authentication context provider.
///
/// Renders BOTH the login form and the children into the DOM, wrapped
/// in data-attribute containers.  A reactive effect watches
/// `state.authenticated` and toggles visibility between the two.
#[component]
pub fn auth_provider(
    state: Signal<AuthState>,
    children: Children,
) -> View {
    provide_context(state.clone());

    // Reactive effect: toggle login vs app display based on auth state
    {
        let state = state;
        effect(move || {
            let authed = state.get().authenticated;
            let (login_display, app_display) = if authed {
                ("none", "block")
            } else {
                ("block", "none")
            };
            let js = format!(
                r#"(function(){{
                    var login=document.querySelector('[data-auth-view="login"]');
                    var app=document.querySelector('[data-auth-view="app"]');
                    if(login)login.style.display='{}';
                    if(app)app.style.display='{}';
                }})()"#,
                login_display, app_display
            );
            let _ = js_sys::eval(&js);
        });
    }

    let initial_app_display = if state.get().authenticated { "block" } else { "none" };
    let initial_login_display = if state.get().authenticated { "none" } else { "block" };

    view! {
        <div data-auth-view="login" style={format!("display: {};", initial_login_display)}>
            {login_form(state)}
        </div>
        <div data-auth-view="app" style={format!("display: {};", initial_app_display)}>
            {View::fragment(children)}
        </div>
    }
}

/// Login form component.
#[component]
pub fn login_form(auth_state: Signal<AuthState>) -> View {
    let username = signal(String::new());
    let password = signal(String::new());
    let error = signal(Option::<String>::None);
    let loading = signal(false);

    let handle_submit = {
        let loading = loading.clone();
        let error = error.clone();
        let username = username.clone();
        let password = password.clone();
        let auth_state = auth_state.clone();
        move || {
        loading.set(true);
        error.set(None);

        let body = json_stringify(&json!({
            "username": username.get(),
            "password": password.get(),
        }));

        let loading2 = loading.clone();
        let error2 = error.clone();
        let username2 = username.clone();
        let auth_state2 = auth_state.clone();
        spawn(async move {
            let response = fetch("POST", "/api/v1/auth/login", vec![
                ("Content-Type", "application/json"),
            ], Some(&body)).await;

            loading2.set(false);

            if response.status == 200 {
                if let Some(parsed) = json_parse(&response.body) {
                    let token = parsed.get("token")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    let user = username2.get().clone();

                    local_storage_set("swe_auth_token", token);
                    local_storage_set("swe_auth_username", &user);

                    // Set the auth state — the reactive effect in
                    // auth_provider will toggle login/app visibility.
                    auth_state2.set(AuthState {
                        token: Some(token.to_string()),
                        username: Some(user),
                        authenticated: true,
                    });
                }
            } else {
                let msg = json_parse(&response.body)
                    .and_then(|p| p.get("error").cloned())
                    .and_then(|e| e.get("message").and_then(|m| m.as_str().map(String::from)))
                    .unwrap_or_else(|| "login failed".into());
                error2.set(Some(msg));
            }
        });
    }
    };

    view! {
        style {
            .login {
                display: flex;
                align-items: center;
                justify-content: center;
                min-height: 100vh;
                background: var(--color-bg);
            }

            .login__card {
                width: 400px;
                padding: var(--space-8);
            }

            .login__title {
                font-size: var(--font-size-xl);
                font-weight: 700;
                margin-bottom: var(--space-6);
                text-align: center;
            }

            .login__error {
                color: var(--color-error);
                font-size: var(--font-size-sm);
                margin-bottom: var(--space-4);
            }
        }

        <div class="login" data-testid="login-screen">
            <Card class="login__card">
                <div class="login__title" data-testid="login-title">
                    "swe-compliance"
                </div>

                {
                    let err_opt = error.get().clone();
                    if let Some(ref err_msg) = err_opt {
                        view! {
                            <LiveRegion>
                                <div class="login__error" data-testid="login-error">{err_msg.as_str()}</div>
                            </LiveRegion>
                        }
                    } else {
                        view! {}
                    }
                }

                <FormGroup>
                    <FormField label="Username">
                        <Input
                            value={username}
                            on:input={let u = username.clone(); move |v: String| u.set(v)}
                            placeholder="Enter username"
                            data-testid="login-username"
                        />
                    </FormField>

                    <FormField label="Password">
                        <Input
                            value={password}
                            on:input={let p = password.clone(); move |v: String| p.set(v)}
                            input_type="password"
                            placeholder="Enter password"
                            data-testid="login-password"
                        />
                    </FormField>

                    <Button
                        label="Sign In"
                        variant="primary"
                        disabled={loading.get()}
                        on:click={handle_submit}
                        data-testid="login-submit"
                    />
                </FormGroup>
            </Card>
        </div>
    }
}

/// Hook to access authentication state from any component.
pub fn use_auth() -> Signal<AuthState> {
    use_context::<Signal<AuthState>>()
}

/// Get the current JWT token from localStorage.
pub fn get_token() -> Option<String> {
    local_storage_get("swe_auth_token")
}

/// Log out — clear token and reset auth state.
pub fn logout() {
    local_storage_remove("swe_auth_token");
    local_storage_remove("swe_auth_username");
}
