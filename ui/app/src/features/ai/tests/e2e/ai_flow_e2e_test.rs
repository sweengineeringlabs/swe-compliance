use rsc_e2e_test::*;
use crate::test_common::{test_config, setup_auth};

/// AI landing must render its container with tabs.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn ai_landing_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/ai", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='ai-landing']").await.unwrap();
    ctx.assert_element_exists("[data-testid='ai-landing']").await;
    ctx.assert_element_visible("[data-testid='ai-landing']").await;
}

/// The chat panel must be rendered and visible.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn ai_chat_panel_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/ai", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='ai-landing']").await.unwrap();
    ctx.wait_for("[data-testid='chat-panel']").await.unwrap();
    ctx.assert_element_visible("[data-testid='chat-panel']").await;
}

/// Typing a message in the chat input must enable the send button.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn ai_message_input_works(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/ai", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='ai-landing']").await.unwrap();
    ctx.wait_for("[data-testid='chat-input']").await.unwrap();
    ctx.fill("[data-testid='chat-input']", "Hello, AI assistant").await.unwrap();
    let send_btn = ctx.wait_for("[data-testid='chat-send-btn']").await.unwrap();
    let enabled = send_btn.is_enabled().await.unwrap();
    assert!(enabled, "Send button should be enabled after typing a message");
}

/// The command generation panel must be rendered.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn command_gen_panel_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/ai", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='ai-landing']").await.unwrap();
    ctx.wait_for("[data-testid='command-gen']").await.unwrap();
    ctx.assert_element_visible("[data-testid='command-gen']").await;
}

/// Command gen input must accept text and reflect it.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn command_gen_input_works(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/ai", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='ai-landing']").await.unwrap();
    let input = ctx.wait_for("[data-testid='command-gen-context-input']").await.unwrap();
    ctx.fill("[data-testid='command-gen-context-input']", "scan all projects").await.unwrap();
    let value = input.get_property("value").await.unwrap();
    assert!(
        value.as_str().map_or(false, |v| v.contains("scan")),
        "Command gen input should contain typed text"
    );
}
