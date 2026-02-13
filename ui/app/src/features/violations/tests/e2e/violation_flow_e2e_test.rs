use rsc_e2e_test::*;
use crate::test_common::{test_config, setup_auth};

/// Violations landing must render its container.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn violation_list_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/violations", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='violations-landing']").await.unwrap();
    ctx.assert_element_exists("[data-testid='violations-landing']").await;
}

/// The violation filter bar must be rendered and visible.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn violation_filter_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/violations", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='violation-filter']").await.unwrap();
    ctx.assert_element_visible("[data-testid='violation-filter']").await;
}

/// Clicking the error filter checkbox must trigger a DOM interaction.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn filter_by_severity(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/violations", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='violation-filter']").await.unwrap();
    ctx.wait_for("[data-testid='filter-error']").await.unwrap();
    ctx.click("[data-testid='filter-error']").await.unwrap();
    ctx.wait_for_timeout(500).await;
    ctx.assert_element_exists("[data-testid='violations-landing']").await;
}

/// The search input must accept text.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn search_violations(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/violations", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='violation-filter']").await.unwrap();
    let input = ctx.wait_for("[data-testid='filter-search']").await.unwrap();
    ctx.fill("[data-testid='filter-search']", "missing").await.unwrap();
    ctx.wait_for_timeout(500).await;
    let value = input.get_property("value").await.unwrap();
    assert!(
        value.as_str().map_or(false, |v| v.contains("missing")),
        "Search input should contain typed text"
    );
}
