use rsc_ui::prelude::*;

/// Authentication state.
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
component AuthProvider(
    state: Signal<AuthState>,
    children: Children,
) {
    provide_context(state);

    render {
        @if state.get().authenticated {
            {children}
        } @else {
            <LoginForm auth_state={state} />
        }
    }
}

/// Login form component.
component LoginForm(auth_state: Signal<AuthState>) {
    let username = signal(String::new());
    let password = signal(String::new());
    let error = signal(Option::<String>::None);
    let loading = signal(false);

    let handle_submit = move || {
        loading.set(true);
        error.set(None);

        let body = json_stringify(&json!({
            "username": username.get(),
            "password": password.get(),
        }));

        spawn(async move {
            let response = fetch("POST", "/api/v1/auth/login", vec![
                ("Content-Type", "application/json"),
            ], Some(&body)).await;

            loading.set(false);

            if response.status == 200 {
                if let Some(parsed) = json_parse(&response.body) {
                    let token = parsed.get_str("token").unwrap_or_default();
                    let user = username.get().clone();

                    local_storage_set("swe_auth_token", &token);
                    local_storage_set("swe_auth_username", &user);

                    auth_state.set(AuthState {
                        token: Some(token.into()),
                        username: Some(user),
                        authenticated: true,
                    });
                }
            } else {
                let msg = json_parse(&response.body)
                    .and_then(|p| p.get("error"))
                    .and_then(|e| e.get_str("message"))
                    .unwrap_or("login failed".into());
                error.set(Some(msg.into()));
            }
        });
    };

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

    render {
        <div class="login" data-testid="login-screen">
            <Card class="login__card">
                <div class="login__title" data-testid="login-title">
                    "swe-compliance"
                </div>

                @if let Some(err) = error.get() {
                    <LiveRegion>
                        <div class="login__error" data-testid="login-error">{err}</div>
                    </LiveRegion>
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
