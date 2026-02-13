use rsc_e2e_test::*;
use crate::test_common::{test_config, setup_auth};

/// Dashboard landing must render its projects grid.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn dashboard_stats_render(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/dashboard", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='dashboard-landing']").await.unwrap();
    ctx.assert_element_exists("[data-testid='dashboard-landing']").await;
}

/// Category chart component must render inside dashboard.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn dashboard_category_chart_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/dashboard", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='dashboard-landing']").await.unwrap();
    ctx.wait_for("[data-testid='category-chart']").await.unwrap();
    ctx.assert_element_visible("[data-testid='category-chart']").await;
}

/// Trend chart component must render inside dashboard.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn dashboard_trend_chart_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/dashboard", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='dashboard-landing']").await.unwrap();
    ctx.wait_for("[data-testid='trend-chart']").await.unwrap();
    ctx.assert_element_visible("[data-testid='trend-chart']").await;
}
