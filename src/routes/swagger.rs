use axum::{
    http::header,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};

use crate::state::AppState;

const OPENAPI_SPEC: &str = r#"{
  "openapi": "3.0.3",
  "info": {
    "title": "pdfgenrs",
    "description": "Library written in Rust used to create PDFs through an API",
    "version": "0.1.0"
  },
  "paths": {
    "/api/v1/genpdf/{app_name}/{template}": {
      "post": {
        "summary": "Generate PDF from Typst template",
        "description": "Accepts a JSON body and compiles the named Typst template with that data, returning the result as application/pdf.",
        "operationId": "postPdf",
        "parameters": [
          {
            "name": "app_name",
            "in": "path",
            "required": true,
            "schema": { "type": "string" }
          },
          {
            "name": "template",
            "in": "path",
            "required": true,
            "schema": { "type": "string" }
          }
        ],
        "requestBody": {
          "required": true,
          "content": {
            "application/json": {
              "schema": { "type": "object" }
            }
          }
        },
        "responses": {
          "200": {
            "description": "PDF document",
            "content": { "application/pdf": { "schema": { "type": "string", "format": "binary" } } }
          },
          "404": { "description": "Template not found" },
          "500": { "description": "PDF generation failed" }
        }
      },
      "get": {
        "summary": "Generate PDF from Typst template using pre-loaded test data (dev mode only)",
        "description": "Looks up the template source and pre-loaded test JSON data for the given app_name/template combination and returns a PDF response.",
        "operationId": "getPdf",
        "parameters": [
          {
            "name": "app_name",
            "in": "path",
            "required": true,
            "schema": { "type": "string" }
          },
          {
            "name": "template",
            "in": "path",
            "required": true,
            "schema": { "type": "string" }
          }
        ],
        "responses": {
          "200": {
            "description": "PDF document",
            "content": { "application/pdf": { "schema": { "type": "string", "format": "binary" } } }
          },
          "404": { "description": "Template or test data not found" },
          "500": { "description": "PDF generation failed" }
        }
      }
    },
    "/api/v1/genpdf/html/{app_name}": {
      "post": {
        "summary": "Generate PDF from HTML",
        "description": "Accepts an HTML body and converts it to a PDF document.",
        "operationId": "postPdfFromHtml",
        "parameters": [
          {
            "name": "app_name",
            "in": "path",
            "required": true,
            "schema": { "type": "string" }
          }
        ],
        "requestBody": {
          "required": true,
          "content": {
            "text/html": {
              "schema": { "type": "string" }
            }
          }
        },
        "responses": {
          "200": {
            "description": "PDF document",
            "content": { "application/pdf": { "schema": { "type": "string", "format": "binary" } } }
          },
          "500": { "description": "PDF generation failed" }
        }
      }
    },
    "/api/v1/genpdf/image/{app_name}": {
      "post": {
        "summary": "Generate PDF from image",
        "description": "Accepts a PNG or JPEG image body and converts it to a PDF document.",
        "operationId": "postPdfFromImage",
        "parameters": [
          {
            "name": "app_name",
            "in": "path",
            "required": true,
            "schema": { "type": "string" }
          }
        ],
        "requestBody": {
          "required": true,
          "content": {
            "image/png": {
              "schema": { "type": "string", "format": "binary" }
            },
            "image/jpeg": {
              "schema": { "type": "string", "format": "binary" }
            }
          }
        },
        "responses": {
          "200": {
            "description": "PDF document",
            "content": { "application/pdf": { "schema": { "type": "string", "format": "binary" } } }
          },
          "415": { "description": "Unsupported media type" },
          "500": { "description": "PDF generation failed" }
        }
      }
    },
    "/api/v1/genhtml/{app_name}/{template}": {
      "post": {
        "summary": "Generate HTML from Typst template",
        "description": "Accepts a JSON body and compiles the named Typst template with that data, returning the result as text/html.",
        "operationId": "postHtml",
        "parameters": [
          {
            "name": "app_name",
            "in": "path",
            "required": true,
            "schema": { "type": "string" }
          },
          {
            "name": "template",
            "in": "path",
            "required": true,
            "schema": { "type": "string" }
          }
        ],
        "requestBody": {
          "required": true,
          "content": {
            "application/json": {
              "schema": { "type": "object" }
            }
          }
        },
        "responses": {
          "200": {
            "description": "HTML document",
            "content": { "text/html": { "schema": { "type": "string" } } }
          },
          "404": { "description": "Template not found" },
          "500": { "description": "HTML generation failed" }
        }
      },
      "get": {
        "summary": "Generate HTML from Typst template using pre-loaded test data (dev mode only)",
        "description": "Looks up the template source and pre-loaded test JSON data for the given app_name/template combination and returns an HTML response.",
        "operationId": "getHtml",
        "parameters": [
          {
            "name": "app_name",
            "in": "path",
            "required": true,
            "schema": { "type": "string" }
          },
          {
            "name": "template",
            "in": "path",
            "required": true,
            "schema": { "type": "string" }
          }
        ],
        "responses": {
          "200": {
            "description": "HTML document",
            "content": { "text/html": { "schema": { "type": "string" } } }
          },
          "404": { "description": "Template or test data not found" },
          "500": { "description": "HTML generation failed" }
        }
      }
    },
    "/internal/is_alive": {
      "get": {
        "summary": "Liveness probe",
        "description": "Returns 200 OK when the application is alive.",
        "operationId": "isAlive",
        "responses": {
          "200": { "description": "Application is alive" },
          "500": { "description": "Application is not alive" }
        }
      }
    },
    "/internal/is_ready": {
      "get": {
        "summary": "Readiness probe",
        "description": "Returns 200 OK when the application is ready to serve traffic.",
        "operationId": "isReady",
        "responses": {
          "200": { "description": "Application is ready" },
          "500": { "description": "Application is not ready" }
        }
      }
    }
  }
}"#;

const SWAGGER_UI_HTML: &str = r#"<!DOCTYPE html>
<html>
  <head>
    <title>pdfgenrs API</title>
    <meta charset="utf-8"/>
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="stylesheet" type="text/css" href="https://unpkg.com/swagger-ui-dist/swagger-ui.css">
  </head>
  <body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist/swagger-ui-bundle.js"></script>
    <script src="https://unpkg.com/swagger-ui-dist/swagger-ui-standalone-preset.js"></script>
    <script>
      window.onload = function() {
        SwaggerUIBundle({
          url: "/swagger-ui/openapi.json",
          dom_id: '#swagger-ui',
          presets: [
            SwaggerUIBundle.presets.apis,
            SwaggerUIStandalonePreset
          ],
          layout: "StandaloneLayout"
        })
      }
    </script>
  </body>
</html>"#;

/// Builds the Swagger UI router with `/swagger-ui` and `/swagger-ui/openapi.json` endpoints.
///
/// This router should only be mounted when dev mode is enabled.
pub fn swagger_router() -> Router<AppState> {
    Router::new()
        .route("/swagger-ui", get(swagger_ui))
        .route("/swagger-ui/openapi.json", get(openapi_spec))
}

async fn swagger_ui() -> Html<&'static str> {
    Html(SWAGGER_UI_HTML)
}

async fn openapi_spec() -> Response {
    ([(header::CONTENT_TYPE, "application/json")], OPENAPI_SPEC).into_response()
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::TestServer;

    use super::swagger_router;
    use crate::state::AppState;
    use crate::{config, state, typst_world};
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    fn test_state() -> anyhow::Result<AppState> {
        let cfg = config::Config {
            port: 8080,
            root_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            templates_dir: PathBuf::from("templates"),
            resources_dir: PathBuf::from("resources"),
            data_dir: PathBuf::from("data"),
            fonts_dir: PathBuf::from("fonts"),
            dev_mode: true,
        };
        Ok(AppState {
            templates: Arc::new(HashMap::new()),
            data: Arc::new(RwLock::new(HashMap::new())),
            aliveness: state::AppAliveness::new(),
            fonts: Arc::new(typst_world::load_fonts(
                &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fonts"),
            )?),
            config: cfg,
        })
    }

    #[tokio::test]
    async fn swagger_ui_returns_html() -> anyhow::Result<()> {
        let server = TestServer::new(swagger_router().with_state(test_state()?));
        let response = server.get("/swagger-ui").await;
        assert_eq!(response.status_code(), StatusCode::OK);
        assert!(response
            .headers()
            .get("content-type")
            .ok_or_else(|| anyhow::anyhow!("missing content-type header"))?
            .to_str()?
            .starts_with("text/html"));
        assert!(response.text().contains("swagger-ui"));
        Ok(())
    }

    #[tokio::test]
    async fn openapi_spec_returns_json() -> anyhow::Result<()> {
        let server = TestServer::new(swagger_router().with_state(test_state()?));
        let response = server.get("/swagger-ui/openapi.json").await;
        assert_eq!(response.status_code(), StatusCode::OK);
        assert_eq!(
            response
                .headers()
                .get("content-type")
                .ok_or_else(|| anyhow::anyhow!("missing content-type header"))?,
            "application/json"
        );
        let spec: serde_json::Value = serde_json::from_str(response.text().as_str())?;
        assert_eq!(spec["openapi"], "3.0.3");
        Ok(())
    }
}
