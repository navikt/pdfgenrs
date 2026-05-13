#[cfg(test)]
mod tests {
    use std::future::IntoFuture;
    use std::sync::Arc;

    use axum_test::TestServer;
    use tokio::task::JoinSet;
    use tracing::info;

    use crate::state::AppState;
    use crate::{build_router, config, state, template, typst_world};
    use tokio::sync::RwLock;

    fn create_test_state() -> anyhow::Result<AppState> {
        let cfg = config::Config::default();
        let templates =
            Arc::new(template::load_templates_from_dir(&cfg.templates_dir).unwrap_or_default());
        let data = template::load_test_data(&cfg.data_dir).data;
        let fonts = Arc::new(typst_world::load_fonts(&cfg.fonts_dir)?);
        Ok(AppState {
            templates,
            data: Arc::new(RwLock::new(data)),
            aliveness: state::AppAliveness::new(),
            fonts,
            config: cfg,
        })
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn performance_test_multi_thread() -> anyhow::Result<()> {
        let _guard = crate::memory_sensitive_test_lock().lock().await;
        let app_state = create_test_state()?;

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        let port = listener.local_addr()?.port();
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
                    let response = client.post(&url).json(&data).send().await?;
                    assert!(response.status().is_success());
                    let bytes = response.bytes().await?;
                    assert!(!bytes.is_empty());
                    anyhow::Ok(())
                });
            }

            while let Some(task_result) = join_set.join_next().await {
                // First `?` propagates JoinError (task panicked); second `?` propagates the inner anyhow::Result.
                task_result??;
            }

            info!(
                template = %template_name,
                app = %app_name,
                duration_ms = start.elapsed().as_millis(),
                "Multi-thread performance test completed"
            );
        }

        server_handle.abort();
        Ok(())
    }

    #[tokio::test]
    async fn performance_test_single_thread() -> anyhow::Result<()> {
        let _guard = crate::memory_sensitive_test_lock().lock().await;
        let app_state = create_test_state()?;
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

            info!(
                template = %template_name,
                app = %app_name,
                duration_ms = start.elapsed().as_millis(),
                "Single-thread performance test completed"
            );
        }

        Ok(())
    }
}
