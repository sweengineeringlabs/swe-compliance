use rsc_ui::prelude::*;
use crate::util::auth::use_auth;

/// Root application layout with sidebar navigation and content area.
component App() {
    let auth = use_auth();
    let current_route = use_route();

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

    render {
        <SkipLink target="main-content" />
        <div class="app" data-testid="app-shell">
            <nav class="app__sidebar" data-testid="sidebar" role="navigation" aria-label="Main navigation">
                <div class="app__logo" data-testid="app-logo">
                    "swe-compliance"
                </div>
                <div class="app__nav">
                    <a href="/dashboard"
                       class="app__nav-item"
                       class:app__nav-item--active={current_route.starts_with("/dashboard")}
                       data-testid="nav-dashboard">
                        "Dashboard"
                    </a>
                    <a href="/projects"
                       class="app__nav-item"
                       class:app__nav-item--active={current_route.starts_with("/projects")}
                       data-testid="nav-projects">
                        "Projects"
                    </a>
                    <a href="/scans"
                       class="app__nav-item"
                       class:app__nav-item--active={current_route.starts_with("/scans")}
                       data-testid="nav-scans">
                        "Scans"
                    </a>
                    <a href="/violations"
                       class="app__nav-item"
                       class:app__nav-item--active={current_route.starts_with("/violations")}
                       data-testid="nav-violations">
                        "Violations"
                    </a>
                    <a href="/scaffold"
                       class="app__nav-item"
                       class:app__nav-item--active={current_route.starts_with("/scaffold")}
                       data-testid="nav-scaffold">
                        "Scaffold"
                    </a>
                    <a href="/templates"
                       class="app__nav-item"
                       class:app__nav-item--active={current_route.starts_with("/templates")}
                       data-testid="nav-templates">
                        "Templates"
                    </a>
                    <a href="/reports"
                       class="app__nav-item"
                       class:app__nav-item--active={current_route.starts_with("/reports")}
                       data-testid="nav-reports">
                        "Reports"
                    </a>
                    <a href="/ai"
                       class="app__nav-item"
                       class:app__nav-item--active={current_route.starts_with("/ai")}
                       data-testid="nav-ai">
                        "AI Features"
                    </a>
                    <a href="/editor"
                       class="app__nav-item"
                       class:app__nav-item--active={current_route.starts_with("/editor")}
                       data-testid="nav-editor">
                        "SRS Editor"
                    </a>
                    <a href="/specs"
                       class="app__nav-item"
                       class:app__nav-item--active={current_route.starts_with("/specs")}
                       data-testid="nav-specs">
                        "Spec Viewer"
                    </a>
                    <a href="/struct-engine"
                       class="app__nav-item"
                       class:app__nav-item--active={current_route.starts_with("/struct-engine")}
                       data-testid="nav-struct-engine">
                        "Struct Engine"
                    </a>
                </div>
                <div class="app__footer">
                    "v0.1.0"
                </div>
            </nav>
            <main id="main-content" class="app__content" data-testid="main-content" role="main">
                <slot />
            </main>
        </div>
    }
}
