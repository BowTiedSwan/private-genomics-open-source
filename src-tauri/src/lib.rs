mod analysis;
mod error;
mod exports;
mod genomics;
mod hermes;
mod local_analysis;
mod markers;
mod medication_checker;
mod morpheus;
mod pdf_export;
mod settings;

use analysis::AnalysisPackage;
use error::AppResult;
use genomics::ParsedGenome;
use medication_checker::MedicationCheckResponse;
use morpheus::{ChatMessage, ModelInfo};
use std::path::PathBuf;
use tauri::AppHandle;

#[tauri::command]
fn list_models() -> Vec<ModelInfo> {
    morpheus::curated_models()
}

#[tauri::command]
fn create_analysis(app: AppHandle, path: String) -> AppResult<AnalysisPackage> {
    let path_buf = PathBuf::from(&path);
    let genome = genomics::parse(&path_buf)?;
    analysis::create(&app, &path_buf, genome)
}

#[tauri::command]
fn load_analysis(app: AppHandle, analysis_id: String) -> AppResult<AnalysisPackage> {
    analysis::load(&app, &analysis_id)
}

#[tauri::command]
fn export_analysis_formats(app: AppHandle, analysis_id: String) -> AppResult<AnalysisPackage> {
    let mut analysis = analysis::load(&app, &analysis_id)?;
    exports::export_all(&mut analysis)?;
    Ok(analysis)
}

#[tauri::command]
fn export_pdf(
    app: AppHandle,
    analysis_id: String,
    output_path: String,
) -> AppResult<AnalysisPackage> {
    let mut analysis = analysis::load(&app, &analysis_id)?;
    pdf_export::export_pdf(&mut analysis, &output_path)?;
    Ok(analysis)
}

#[tauri::command]
fn check_medication_interactions(
    app: AppHandle,
    analysis_id: String,
    medications: Vec<String>,
) -> AppResult<MedicationCheckResponse> {
    let analysis = analysis::load(&app, &analysis_id)?;
    Ok(medication_checker::check_medications(
        &analysis,
        medications,
    ))
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
    analysis_id: String,
    model: String,
) -> AppResult<AnalysisPackage> {
    let api_key = settings::load_api_key()?.ok_or(error::AppError::MissingApiKey)?;
    let analysis = analysis::load(&app, &analysis_id)?;
    let report = hermes::generate_report(&app, &api_key, &model, &analysis, "report-token").await?;
    analysis::save_report(&app, &analysis_id, report, model)
}

#[tauri::command]
async fn explain_marker(
    model: String,
    genome: Option<ParsedGenome>,
    finding: String,
) -> AppResult<String> {
    let api_key = settings::load_api_key()?.ok_or(error::AppError::MissingApiKey)?;
    hermes::explain_marker(&api_key, &model, genome.as_ref(), &finding).await
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
            create_analysis,
            load_analysis,
            export_analysis_formats,
            export_pdf,
            check_medication_interactions,
            save_api_key,
            has_api_key,
            clear_api_key,
            generate_report,
            explain_marker,
            chat,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
