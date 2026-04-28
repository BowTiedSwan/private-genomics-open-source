use crate::analysis::{save, AnalysisPackage};
use crate::error::AppResult;
use crate::local_analysis::StructuredFinding;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const EXPORTS_SCHEMA_VERSION: &str = "1.0.0";

pub fn export_all(package: &mut AnalysisPackage) -> AppResult<()> {
    let exports_dir = PathBuf::from(&package.artifacts.exports_dir);
    fs::create_dir_all(&exports_dir)?;

    let counselor_path = exports_dir.join("clinical_export.json");
    let apple_health_path = exports_dir.join("apple_health_export.json");
    let api_path = exports_dir.join("api_export.json");
    let api_full_path = exports_dir.join("api_export_full.json");
    let integration_hooks_path = exports_dir.join("integration_hooks.json");

    write_json(&counselor_path, &build_counselor_export(package))?;
    write_json(&apple_health_path, &build_apple_health_export(package))?;
    write_json(&api_path, &build_api_export(package, false))?;
    write_json(&api_full_path, &build_api_export(package, true))?;
    write_json(
        &integration_hooks_path,
        &build_integration_hooks_export(package),
    )?;

    package.artifacts.counselor_export_json_path = Some(counselor_path.display().to_string());
    package.artifacts.apple_health_export_json_path = Some(apple_health_path.display().to_string());
    package.artifacts.api_export_json_path = Some(api_path.display().to_string());
    package.artifacts.api_export_full_json_path = Some(api_full_path.display().to_string());
    package.artifacts.integration_hooks_json_path =
        Some(integration_hooks_path.display().to_string());
    package.updated_at_unix_ms = now_unix_ms()?;

    save(package)
}

fn write_json<T: Serialize>(path: &PathBuf, value: &T) -> AppResult<()> {
    fs::write(path, serde_json::to_vec_pretty(value)?)?;
    Ok(())
}

#[derive(Serialize)]
struct CounselorExport {
    report_type: &'static str,
    version: &'static str,
    generated_at_unix_ms: u64,
    analysis_id: String,
    classification_framework: &'static str,
    data_source: CounselorDataSource,
    summary: CounselorSummary,
    findings: CounselorFindingGroups,
    recommendations: CounselorRecommendations,
    limitations: Vec<String>,
}

#[derive(Serialize)]
struct CounselorDataSource {
    file_name: String,
    platform: String,
    snps_analyzed: usize,
    call_rate_percent: f64,
    marker_panel_name: String,
    marker_panel_version: String,
}

#[derive(Serialize)]
struct CounselorSummary {
    actionable_finding_count: usize,
    high_confidence_finding_count: usize,
    pharmacogenomics_count: usize,
    cardiometabolic_count: usize,
    trait_count: usize,
}

#[derive(Serialize)]
struct CounselorFindingGroups {
    pharmacogenomics: Vec<CounselorFinding>,
    elevated_risk: Vec<CounselorFinding>,
    baseline_or_informational: Vec<CounselorFinding>,
}

#[derive(Serialize)]
struct CounselorFinding {
    rsid: String,
    genotype: String,
    trait_name: String,
    family: String,
    confidence: String,
    significance: String,
    actionability: String,
    classification: String,
    interpretation: String,
}

#[derive(Serialize)]
struct CounselorRecommendations {
    priority_actions: Vec<String>,
    clinician_discussion_topics: Vec<String>,
    caveats: Vec<String>,
}

fn build_counselor_export(package: &AnalysisPackage) -> CounselorExport {
    let all_findings = all_findings(package);
    let mut elevated_risk = Vec::new();
    let mut baseline_or_informational = Vec::new();
    let mut pharmacogenomics = Vec::new();

    for finding in all_findings {
        let out = CounselorFinding {
            rsid: finding.rsid.clone(),
            genotype: finding.genotype.clone(),
            trait_name: finding.trait_name.clone(),
            family: finding.family.clone(),
            confidence: finding.confidence.clone(),
            significance: finding.significance.clone(),
            actionability: finding.actionability.clone(),
            classification: counselor_classification(&finding).to_string(),
            interpretation: finding.interpretation.clone(),
        };

        if finding.family == "pharmacogenomics" {
            pharmacogenomics.push(out);
        } else if finding.significance == "elevated_risk" || finding.actionability == "high" {
            elevated_risk.push(out);
        } else {
            baseline_or_informational.push(out);
        }
    }

    CounselorExport {
        report_type: "genetic_counselor_clinical_export",
        version: EXPORTS_SCHEMA_VERSION,
        generated_at_unix_ms: now_unix_ms().unwrap_or_default(),
        analysis_id: package.id.clone(),
        classification_framework: "acmg_style_actionability_for_consumer_genomics",
        data_source: CounselorDataSource {
            file_name: package.source.file_name.clone(),
            platform: package.results.quality.format_label.clone(),
            snps_analyzed: package.results.quality.total_snps,
            call_rate_percent: package.results.quality.call_rate_percent,
            marker_panel_name: package.results.provenance.marker_panel_name.clone(),
            marker_panel_version: package.results.provenance.marker_panel_version.clone(),
        },
        summary: CounselorSummary {
            actionable_finding_count: package.results.summary.actionable_finding_count,
            high_confidence_finding_count: package.results.summary.high_confidence_finding_count,
            pharmacogenomics_count: package.results.summary.family_counts.pharmacogenomics,
            cardiometabolic_count: package.results.summary.family_counts.metabolic_cardiovascular,
            trait_count: package.results.summary.family_counts.traits,
        },
        findings: CounselorFindingGroups {
            pharmacogenomics,
            elevated_risk,
            baseline_or_informational,
        },
        recommendations: CounselorRecommendations {
            priority_actions: package.results.recommendations.priority_actions.clone(),
            clinician_discussion_topics: package
                .results
                .recommendations
                .clinician_discussion_topics
                .clone(),
            caveats: package.results.quality.caveats.clone(),
        },
        limitations: vec![
            "This export is derived from a consumer-genomics marker panel and does not assert pathogenicity in the clinical ACMG sense.".to_string(),
            "Clinically meaningful decisions require confirmation with a clinician and, when appropriate, confirmatory testing.".to_string(),
            "The current export reflects a small curated panel rather than comprehensive genomic screening.".to_string(),
        ],
    }
}

fn counselor_classification(finding: &StructuredFinding) -> &'static str {
    if finding.family == "pharmacogenomics" {
        "drug_response"
    } else if finding.significance == "elevated_risk" {
        "risk_factor"
    } else if finding.significance == "baseline" {
        "baseline_finding"
    } else {
        "informational"
    }
}

#[derive(Serialize)]
struct AppleHealthExport {
    format: &'static str,
    version: &'static str,
    generated_at_unix_ms: u64,
    analysis_id: String,
    source: AppleHealthSource,
    records: Vec<AppleHealthRecord>,
}

#[derive(Serialize)]
struct AppleHealthSource {
    name: &'static str,
    bundle_id: &'static str,
}

#[derive(Serialize)]
struct AppleHealthRecord {
    record_type: String,
    identifier: String,
    display_name: String,
    value: String,
    interpretation: String,
    metadata: serde_json::Value,
}

fn build_apple_health_export(package: &AnalysisPackage) -> AppleHealthExport {
    let mut records = Vec::new();

    records.push(AppleHealthRecord {
        record_type: "analysis_quality".to_string(),
        identifier: "call_rate".to_string(),
        display_name: "DNA call rate".to_string(),
        value: format!("{:.2}%", package.results.quality.call_rate_percent),
        interpretation: package.results.quality.quality_tier.clone(),
        metadata: serde_json::json!({
            "total_snps": package.results.quality.total_snps,
            "no_calls": package.results.quality.no_calls,
        }),
    });

    for finding in all_findings(package) {
        records.push(AppleHealthRecord {
            record_type: finding.family.clone(),
            identifier: format!("{}:{}", finding.family, finding.rsid),
            display_name: finding.trait_name.clone(),
            value: finding.genotype.clone(),
            interpretation: finding.interpretation.clone(),
            metadata: serde_json::json!({
                "rsid": finding.rsid,
                "confidence": finding.confidence,
                "actionability": finding.actionability,
                "significance": finding.significance,
            }),
        });
    }

    AppleHealthExport {
        format: "apple_health_compatible",
        version: EXPORTS_SCHEMA_VERSION,
        generated_at_unix_ms: now_unix_ms().unwrap_or_default(),
        analysis_id: package.id.clone(),
        source: AppleHealthSource {
            name: "Personal Genomics",
            bundle_id: "com.morpheus.genomics",
        },
        records,
    }
}

#[derive(Serialize)]
struct ApiExport {
    api_version: &'static str,
    schema: &'static str,
    generated_at_unix_ms: u64,
    analysis_id: String,
    metadata: ApiMetadata,
    summary: ApiSummary,
    results: crate::local_analysis::AnalysisResults,
    report: ApiReport,
    #[serde(skip_serializing_if = "Option::is_none")]
    genome: Option<crate::genomics::ParsedGenome>,
}

#[derive(Serialize)]
struct ApiMetadata {
    source_file_name: String,
    format: String,
    schema_version: String,
    marker_panel_name: String,
    marker_panel_version: String,
}

#[derive(Serialize)]
struct ApiSummary {
    matched_marker_count: usize,
    actionable_finding_count: usize,
    high_confidence_finding_count: usize,
    report_available: bool,
}

#[derive(Serialize)]
struct ApiReport {
    model_id: Option<String>,
    generated_at_unix_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    markdown: Option<String>,
}

fn build_api_export(package: &AnalysisPackage, include_genome: bool) -> ApiExport {
    ApiExport {
        api_version: "2.0.0",
        schema: "com.morpheus.genomics.analysis",
        generated_at_unix_ms: now_unix_ms().unwrap_or_default(),
        analysis_id: package.id.clone(),
        metadata: ApiMetadata {
            source_file_name: package.source.file_name.clone(),
            format: package.results.quality.format_label.clone(),
            schema_version: package.schema_version.clone(),
            marker_panel_name: package.results.provenance.marker_panel_name.clone(),
            marker_panel_version: package.results.provenance.marker_panel_version.clone(),
        },
        summary: ApiSummary {
            matched_marker_count: package.results.summary.matched_marker_count,
            actionable_finding_count: package.results.summary.actionable_finding_count,
            high_confidence_finding_count: package.results.summary.high_confidence_finding_count,
            report_available: !package.report.markdown.trim().is_empty(),
        },
        results: package.results.clone(),
        report: ApiReport {
            model_id: package.report.model_id.clone(),
            generated_at_unix_ms: package.report.generated_at_unix_ms,
            markdown: if include_genome {
                Some(package.report.markdown.clone())
            } else {
                None
            },
        },
        genome: if include_genome {
            Some(package.genome.clone())
        } else {
            None
        },
    }
}

#[derive(Serialize)]
struct IntegrationHooksExport {
    version: &'static str,
    generated_at_unix_ms: u64,
    analysis_id: String,
    capabilities: HookCapabilities,
    webhook_payloads: HookPayloads,
}

#[derive(Serialize)]
struct HookCapabilities {
    supports_counselor_export: bool,
    supports_apple_health_json: bool,
    supports_api_export: bool,
    supports_webhook_payloads: bool,
}

#[derive(Serialize)]
struct HookPayloads {
    analysis_ready: serde_json::Value,
    report_ready: serde_json::Value,
    exports_ready: serde_json::Value,
}

fn build_integration_hooks_export(package: &AnalysisPackage) -> IntegrationHooksExport {
    let report_available = !package.report.markdown.trim().is_empty();
    IntegrationHooksExport {
        version: EXPORTS_SCHEMA_VERSION,
        generated_at_unix_ms: now_unix_ms().unwrap_or_default(),
        analysis_id: package.id.clone(),
        capabilities: HookCapabilities {
            supports_counselor_export: true,
            supports_apple_health_json: true,
            supports_api_export: true,
            supports_webhook_payloads: true,
        },
        webhook_payloads: HookPayloads {
            analysis_ready: serde_json::json!({
                "event": "analysis.ready",
                "analysisId": package.id,
                "summary": package.results.summary,
                "quality": {
                    "callRatePercent": package.results.quality.call_rate_percent,
                    "qualityTier": package.results.quality.quality_tier,
                }
            }),
            report_ready: serde_json::json!({
                "event": "report.ready",
                "analysisId": package.id,
                "reportAvailable": report_available,
                "reportModelId": package.report.model_id,
                "reportGeneratedAtUnixMs": package.report.generated_at_unix_ms,
            }),
            exports_ready: serde_json::json!({
                "event": "exports.ready",
                "analysisId": package.id,
                "exportsDir": package.artifacts.exports_dir,
                "files": {
                    "clinical": package.artifacts.counselor_export_json_path,
                    "appleHealth": package.artifacts.apple_health_export_json_path,
                    "api": package.artifacts.api_export_json_path,
                    "apiFull": package.artifacts.api_export_full_json_path,
                    "integrationHooks": package.artifacts.integration_hooks_json_path,
                    "exportedPdf": package.artifacts.exported_pdf_path,
                }
            }),
        },
    }
}

fn all_findings(package: &AnalysisPackage) -> Vec<StructuredFinding> {
    [
        package.results.finding_groups.pharmacogenomics.clone(),
        package
            .results
            .finding_groups
            .metabolic_cardiovascular
            .clone(),
        package.results.finding_groups.traits.clone(),
        package
            .results
            .finding_groups
            .neuropsychiatric_cognitive
            .clone(),
        package.results.finding_groups.other.clone(),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn now_unix_ms() -> AppResult<u64> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| {
            crate::error::AppError::Other(format!("system clock is before unix epoch: {e}"))
        })?
        .as_millis() as u64)
}
