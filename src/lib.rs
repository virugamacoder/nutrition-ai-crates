mod types;
use anyhow::{Result, anyhow};
use google_generative_ai_rs::v1::api::{Client, PostResult};
use google_generative_ai_rs::v1::gemini::request::{InlineData, Request};
use google_generative_ai_rs::v1::gemini::{Content, Model, Part, Role};
use tokio::time::{Duration, sleep};
pub use types::{GeminiRequest, MimeType};

/// Analyze a food image using Gemini API and return nutrition markdown string
pub async fn generate_answer(req: GeminiRequest) -> Result<String, anyhow::Error> {
    // Validate base64 input (rough check)
    if req.file_base64.trim().is_empty() {
        return Err(anyhow!("File base64 string is empty"));
    }

    // Validate MIME type
    let mime = req.file_mime_type.as_str();
    if mime != "image/png" && mime != "image/jpeg" {
        return Err(anyhow!(
            "Unsupported MIME type: '{}'. Must be 'image/png' or 'image/jpeg'",
            mime
        ));
    }

    let prompt = include_str!("prompts/prompt.txt").to_string();

    // Determine model
    let model = match req.model.as_deref() {
        Some("Gemini1_0Pro") => Model::Gemini1_0Pro,
        Some("Gemini1_5Pro") => Model::Gemini1_5Pro,
        Some("Gemini1_5Flash") => Model::Gemini1_5Flash,
        Some("Gemini1_5Flash8B") => Model::Gemini1_5Flash8B,
        Some("Gemini2_0Flash") => Model::Gemini2_0Flash,
        Some(custom) => Model::Custom(custom.to_string()),
        None => Model::Gemini1_5Flash,
    };

    // Check Google Key
    if req.google_key.trim().is_empty() {
        return Err(anyhow!("Google API key is empty"));
    }

    // Initialize client
    let client = Client::new_from_model(model, req.google_key.clone());

    let parts = vec![
        Part {
            text: Some(prompt),
            inline_data: None,
            file_data: None,
            video_metadata: None,
        },
        Part {
            text: None,
            inline_data: Some(InlineData {
                mime_type: mime.to_string(),
                data: req.file_base64.clone(),
            }),
            file_data: None,
            video_metadata: None,
        },
    ];

    let contents = vec![Content {
        role: Role::User,
        parts,
    }];

    let request = Request {
        contents,
        tools: vec![],
        safety_settings: vec![],
        generation_config: None,
        system_instruction: None,
    };

    let mut retries = 0;
    let result: PostResult;

    loop {
        match client.post(30, &request).await {
            Ok(res) => {
                result = res;
                break;
            }
            Err(e) => {
                let err_msg = e.to_string();
                if err_msg.contains("503") || err_msg.contains("overloaded") {
                    retries += 1;
                    if retries >= 3 {
                        return Err(anyhow!(
                            "Gemini model is overloaded after 3 attempts. Please try again later."
                        ));
                    } else {
                        let delay = Duration::from_secs(2 * retries); // 2s, 4s, 6s
                        eprintln!("Model overloaded. Retrying in {:?}...", delay);
                        sleep(delay).await;
                        continue;
                    }
                } else {
                    return Err(anyhow!("Gemini API error: {}", err_msg));
                }
            }
        }
    }

    if let Some(rest) = result.rest() {
        if let Some(candidate) = rest.candidates.get(0) {
            if let Some(part) = candidate.content.parts.get(0) {
                if let Some(text) = &part.text {
                    return Ok(text.clone());
                } else {
                    return Err(anyhow!("No text found in candidate part"));
                }
            } else {
                return Err(anyhow!("No parts in candidate"));
            }
        } else {
            return Err(anyhow!("No candidates returned by Gemini API"));
        }
    } else {
        return Err(anyhow!("Unexpected response type from Gemini API"));
    }
}
