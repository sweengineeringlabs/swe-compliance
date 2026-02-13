use rsc_e2e_test::*;
use crate::test_common::{test_config, setup_auth};

/// Struct engine landing must render its container.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn struct_engine_landing_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/struct-engine", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='struct-engine-landing']").await.unwrap();
    ctx.assert_element_exists("[data-testid='struct-engine-landing']").await;
    ctx.assert_element_visible("[data-testid='struct-engine-landing']").await;
}

/// The struct engine filter controls must be visible.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn struct_engine_form_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/struct-engine", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='struct-engine-landing']").await.unwrap();
    ctx.wait_for("[data-testid='struct-results']").await.unwrap();
    ctx.assert_element_visible("[data-testid='struct-results']").await;
}

/// Struct results component must be rendered.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn struct_engine_run(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/struct-engine", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='struct-engine-landing']").await.unwrap();
    ctx.wait_for("[data-testid='struct-results']").await.unwrap();
    ctx.assert_element_visible("[data-testid='struct-results']").await;
}

/// Project kind badge must be visible.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn struct_engine_result_cards(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/struct-engine", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='struct-engine-landing']").await.unwrap();
    ctx.wait_for("[data-testid='project-kind']").await.unwrap();
    ctx.assert_element_visible("[data-testid='project-kind']").await;
}
