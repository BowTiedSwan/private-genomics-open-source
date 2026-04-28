export type GenomicFormat =
  | "twenty_three_and_me"
  | "ancestry_dna"
  | "my_heritage"
  | "family_tree_dna"
  | "vcf"
  | "rsid_tsv"
  | "unknown";

export interface Snp {
  rsid: string;
  chromosome: string;
  position: number;
  genotype: string;
}

export interface MatchedMarker {
  rsid: string;
  genotype: string;
  category: string;
  trait_name: string;
  interpretation: string;
  confidence: string;
}

export interface ParsedGenome {
  format: GenomicFormat;
  total_snps: number;
  autosomal: number;
  x_chromosome: number;
  y_chromosome: number;
  mitochondrial: number;
  no_calls: number;
  call_rate: number;
  chromosomes: Record<string, number>;
  sample_snps: Snp[];
  matched_markers: MatchedMarker[];
  sex_inference: string;
}

export interface AnalysisSource {
  original_path: string;
  file_name: string;
  file_size_bytes: number;
}

export interface AnalysisArtifacts {
  root_dir: string;
  package_json_path: string;
  report_markdown_path: string;
  exports_dir: string;
  counselor_export_json_path: string | null;
  apple_health_export_json_path: string | null;
  api_export_json_path: string | null;
  api_export_full_json_path: string | null;
  integration_hooks_json_path: string | null;
  exported_pdf_path: string | null;
}

export interface AnalysisReport {
  markdown: string;
  model_id: string | null;
  generated_at_unix_ms: number | null;
}

export interface AnalysisFamilyCounts {
  pharmacogenomics: number;
  metabolic_cardiovascular: number;
  traits: number;
  neuropsychiatric_cognitive: number;
  other: number;
}

export interface AnalysisSummary {
  matched_marker_count: number;
  actionable_finding_count: number;
  high_confidence_finding_count: number;
  family_counts: AnalysisFamilyCounts;
}

export interface AnalysisQuality {
  format_label: string;
  sex_inference: string;
  total_snps: number;
  no_calls: number;
  call_rate: number;
  call_rate_percent: number;
  matched_marker_count: number;
  matched_marker_rate: number;
  quality_tier: string;
  caveats: string[];
}

export interface StructuredFinding {
  rsid: string;
  genotype: string;
  source_category: string;
  family: string;
  trait_name: string;
  interpretation: string;
  confidence: string;
  evidence_level: string;
  significance: string;
  actionability: string;
  summary: string;
}

export interface FindingGroups {
  pharmacogenomics: StructuredFinding[];
  metabolic_cardiovascular: StructuredFinding[];
  traits: StructuredFinding[];
  neuropsychiatric_cognitive: StructuredFinding[];
  other: StructuredFinding[];
}

export interface AnalysisRecommendations {
  priority_actions: string[];
  clinician_discussion_topics: string[];
  lifestyle_focus: string[];
  informational_notes: string[];
}

export interface AnalysisProvenance {
  analysis_engine: string;
  marker_panel_name: string;
  marker_panel_version: string;
  marker_panel_size: number;
  interpretation_method: string;
  derived_locally: boolean;
  raw_genotype_sent_off_device: boolean;
}

export interface AnalysisResults {
  summary: AnalysisSummary;
  quality: AnalysisQuality;
  finding_groups: FindingGroups;
  recommendations: AnalysisRecommendations;
  provenance: AnalysisProvenance;
}

export interface AnalysisPackage {
  id: string;
  schema_version: string;
  created_at_unix_ms: number;
  updated_at_unix_ms: number;
  source: AnalysisSource;
  genome: ParsedGenome;
  results: AnalysisResults;
  report: AnalysisReport;
  artifacts: AnalysisArtifacts;
}

export interface MedicationCheckResult {
  requested_medication: string;
  matched_medication: string;
  category: string;
  severity: string;
  gene: string;
  phenotype: string;
  summary: string;
  recommendation: string;
  evidence_note: string;
  supporting_rsids: string[];
}

export interface MedicationCheckResponse {
  requested_count: number;
  matched_count: number;
  unmatched_medications: string[];
  results: MedicationCheckResult[];
}

export interface ModelInfo {
  id: string;
  name: string;
  tee: boolean;
  web: boolean;
  description: string;
}

export interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}
