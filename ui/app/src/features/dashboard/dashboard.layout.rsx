use rsc_ui::prelude::*;

/// Dashboard feature layout wrapper.
/// Provides the page heading and consistent spacing for all dashboard routes.
component DashboardLayout(children: Children) {
    style {
        .dashboard-layout {
            display: flex;
            flex-direction: column;
            gap: var(--space-6);
            width: 100%;
            max-width: 1400px;
            margin: 0 auto;
        }

        .dashboard-layout__header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: var(--space-4);
        }

        .dashboard-layout__heading {
            font-size: var(--font-size-2xl);
            font-weight: 700;
            color: var(--color-text);
            margin: 0;
        }

        .dashboard-layout__timestamp {
            font-size: var(--font-size-xs);
            color: var(--color-text-muted);
        }

        .dashboard-layout__body {
            display: flex;
            flex-direction: column;
            gap: var(--space-6);
        }
    }

    let now = derived(|| {
        let date = js_date_now();
        format_date(date, "MMM d, yyyy HH:mm")
    });

    render {
        <section class="dashboard-layout" data-testid="dashboard-layout" aria-label="Compliance Dashboard">
            <header class="dashboard-layout__header">
                <h1 class="dashboard-layout__heading" data-testid="dashboard-heading">
                    "Compliance Dashboard"
                </h1>
                <span class="dashboard-layout__timestamp" data-testid="dashboard-timestamp">
                    "Last refreshed: " {now.get()}
                </span>
            </header>
            <div class="dashboard-layout__body" data-testid="dashboard-body">
                <slot />
            </div>
        </section>
    }
}
