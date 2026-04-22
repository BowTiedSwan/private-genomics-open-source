import { create } from "zustand";
import type { ChatMessage, ModelInfo, ParsedGenome } from "./types";

export type View = "upload" | "analyze" | "report" | "chat" | "settings";

interface AppState {
  view: View;
  models: ModelInfo[];
  selectedModelId: string;
  teeOnly: boolean;
  hasApiKey: boolean;
  genome: ParsedGenome | null;
  genomePath: string | null;
  report: string;
  reportInProgress: boolean;
  chat: ChatMessage[];
  setView: (v: View) => void;
  setModels: (m: ModelInfo[]) => void;
  setSelectedModelId: (id: string) => void;
  setTeeOnly: (v: boolean) => void;
  setHasApiKey: (v: boolean) => void;
  setGenome: (g: ParsedGenome | null, path?: string | null) => void;
  setReport: (s: string) => void;
  appendReport: (s: string) => void;
  setReportInProgress: (v: boolean) => void;
  appendChat: (m: ChatMessage) => void;
  updateLastChat: (delta: string) => void;
}

export const useApp = create<AppState>((set) => ({
  view: "upload",
  models: [],
  selectedModelId: "kimi-k2.6",
  teeOnly: false,
  hasApiKey: false,
  genome: null,
  genomePath: null,
  report: "",
  reportInProgress: false,
  chat: [],
  setView: (view) => set({ view }),
  setModels: (models) => set({ models }),
  setSelectedModelId: (selectedModelId) => set({ selectedModelId }),
  setTeeOnly: (teeOnly) =>
    set((s) => {
      if (teeOnly) {
        const tee = s.models.find((m) => m.tee);
        return { teeOnly, selectedModelId: tee?.id ?? s.selectedModelId };
      }
      return { teeOnly };
    }),
  setHasApiKey: (hasApiKey) => set({ hasApiKey }),
  setGenome: (genome, genomePath = null) => set({ genome, genomePath }),
  setReport: (report) => set({ report }),
  appendReport: (delta) => set((s) => ({ report: s.report + delta })),
  setReportInProgress: (reportInProgress) => set({ reportInProgress }),
  appendChat: (m) => set((s) => ({ chat: [...s.chat, m] })),
  updateLastChat: (delta) =>
    set((s) => {
      const chat = [...s.chat];
      const last = chat[chat.length - 1];
      if (last && last.role === "assistant") {
        chat[chat.length - 1] = { ...last, content: last.content + delta };
      }
      return { chat };
    }),
}));
