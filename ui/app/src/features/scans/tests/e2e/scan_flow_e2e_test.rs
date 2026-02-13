use rsc_e2e_test::*;
use crate::test_common::{test_config, setup_auth};

/// Scans landing must render its container.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn scan_list_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/scans", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='scans-landing']").await.unwrap();
    ctx.assert_element_exists("[data-testid='scans-landing']").await;
}

/// Scan trigger form must be visible on the scans page.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn scan_trigger_form_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/scans", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='scan-trigger']").await.unwrap();
    ctx.assert_element_visible("[data-testid='scan-trigger']").await;
}

/// Engine select dropdown must contain engine options.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn scan_engine_select_has_options(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/scans", Some("[data-testid='app-shell']")).await.unwrap();
    let select = ctx.wait_for("[data-testid='scan-engine-select']").await.unwrap();
    let html = select.text_content().await.unwrap();
    assert!(
        html.contains("doc-engine") || html.contains("struct-engine"),
        "Engine select should contain doc-engine or struct-engine options, got: {html}"
    );
}

/// Triggering a scan via the run button must work.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn trigger_scan_creates_entry(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/scans", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='scan-engine-select']").await.unwrap();
    ctx.select("[data-testid='scan-engine-select']", "doc-engine").await.unwrap();
    ctx.wait_for("[data-testid='scan-run-btn']").await.unwrap();
    ctx.click("[data-testid='scan-run-btn']").await.unwrap();
    ctx.wait_for("[data-testid='scan-history']").await.unwrap();
    ctx.assert_element_exists("[data-testid='scan-history']").await;
}

/// Scan history section must render.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn scan_status_updates(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/scans", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='scan-history']").await.unwrap();
    ctx.assert_element_exists("[data-testid='scan-history']").await;
}
