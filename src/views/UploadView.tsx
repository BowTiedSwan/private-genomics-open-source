import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { Upload, ShieldCheck, AlertTriangle, FileText, Trash2 } from "lucide-react";
import { useApp } from "../store";
import { api } from "../api";

const ACCEPTED = ["txt", "csv", "tsv", "vcf", "gz"];

export default function UploadView() {
  const { hasApiKey, analysis, setAnalysis, setView, setReport, setReportError, clearLoadedData } = useApp();
  const [busy, setBusy] = useState(false);
  const [err, setErr] = useState<string | null>(null);

  async function pickFile() {
    setErr(null);
    const picked = await open({
      multiple: false,
      directory: false,
      filters: [
        { name: "Raw genomic data", extensions: ACCEPTED },
        { name: "All files", extensions: ["*"] },
      ],
    });
    if (!picked || typeof picked !== "string") return;
    setBusy(true);
    try {
      const createdAnalysis = await api.createAnalysis(picked);
      setAnalysis(createdAnalysis);
      setReport("");
      setReportError(null);
      setView("analyze");
    } catch (e: any) {
      setErr(String(e));
    } finally {
      setBusy(false);
    }
  }

  const fileName = analysis?.source.file_name ?? null;

  function removeLoadedFile() {
    clearLoadedData();
    setAnalysis(null);
    setReport("");
    setReportError(null);
    setErr(null);
  }

  return (
    <>
      <h1 className="page-title">Upload your raw DNA</h1>
      <p className="page-sub">
        Pick a raw genotype export from 23andMe, AncestryDNA, MyHeritage, FamilyTreeDNA, or any{" "}
        <code>.vcf</code> (gzipped or plain). Files are parsed locally — nothing is uploaded yet.
      </p>

      {!hasApiKey && (
        <div className="banner banner-warn">
          <AlertTriangle size={16} />
          <div>
            A Morpheus API key is required to generate a report. Startup no longer probes macOS Keychain automatically, so if you already saved a key you can try generating a report directly or manage it in Settings.{" "}
            <button className="btn btn-ghost" onClick={() => setView("settings")} style={{ padding: 0, height: "auto", display: "inline" }}>Open Settings →</button>
          </div>
        </div>
      )}

      {analysis && fileName ? (
        <div className="card uploaded-file-card">
          <div className="row uploaded-file-row">
            <div className="uploaded-file-meta">
              <div className="uploaded-file-icon">
                <FileText size={18} />
              </div>
              <div>
                <h3 className="card-title" style={{ marginBottom: 2 }}>Uploaded raw DNA file</h3>
                <p className="card-sub" style={{ marginBottom: 6 }}>{fileName}</p>
                <div className="helper" style={{ marginTop: 0 }}>
                  Parsed locally and ready for analysis.
                </div>
              </div>
            </div>
            <div className="row uploaded-file-actions">
              <button className="btn" onClick={() => setView("analyze")}>View analysis</button>
              <button className="btn btn-ghost" onClick={removeLoadedFile}>
                <Trash2 size={14} /> Remove file
              </button>
            </div>
          </div>
        </div>
      ) : (
        <div
          className="dropzone"
          onClick={pickFile}
          onKeyDown={(e) => {
            if (e.key === "Enter" || e.key === " ") {
              e.preventDefault();
              pickFile();
            }
          }}
          role="button"
          tabIndex={0}
        >
          <Upload size={28} />
          <h3>{busy ? "Parsing…" : "Click to select a file"}</h3>
          <small>
            Accepted: {ACCEPTED.map((e) => "." + e).join(", ")}
          </small>
        </div>
      )}

      {err && (
        <div className="banner banner-warn" style={{ marginTop: 16 }}>
          <AlertTriangle size={16} />
          <div>{err}</div>
        </div>
      )}

      <div className="card" style={{ marginTop: 24 }}>
        <div className="row" style={{ alignItems: "flex-start", gap: 12 }}>
          <ShieldCheck size={18} style={{ color: "var(--success)", flexShrink: 0, marginTop: 2 }} />
          <div>
            <h3 className="card-title">How privacy works</h3>
            <p className="card-sub" style={{ marginBottom: 0 }}>
              Parsing and marker matching happens entirely on your Mac. Only a compact, de-identified
              summary (counts, matched rsIDs with interpretations — never raw SNP lists) is sent to
              the inference model you select. Toggle{" "}
              <strong>TEE-only mode</strong> in Settings to restrict inference to attested trusted-execution
              providers on Morpheus.
            </p>
          </div>
        </div>
      </div>
    </>
  );
}
