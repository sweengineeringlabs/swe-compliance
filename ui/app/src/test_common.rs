use rsc_e2e_test::{BrowserTestConfig, BrowserTestContext, WebServerConfig, Viewport};

/// Sets localStorage tokens to bypass login screen for authenticated tests.
pub async fn setup_auth(ctx: &BrowserTestContext) {
    ctx.set_local_storage("swe_auth_token", "test-token-e2e").await.unwrap();
    ctx.set_local_storage("swe_auth_username", "admin").await.unwrap();
}

pub fn test_config() -> BrowserTestConfig {
    let port: u16 = std::env::var("SWE_TEST_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8081);

    let base_url = std::env::var("RSC_TEST_BASE_URL")
        .unwrap_or_else(|_| format!("http://localhost:{}", port));

    let headless = std::env::var("RSC_TEST_HEADLESS")
        .map(|v| v != "false" && v != "0")
        .unwrap_or(true);

    let timeout_secs: u64 = std::env::var("RSC_TEST_TIMEOUT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(60);

    BrowserTestConfig::new()
        .headless(headless)
        .timeout(std::time::Duration::from_secs(timeout_secs))
        .viewport(Viewport {
            width: 1280,
            height: 720,
            device_scale_factor: 1.0,
            is_mobile: false,
            has_touch: false,
        }.into())
        .base_url(base_url)
        .web_server(
            WebServerConfig::new("cargo run -p swe-compliance-server")
                .port(port)
                .env("SWE_PORT", port.to_string())
                .env("SWE_JWT_SECRET", "test-secret")
                .env("SWE_DB_PATH", ":memory:")
                .env("SWE_DIST_DIR", "ui/app/dist")
        )
}
