use rsc_compat::prelude::*;
use crate::util::auth::use_auth;
use crate::features::ai::ai_landing::ai_landing;
use crate::features::dashboard::dashboard_landing::dashboard_landing;
use crate::features::editor::editor_landing::editor_landing;
use crate::features::projects::project_landing::projects_landing;
use crate::features::reports::report_landing::reports_landing;
use crate::features::scaffold::scaffold_landing::scaffold_landing;
use crate::features::scans::scan_landing::scans_landing;
use crate::features::specs::spec_landing::specs_landing;
use crate::features::struct_engine::struct_engine_landing::struct_engine_landing;
use crate::features::templates::template_landing::templates_landing;
use crate::features::violations::violation_landing::violations_landing;

/// Navigate to a path and update the route signal.
fn go(route: &Signal<String>, path: &str) {
    navigate(path);
    route.set(path.to_string());
}

/// Root application layout with sidebar navigation and client-side SPA router.
///
/// Uses `use_route()` to read the initial URL pathname, stores it in a
/// reactive `Signal<String>`, and renders the matching feature landing page.
/// Navigation links call `navigate()` (History.pushState) and update the
/// signal so the content area re-renders without a full page reload.
#[component]
pub fn app_shell() -> View {
    let _auth = use_auth();
    let route = signal(use_route());

    // --- helper clones for each nav on:click closure ---
    let r_dashboard    = route.clone();
    let r_projects     = route.clone();
    let r_scans        = route.clone();
    let r_violations   = route.clone();
    let r_scaffold     = route.clone();
    let r_templates    = route.clone();
    let r_reports      = route.clone();
    let r_ai           = route.clone();
    let r_editor       = route.clone();
    let r_specs        = route.clone();
    let r_struct       = route.clone();

    // Clone for route matching in the content area.
    let r_view = route.clone();

    view! {
        style {
            .app {
                display: flex;
                min-height: 100vh;
                background: var(--color-bg);
                color: var(--color-text);
                font-family: var(--font-family);
            }

            .app__sidebar {
                width: 240px;
                background: var(--color-surface);
                border-right: 1px solid var(--color-border);
                display: flex;
                flex-direction: column;
                padding: var(--space-4) 0;
            }

            .app__logo {
                padding: var(--space-4) var(--space-6);
                font-size: var(--font-size-lg);
                font-weight: 700;
                color: var(--color-primary);
                border-bottom: 1px solid var(--color-border);
                margin-bottom: var(--space-4);
            }

            .app__nav {
                display: flex;
                flex-direction: column;
                gap: var(--space-1);
                padding: 0 var(--space-3);
                flex: 1;
            }

            .app__nav-item {
                display: flex;
                align-items: center;
                gap: var(--space-3);
                padding: var(--space-2) var(--space-3);
                border-radius: var(--radius-md);
                color: var(--color-text-secondary);
                text-decoration: none;
                font-size: var(--font-size-sm);
                transition: background 0.15s, color 0.15s;
                cursor: pointer;
                background: none;
                border: none;
                width: 100%;
                text-align: left;
                font-family: inherit;
            }

            .app__nav-item:hover {
                background: var(--color-hover);
                color: var(--color-text);
            }

            .app__nav-item--active {
                background: var(--color-primary-bg);
                color: var(--color-primary);
                font-weight: 600;
            }

            .app__content {
                flex: 1;
                padding: var(--space-6);
                overflow-y: auto;
            }

            .app__footer {
                padding: var(--space-3) var(--space-6);
                font-size: var(--font-size-xs);
                color: var(--color-text-muted);
                border-top: 1px solid var(--color-border);
            }
        }

        <SkipLink target="main-content" />
        <div class="app" data-testid="app-shell">
            <nav class="app__sidebar" data-testid="sidebar" role="navigation" aria-label="Main navigation">
                <div class="app__logo" data-testid="app-logo">
                    "swe-compliance"
                </div>
                <div class="app__nav">
                    <a class="app__nav-item"
                       class:app__nav-item--active={route.get().starts_with("/dashboard") || route.get() == "/"}
                       on:click={move || go(&r_dashboard, "/dashboard")}
                       role="link"
                       tabindex="0"
                       data-testid="nav-dashboard">
                        "Dashboard"
                    </a>
                    <a class="app__nav-item"
                       class:app__nav-item--active={route.get().starts_with("/projects")}
                       on:click={move || go(&r_projects, "/projects")}
                       role="link"
                       tabindex="0"
                       data-testid="nav-projects">
                        "Projects"
                    </a>
                    <a class="app__nav-item"
                       class:app__nav-item--active={route.get().starts_with("/scans")}
                       on:click={move || go(&r_scans, "/scans")}
                       role="link"
                       tabindex="0"
                       data-testid="nav-scans">
                        "Scans"
                    </a>
                    <a class="app__nav-item"
                       class:app__nav-item--active={route.get().starts_with("/violations")}
                       on:click={move || go(&r_violations, "/violations")}
                       role="link"
                       tabindex="0"
                       data-testid="nav-violations">
                        "Violations"
                    </a>
                    <a class="app__nav-item"
                       class:app__nav-item--active={route.get().starts_with("/scaffold")}
                       on:click={move || go(&r_scaffold, "/scaffold")}
                       role="link"
                       tabindex="0"
                       data-testid="nav-scaffold">
                        "Scaffold"
                    </a>
                    <a class="app__nav-item"
                       class:app__nav-item--active={route.get().starts_with("/templates")}
                       on:click={move || go(&r_templates, "/templates")}
                       role="link"
                       tabindex="0"
                       data-testid="nav-templates">
                        "Templates"
                    </a>
                    <a class="app__nav-item"
                       class:app__nav-item--active={route.get().starts_with("/reports")}
                       on:click={move || go(&r_reports, "/reports")}
                       role="link"
                       tabindex="0"
                       data-testid="nav-reports">
                        "Reports"
                    </a>
                    <a class="app__nav-item"
                       class:app__nav-item--active={route.get().starts_with("/ai")}
                       on:click={move || go(&r_ai, "/ai")}
                       role="link"
                       tabindex="0"
                       data-testid="nav-ai">
                        "AI Features"
                    </a>
                    <a class="app__nav-item"
                       class:app__nav-item--active={route.get().starts_with("/editor")}
                       on:click={move || go(&r_editor, "/editor")}
                       role="link"
                       tabindex="0"
                       data-testid="nav-editor">
                        "SRS Editor"
                    </a>
                    <a class="app__nav-item"
                       class:app__nav-item--active={route.get().starts_with("/specs")}
                       on:click={move || go(&r_specs, "/specs")}
                       role="link"
                       tabindex="0"
                       data-testid="nav-specs">
                        "Spec Viewer"
                    </a>
                    <a class="app__nav-item"
                       class:app__nav-item--active={route.get().starts_with("/struct-engine")}
                       on:click={move || go(&r_struct, "/struct-engine")}
                       role="link"
                       tabindex="0"
                       data-testid="nav-struct-engine">
                        "Struct Engine"
                    </a>
                </div>
                <div class="app__footer">
                    "v0.1.0"
                </div>
            </nav>
            <main id="main-content" class="app__content" data-testid="main-content" role="main">
                {match r_view.get().as_str() {
                    "/" | "/dashboard" => dashboard_landing(),
                    "/projects"       => projects_landing(),
                    "/scans"          => scans_landing(),
                    "/editor"         => editor_landing(),
                    "/specs"          => specs_landing(),
                    "/violations"     => violations_landing(),
                    "/reports"        => reports_landing(),
                    "/scaffold"       => scaffold_landing(),
                    "/templates"      => templates_landing(),
                    "/ai"             => ai_landing(),
                    "/struct-engine"  => struct_engine_landing(),
                    _                 => dashboard_landing(),
                }}
            </main>
        </div>
    }
}
