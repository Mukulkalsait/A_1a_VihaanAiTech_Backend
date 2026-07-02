use std::path::Path;
use crate::errors::ApiError;

pub async fn save_screenshot(
    bytes: axum::body::Bytes,
    _original_filename: &str,
    content_type: &str,
    upload_dir: &Path,
) -> Result<String, ApiError> {
    // Create directory if it doesn't exist
    if !upload_dir.exists() {
        tokio::fs::create_dir_all(upload_dir)
            .await
            .map_err(|e| ApiError::InternalWithMessage(format!("Failed to create upload dir: {}", e)))?;
    }

    // Get extension from content type
    let extension = match content_type {
        "image/jpeg" => "jpg",
        "image/jpg" => "jpg",
        "image/png" => "png",
        _ => "jpg",
    };

    // Generate unique filename
    let timestamp = chrono::Utc::now().timestamp();
    let filename = format!("screenshot_{}.{}", timestamp, extension);
    let full_path = upload_dir.join(&filename);

    // Save file
    tokio::fs::write(&full_path, bytes)
        .await
        .map_err(|e| ApiError::InternalWithMessage(format!("Failed to save file: {}", e)))?;

    Ok(filename)
}

pub async fn send_confirmation_email(
    name: &str,
    email: &str,
    registration_id: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Sending confirmation email to: {} at {}, Registration ID: {}",
        name, email, registration_id
    );
    Ok(())
}
