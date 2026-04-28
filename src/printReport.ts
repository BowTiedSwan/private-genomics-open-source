import { save } from "@tauri-apps/plugin-dialog";
import { api } from "./api";
import type { AnalysisPackage } from "./types";

export async function exportPdfReport(
  analysisId: string,
  suggestedFileName: string,
): Promise<AnalysisPackage | null> {
  const outputPath = await save({
    title: "Export PDF report",
    defaultPath: suggestedFileName,
    filters: [{ name: "PDF document", extensions: ["pdf"] }],
  });

  if (!outputPath) {
    return null;
  }

  return api.exportPdf(analysisId, outputPath);
}
