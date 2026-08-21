use std::collections::HashMap;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use reqwest::{Client, StatusCode, header};
use typst::foundations::Bytes;

use crate::config::ExternalResourceConfig;

const MAX_RESOURCE_BYTES: usize = 5 * 1024 * 1024;
const FETCH_TIMEOUT: Duration = Duration::from_secs(10);

/// Fetches server-approved external image resources once during startup.
///
/// Templates can only access the configured virtual paths; they never control
/// the outbound URL. Redirects are rejected so the configured host remains the
/// destination, and response type and size are checked before caching.
pub async fn load(resources: &[ExternalResourceConfig]) -> Result<HashMap<String, Bytes>> {
    if resources.is_empty() {
        return Ok(HashMap::new());
    }

    let client = Client::builder()
        .timeout(FETCH_TIMEOUT)
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .context("Failed to build external resource client")?;
    let mut cached = HashMap::with_capacity(resources.len());

    for resource in resources {
        if cached.contains_key(&resource.virtual_path) {
            bail!(
                "External resource virtual path '{}' is configured more than once",
                resource.virtual_path
            );
        }
        let mut response = client
            .get(&resource.url)
            .send()
            .await
            .with_context(|| format!("Failed to fetch external resource '{}'", resource.url))?;
        if response.status() != StatusCode::OK {
            bail!(
                "External resource '{}' returned HTTP {}",
                resource.url,
                response.status()
            );
        }
        validate_content_type(
            &resource.virtual_path,
            response.headers().get(header::CONTENT_TYPE),
        )?;
        if response
            .content_length()
            .is_some_and(|size| size > MAX_RESOURCE_BYTES as u64)
        {
            bail!(
                "External resource '{}' exceeds the {} byte limit",
                resource.url,
                MAX_RESOURCE_BYTES
            );
        }
        let mut bytes = Vec::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .with_context(|| format!("Failed to read external resource '{}'", resource.url))?
        {
            if chunk.len() > MAX_RESOURCE_BYTES.saturating_sub(bytes.len()) {
                bail!(
                    "External resource '{}' exceeds the {} byte limit",
                    resource.url,
                    MAX_RESOURCE_BYTES
                );
            }
            bytes.extend_from_slice(&chunk);
        }
        cached.insert(resource.virtual_path.clone(), Bytes::new(bytes));
    }
    Ok(cached)
}

fn validate_content_type(
    virtual_path: &str,
    content_type: Option<&header::HeaderValue>,
) -> Result<()> {
    let content_type = content_type
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(';').next())
        .map(str::trim);
    let expected = match virtual_path.rsplit('.').next().unwrap_or("") {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "svg" => "image/svg+xml",
        "webp" => "image/webp",
        _ => bail!("External resource '{virtual_path}' has an unsupported file extension"),
    };
    if content_type != Some(expected) {
        bail!("External resource '{virtual_path}' must return Content-Type '{expected}'");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_matching_image_content_type() -> Result<()> {
        let value = header::HeaderValue::from_static("image/png; charset=binary");
        validate_content_type("/external/logo.png", Some(&value))
    }

    #[test]
    fn rejects_mismatched_or_missing_content_type() {
        let jpeg = header::HeaderValue::from_static("image/jpeg");
        assert!(validate_content_type("/external/logo.png", Some(&jpeg)).is_err());
        assert!(validate_content_type("/external/logo.png", None).is_err());
    }
}
