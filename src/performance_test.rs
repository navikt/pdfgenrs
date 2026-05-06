#[cfg(test)]
mod tests {
    use std::future::IntoFuture;
    use std::sync::Arc;

    use axum_test::TestServer;
    use tokio::task::JoinSet;

    use crate::{build_router, config, state, template, typst_world, AppState};
    use tokio::sync::RwLock;

    fn create_test_state() -> AppState {
        let cfg = config::Config::default();
        let templates = Arc::new(
            template::load_templates_from_dir(&cfg.templates_dir).unwrap_or_default(),
        );
        let data = template::load_test_data(&cfg.data_dir);
        let fonts = Arc::new(typst_world::load_fonts());
        AppState {
            templates,
            data: Arc::new(RwLock::new(data)),
            aliveness: state::AppAliveness::new(),
            fonts,
            config: cfg,
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn performance_test_multi_thread() {
        let app_state = create_test_state();

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .unwrap();
        let port = listener.local_addr().unwrap().port();
        let base_url = format!("http://127.0.0.1:{port}");

        let server_handle =
            tokio::spawn(axum::serve(listener, build_router(app_state.clone())).into_future());

        let client = Arc::new(reqwest::Client::new());
        let passes = 20;

        for template_path in app_state.templates.keys() {
            let parts: Vec<&str> = template_path.splitn(2, '/').collect();
            if parts.len() != 2 {
                continue;
            }
            let app_name = parts[0].to_string();
            let template_name = parts[1].to_string();

            let json_data = {
                let data = app_state.data.read().await;
                data.get(&(app_name.clone(), template_name.clone()))
                    .cloned()
                    .unwrap_or_else(|| serde_json::json!({}))
            };

            let start = std::time::Instant::now();

            let mut join_set = JoinSet::new();
            for _ in 0..passes {
                let client = Arc::clone(&client);
                let url = format!("{base_url}/api/v1/genpdf/{app_name}/{template_name}");
                let data = json_data.clone();
                join_set.spawn(async move {
                    let response = client.post(&url).json(&data).send().await.unwrap();
                    assert!(response.status().is_success());
                    let bytes = response.bytes().await.unwrap();
                    assert!(!bytes.is_empty());
                });
            }

            while let Some(result) = join_set.join_next().await {
                result.unwrap();
            }

            println!(
                "Multi-thread performance testing {template_name} for {app_name} took {}ms",
                start.elapsed().as_millis()
            );
        }

        server_handle.abort();
    }

    #[tokio::test]
    async fn performance_test_single_thread() {
        let app_state = create_test_state();
        let server = TestServer::new(build_router(app_state.clone()));

        let passes = 30;

        for template_path in app_state.templates.keys() {
            let parts: Vec<&str> = template_path.splitn(2, '/').collect();
            if parts.len() != 2 {
                continue;
            }
            let app_name = parts[0].to_string();
            let template_name = parts[1].to_string();

            let json_data = {
                let data = app_state.data.read().await;
                data.get(&(app_name.clone(), template_name.clone()))
                    .cloned()
                    .unwrap_or_else(|| serde_json::json!({}))
            };

            let start = std::time::Instant::now();

            for _ in 0..passes {
                let url = format!("/api/v1/genpdf/{app_name}/{template_name}");
                let response = server.post(&url).json(&json_data).await;
                response.assert_status_success();
                assert!(!response.as_bytes().is_empty());
            }

            println!(
                "Single-thread performance testing {template_name} for {app_name} took {}ms",
                start.elapsed().as_millis()
            );
        }
    }
}
