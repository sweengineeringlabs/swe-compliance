use rsc_e2e_test::*;
use crate::test_common::{test_config, setup_auth};

/// Editor landing must render its container.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn editor_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/editor", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='editor-landing']").await.unwrap();
    ctx.assert_element_exists("[data-testid='editor-landing']").await;
    ctx.assert_element_visible("[data-testid='editor-landing']").await;
}

/// Editor action buttons (validate, save) must be visible.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn editor_toolbar_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/editor", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='editor-landing']").await.unwrap();
    ctx.wait_for("[data-testid='editor-actions']").await.unwrap();
    ctx.assert_element_visible("[data-testid='editor-actions']").await;
}

/// Markdown editor textarea must be rendered and visible.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn editor_textarea_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/editor", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='editor-landing']").await.unwrap();
    ctx.wait_for("[data-testid='markdown-editor']").await.unwrap();
    ctx.assert_element_visible("[data-testid='markdown-editor']").await;
}

/// Markdown preview pane must be rendered and visible.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn editor_preview_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/editor", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='editor-landing']").await.unwrap();
    ctx.wait_for("[data-testid='markdown-preview']").await.unwrap();
    ctx.assert_element_visible("[data-testid='markdown-preview']").await;
}

/// Typing in the markdown editor must update the preview.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn editor_type_text(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/editor", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='editor-landing']").await.unwrap();
    ctx.wait_for("[data-testid='markdown-editor-playground']").await.unwrap();
    ctx.fill("[data-testid='markdown-editor-playground']", "# Hello E2E\n\nThis is a test.").await.unwrap();
    ctx.wait_for_timeout(500).await;
    let preview = ctx.wait_for("[data-testid='markdown-preview-content']").await.unwrap();
    let html = preview.text_content().await.unwrap();
    assert!(html.contains("Hello"), "Preview should update with typed content");
}
