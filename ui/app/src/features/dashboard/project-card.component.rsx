use rsc_ui::prelude::*;
use crate::features::dashboard::dashboard_type::DashboardProject;

/// Displays a single project's compliance status as a card.
/// Shows the project name, scope badge, compliance progress bar,
/// and individual passed/failed/skipped counts.
component ProjectCard(
    project: DashboardProject,
    selected: bool,
    on_select: Callback<String>,
) {
    let percentage = derived(|| project.compliance_summary.percentage());
    let passed = project.compliance_summary.passed;
    let failed = project.compliance_summary.failed;
    let skipped = project.compliance_summary.skipped;
    let total = project.compliance_summary.total;

    let variant = derived(|| {
        let pct = percentage.get();
        if pct >= 90.0 {
            "success"
        } else if pct >= 70.0 {
            "warning"
        } else {
            "error"
        }
    });

    let scope_variant = derived(|| {
        match project.scope.as_str() {
            "full" => "primary",
            "partial" => "warning",
            "minimal" => "neutral",
            _ => "neutral",
        }
    });

    let handle_click = move || {
        on_select.call(project.id.clone());
    };

    let handle_keydown = move |event: KeyboardEvent| {
        if event.key == "Enter" || event.key == " " {
            event.prevent_default();
            on_select.call(project.id.clone());
        }
    };

    style {
        .project-card {
            cursor: pointer;
            transition: box-shadow 0.15s, border-color 0.15s;
            border: 2px solid transparent;
        }

        .project-card:hover {
            box-shadow: var(--shadow-md);
        }

        .project-card:focus-visible {
            outline: 2px solid var(--color-primary);
            outline-offset: 2px;
        }

        .project-card--selected {
            border-color: var(--color-primary);
            box-shadow: var(--shadow-md);
        }

        .project-card__header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            margin-bottom: var(--space-3);
        }

        .project-card__name {
            font-size: var(--font-size-md);
            font-weight: 600;
            color: var(--color-text);
            margin: 0;
        }

        .project-card__type {
            font-size: var(--font-size-xs);
            color: var(--color-text-muted);
            margin-top: var(--space-1);
        }

        .project-card__progress {
            margin: var(--space-3) 0;
        }

        .project-card__percentage {
            font-size: var(--font-size-2xl);
            font-weight: 700;
            margin-bottom: var(--space-1);
        }

        .project-card__percentage--success {
            color: var(--color-success);
        }

        .project-card__percentage--warning {
            color: var(--color-warning);
        }

        .project-card__percentage--error {
            color: var(--color-error);
        }

        .project-card__counts {
            display: flex;
            gap: var(--space-4);
            margin-top: var(--space-3);
            font-size: var(--font-size-sm);
        }

        .project-card__count {
            display: flex;
            align-items: center;
            gap: var(--space-1);
        }

        .project-card__count-dot {
            width: 8px;
            height: 8px;
            border-radius: 50%;
        }

        .project-card__count-dot--passed {
            background: var(--color-success);
        }

        .project-card__count-dot--failed {
            background: var(--color-error);
        }

        .project-card__count-dot--skipped {
            background: var(--color-text-muted);
        }

        .project-card__total {
            font-size: var(--font-size-xs);
            color: var(--color-text-muted);
            margin-top: var(--space-2);
        }
    }

    render {
        <Card
            class="project-card"
            class:project-card--selected={selected}
            on:click={handle_click}
            on:keydown={handle_keydown}
            tabindex="0"
            role="button"
            aria-pressed={selected.to_string()}
            aria-label={format!("Project {} - {:.1}% compliant", project.name, percentage.get())}
            data-testid={format!("project-card-{}", project.id)}
        >
            <div class="project-card__header">
                <div>
                    <h3 class="project-card__name" data-testid="project-card-name">
                        {&project.name}
                    </h3>
                    <div class="project-card__type" data-testid="project-card-type">
                        {&project.project_type}
                    </div>
                </div>
                <Badge
                    variant={scope_variant.get()}
                    data-testid="project-card-scope"
                >
                    {&project.scope}
                </Badge>
            </div>

            <div class="project-card__progress">
                <div
                    class="project-card__percentage"
                    class:project-card__percentage--success={variant.get() == "success"}
                    class:project-card__percentage--warning={variant.get() == "warning"}
                    class:project-card__percentage--error={variant.get() == "error"}
                    data-testid="project-card-percentage"
                >
                    {format!("{:.1}%", percentage.get())}
                </div>
                <Progress
                    value={percentage.get()}
                    max={100.0}
                    variant={variant.get()}
                    aria-label={format!("Compliance: {:.1}%", percentage.get())}
                    data-testid="project-card-progress"
                />
            </div>

            <div class="project-card__counts" data-testid="project-card-counts">
                <span class="project-card__count">
                    <span class="project-card__count-dot project-card__count-dot--passed" />
                    <span data-testid="project-card-passed">{passed} " passed"</span>
                </span>
                <span class="project-card__count">
                    <span class="project-card__count-dot project-card__count-dot--failed" />
                    <span data-testid="project-card-failed">{failed} " failed"</span>
                </span>
                <span class="project-card__count">
                    <span class="project-card__count-dot project-card__count-dot--skipped" />
                    <span data-testid="project-card-skipped">{skipped} " skipped"</span>
                </span>
            </div>

            <div class="project-card__total" data-testid="project-card-total">
                {format!("{total} total checks")}
            </div>
        </Card>
    }
}
