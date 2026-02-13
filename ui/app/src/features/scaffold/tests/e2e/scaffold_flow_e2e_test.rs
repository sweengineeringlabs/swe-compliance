use rsc_e2e_test::*;
use crate::test_common::{test_config, setup_auth};

/// Scaffold landing must render its container.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn scaffold_form_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/scaffold", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='scaffold-landing']").await.unwrap();
    ctx.assert_element_exists("[data-testid='scaffold-landing']").await;
    ctx.assert_element_visible("[data-testid='scaffold-landing']").await;
}

/// SRS upload area must be present and visible.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn scaffold_select_scope(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/scaffold", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='scaffold-landing']").await.unwrap();
    ctx.wait_for("[data-testid='srs-upload']").await.unwrap();
    ctx.assert_element_visible("[data-testid='srs-upload']").await;
}

/// The SRS content input must accept text.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn scaffold_run_generates(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/scaffold", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='scaffold-landing']").await.unwrap();
    ctx.wait_for("[data-testid='srs-upload']").await.unwrap();
    let input = ctx.wait_for("[data-testid='srs-content-input']").await.unwrap();
    ctx.fill("[data-testid='srs-content-input']", "# Test SRS\n\n## Requirements\n\n- FR-001: Test requirement").await.unwrap();
    let value = input.get_property("value").await.unwrap();
    assert!(
        value.as_str().map_or(false, |v| v.contains("FR-001")),
        "SRS content input should contain typed content"
    );
}
