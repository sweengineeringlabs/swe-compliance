use rsc_e2e_test::*;
use crate::test_common::{test_config, setup_auth};

/// Reports landing must render its container.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn report_list_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/reports", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='reports-landing']").await.unwrap();
    ctx.assert_element_exists("[data-testid='reports-landing']").await;
    ctx.assert_element_visible("[data-testid='reports-landing']").await;
}

/// Report export controls must be rendered.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn report_detail_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/reports", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='reports-landing']").await.unwrap();
    ctx.wait_for("[data-testid='report-export']").await.unwrap();
    ctx.assert_element_visible("[data-testid='report-export']").await;
}

/// Report export button must be present and visible.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn report_export_button(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/reports", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='reports-landing']").await.unwrap();
    ctx.wait_for("[data-testid='report-export-btn']").await.unwrap();
    ctx.assert_element_visible("[data-testid='report-export-btn']").await;
}
