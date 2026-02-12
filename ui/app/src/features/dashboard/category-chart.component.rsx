use rsc_ui::prelude::*;
use crate::features::dashboard::dashboard_type::CategoryBreakdown;

/// Displays a stacked horizontal bar chart of compliance results by SDLC category.
/// Implemented as a styled table with inline bar segments since a dedicated charting
/// component may not be available. Each row shows the category name, a stacked bar
/// representing passed/failed/skipped proportions, and numeric counts.
component CategoryChart(
    categories: Signal<Vec<CategoryBreakdown>>,
) {
    let sorted_categories = derived(move || {
        let mut cats = categories.get().clone();
        cats.sort_by(|a, b| a.category.cmp(&b.category));
        cats
    });

    let has_data = derived(move || !categories.get().is_empty());

    style {
        .category-chart {
            width: 100%;
        }

        .category-chart__title {
            font-size: var(--font-size-md);
            font-weight: 600;
            color: var(--color-text);
            margin: 0 0 var(--space-4) 0;
        }

        .category-chart__empty {
            padding: var(--space-8);
            text-align: center;
            color: var(--color-text-muted);
            font-size: var(--font-size-sm);
        }

        .category-chart__table {
            width: 100%;
            border-collapse: collapse;
        }

        .category-chart__th {
            text-align: left;
            font-size: var(--font-size-xs);
            font-weight: 600;
            color: var(--color-text-secondary);
            text-transform: uppercase;
            letter-spacing: 0.05em;
            padding: var(--space-2) var(--space-3);
            border-bottom: 2px solid var(--color-border);
        }

        .category-chart__th--right {
            text-align: right;
        }

        .category-chart__td {
            padding: var(--space-2) var(--space-3);
            border-bottom: 1px solid var(--color-border);
            font-size: var(--font-size-sm);
            vertical-align: middle;
        }

        .category-chart__category-name {
            font-weight: 500;
            color: var(--color-text);
            white-space: nowrap;
            max-width: 200px;
            overflow: hidden;
            text-overflow: ellipsis;
        }

        .category-chart__bar-cell {
            width: 40%;
            padding: var(--space-2) var(--space-3);
            border-bottom: 1px solid var(--color-border);
        }

        .category-chart__bar-container {
            display: flex;
            height: 20px;
            border-radius: var(--radius-sm);
            overflow: hidden;
            background: var(--color-surface-alt);
        }

        .category-chart__bar-segment {
            height: 100%;
            transition: width 0.3s ease;
            min-width: 0;
        }

        .category-chart__bar-segment--passed {
            background: var(--color-success);
        }

        .category-chart__bar-segment--failed {
            background: var(--color-error);
        }

        .category-chart__bar-segment--skipped {
            background: var(--color-text-muted);
            opacity: 0.4;
        }

        .category-chart__count {
            text-align: right;
            font-variant-numeric: tabular-nums;
            color: var(--color-text-secondary);
        }

        .category-chart__count--passed {
            color: var(--color-success);
        }

        .category-chart__count--failed {
            color: var(--color-error);
        }

        .category-chart__count--skipped {
            color: var(--color-text-muted);
        }

        .category-chart__rate {
            text-align: right;
            font-weight: 600;
            font-variant-numeric: tabular-nums;
        }

        .category-chart__rate--high {
            color: var(--color-success);
        }

        .category-chart__rate--medium {
            color: var(--color-warning);
        }

        .category-chart__rate--low {
            color: var(--color-error);
        }

        .category-chart__footer {
            font-size: var(--font-size-xs);
            color: var(--color-text-muted);
            padding-top: var(--space-2);
            text-align: right;
        }
    }

    render {
        <Card class="category-chart" data-testid="category-chart">
            <h3 class="category-chart__title" data-testid="category-chart-title">
                "Category Breakdown"
            </h3>

            @if !has_data.get() {
                <div class="category-chart__empty" data-testid="category-chart-empty">
                    "No category data available. Select a project to view breakdown."
                </div>
            } @else {
                <table class="category-chart__table" role="table" aria-label="Compliance by category" data-testid="category-chart-table">
                    <thead>
                        <tr>
                            <th class="category-chart__th">"Category"</th>
                            <th class="category-chart__th">"Distribution"</th>
                            <th class="category-chart__th category-chart__th--right">"Passed"</th>
                            <th class="category-chart__th category-chart__th--right">"Failed"</th>
                            <th class="category-chart__th category-chart__th--right">"Skipped"</th>
                            <th class="category-chart__th category-chart__th--right">"Rate"</th>
                        </tr>
                    </thead>
                    <tbody>
                        @for cat in sorted_categories.get().iter() {
                            <tr data-testid={format!("category-row-{}", slug(&cat.category))}>
                                <td class="category-chart__td">
                                    <span class="category-chart__category-name" title={&cat.category}>
                                        {&cat.category}
                                    </span>
                                </td>
                                <td class="category-chart__bar-cell">
                                    <div
                                        class="category-chart__bar-container"
                                        role="img"
                                        aria-label={format!(
                                            "{}: {} passed, {} failed, {} skipped",
                                            cat.category, cat.passed, cat.failed, cat.skipped
                                        )}
                                        data-testid={format!("category-bar-{}", slug(&cat.category))}
                                    >
                                        @if cat.total() > 0 {
                                            <div
                                                class="category-chart__bar-segment category-chart__bar-segment--passed"
                                                style={format!("width: {:.1}%", bar_pct(cat.passed, cat.total()))}
                                            />
                                            <div
                                                class="category-chart__bar-segment category-chart__bar-segment--failed"
                                                style={format!("width: {:.1}%", bar_pct(cat.failed, cat.total()))}
                                            />
                                            <div
                                                class="category-chart__bar-segment category-chart__bar-segment--skipped"
                                                style={format!("width: {:.1}%", bar_pct(cat.skipped, cat.total()))}
                                            />
                                        }
                                    </div>
                                </td>
                                <td class="category-chart__td category-chart__count category-chart__count--passed"
                                    data-testid={format!("category-passed-{}", slug(&cat.category))}
                                >
                                    {cat.passed}
                                </td>
                                <td class="category-chart__td category-chart__count category-chart__count--failed"
                                    data-testid={format!("category-failed-{}", slug(&cat.category))}
                                >
                                    {cat.failed}
                                </td>
                                <td class="category-chart__td category-chart__count category-chart__count--skipped"
                                    data-testid={format!("category-skipped-{}", slug(&cat.category))}
                                >
                                    {cat.skipped}
                                </td>
                                <td class="category-chart__td">
                                    {
                                        let rate = cat.pass_rate();
                                        let rate_class = if rate >= 90.0 {
                                            "category-chart__rate category-chart__rate--high"
                                        } else if rate >= 70.0 {
                                            "category-chart__rate category-chart__rate--medium"
                                        } else {
                                            "category-chart__rate category-chart__rate--low"
                                        };
                                        <span
                                            class={rate_class}
                                            data-testid={format!("category-rate-{}", slug(&cat.category))}
                                        >
                                            {format!("{:.0}%", rate)}
                                        </span>
                                    }
                                </td>
                            </tr>
                        }
                    </tbody>
                </table>

                <div class="category-chart__footer" data-testid="category-chart-footer">
                    {format!("{} categories", sorted_categories.get().len())}
                </div>
            }
        </Card>
    }
}

/// Calculate percentage for a bar segment, avoiding division by zero.
fn bar_pct(value: u32, total: u32) -> f64 {
    if total == 0 {
        0.0
    } else {
        (value as f64 / total as f64) * 100.0
    }
}

/// Convert a category name to a URL/test-id friendly slug.
fn slug(name: &str) -> String {
    name.to_lowercase()
        .replace(' ', "-")
        .replace('/', "-")
        .replace('_', "-")
}
