import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { Upload, FileText, MessageSquare, Settings, Loader2, Dna } from "lucide-react";
import { useApp } from "./store";
import { api } from "./api";
import UploadView from "./views/UploadView";
import AnalyzeView from "./views/AnalyzeView";
import ReportView from "./views/ReportView";
import ChatView from "./views/ChatView";
import SettingsView from "./views/SettingsView";
import "./App.css";

export default function App() {
  const {
    view,
    setView,
    setModels,
    setHasApiKey,
    appendReport,
    setReportInProgress,
    updateLastChat,
    genome,
    reportInProgress,
  } = useApp();

  useEffect(() => {
    api.listModels().then(setModels);
    api.hasApiKey().then(setHasApiKey);
    const unsubReport = listen<{ delta: string; done: boolean }>("report-token", (e) => {
      if (e.payload.delta) appendReport(e.payload.delta);
      if (e.payload.done) setReportInProgress(false);
    });
    const unsubChat = listen<{ delta: string; done: boolean }>("chat-token", (e) => {
      if (e.payload.delta) updateLastChat(e.payload.delta);
    });
    return () => {
      unsubReport.then((f) => f());
      unsubChat.then((f) => f());
    };
  }, []);

  return (
    <div className="app">
      <div className="titlebar-drag" />
      <aside className="sidebar">
        <div className="brand">
          <div className="brand-logo" />
          <div>
            <div className="brand-name">Personal Genomics</div>
            <div className="brand-sub">Hermes · Morpheus</div>
          </div>
        </div>

        <NavItem icon={<Upload />} active={view === "upload"} onClick={() => setView("upload")}>
          Upload
        </NavItem>
        <NavItem
          icon={reportInProgress ? <Loader2 className="spin" /> : <Dna />}
          active={view === "analyze"}
          onClick={() => setView("analyze")}
          disabled={!genome}
        >
          Analysis
        </NavItem>
        <NavItem
          icon={<FileText />}
          active={view === "report"}
          onClick={() => setView("report")}
          disabled={!genome}
        >
          Report
        </NavItem>
        <NavItem
          icon={<MessageSquare />}
          active={view === "chat"}
          onClick={() => setView("chat")}
          disabled={!genome}
        >
          Ask Hermes
        </NavItem>

        <div className="spacer" />

        <NavItem icon={<Settings />} active={view === "settings"} onClick={() => setView("settings")}>
          Settings
        </NavItem>

        <div className="sidebar-footer">
          Private by design. Your raw DNA never leaves your device unless you choose to send
          redacted context to the inference endpoint.
        </div>
      </aside>

      <main className="main">
        <div className="main-inner">
          {view === "upload" && <UploadView />}
          {view === "analyze" && <AnalyzeView />}
          {view === "report" && <ReportView />}
          {view === "chat" && <ChatView />}
          {view === "settings" && <SettingsView />}
        </div>
      </main>
    </div>
  );
}

function NavItem({
  icon,
  active,
  children,
  onClick,
  disabled,
}: {
  icon: React.ReactNode;
  active?: boolean;
  children: React.ReactNode;
  onClick?: () => void;
  disabled?: boolean;
}) {
  return (
    <div
      className={"nav-item " + (active ? "active" : "")}
      onClick={disabled ? undefined : onClick}
      style={disabled ? { opacity: 0.4, cursor: "not-allowed" } : undefined}
    >
      {icon}
      <span>{children}</span>
    </div>
  );
}
