mod error;
mod genomics;
mod hermes;
mod markers;
mod morpheus;
mod settings;

use error::AppResult;
use genomics::ParsedGenome;
use morpheus::{ChatMessage, ModelInfo};
use std::path::PathBuf;
use tauri::AppHandle;

#[tauri::command]
fn list_models() -> Vec<ModelInfo> {
    morpheus::curated_models()
}

#[tauri::command]
fn parse_genome(path: String) -> AppResult<ParsedGenome> {
    genomics::parse(&PathBuf::from(path))
}

#[tauri::command]
fn save_api_key(key: String) -> AppResult<()> {
    settings::save_api_key(&key)
}

#[tauri::command]
fn has_api_key() -> AppResult<bool> {
    Ok(settings::load_api_key()?.is_some())
}

#[tauri::command]
fn clear_api_key() -> AppResult<()> {
    settings::clear_api_key()
}

#[tauri::command]
async fn generate_report(
    app: AppHandle,
    genome: ParsedGenome,
    model: String,
) -> AppResult<String> {
    let api_key = settings::load_api_key()?.ok_or(error::AppError::MissingApiKey)?;
    hermes::generate_report(&app, &api_key, &model, &genome, "report-token").await
}

#[tauri::command]
async fn chat(
    app: AppHandle,
    model: String,
    genome: Option<ParsedGenome>,
    history: Vec<ChatMessage>,
    message: String,
) -> AppResult<String> {
    let api_key = settings::load_api_key()?.ok_or(error::AppError::MissingApiKey)?;
    hermes::freeform_chat(
        &app,
        &api_key,
        &model,
        genome.as_ref(),
        history,
        message,
        "chat-token",
    )
    .await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            list_models,
            parse_genome,
            save_api_key,
            has_api_key,
            clear_api_key,
            generate_report,
            chat,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
