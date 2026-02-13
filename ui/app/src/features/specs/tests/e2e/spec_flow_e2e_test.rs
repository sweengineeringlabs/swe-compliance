use rsc_e2e_test::*;
use crate::test_common::{test_config, setup_auth};

/// Specs landing must render its container.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn spec_list_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/specs", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='specs-landing']").await.unwrap();
    ctx.assert_element_exists("[data-testid='specs-landing']").await;
    ctx.assert_element_visible("[data-testid='specs-landing']").await;
}

/// The spec tree component must be rendered.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn spec_tree_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/specs", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='specs-landing']").await.unwrap();
    ctx.wait_for("[data-testid='spec-tree']").await.unwrap();
    ctx.assert_element_visible("[data-testid='spec-tree']").await;
}

/// The specs browser layout (tree + content) must render.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn spec_detail_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/specs", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='specs-landing']").await.unwrap();
    ctx.wait_for("[data-testid='specs-browser']").await.unwrap();
    ctx.assert_element_visible("[data-testid='specs-browser']").await;
}

/// The spec search input must accept text.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn spec_search_works(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/specs", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='specs-landing']").await.unwrap();
    let input = ctx.wait_for("[data-testid='specs-search-input']").await.unwrap();
    ctx.fill("[data-testid='specs-search-input']", "FR-").await.unwrap();
    ctx.wait_for_timeout(500).await;
    let value = input.get_property("value").await.unwrap();
    assert!(
        value.as_str().map_or(false, |v| v.contains("FR-")),
        "Spec search input should contain typed text"
    );
}
