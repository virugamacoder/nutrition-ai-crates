# 🧠 nutrition-ai (Rust)

**Analyze food images using Google Gemini AI with full nutritional breakdown – written in pure Rust.**

> 🔄 **Node.js developer?** Use the official [nutrition-ai](https://www.npmjs.com/package/nutrition-ai) npm package.

---

## 🍽️ Overview

This crate replicates the functionality of [`nutrition-ai`](https://www.npmjs.com/package/nutrition-ai) in Rust.

- 📸 Upload food images as base64 (`image/png` or `image/jpeg`)
- 🤖 Google Gemini AI generates:
  - Calorie + macro breakdown per food item
  - Total calorie estimation
  - Nutritional balance and dietary tips
- 🛡️ Input validation and error handling
- 🔁 Auto-retries on 503 model overloads

---

## 🚀 Quick Example

```rust
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use nutrition_ai::{GeminiRequest, MimeType, generate_answer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let image_bytes = std::fs::read("img.png")?;
    let file_base64 = STANDARD.encode(&image_bytes);

    let req = GeminiRequest {
        file_base64,
        file_mime_type: MimeType::ImagePng,
        google_key: "<YOUR_GOOGLE_KEY>".to_string(),
        model: Some("Gemini2_0Flash".to_string()),
    };

    let result = generate_answer(req).await?;
    println!("{}", result);
    Ok(())
}
````

---

## 📥 API Parameters

| Field            | Type             | Description                                                                          |
| ---------------- | ---------------- | ------------------------------------------------------------------------------------ |
| `file_base64`    | `String`         | Base64-encoded food image                                                            |
| `file_mime_type` | `MimeType` enum  | `MimeType::ImagePng` or `MimeType::ImageJpeg`                                        |
| `google_key`     | `String`         | Your Google Gemini API key (get it from [makersuite](https://makersuite.google.com)) |
| `model`          | `Option<String>` | Optional Gemini model (e.g. `"Gemini2_0Flash"`)                                      |

---

## 🛠️ Supported Models

* `Gemini1_0Pro`
* `Gemini1_5Pro`
* `Gemini1_5Flash` **(default)**
* `Gemini1_5Flash8B`
* `Gemini2_0Flash`

Default fallback model if none is provided: `Gemini1_5Flash`

---

## 🧪 Error Handling

* ❌ Empty or invalid base64 input
* ❌ Unsupported MIME type
* ❌ Missing Google API key
* 🔁 Retries up to 3 times on Gemini API 503 (model overload)
* ❌ Missing or empty Gemini response content

---

## 🧑‍💻 Author

[`@virugamacoder`](https://github.com/virugamacoder) 

---

## 📄 License

[MIT](./LICENSE) © 2025
