use rsc_e2e_test::*;
use crate::common::{test_config, setup_auth};

// ---------------------------------------------------------------------------
// App shell structure (requires auth)
// ---------------------------------------------------------------------------

#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn app_shell_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.assert_element_exists("[data-testid='app-shell']").await;
    ctx.assert_element_exists("[data-testid='sidebar']").await;
    ctx.assert_element_exists("[data-testid='main-content']").await;
}

#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn nav_links_present(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/", Some("[data-testid='sidebar']")).await.unwrap();
    ctx.assert_element_exists("[data-testid='nav-dashboard']").await;
    ctx.assert_element_exists("[data-testid='nav-projects']").await;
    ctx.assert_element_exists("[data-testid='nav-scans']").await;
    ctx.assert_element_exists("[data-testid='nav-violations']").await;
    ctx.assert_element_exists("[data-testid='nav-scaffold']").await;
    ctx.assert_element_exists("[data-testid='nav-templates']").await;
    ctx.assert_element_exists("[data-testid='nav-reports']").await;
    ctx.assert_element_exists("[data-testid='nav-ai']").await;
    ctx.assert_element_exists("[data-testid='nav-editor']").await;
    ctx.assert_element_exists("[data-testid='nav-specs']").await;
    ctx.assert_element_exists("[data-testid='nav-struct-engine']").await;
}

/// Default route must render the dashboard landing with its specific content.
#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn default_route_shows_dashboard(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='dashboard-landing']").await.unwrap();
    ctx.assert_element_exists("[data-testid='dashboard-landing']").await;
}

// ---------------------------------------------------------------------------
// SPA route navigation — each route must render its feature-specific landing
// ---------------------------------------------------------------------------

#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn route_projects_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/projects", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='projects-landing']").await.unwrap();
    ctx.assert_element_exists("[data-testid='projects-landing']").await;
}

#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn route_scans_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/scans", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='scans-landing']").await.unwrap();
    ctx.assert_element_exists("[data-testid='scans-landing']").await;
}

#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn route_violations_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/violations", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='violations-landing']").await.unwrap();
    ctx.assert_element_exists("[data-testid='violations-landing']").await;
}

#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn route_scaffold_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/scaffold", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='scaffold-landing']").await.unwrap();
    ctx.assert_element_exists("[data-testid='scaffold-landing']").await;
}

#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn route_templates_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/templates", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='templates-landing']").await.unwrap();
    ctx.assert_element_exists("[data-testid='templates-landing']").await;
}

#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn route_reports_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/reports", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='reports-landing']").await.unwrap();
    ctx.assert_element_exists("[data-testid='reports-landing']").await;
}

#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn route_ai_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/ai", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='ai-landing']").await.unwrap();
    ctx.assert_element_exists("[data-testid='ai-landing']").await;
}

#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn route_editor_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/editor", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='editor-landing']").await.unwrap();
    ctx.assert_element_exists("[data-testid='editor-landing']").await;
}

#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn route_specs_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/specs", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='specs-landing']").await.unwrap();
    ctx.assert_element_exists("[data-testid='specs-landing']").await;
}

#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn route_struct_engine_renders(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/struct-engine", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='struct-engine-landing']").await.unwrap();
    ctx.assert_element_exists("[data-testid='struct-engine-landing']").await;
}

// ---------------------------------------------------------------------------
// Shell persistence and branding
// ---------------------------------------------------------------------------

#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn app_logo_visible(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/", Some("[data-testid='app-logo']")).await.unwrap();
    ctx.assert_element_exists("[data-testid='app-logo']").await;
    ctx.assert_element_visible("[data-testid='app-logo']").await;
}

#[e2e(config = "test_config", route = "/", wait_for = "[data-testid='login-screen']")]
async fn unknown_route_shows_dashboard_fallback(ctx: BrowserTestContext) {
    setup_auth(&ctx).await;
    ctx.navigate("/nonexistent", Some("[data-testid='app-shell']")).await.unwrap();
    ctx.wait_for("[data-testid='dashboard-landing']").await.unwrap();
    ctx.assert_element_exists("[data-testid='sidebar']").await;
    ctx.assert_element_exists("[data-testid='dashboard-landing']").await;
}
