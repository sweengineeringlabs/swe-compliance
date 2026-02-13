use rsc_compat::prelude::*;
use crate::features::dashboard::store::{self, DashboardStore};
use crate::features::dashboard::service;
use crate::features::dashboard::project_card::project_card;
use crate::features::dashboard::category_chart::category_chart;
use crate::features::dashboard::trend_chart::trend_chart;
use crate::features::dashboard::engine_summary::engine_summary;

/// Main dashboard page composing project cards, engine summary, and trend chart (FR-200..203).
#[component]
pub fn dashboard_landing() -> View {
    let s = use_context::<DashboardStore>();
    effect({
        let s = s.clone();
        move || {
            let s = s.clone();
            spawn(async move { service::load_dashboard(&s).await; });
        }
    });

    let handle_select = Callback::new({
        let s = s.clone();
        move |id: String| { store::select_project(&s, &id); }
    });

    view! {
        style {
            .dashboard { display: flex; flex-direction: column; gap: var(--space-6); }
            .dashboard__grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: var(--space-4); }
            .dashboard__details { display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-4); }
        }

        <div class="dashboard" data-testid="dashboard-landing">
            if s.loading.get() {
                <Progress indeterminate={true} data-testid="dashboard-loading" />
            }
            <Grid class="dashboard__grid" data-testid="dashboard-projects-grid">
                for project in s.projects.get().iter() {
                    {project_card(project.clone(), false, handle_select.clone())}
                }
            </Grid>
            if s.selected_project.get().is_some() {
                <div class="dashboard__details" data-testid="dashboard-details">
                    {engine_summary(s.category_breakdown.clone())}
                    {category_chart(s.category_breakdown.clone())}
                </div>
                {trend_chart(s.trend_data.clone())}
            }
        </div>
    }
}
