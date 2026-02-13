use rsc_e2e_test::*;
use crate::common::test_config;

/// Login screen must render all form elements.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn login_form_renders(ctx: BrowserTestContext) {
    ctx.assert_element_exists("[data-testid='login-screen']").await;
    ctx.assert_element_exists("[data-testid='login-title']").await;
    ctx.assert_element_visible("[data-testid='login-title']").await;
    ctx.assert_element_exists("[data-testid='login-username']").await;
    ctx.assert_element_exists("[data-testid='login-password']").await;
    ctx.assert_element_exists("[data-testid='login-submit']").await;
}

/// Filling credentials and clicking login must navigate to app-shell.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn login_with_credentials(ctx: BrowserTestContext) {
    ctx.fill("[data-testid='login-username']", "admin").await.unwrap();
    ctx.fill("[data-testid='login-password']", "admin").await.unwrap();
    ctx.click("[data-testid='login-submit']").await.unwrap();
    ctx.wait_for("[data-testid='app-shell']").await.unwrap();
    ctx.assert_element_exists("[data-testid='app-shell']").await;
}

/// After login, app shell must have sidebar and main content.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn auth_status_after_login(ctx: BrowserTestContext) {
    ctx.fill("[data-testid='login-username']", "admin").await.unwrap();
    ctx.fill("[data-testid='login-password']", "admin").await.unwrap();
    ctx.click("[data-testid='login-submit']").await.unwrap();
    ctx.wait_for("[data-testid='app-shell']").await.unwrap();
    ctx.assert_element_exists("[data-testid='sidebar']").await;
    ctx.assert_element_exists("[data-testid='main-content']").await;
}

/// Clearing auth tokens must return to login screen.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn logout_clears_session(ctx: BrowserTestContext) {
    ctx.set_local_storage("swe_auth_token", "test-token").await.unwrap();
    ctx.set_local_storage("swe_auth_username", "admin").await.unwrap();
    ctx.navigate("/", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.assert_element_exists("[data-testid='app-shell']").await;

    ctx.remove_local_storage("swe_auth_token").await.unwrap();
    ctx.remove_local_storage("swe_auth_username").await.unwrap();
    ctx.navigate("/", Some("[data-testid='login-screen']")).await.unwrap();
    ctx.assert_element_exists("[data-testid='login-screen']").await;
    ctx.assert_element_visible("[data-testid='login-screen']").await;
}
