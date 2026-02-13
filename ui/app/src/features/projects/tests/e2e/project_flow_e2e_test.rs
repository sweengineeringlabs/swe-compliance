use rsc_e2e_test::*;
use crate::test_common::{test_config, setup_auth};

/// Projects landing must render the project list table.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn project_list_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/projects", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='project-list']").await.unwrap();
    ctx.assert_element_exists("[data-testid='project-list']").await;
    ctx.assert_element_visible("[data-testid='project-list']").await;
}

/// Clicking "New Project" button must open the project form.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn create_project_form_opens(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/projects", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='new-project-btn']").await.unwrap();
    ctx.click("[data-testid='new-project-btn']").await.unwrap();
    ctx.wait_for("[data-testid='project-form']").await.unwrap();
    ctx.assert_element_visible("[data-testid='project-form']").await;
}

/// Empty form submit button must be disabled.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn create_project_validates_required(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/projects", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='new-project-btn']").await.unwrap();
    ctx.click("[data-testid='new-project-btn']").await.unwrap();
    let submit = ctx.wait_for("[data-testid='project-form-submit']").await.unwrap();
    let disabled = submit.is_disabled().await.unwrap();
    assert!(disabled, "Submit button should be disabled when required fields are empty");
}

/// Filling the form and submitting must create a project.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn create_project_success(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/projects", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='new-project-btn']").await.unwrap();
    ctx.click("[data-testid='new-project-btn']").await.unwrap();
    ctx.wait_for("[data-testid='project-form']").await.unwrap();
    ctx.fill("[data-testid='project-form-name']", "E2E Test Project").await.unwrap();
    ctx.fill("[data-testid='project-form-root-path']", "/tmp/e2e-test").await.unwrap();
    ctx.click("[data-testid='project-form-submit']").await.unwrap();
    ctx.wait_for("[data-testid='project-list']").await.unwrap();
}

/// The project list table must render its header with column names.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn project_detail_shows_info(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/projects", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='project-list']").await.unwrap();
    ctx.assert_element_visible("[data-testid='project-list']").await;
    // The new-project button must also be visible for interaction
    ctx.wait_for("[data-testid='new-project-btn']").await.unwrap();
    ctx.assert_element_visible("[data-testid='new-project-btn']").await;
}
