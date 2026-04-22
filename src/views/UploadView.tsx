import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { Upload, ShieldCheck, AlertTriangle } from "lucide-react";
import { useApp } from "../store";
import { api } from "../api";

const ACCEPTED = ["txt", "csv", "tsv", "vcf", "gz"];

export default function UploadView() {
  const { hasApiKey, setGenome, setView, setReport } = useApp();
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
      const g = await api.parseGenome(picked);
      setGenome(g, picked);
      setReport("");
      setView("analyze");
    } catch (e: any) {
      setErr(String(e));
    } finally {
      setBusy(false);
    }
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
            No Morpheus API key saved yet. You can still parse your file locally, but you'll need a key to generate a report.{" "}
            <a onClick={() => setView("settings")} style={{ cursor: "pointer" }}>Open Settings →</a>
          </div>
        </div>
      )}

      <div className="dropzone" onClick={pickFile} role="button">
        <Upload size={28} />
        <h3>{busy ? "Parsing…" : "Click to select a file"}</h3>
        <small>
          Accepted: {ACCEPTED.map((e) => "." + e).join(", ")}
        </small>
      </div>

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
