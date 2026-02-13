use rsc_ui::prelude::*;
use crate::features::dashboard::store::{self, DashboardStore};
use crate::features::dashboard::service;
use crate::features::dashboard::project_card::ProjectCard;
use crate::features::dashboard::category_chart::CategoryChart;
use crate::features::dashboard::trend_chart::TrendChart;
use crate::features::dashboard::engine_summary::EngineSummary;

/// Main dashboard page composing project cards, engine summary, and trend chart (FR-200..203).
component DashboardLanding() {
    let s = use_context::<DashboardStore>();

    { let s = s.clone(); effect(move || {
        let s = s.clone();
        spawn(async move { service::load_dashboard(&s).await; });
    }); }

    let handle_select = { let s = s.clone(); move |id: String| { store::select_project(&s, &id); } };

    style {
        .dashboard { display: flex; flex-direction: column; gap: var(--space-6); }
        .dashboard__grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: var(--space-4); }
        .dashboard__details { display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-4); }
    }

    render {
        <div class="dashboard" data-testid="dashboard-landing">
            @if s.loading.get() {
                <Progress indeterminate={true} data-testid="dashboard-loading" />
            }
            <Grid class="dashboard__grid" data-testid="dashboard-projects-grid">
                @for project in s.projects.get().iter() {
                    <ProjectCard
                        project={project.clone()}
                        on_select={handle_select.clone()}
                    />
                }
            </Grid>
            @if s.selected_project.get().is_some() {
                <div class="dashboard__details" data-testid="dashboard-details">
                    <EngineSummary categories={s.category_breakdown.clone()} />
                    <CategoryChart categories={s.category_breakdown.clone()} />
                </div>
                <TrendChart trend_data={s.trend_data.clone()} />
            }
        </div>
    }
}
