use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use nutrition_ai::{GeminiRequest, MimeType, generate_answer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let image_bytes = std::fs::read("img.png")?;

    // Then convert to Base64:
    let file_base64 = STANDARD.encode(&image_bytes);

    let req = GeminiRequest {
        file_base64,
        file_mime_type: MimeType::ImagePng,
        google_key: "GOOGLE_API_KEY".to_string(),
        model: None,
    };

    let result = generate_answer(req).await?;
    println!("{}", result);
    Ok(())
}