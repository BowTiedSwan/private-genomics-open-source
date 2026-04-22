use crate::error::AppResult;
use crate::genomics::ParsedGenome;
use crate::morpheus::{chat_stream, ChatMessage, ChatRequest};
use tauri::AppHandle;

const HERMES_SYSTEM: &str = r#"You are Hermes, a careful, evidence-grounded genomics analyst.
You have been given a structured summary of a user's consumer-grade DNA data (SNP array) plus
curated literature-backed marker interpretations. Your job is to produce a clear, trustworthy,
personal genomic report.

Hard rules:
- Never invent markers or studies. Only reason about markers present in the provided summary.
- Distinguish strong (GWAS-replicated) vs weaker (single-study) associations.
- Do not provide diagnoses. Frame findings as probabilistic tendencies, not medical advice.
- For every claim, anchor to the rsID the user actually has and the matched genotype.
- Use calm, second-person voice. No hype, no scare language, no emojis.
- If the underlying call rate is low (<97%), open with a data-quality caveat.

Output format: Markdown, with these sections (use `## ` headings, exactly these titles):
## Overview
## Data Quality
## Pharmacogenomics
## Metabolic & Cardiovascular
## Neuropsychiatric & Cognitive
## Traits
## Lifestyle Recommendations
## Caveats & Next Steps
"#;

pub fn build_genome_context(g: &ParsedGenome) -> String {
    let mut markers_json = serde_json::to_string_pretty(&g.matched_markers)
        .unwrap_or_else(|_| "[]".to_string());
    if markers_json.len() > 18_000 {
        markers_json.truncate(18_000);
        markers_json.push_str("\n... (truncated)");
    }
    format!(
        "Format: {:?}\nTotal SNPs: {}\nAutosomal: {}\nX: {}  Y: {}  MT: {}\nNo-calls: {}\nCall rate: {:.4}\nSex inference: {}\n\nMatched curated markers (JSON):\n{}\n",
        g.format,
        g.total_snps,
        g.autosomal,
        g.x_chromosome,
        g.y_chromosome,
        g.mitochondrial,
        g.no_calls,
        g.call_rate,
        g.sex_inference,
        markers_json,
    )
}

pub async fn generate_report(
    app: &AppHandle,
    api_key: &str,
    model: &str,
    genome: &ParsedGenome,
    event: &str,
) -> AppResult<String> {
    let context = build_genome_context(genome);
    let user_prompt = format!(
        "Here is the user's parsed genome data. Produce the full report now.\n\n---\n{}\n---",
        context
    );
    let messages = vec![
        ChatMessage { role: "system".into(), content: HERMES_SYSTEM.into() },
        ChatMessage { role: "user".into(), content: user_prompt },
    ];
    let req = ChatRequest {
        model: model.to_string(),
        messages,
        temperature: 0.4,
        max_tokens: Some(4096),
        stream: true,
    };
    chat_stream(app, event, api_key, &req).await
}

pub async fn freeform_chat(
    app: &AppHandle,
    api_key: &str,
    model: &str,
    genome: Option<&ParsedGenome>,
    history: Vec<ChatMessage>,
    user_message: String,
    event: &str,
) -> AppResult<String> {
    let mut messages: Vec<ChatMessage> = Vec::with_capacity(history.len() + 3);
    let mut system = HERMES_SYSTEM.to_string();
    if let Some(g) = genome {
        system.push_str("\n\nThe user's genome context:\n");
        system.push_str(&build_genome_context(g));
    }
    messages.push(ChatMessage { role: "system".into(), content: system });
    messages.extend(history);
    messages.push(ChatMessage { role: "user".into(), content: user_message });

    let req = ChatRequest {
        model: model.to_string(),
        messages,
        temperature: 0.5,
        max_tokens: Some(2048),
        stream: true,
    };
    chat_stream(app, event, api_key, &req).await
}
