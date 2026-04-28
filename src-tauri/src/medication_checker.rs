use crate::analysis::AnalysisPackage;
use crate::genomics::MatchedMarker;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MedicationCheckResult {
    pub requested_medication: String,
    pub matched_medication: String,
    pub category: String,
    pub severity: String,
    pub gene: String,
    pub phenotype: String,
    pub summary: String,
    pub recommendation: String,
    pub evidence_note: String,
    pub supporting_rsids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MedicationCheckResponse {
    pub requested_count: usize,
    pub matched_count: usize,
    pub unmatched_medications: Vec<String>,
    pub results: Vec<MedicationCheckResult>,
}

pub fn check_medications(
    analysis: &AnalysisPackage,
    medications: Vec<String>,
) -> MedicationCheckResponse {
    let cleaned_requests: Vec<String> = medications
        .into_iter()
        .map(|m| m.trim().to_string())
        .filter(|m| !m.is_empty())
        .collect();

    let marker_map: HashMap<&str, &MatchedMarker> = analysis
        .genome
        .matched_markers
        .iter()
        .map(|marker| (marker.rsid.as_str(), marker))
        .collect();

    let cyp2c19 = infer_cyp2c19(marker_map.get("rs4244285"), marker_map.get("rs4986893"));
    let cyp2d6 = infer_cyp2d6(marker_map.get("rs3892097"));
    let adrb2 = infer_adrb2(marker_map.get("rs1042713"));

    let normalized_requests = cleaned_requests
        .iter()
        .map(|request| normalize_name(request))
        .collect::<Vec<_>>();
    let has_tamoxifen = normalized_requests
        .iter()
        .any(|name| matches!(classify_medication(name), Some(MedicationRule::Tamoxifen)));
    let has_strong_cyp2d6_inhibitor = normalized_requests.iter().any(|name| {
        matches!(
            classify_medication(name),
            Some(MedicationRule::Cyp2d6InhibitorAntidepressant)
        )
    });

    let mut unmatched_medications = Vec::new();
    let mut results = Vec::new();

    for requested in cleaned_requests.iter() {
        match classify_medication(requested) {
            Some(MedicationRule::Clopidogrel) => push_or_unmatched(
                &mut results,
                &mut unmatched_medications,
                requested,
                clopidogrel_result(requested, &cyp2c19),
            ),
            Some(MedicationRule::Cyp2c19Ssri) => push_or_unmatched(
                &mut results,
                &mut unmatched_medications,
                requested,
                cyp2c19_ssri_result(requested, &cyp2c19),
            ),
            Some(MedicationRule::ProtonPumpInhibitor) => push_or_unmatched(
                &mut results,
                &mut unmatched_medications,
                requested,
                ppi_result(requested, &cyp2c19),
            ),
            Some(MedicationRule::Cyp2d6Opioid) => push_or_unmatched(
                &mut results,
                &mut unmatched_medications,
                requested,
                cyp2d6_opioid_result(requested, &cyp2d6),
            ),
            Some(MedicationRule::Cyp2d6Tca) => push_or_unmatched(
                &mut results,
                &mut unmatched_medications,
                requested,
                cyp2d6_tca_result(requested, &cyp2d6, &cyp2c19),
            ),
            Some(MedicationRule::Cyp2d6Metoprolol) => push_or_unmatched(
                &mut results,
                &mut unmatched_medications,
                requested,
                metoprolol_result(requested, &cyp2d6),
            ),
            Some(MedicationRule::Tamoxifen) => push_or_unmatched(
                &mut results,
                &mut unmatched_medications,
                requested,
                tamoxifen_result(requested, &cyp2d6, has_strong_cyp2d6_inhibitor),
            ),
            Some(MedicationRule::Clobazam) => push_or_unmatched(
                &mut results,
                &mut unmatched_medications,
                requested,
                clobazam_result(requested, &cyp2c19),
            ),
            Some(MedicationRule::Diazepam) => push_or_unmatched(
                &mut results,
                &mut unmatched_medications,
                requested,
                diazepam_result(requested, &cyp2c19),
            ),
            Some(MedicationRule::Cyp2d6InhibitorAntidepressant) => push_or_unmatched(
                &mut results,
                &mut unmatched_medications,
                requested,
                cyp2d6_inhibitor_result(requested, &cyp2d6, has_tamoxifen),
            ),
            Some(MedicationRule::BetaAgonist) => push_or_unmatched(
                &mut results,
                &mut unmatched_medications,
                requested,
                beta_agonist_result(requested, &adrb2),
            ),
            None => unmatched_medications.push(requested.clone()),
        }
    }

    sort_results(&mut results);

    MedicationCheckResponse {
        requested_count: cleaned_requests.len(),
        matched_count: results.len(),
        unmatched_medications,
        results,
    }
}

#[derive(Debug, Clone)]
struct GenePhenotype {
    gene: &'static str,
    phenotype: &'static str,
    supporting_rsids: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
enum MedicationRule {
    Clopidogrel,
    Cyp2c19Ssri,
    ProtonPumpInhibitor,
    Cyp2d6Opioid,
    Cyp2d6Tca,
    Cyp2d6Metoprolol,
    Tamoxifen,
    Clobazam,
    Diazepam,
    Cyp2d6InhibitorAntidepressant,
    BetaAgonist,
}

fn classify_medication(input: &str) -> Option<MedicationRule> {
    let normalized = normalize_name(input);
    match normalized.as_str() {
        "clopidogrel" | "plavix" | "iscover" => Some(MedicationRule::Clopidogrel),
        "citalopram" | "celexa" | "escitalopram" | "lexapro" | "sertraline" | "zoloft" => {
            Some(MedicationRule::Cyp2c19Ssri)
        }
        "omeprazole" | "prilosec" | "lansoprazole" | "prevacid" | "pantoprazole"
        | "protonix" | "dexlansoprazole" | "dexilant" | "esomeprazole" | "nexium" => {
            Some(MedicationRule::ProtonPumpInhibitor)
        }
        "codeine" | "tramadol" | "ultram" | "hydrocodone" | "vicodin" => {
            Some(MedicationRule::Cyp2d6Opioid)
        }
        "amitriptyline" | "elavil" | "clomipramine" | "anafranil" | "imipramine"
        | "tofranil" | "doxepin" | "sinequan" | "trimipramine" | "surmontil"
        | "nortriptyline" | "pamelor" | "desipramine" | "norpramin" => {
            Some(MedicationRule::Cyp2d6Tca)
        }
        "metoprolol" | "lopressor" | "toprolxl" | "toprol" => {
            Some(MedicationRule::Cyp2d6Metoprolol)
        }
        "tamoxifen" | "nolvadex" | "soltamox" => Some(MedicationRule::Tamoxifen),
        "clobazam" | "onfi" | "frisium" => Some(MedicationRule::Clobazam),
        "diazepam" | "valium" => Some(MedicationRule::Diazepam),
        "paroxetine" | "paxil" | "fluoxetine" | "prozac" | "bupropion" | "wellbutrin"
        | "duloxetine" | "cymbalta" => Some(MedicationRule::Cyp2d6InhibitorAntidepressant),
        "albuterol" | "salbutamol" | "ventolin" => Some(MedicationRule::BetaAgonist),
        _ => None,
    }
}

fn clopidogrel_result(requested: &str, phenotype: &Option<GenePhenotype>) -> Option<MedicationCheckResult> {
    let phenotype = phenotype.as_ref()?;
    let (severity, recommendation) = match phenotype.phenotype {
        "likely reduced metabolizer" => (
            "critical",
            "Discuss an alternative antiplatelet strategy because reduced CYP2C19 activity can meaningfully limit clopidogrel activation.",
        ),
        "likely intermediate metabolizer" => (
            "serious",
            "Discuss whether clopidogrel efficacy may be reduced and whether closer monitoring or an alternative is appropriate.",
        ),
        _ => (
            "informational",
            "No obvious reduced-activation warning from the currently available CYP2C19 markers.",
        ),
    };

    Some(MedicationCheckResult {
        requested_medication: requested.to_string(),
        matched_medication: "clopidogrel".to_string(),
        category: "Heart / Blood Thinners".to_string(),
        severity: severity.to_string(),
        gene: phenotype.gene.to_string(),
        phenotype: phenotype.phenotype.to_string(),
        summary: format!(
            "CYP2C19 {} can change how well clopidogrel is activated.",
            phenotype.phenotype
        ),
        recommendation: recommendation.to_string(),
        evidence_note: "Based on CYP2C19 *2/*3 consumer-marker inference only; the current panel cannot infer CYP2C19 ultrarapid status.".to_string(),
        supporting_rsids: phenotype.supporting_rsids.clone(),
    })
}

fn cyp2c19_ssri_result(requested: &str, phenotype: &Option<GenePhenotype>) -> Option<MedicationCheckResult> {
    let phenotype = phenotype.as_ref()?;
    let (severity, recommendation) = match phenotype.phenotype {
        "likely reduced metabolizer" => (
            "serious",
            "Reduced CYP2C19 activity may increase exposure for some CYP2C19-sensitive antidepressants; dose sensitivity or alternatives are worth discussing.",
        ),
        "likely intermediate metabolizer" => (
            "moderate",
            "This CYP2C19 profile may modestly raise drug exposure for some antidepressants in this family.",
        ),
        _ => (
            "informational",
            "Current CYP2C19 markers do not suggest a reduced-metabolism warning for this antidepressant family.",
        ),
    };

    Some(MedicationCheckResult {
        requested_medication: requested.to_string(),
        matched_medication: canonical_name(requested),
        category: "Mood / Mental Health".to_string(),
        severity: severity.to_string(),
        gene: phenotype.gene.to_string(),
        phenotype: phenotype.phenotype.to_string(),
        summary: format!(
            "CYP2C19 {} may affect blood levels for this antidepressant.",
            phenotype.phenotype
        ),
        recommendation: recommendation.to_string(),
        evidence_note: "This checker uses a limited CYP2C19 consumer-marker inference and cannot identify increased-function *17 status.".to_string(),
        supporting_rsids: phenotype.supporting_rsids.clone(),
    })
}

fn ppi_result(requested: &str, phenotype: &Option<GenePhenotype>) -> Option<MedicationCheckResult> {
    let phenotype = phenotype.as_ref()?;
    let normalized = normalize_name(requested);

    let (severity, recommendation) = match phenotype.phenotype {
        "likely reduced metabolizer" => (
            "moderate",
            "For longer-term therapy, reduced CYP2C19 metabolism can raise first-generation PPI exposure. If treatment is working and side effects appear, a lower chronic dose may be worth discussing.",
        ),
        "likely intermediate metabolizer" => (
            "moderate",
            "This CYP2C19 profile may modestly raise first-generation PPI exposure. Review dose and duration if side effects matter.",
        ),
        _ => {
            if normalized == "esomeprazole" || normalized == "nexium" {
                (
                    "informational",
                    "Esomeprazole is generally less CYP2C19-sensitive than older PPIs, so current local markers are mostly informational here.",
                )
            } else {
                (
                    "informational",
                    "No obvious reduced-metabolism warning from the current CYP2C19 markers for this PPI.",
                )
            }
        }
    };

    Some(MedicationCheckResult {
        requested_medication: requested.to_string(),
        matched_medication: canonical_name(requested),
        category: "Stomach Acid / Reflux".to_string(),
        severity: severity.to_string(),
        gene: phenotype.gene.to_string(),
        phenotype: phenotype.phenotype.to_string(),
        summary: format!(
            "CYP2C19 {} may affect exposure for this proton pump inhibitor.",
            phenotype.phenotype
        ),
        recommendation: recommendation.to_string(),
        evidence_note: "Most relevant for omeprazole, lansoprazole, pantoprazole, and dexlansoprazole; the current panel cannot identify CYP2C19 ultrarapid metabolizers.".to_string(),
        supporting_rsids: phenotype.supporting_rsids.clone(),
    })
}

fn cyp2d6_opioid_result(requested: &str, phenotype: &Option<GenePhenotype>) -> Option<MedicationCheckResult> {
    let phenotype = phenotype.as_ref()?;
    let (severity, recommendation) = match phenotype.phenotype {
        "likely poor metabolizer" => (
            "critical",
            "This CYP2D6 profile can make codeine or tramadol ineffective and may reduce activation of some related opioids. Ask about alternatives.",
        ),
        "likely intermediate metabolizer" => (
            "serious",
            "Reduced CYP2D6 activity may blunt pain relief or alter response. Monitor closely and discuss alternatives if benefit is weak.",
        ),
        _ => (
            "informational",
            "Current CYP2D6 marker does not suggest a reduced-metabolism warning for this opioid family.",
        ),
    };

    Some(MedicationCheckResult {
        requested_medication: requested.to_string(),
        matched_medication: canonical_name(requested),
        category: "Pain Relief".to_string(),
        severity: severity.to_string(),
        gene: phenotype.gene.to_string(),
        phenotype: phenotype.phenotype.to_string(),
        summary: format!(
            "CYP2D6 {} may alter activation or effect for this opioid family.",
            phenotype.phenotype
        ),
        recommendation: recommendation.to_string(),
        evidence_note: "The current consumer marker supports reduced-function CYP2D6 inference only; it does not identify ultrarapid metabolizers from copy-number variation.".to_string(),
        supporting_rsids: phenotype.supporting_rsids.clone(),
    })
}

fn cyp2d6_tca_result(
    requested: &str,
    cyp2d6: &Option<GenePhenotype>,
    cyp2c19: &Option<GenePhenotype>,
) -> Option<MedicationCheckResult> {
    let cyp2d6 = cyp2d6.as_ref()?;
    let cyp2c19_note = cyp2c19
        .as_ref()
        .map(|phenotype| format!(" CYP2C19 also looks like {}.", phenotype.phenotype))
        .unwrap_or_default();

    let (severity, recommendation) = match cyp2d6.phenotype {
        "likely poor metabolizer" => (
            "serious",
            "For tricyclic antidepressants, this CYP2D6 profile supports lower starting doses and closer follow-up for side effects.",
        ),
        "likely intermediate metabolizer" => (
            "moderate",
            "A modest dose reduction or closer monitoring may be reasonable for tricyclic antidepressants with CYP2D6 sensitivity.",
        ),
        _ => (
            "informational",
            "Current local CYP2D6 marker does not suggest a reduced-metabolism warning for this tricyclic antidepressant family.",
        ),
    };

    Some(MedicationCheckResult {
        requested_medication: requested.to_string(),
        matched_medication: canonical_name(requested),
        category: "Mood / Mental Health".to_string(),
        severity: severity.to_string(),
        gene: cyp2d6.gene.to_string(),
        phenotype: cyp2d6.phenotype.to_string(),
        summary: format!(
            "CYP2D6 {} may meaningfully affect tricyclic antidepressant exposure.{}",
            cyp2d6.phenotype,
            cyp2c19_note
        ),
        recommendation: recommendation.to_string(),
        evidence_note: "This checker uses a reduced-function CYP2D6 consumer marker and only partial CYP2C19 coverage, so it should not replace full PGx testing for TCA decisions.".to_string(),
        supporting_rsids: cyp2d6
            .supporting_rsids
            .iter()
            .cloned()
            .chain(
                cyp2c19
                    .as_ref()
                    .into_iter()
                    .flat_map(|phenotype| phenotype.supporting_rsids.clone()),
            )
            .collect(),
    })
}

fn metoprolol_result(requested: &str, phenotype: &Option<GenePhenotype>) -> Option<MedicationCheckResult> {
    let phenotype = phenotype.as_ref()?;
    let (severity, recommendation) = match phenotype.phenotype {
        "likely poor metabolizer" => (
            "serious",
            "Poor CYP2D6 metabolism can increase metoprolol exposure. A low starting dose and slower titration may be appropriate.",
        ),
        "likely intermediate metabolizer" => (
            "moderate",
            "This CYP2D6 profile may increase metoprolol exposure modestly. Monitor for bradycardia or dizziness.",
        ),
        _ => (
            "informational",
            "Current local CYP2D6 marker does not suggest a reduced-metabolism warning for metoprolol.",
        ),
    };

    Some(MedicationCheckResult {
        requested_medication: requested.to_string(),
        matched_medication: canonical_name(requested),
        category: "Heart / Blood Pressure".to_string(),
        severity: severity.to_string(),
        gene: phenotype.gene.to_string(),
        phenotype: phenotype.phenotype.to_string(),
        summary: format!(
            "CYP2D6 {} may affect metoprolol blood levels.",
            phenotype.phenotype
        ),
        recommendation: recommendation.to_string(),
        evidence_note: "This uses the local CYP2D6*4 marker only and cannot infer ultrarapid metabolism or full star-allele activity scores.".to_string(),
        supporting_rsids: phenotype.supporting_rsids.clone(),
    })
}

fn tamoxifen_result(
    requested: &str,
    phenotype: &Option<GenePhenotype>,
    has_strong_inhibitor: bool,
) -> Option<MedicationCheckResult> {
    let phenotype = phenotype.as_ref()?;
    let (severity, recommendation) = if has_strong_inhibitor {
        (
            "critical",
            "A strong CYP2D6-inhibiting antidepressant on top of reduced CYP2D6 activity can further lower tamoxifen activation. This combination deserves clinician review.",
        )
    } else {
        match phenotype.phenotype {
            "likely poor metabolizer" | "likely intermediate metabolizer" => (
                "serious",
                "Reduced CYP2D6 activity may lower tamoxifen activation. Ask your oncology team whether this matters for your treatment plan.",
            ),
            _ => (
                "informational",
                "Current local CYP2D6 marker does not suggest a reduced-activation warning for tamoxifen, but medication interactions still matter.",
            ),
        }
    };

    Some(MedicationCheckResult {
        requested_medication: requested.to_string(),
        matched_medication: canonical_name(requested),
        category: "Cancer / Hormones".to_string(),
        severity: severity.to_string(),
        gene: phenotype.gene.to_string(),
        phenotype: phenotype.phenotype.to_string(),
        summary: format!(
            "CYP2D6 {} may reduce conversion of tamoxifen to its more active metabolites.",
            phenotype.phenotype
        ),
        recommendation: recommendation.to_string(),
        evidence_note: "The strongest local warning here is reduced-function CYP2D6 plus concurrent strong CYP2D6 inhibitors such as paroxetine or fluoxetine.".to_string(),
        supporting_rsids: phenotype.supporting_rsids.clone(),
    })
}

fn clobazam_result(requested: &str, phenotype: &Option<GenePhenotype>) -> Option<MedicationCheckResult> {
    let phenotype = phenotype.as_ref()?;
    let (severity, recommendation) = match phenotype.phenotype {
        "likely reduced metabolizer" => (
            "serious",
            "Reduced CYP2C19 metabolism can substantially raise active clobazam metabolite levels. Slower titration and lower dosing may be appropriate.",
        ),
        "likely intermediate metabolizer" => (
            "moderate",
            "This CYP2C19 profile may increase clobazam exposure. Monitor closely for sedation or cognitive side effects.",
        ),
        _ => (
            "informational",
            "Current local CYP2C19 markers do not suggest a reduced-metabolism warning for clobazam.",
        ),
    };

    Some(MedicationCheckResult {
        requested_medication: requested.to_string(),
        matched_medication: canonical_name(requested),
        category: "Seizures / Anxiety".to_string(),
        severity: severity.to_string(),
        gene: phenotype.gene.to_string(),
        phenotype: phenotype.phenotype.to_string(),
        summary: format!(
            "CYP2C19 {} may increase clobazam and norclobazam exposure.",
            phenotype.phenotype
        ),
        recommendation: recommendation.to_string(),
        evidence_note: "This checker only supports reduced-function CYP2C19 inference from *2/*3 markers and cannot infer increased-function *17 status.".to_string(),
        supporting_rsids: phenotype.supporting_rsids.clone(),
    })
}

fn diazepam_result(requested: &str, phenotype: &Option<GenePhenotype>) -> Option<MedicationCheckResult> {
    let phenotype = phenotype.as_ref()?;
    let (severity, recommendation) = match phenotype.phenotype {
        "likely reduced metabolizer" => (
            "moderate",
            "Reduced CYP2C19 metabolism may slow diazepam clearance, so sedation can last longer in some people.",
        ),
        "likely intermediate metabolizer" => (
            "informational",
            "There may be a modest diazepam exposure increase, but the real-world effect is often variable.",
        ),
        _ => (
            "informational",
            "No obvious local CYP2C19 warning for diazepam from the current marker set.",
        ),
    };

    Some(MedicationCheckResult {
        requested_medication: requested.to_string(),
        matched_medication: canonical_name(requested),
        category: "Seizures / Anxiety".to_string(),
        severity: severity.to_string(),
        gene: phenotype.gene.to_string(),
        phenotype: phenotype.phenotype.to_string(),
        summary: format!(
            "CYP2C19 {} may modestly change diazepam clearance.",
            phenotype.phenotype
        ),
        recommendation: recommendation.to_string(),
        evidence_note: "Diazepam evidence is weaker and more mixed than clobazam, so this local result should be treated as a lower-confidence caution.".to_string(),
        supporting_rsids: phenotype.supporting_rsids.clone(),
    })
}

fn cyp2d6_inhibitor_result(
    requested: &str,
    phenotype: &Option<GenePhenotype>,
    has_tamoxifen: bool,
) -> Option<MedicationCheckResult> {
    let phenotype = phenotype.as_ref()?;
    let normalized = normalize_name(requested);
    let strong_inhibitor = matches!(
        normalized.as_str(),
        "paroxetine" | "paxil" | "fluoxetine" | "prozac" | "bupropion" | "wellbutrin"
    );
    let (severity, recommendation) = if has_tamoxifen && strong_inhibitor {
        (
            "critical",
            "This medication is a strong CYP2D6 inhibitor and may be an especially important interaction if tamoxifen is also being used.",
        )
    } else if strong_inhibitor {
        (
            "moderate",
            "This antidepressant can strongly inhibit CYP2D6 and may interact with other CYP2D6-sensitive medications.",
        )
    } else {
        (
            "informational",
            "This medication has some CYP2D6-inhibiting potential, but the interaction relevance depends on what else is being taken.",
        )
    };

    Some(MedicationCheckResult {
        requested_medication: requested.to_string(),
        matched_medication: canonical_name(requested),
        category: "Mood / Mental Health".to_string(),
        severity: severity.to_string(),
        gene: phenotype.gene.to_string(),
        phenotype: phenotype.phenotype.to_string(),
        summary: format!(
            "{} is treated here primarily as a CYP2D6 inhibitor rather than a metabolized substrate.",
            title_case(&canonical_name(requested))
        ),
        recommendation: recommendation.to_string(),
        evidence_note: "This warning is most clinically important when another CYP2D6-sensitive medication is also involved, especially tamoxifen.".to_string(),
        supporting_rsids: phenotype.supporting_rsids.clone(),
    })
}

fn beta_agonist_result(requested: &str, phenotype: &Option<GenePhenotype>) -> Option<MedicationCheckResult> {
    let phenotype = phenotype.as_ref()?;
    let (severity, recommendation) = match phenotype.phenotype {
        "reduced short-term response" => (
            "moderate",
            "Discuss whether short-term bronchodilator response has matched expectations clinically, especially if symptom control is inconsistent.",
        ),
        "intermediate response" => (
            "informational",
            "There may be a modest response difference, but this is generally lower priority than real-world symptom control.",
        ),
        _ => (
            "informational",
            "No obvious reduced-response warning from the current ADRB2 marker.",
        ),
    };

    Some(MedicationCheckResult {
        requested_medication: requested.to_string(),
        matched_medication: canonical_name(requested),
        category: "Asthma / Breathing".to_string(),
        severity: severity.to_string(),
        gene: phenotype.gene.to_string(),
        phenotype: phenotype.phenotype.to_string(),
        summary: format!(
            "ADRB2 {} may influence bronchodilator response.",
            phenotype.phenotype
        ),
        recommendation: recommendation.to_string(),
        evidence_note: "This result reflects a single ADRB2 marker and should be interpreted alongside clinical response.".to_string(),
        supporting_rsids: phenotype.supporting_rsids.clone(),
    })
}

fn infer_cyp2c19(
    star2: Option<&&MatchedMarker>,
    star3: Option<&&MatchedMarker>,
) -> Option<GenePhenotype> {
    let star2_genotype = star2.map(|marker| marker.genotype.as_str());
    let star3_genotype = star3.map(|marker| marker.genotype.as_str());

    let supporting_rsids = [star2, star3]
        .into_iter()
        .flatten()
        .map(|marker| marker.rsid.clone())
        .collect::<Vec<_>>();

    if supporting_rsids.is_empty() {
        return None;
    }

    let variant_score = [star2_genotype, star3_genotype]
        .into_iter()
        .flatten()
        .map(|genotype| match genotype {
            "AA" => 2,
            "AG" => 1,
            _ => 0,
        })
        .sum::<u32>();

    let phenotype = if variant_score >= 2 {
        "likely reduced metabolizer"
    } else if variant_score == 1 {
        "likely intermediate metabolizer"
    } else {
        "likely normal metabolizer"
    };

    Some(GenePhenotype {
        gene: "CYP2C19",
        phenotype,
        supporting_rsids,
    })
}

fn infer_cyp2d6(marker: Option<&&MatchedMarker>) -> Option<GenePhenotype> {
    let marker = marker?;
    let phenotype = match marker.genotype.as_str() {
        "AA" => "likely poor metabolizer",
        "AG" => "likely intermediate metabolizer",
        _ => "likely normal metabolizer",
    };

    Some(GenePhenotype {
        gene: "CYP2D6",
        phenotype,
        supporting_rsids: vec![marker.rsid.clone()],
    })
}

fn infer_adrb2(marker: Option<&&MatchedMarker>) -> Option<GenePhenotype> {
    let marker = marker?;
    let phenotype = match marker.genotype.as_str() {
        "AA" => "reduced short-term response",
        "AG" => "intermediate response",
        _ => "typical response",
    };

    Some(GenePhenotype {
        gene: "ADRB2",
        phenotype,
        supporting_rsids: vec![marker.rsid.clone()],
    })
}

fn normalize_name(input: &str) -> String {
    input
        .to_ascii_lowercase()
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .collect()
}

fn canonical_name(input: &str) -> String {
    match normalize_name(input).as_str() {
        "plavix" | "iscover" => "clopidogrel".to_string(),
        "celexa" => "citalopram".to_string(),
        "lexapro" => "escitalopram".to_string(),
        "zoloft" => "sertraline".to_string(),
        "prilosec" => "omeprazole".to_string(),
        "prevacid" => "lansoprazole".to_string(),
        "protonix" => "pantoprazole".to_string(),
        "dexilant" => "dexlansoprazole".to_string(),
        "nexium" => "esomeprazole".to_string(),
        "ultram" => "tramadol".to_string(),
        "vicodin" => "hydrocodone".to_string(),
        "elavil" => "amitriptyline".to_string(),
        "anafranil" => "clomipramine".to_string(),
        "tofranil" => "imipramine".to_string(),
        "sinequan" => "doxepin".to_string(),
        "surmontil" => "trimipramine".to_string(),
        "pamelor" => "nortriptyline".to_string(),
        "norpramin" => "desipramine".to_string(),
        "lopressor" | "toprol" | "toprolxl" => "metoprolol".to_string(),
        "nolvadex" | "soltamox" => "tamoxifen".to_string(),
        "onfi" | "frisium" => "clobazam".to_string(),
        "valium" => "diazepam".to_string(),
        "paxil" => "paroxetine".to_string(),
        "prozac" => "fluoxetine".to_string(),
        "wellbutrin" => "bupropion".to_string(),
        "cymbalta" => "duloxetine".to_string(),
        "ventolin" | "salbutamol" => "albuterol".to_string(),
        other => other.to_string(),
    }
}

fn title_case(value: &str) -> String {
    value
        .split([' ', '-', '/'])
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn push_or_unmatched(
    results: &mut Vec<MedicationCheckResult>,
    unmatched: &mut Vec<String>,
    requested: &str,
    result: Option<MedicationCheckResult>,
) {
    match result {
        Some(result) => results.push(result),
        None => unmatched.push(requested.to_string()),
    }
}

fn sort_results(results: &mut [MedicationCheckResult]) {
    results.sort_by(|a, b| {
        severity_rank(&a.severity)
            .cmp(&severity_rank(&b.severity))
            .then(a.category.cmp(&b.category))
            .then(a.matched_medication.cmp(&b.matched_medication))
    });
}

fn severity_rank(value: &str) -> usize {
    match value {
        "critical" => 0,
        "serious" => 1,
        "moderate" => 2,
        _ => 3,
    }
}
