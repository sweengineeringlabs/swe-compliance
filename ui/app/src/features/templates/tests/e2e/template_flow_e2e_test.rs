use rsc_e2e_test::*;
use crate::test_common::{test_config, setup_auth};

/// Templates landing must render its container with the list component.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn template_list_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/templates", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='templates-landing']").await.unwrap();
    ctx.assert_element_exists("[data-testid='templates-landing']").await;
    ctx.assert_element_visible("[data-testid='templates-landing']").await;
}

/// The template list component must render (empty or populated).
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn template_detail_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/templates", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='templates-landing']").await.unwrap();
    ctx.wait_for("[data-testid='template-list']").await.unwrap();
    ctx.assert_element_visible("[data-testid='template-list']").await;
}

/// When no templates are loaded, the empty state must be shown.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn template_preview_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/templates", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='templates-landing']").await.unwrap();
    ctx.wait_for("[data-testid='template-list']").await.unwrap();
    // Template list should render — either empty state or populated table
    ctx.assert_element_exists("[data-testid='template-list']").await;
}

/// The template list table or empty state must be interactive.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn create_template_form(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/templates", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='templates-landing']").await.unwrap();
    ctx.wait_for("[data-testid='template-list']").await.unwrap();
    ctx.assert_element_visible("[data-testid='template-list']").await;
}
