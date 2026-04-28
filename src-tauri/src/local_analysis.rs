use crate::genomics::{GenomicFormat, MatchedMarker, ParsedGenome};
use crate::markers;
use serde::{Deserialize, Serialize};

const MARKER_PANEL_NAME: &str = "local curated marker panel";
const MARKER_PANEL_VERSION: &str = "curated-28-v1";
const INTERPRETATION_METHOD: &str =
    "rule-based genotype matching against literature-backed curated markers";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnalysisResults {
    pub summary: AnalysisSummary,
    pub quality: AnalysisQuality,
    pub finding_groups: FindingGroups,
    pub recommendations: AnalysisRecommendations,
    pub provenance: AnalysisProvenance,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnalysisSummary {
    pub matched_marker_count: usize,
    pub actionable_finding_count: usize,
    pub high_confidence_finding_count: usize,
    pub family_counts: AnalysisFamilyCounts,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnalysisFamilyCounts {
    pub pharmacogenomics: usize,
    pub metabolic_cardiovascular: usize,
    pub traits: usize,
    pub neuropsychiatric_cognitive: usize,
    pub other: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnalysisQuality {
    pub format_label: String,
    pub sex_inference: String,
    pub total_snps: usize,
    pub no_calls: usize,
    pub call_rate: f64,
    pub call_rate_percent: f64,
    pub matched_marker_count: usize,
    pub matched_marker_rate: f64,
    pub quality_tier: String,
    pub caveats: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FindingGroups {
    pub pharmacogenomics: Vec<StructuredFinding>,
    pub metabolic_cardiovascular: Vec<StructuredFinding>,
    pub traits: Vec<StructuredFinding>,
    pub neuropsychiatric_cognitive: Vec<StructuredFinding>,
    pub other: Vec<StructuredFinding>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StructuredFinding {
    pub rsid: String,
    pub genotype: String,
    pub source_category: String,
    pub family: String,
    pub trait_name: String,
    pub interpretation: String,
    pub confidence: String,
    pub evidence_level: String,
    pub significance: String,
    pub actionability: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnalysisRecommendations {
    pub priority_actions: Vec<String>,
    pub clinician_discussion_topics: Vec<String>,
    pub lifestyle_focus: Vec<String>,
    pub informational_notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnalysisProvenance {
    pub analysis_engine: String,
    pub marker_panel_name: String,
    pub marker_panel_version: String,
    pub marker_panel_size: usize,
    pub interpretation_method: String,
    pub derived_locally: bool,
    pub raw_genotype_sent_off_device: bool,
}

pub fn build_analysis_results(genome: &ParsedGenome) -> AnalysisResults {
    let mut finding_groups = FindingGroups::default();

    for finding in genome
        .matched_markers
        .iter()
        .map(structured_finding_from_marker)
    {
        match finding.family.as_str() {
            "pharmacogenomics" => finding_groups.pharmacogenomics.push(finding),
            "metabolic_cardiovascular" => finding_groups.metabolic_cardiovascular.push(finding),
            "traits" => finding_groups.traits.push(finding),
            "neuropsychiatric_cognitive" => finding_groups.neuropsychiatric_cognitive.push(finding),
            _ => finding_groups.other.push(finding),
        }
    }

    sort_findings(&mut finding_groups.pharmacogenomics);
    sort_findings(&mut finding_groups.metabolic_cardiovascular);
    sort_findings(&mut finding_groups.traits);
    sort_findings(&mut finding_groups.neuropsychiatric_cognitive);
    sort_findings(&mut finding_groups.other);

    let family_counts = AnalysisFamilyCounts {
        pharmacogenomics: finding_groups.pharmacogenomics.len(),
        metabolic_cardiovascular: finding_groups.metabolic_cardiovascular.len(),
        traits: finding_groups.traits.len(),
        neuropsychiatric_cognitive: finding_groups.neuropsychiatric_cognitive.len(),
        other: finding_groups.other.len(),
    };

    let quality = build_quality(genome);
    let recommendations = build_recommendations(genome, &finding_groups, &quality);

    AnalysisResults {
        summary: AnalysisSummary {
            matched_marker_count: genome.matched_markers.len(),
            actionable_finding_count: actionable_finding_count(&finding_groups),
            high_confidence_finding_count: high_confidence_finding_count(&finding_groups),
            family_counts,
        },
        quality,
        finding_groups,
        recommendations,
        provenance: AnalysisProvenance {
            analysis_engine: "stage-2-local-analysis".to_string(),
            marker_panel_name: MARKER_PANEL_NAME.to_string(),
            marker_panel_version: MARKER_PANEL_VERSION.to_string(),
            marker_panel_size: markers::curated_markers().len(),
            interpretation_method: INTERPRETATION_METHOD.to_string(),
            derived_locally: true,
            raw_genotype_sent_off_device: false,
        },
    }
}

fn build_quality(genome: &ParsedGenome) -> AnalysisQuality {
    let matched_marker_count = genome.matched_markers.len();
    let matched_marker_rate = if genome.total_snps > 0 {
        matched_marker_count as f64 / genome.total_snps as f64
    } else {
        0.0
    };

    let quality_tier = if genome.call_rate >= 0.995 {
        "excellent"
    } else if genome.call_rate >= 0.99 {
        "very_good"
    } else if genome.call_rate >= 0.97 {
        "good"
    } else if genome.call_rate >= 0.94 {
        "caution"
    } else {
        "limited"
    };

    let mut caveats = Vec::new();
    if genome.call_rate < 0.97 {
        caveats.push(
            "Call rate is below the 97% threshold, so any important finding should be confirmed before acting on it."
                .to_string(),
        );
    }
    if matched_marker_count == 0 {
        caveats.push(
            "No curated markers matched the current panel, so the downstream report will be sparse and mostly quality-focused."
                .to_string(),
        );
    }
    if genome.format == GenomicFormat::Unknown {
        caveats.push(
            "The file format could not be identified confidently, which may reduce trust in downstream summaries."
                .to_string(),
        );
    }

    AnalysisQuality {
        format_label: format_label(genome.format).to_string(),
        sex_inference: genome.sex_inference.clone(),
        total_snps: genome.total_snps,
        no_calls: genome.no_calls,
        call_rate: genome.call_rate,
        call_rate_percent: genome.call_rate * 100.0,
        matched_marker_count,
        matched_marker_rate,
        quality_tier: quality_tier.to_string(),
        caveats,
    }
}

fn build_recommendations(
    genome: &ParsedGenome,
    finding_groups: &FindingGroups,
    quality: &AnalysisQuality,
) -> AnalysisRecommendations {
    let mut priority_actions = Vec::new();
    let mut clinician_discussion_topics = Vec::new();
    let mut lifestyle_focus = Vec::new();
    let mut informational_notes = Vec::new();

    if quality.quality_tier == "caution" || quality.quality_tier == "limited" {
        push_unique(
            &mut priority_actions,
            "Treat this dataset as lower-confidence and confirm any meaningful result with repeat testing or a clinical-grade assay before acting.",
        );
    }

    if !finding_groups.pharmacogenomics.is_empty() {
        push_unique(
            &mut priority_actions,
            "Share the pharmacogenomics findings with your clinician or pharmacist before making medication changes.",
        );
    }

    if has_any_rsid(
        &finding_groups.pharmacogenomics,
        &["rs4986893", "rs4244285", "rs3892097", "rs1042713"],
    ) {
        push_unique(
            &mut clinician_discussion_topics,
            "Discuss medication-response markers that may affect clopidogrel, CYP2D6 substrates, and beta-agonist response.",
        );
    }

    if has_any_rsid(
        &finding_groups.metabolic_cardiovascular,
        &["rs7903146", "rs10757278", "rs4977574", "rs2231142"],
    ) {
        push_unique(
            &mut clinician_discussion_topics,
            "Review cardiometabolic risk markers together with standard screening data such as lipids, glucose, blood pressure, or uric acid.",
        );
    }

    if has_any_rsid(
        &finding_groups.neuropsychiatric_cognitive,
        &["rs429358", "rs7412"],
    ) {
        push_unique(
            &mut clinician_discussion_topics,
            "If family history is relevant, ask a clinician how APOE-related findings should be interpreted alongside age, family history, and lifestyle.",
        );
    }

    if has_any_rsid(&all_findings(finding_groups), &["rs762551"]) {
        push_unique(
            &mut lifestyle_focus,
            "Adjust caffeine timing and intake if slow-metabolizer CYP1A2 findings are present.",
        );
    }

    if has_any_rsid(&all_findings(finding_groups), &["rs671", "rs1229984"]) {
        push_unique(
            &mut lifestyle_focus,
            "Alcohol tolerance and acetaldehyde handling markers suggest being conservative with alcohol exposure.",
        );
    }

    if has_any_rsid(&all_findings(finding_groups), &["rs4988235"]) {
        push_unique(
            &mut lifestyle_focus,
            "Use lactose tolerance findings to guide dairy choices, especially if symptoms and genotype point in the same direction.",
        );
    }

    if has_any_rsid(&all_findings(finding_groups), &["rs9939609", "rs7903146"]) {
        push_unique(
            &mut lifestyle_focus,
            "Weight and glucose-related markers make sleep, diet quality, and consistent activity higher-yield lifestyle levers.",
        );
    }

    if has_any_rsid(
        &all_findings(finding_groups),
        &["rs2231142", "rs10757278", "rs4977574"],
    ) {
        push_unique(
            &mut lifestyle_focus,
            "Cardiometabolic and urate-related markers are best handled through routine preventive care and repeat monitoring rather than one-off interpretation.",
        );
    }

    if genome.matched_markers.is_empty() {
        push_unique(
            &mut informational_notes,
            "The current curated panel found no matches, so absence of findings here does not imply absence of genetic risk in general.",
        );
    }

    push_unique(
        &mut informational_notes,
        "Trait and behavioral markers are probabilistic and generally lower-stakes than pharmacogenomic findings.",
    );
    push_unique(
        &mut informational_notes,
        "All structured findings here come from a small local curated marker panel and should be treated as decision support, not diagnosis.",
    );

    AnalysisRecommendations {
        priority_actions,
        clinician_discussion_topics,
        lifestyle_focus,
        informational_notes,
    }
}

fn structured_finding_from_marker(marker: &MatchedMarker) -> StructuredFinding {
    let family = family_for_category(&marker.category).to_string();
    let actionability = actionability(marker).to_string();
    let significance = significance(marker).to_string();

    StructuredFinding {
        rsid: marker.rsid.clone(),
        genotype: marker.genotype.clone(),
        source_category: marker.category.clone(),
        family,
        trait_name: marker.trait_name.clone(),
        interpretation: marker.interpretation.clone(),
        confidence: marker.confidence.clone(),
        evidence_level: marker.confidence.to_ascii_uppercase(),
        significance,
        actionability,
        summary: format!("{} — {}", marker.trait_name, marker.interpretation),
    }
}

fn family_for_category(category: &str) -> &'static str {
    match category {
        "Pharmacogenomics" => "pharmacogenomics",
        "Metabolic" | "Cardiovascular" | "Weight" | "Methylation" | "Nutrition"
        | "Detoxification" | "Longevity" => "metabolic_cardiovascular",
        "Traits" | "Athletic" | "Behavioral" => "traits",
        "Neuropsychiatric" | "Neurodegenerative" => "neuropsychiatric_cognitive",
        _ => "other",
    }
}

fn actionability(marker: &MatchedMarker) -> &'static str {
    let lower = marker.interpretation.to_ascii_lowercase();
    if marker.category == "Pharmacogenomics"
        && (lower.contains("poor metabolizer")
            || lower.contains("avoid")
            || lower.contains("reduced clopidogrel activation")
            || lower.contains("dose reductions"))
    {
        return "high";
    }

    if marker.category == "Pharmacogenomics" {
        return "medium";
    }

    if lower.contains("significantly elevated")
        || lower.contains("~2x")
        || lower.contains("higher-risk")
        || lower.contains("higher esophageal cancer risk")
    {
        return "medium";
    }

    if lower.contains("elevated") || lower.contains("risk") || lower.contains("reduced") {
        return "medium";
    }

    "low"
}

fn significance(marker: &MatchedMarker) -> &'static str {
    let lower = marker.interpretation.to_ascii_lowercase();
    if marker.category == "Pharmacogenomics" {
        return "actionable";
    }
    if lower.contains("baseline")
        || lower.contains("typical")
        || lower.contains("wild-type")
        || lower.contains("normal")
    {
        return "baseline";
    }
    if lower.contains("elevated") || lower.contains("risk") || lower.contains("avoid") {
        return "elevated_risk";
    }
    "informational"
}

fn sort_findings(findings: &mut [StructuredFinding]) {
    findings.sort_by(|a, b| {
        sort_rank(&a.actionability)
            .cmp(&sort_rank(&b.actionability))
            .then(sort_rank(&a.confidence).cmp(&sort_rank(&b.confidence)))
            .then(a.rsid.cmp(&b.rsid))
    });
}

fn sort_rank(value: &str) -> usize {
    match value.to_ascii_lowercase().as_str() {
        "high" => 0,
        "medium" => 1,
        "low" => 2,
        "baseline" => 3,
        _ => 4,
    }
}

fn actionable_finding_count(groups: &FindingGroups) -> usize {
    all_findings(groups)
        .iter()
        .filter(|finding| finding.actionability == "high" || finding.actionability == "medium")
        .count()
}

fn high_confidence_finding_count(groups: &FindingGroups) -> usize {
    all_findings(groups)
        .iter()
        .filter(|finding| finding.confidence.eq_ignore_ascii_case("high"))
        .count()
}

fn all_findings(groups: &FindingGroups) -> Vec<StructuredFinding> {
    [
        groups.pharmacogenomics.clone(),
        groups.metabolic_cardiovascular.clone(),
        groups.traits.clone(),
        groups.neuropsychiatric_cognitive.clone(),
        groups.other.clone(),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn has_any_rsid(findings: &[StructuredFinding], rsids: &[&str]) -> bool {
    findings
        .iter()
        .any(|finding| rsids.iter().any(|rsid| finding.rsid == *rsid))
}

fn push_unique(target: &mut Vec<String>, item: impl Into<String>) {
    let item = item.into();
    if !target.contains(&item) {
        target.push(item);
    }
}

fn format_label(format: GenomicFormat) -> &'static str {
    match format {
        GenomicFormat::TwentyThreeAndMe => "23andMe",
        GenomicFormat::AncestryDna => "AncestryDNA",
        GenomicFormat::MyHeritage => "MyHeritage",
        GenomicFormat::FamilyTreeDna => "FamilyTreeDNA",
        GenomicFormat::Vcf => "VCF",
        GenomicFormat::RsidTsv => "rsID TSV",
        GenomicFormat::Unknown => "Unknown",
    }
}
