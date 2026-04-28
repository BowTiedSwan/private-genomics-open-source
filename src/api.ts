import { invoke } from "@tauri-apps/api/core";
import type { AnalysisPackage, ChatMessage, MedicationCheckResponse, ModelInfo, ParsedGenome } from "./types";

export const api = {
  listModels: () => invoke<ModelInfo[]>("list_models"),
  createAnalysis: (path: string) => invoke<AnalysisPackage>("create_analysis", { path }),
  loadAnalysis: (analysisId: string) => invoke<AnalysisPackage>("load_analysis", { analysisId }),
  saveApiKey: (key: string) => invoke<void>("save_api_key", { key }),
  hasApiKey: () => invoke<boolean>("has_api_key"),
  clearApiKey: () => invoke<void>("clear_api_key"),
  generateReport: (analysisId: string, model: string) =>
    invoke<AnalysisPackage>("generate_report", { analysisId, model }),
  exportAnalysisFormats: (analysisId: string) =>
    invoke<AnalysisPackage>("export_analysis_formats", { analysisId }),
  exportPdf: (analysisId: string, outputPath: string) =>
    invoke<AnalysisPackage>("export_pdf", { analysisId, outputPath }),
  checkMedicationInteractions: (analysisId: string, medications: string[]) =>
    invoke<MedicationCheckResponse>("check_medication_interactions", { analysisId, medications }),
  explainMarker: (args: {
    model: string;
    genome: ParsedGenome | null;
    finding: string;
  }) => invoke<string>("explain_marker", args),
  chat: (args: {
    model: string;
    genome: ParsedGenome | null;
    history: ChatMessage[];
    message: string;
  }) => invoke<string>("chat", args),
};
