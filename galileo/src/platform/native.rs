//! Types for native applications.

use async_trait::async_trait;
use bytes::Bytes;
use log::info;

use crate::decoded_image::DecodedImage;
use crate::error::GalileoError;
use crate::platform::PlatformService;

pub mod vt_processor;

/// Platform service for native applications.
#[derive(Debug, Clone)]
pub struct NativePlatformService {
    http_client: reqwest::Client,
}

#[async_trait]
impl PlatformService for NativePlatformService {
    fn new() -> Self {
        let http_client = reqwest::Client::builder()
            .user_agent("galileo/0.1")
            .build()
            .expect("Failed to initialize http client");

        Self { http_client }
    }

    async fn load_image_url(&self, url: &str) -> Result<DecodedImage, GalileoError> {
        let image_source = self.load_from_web(url).await?;
        DecodedImage::decode(&image_source)
    }

    async fn load_bytes_from_url(&self, url: &str) -> Result<Bytes, GalileoError> {
        self.load_from_web(url).await
    }

    async fn decode_image(&self, image_data: Bytes) -> Result<DecodedImage, GalileoError> {
        DecodedImage::decode(&image_data)
    }
}

impl NativePlatformService {
    async fn load_from_web(&self, url: &str) -> Result<Bytes, GalileoError> {
        let response = self.http_client.get(url).send().await?;
        if !response.status().is_success() {
            info!(
                "Failed to load {url}: {}, {:?}",
                response.status(),
                response.text().await
            );
            return Err(GalileoError::IO);
        }

        Ok(response.bytes().await?)
    }
}
