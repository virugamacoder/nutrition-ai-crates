use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GeminiRequest {
    pub file_base64: String,
    pub file_mime_type: MimeType,
    pub google_key: String,
    pub model: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MimeType {
    #[serde(alias = "image/jpeg")]
    ImageJpeg,
    #[serde(alias = "image/png")]
    ImagePng,
}

impl MimeType {
    pub fn as_str(&self) -> &str {
        match self {
            MimeType::ImageJpeg => "image/jpeg",
            MimeType::ImagePng => "image/png",
        }
    }
}
