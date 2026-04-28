use crate::error::{AppError, AppResult};
use crate::genomics::ParsedGenome;
use crate::local_analysis::{build_analysis_results, AnalysisResults};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Manager};
use uuid::Uuid;

const ANALYSIS_SCHEMA_VERSION: &str = "5.3.0";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisSource {
    pub original_path: String,
    pub file_name: String,
    pub file_size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnalysisArtifacts {
    pub root_dir: String,
    pub package_json_path: String,
    pub report_markdown_path: String,
    #[serde(default)]
    pub exports_dir: String,
    #[serde(default)]
    pub counselor_export_json_path: Option<String>,
    #[serde(default)]
    pub apple_health_export_json_path: Option<String>,
    #[serde(default)]
    pub api_export_json_path: Option<String>,
    #[serde(default)]
    pub api_export_full_json_path: Option<String>,
    #[serde(default)]
    pub integration_hooks_json_path: Option<String>,
    #[serde(default)]
    pub exported_pdf_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisReport {
    pub markdown: String,
    pub model_id: Option<String>,
    pub generated_at_unix_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisPackage {
    pub id: String,
    pub schema_version: String,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
    pub source: AnalysisSource,
    pub genome: ParsedGenome,
    #[serde(default)]
    pub results: AnalysisResults,
    pub report: AnalysisReport,
    pub artifacts: AnalysisArtifacts,
}

pub fn create(
    app: &AppHandle,
    source_path: &Path,
    genome: ParsedGenome,
) -> AppResult<AnalysisPackage> {
    let metadata = fs::metadata(source_path)?;
    let id = Uuid::new_v4().to_string();
    let now = now_unix_ms()?;
    let analysis_dir = analysis_dir(app, &id)?;
    fs::create_dir_all(&analysis_dir)?;
    let results = build_analysis_results(&genome);

    let package_json_path = package_json_path(&analysis_dir);
    let report_markdown_path = report_markdown_path(&analysis_dir);

    let package = AnalysisPackage {
        id,
        schema_version: ANALYSIS_SCHEMA_VERSION.to_string(),
        created_at_unix_ms: now,
        updated_at_unix_ms: now,
        source: AnalysisSource {
            original_path: source_path.display().to_string(),
            file_name: source_path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("genome-data")
                .to_string(),
            file_size_bytes: metadata.len(),
        },
        genome,
        results,
        report: AnalysisReport {
            markdown: String::new(),
            model_id: None,
            generated_at_unix_ms: None,
        },
        artifacts: AnalysisArtifacts {
            root_dir: analysis_dir.display().to_string(),
            package_json_path: package_json_path.display().to_string(),
            report_markdown_path: report_markdown_path.display().to_string(),
            exports_dir: exports_dir_path(&analysis_dir).display().to_string(),
            counselor_export_json_path: None,
            apple_health_export_json_path: None,
            api_export_json_path: None,
            api_export_full_json_path: None,
            integration_hooks_json_path: None,
            exported_pdf_path: None,
        },
    };

    save(&package)?;
    Ok(package)
}

pub fn load(app: &AppHandle, analysis_id: &str) -> AppResult<AnalysisPackage> {
    let analysis_dir = analysis_dir(app, analysis_id)?;
    let package_json_path = package_json_path(&analysis_dir);
    let raw = fs::read_to_string(&package_json_path)?;
    let mut package: AnalysisPackage = serde_json::from_str(&raw)?;
    let mut mutated = false;

    if package.results.provenance.marker_panel_name.is_empty() {
        package.results = build_analysis_results(&package.genome);
        package.schema_version = ANALYSIS_SCHEMA_VERSION.to_string();
        mutated = true;
    }

    if package.artifacts.exports_dir.trim().is_empty() {
        package.artifacts.exports_dir = exports_dir_path(&analysis_dir).display().to_string();
        mutated = true;
    }

    if package.artifacts.exported_pdf_path.is_none() {
        package.artifacts.exported_pdf_path =
            default_exported_pdf_path(&analysis_dir).exists().then(|| {
                default_exported_pdf_path(&analysis_dir)
                    .display()
                    .to_string()
            });
        mutated = true;
    }

    if mutated {
        save(&package)?;
    }

    Ok(package)
}

pub fn save_report(
    app: &AppHandle,
    analysis_id: &str,
    markdown: String,
    model_id: String,
) -> AppResult<AnalysisPackage> {
    let mut package = load(app, analysis_id)?;
    package.report.markdown = markdown;
    package.report.model_id = Some(model_id);
    package.report.generated_at_unix_ms = Some(now_unix_ms()?);
    package.updated_at_unix_ms = now_unix_ms()?;
    save(&package)?;
    Ok(package)
}

pub(crate) fn save(package: &AnalysisPackage) -> AppResult<()> {
    let root_dir = PathBuf::from(&package.artifacts.root_dir);
    fs::create_dir_all(&root_dir)?;

    let package_json_path = PathBuf::from(&package.artifacts.package_json_path);
    let report_markdown_path = PathBuf::from(&package.artifacts.report_markdown_path);

    fs::write(&package_json_path, serde_json::to_vec_pretty(package)?)?;

    if package.report.markdown.trim().is_empty() {
        if report_markdown_path.exists() {
            fs::remove_file(report_markdown_path)?;
        }
    } else {
        fs::write(report_markdown_path, &package.report.markdown)?;
    }

    Ok(())
}

fn analyses_root(app: &AppHandle) -> AppResult<PathBuf> {
    let app_data_dir = app
        .path()
        .app_local_data_dir()
        .map_err(|e| AppError::Other(format!("failed to resolve app local data directory: {e}")))?;
    Ok(app_data_dir.join("analyses"))
}

fn analysis_dir(app: &AppHandle, analysis_id: &str) -> AppResult<PathBuf> {
    Ok(analyses_root(app)?.join(analysis_id))
}

fn package_json_path(root_dir: &Path) -> PathBuf {
    root_dir.join("analysis.json")
}

fn report_markdown_path(root_dir: &Path) -> PathBuf {
    root_dir.join("report.md")
}

fn exports_dir_path(root_dir: &Path) -> PathBuf {
    root_dir.join("exports")
}

pub(crate) fn default_exported_pdf_path(root_dir: &Path) -> PathBuf {
    root_dir.join("report.pdf")
}

fn now_unix_ms() -> AppResult<u64> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| AppError::Other(format!("system clock is before unix epoch: {e}")))?
        .as_millis() as u64)
}
