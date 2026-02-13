use rsc_ui::prelude::*;
use crate::features::dashboard::dashboard_store as store;
use crate::features::dashboard::project_card::ProjectCard;
use crate::features::dashboard::category_chart::CategoryChart;
use crate::features::dashboard::trend_chart::TrendChart;
use crate::features::dashboard::engine_summary::EngineSummary;

/// Main dashboard page composing project cards, engine summary, and trend chart (FR-200..203).
component DashboardLanding() {
    effect(|| { store::load_projects(); });

    let handle_select = move |id: String| { store::select_project(&id); };

    style {
        .dashboard { display: flex; flex-direction: column; gap: var(--space-6); }
        .dashboard__grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: var(--space-4); }
        .dashboard__details { display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-4); }
    }

    render {
        <div class="dashboard" data-testid="dashboard-landing">
            @if store::loading.get() {
                <Progress indeterminate={true} data-testid="dashboard-loading" />
            }
            <Grid class="dashboard__grid" data-testid="dashboard-projects-grid">
                @for project in store::projects.get().iter() {
                    <ProjectCard
                        project={project.clone()}
                        on_select={handle_select.clone()}
                    />
                }
            </Grid>
            @if store::selected_project.get().is_some() {
                <div class="dashboard__details" data-testid="dashboard-details">
                    <EngineSummary categories={store::category_breakdown.clone()} />
                    <CategoryChart categories={store::category_breakdown.clone()} />
                </div>
                <TrendChart trend_data={store::trend_data.clone()} />
            }
        </div>
    }
}
