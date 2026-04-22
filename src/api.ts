import { invoke } from "@tauri-apps/api/core";
import type { ChatMessage, ModelInfo, ParsedGenome } from "./types";

export const api = {
  listModels: () => invoke<ModelInfo[]>("list_models"),
  parseGenome: (path: string) => invoke<ParsedGenome>("parse_genome", { path }),
  saveApiKey: (key: string) => invoke<void>("save_api_key", { key }),
  hasApiKey: () => invoke<boolean>("has_api_key"),
  clearApiKey: () => invoke<void>("clear_api_key"),
  generateReport: (genome: ParsedGenome, model: string) =>
    invoke<string>("generate_report", { genome, model }),
  chat: (args: {
    model: string;
    genome: ParsedGenome | null;
    history: ChatMessage[];
    message: string;
  }) => invoke<string>("chat", args),
};
