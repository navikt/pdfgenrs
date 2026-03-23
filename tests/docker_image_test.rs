use std::process::Command;
use testcontainers::core::wait::HttpWaitStrategy;
use testcontainers::core::{IntoContainerPort, Mount, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::{GenericImage, ImageExt};

const IMAGE_NAME: &str = "pdfgenrs-test";
const IMAGE_TAG: &str = "latest";
const PORT: u16 = 8080;

fn build_docker_image() {
    let output = Command::new("docker")
        .args([
            "build",
            "-t",
            &format!("{}:{}", IMAGE_NAME, IMAGE_TAG),
            ".",
        ])
        .output()
        .expect("Failed to execute docker build");

    assert!(
        output.status.success(),
        "Docker build failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[tokio::test]
async fn test_dockerfile() {
    build_docker_image();

    let project_dir = std::env::current_dir()
        .expect("Failed to get current directory")
        .to_string_lossy()
        .to_string();

    let templates_dir = format!("{}/templates", project_dir);
    let fonts_dir = format!("{}/fonts", project_dir);
    let resources_dir = format!("{}/resources", project_dir);

    let container = GenericImage::new(IMAGE_NAME, IMAGE_TAG)
        .with_exposed_port(PORT.tcp())
        .with_wait_for(WaitFor::http(
            HttpWaitStrategy::new("/internal/is_ready")
                .with_expected_status_code(200u16),
        ))
        .with_mount(Mount::bind_mount(&templates_dir, "/app/templates"))
        .with_mount(Mount::bind_mount(&fonts_dir, "/app/fonts"))
        .with_mount(Mount::bind_mount(&resources_dir, "/app/resources"))
        .start()
        .await
        .expect("Failed to start container");

    let host_port = container
        .get_host_port_ipv4(PORT)
        .await
        .expect("Failed to get host port");

    let url = format!("http://localhost:{}/internal/is_ready", host_port);
    let response = reqwest::get(&url)
        .await
        .expect("Failed to send request to /internal/is_ready");

    assert_eq!(response.status(), reqwest::StatusCode::OK);
}
