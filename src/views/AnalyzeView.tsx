import { useState } from "react";
import { AlertTriangle, Play } from "lucide-react";
import { useApp } from "../store";
import { api } from "../api";

export default function AnalyzeView() {
  const {
    genome,
    models,
    selectedModelId,
    teeOnly,
    hasApiKey,
    setView,
    setReport,
    setReportInProgress,
    reportInProgress,
  } = useApp();
  const [err, setErr] = useState<string | null>(null);

  if (!genome) {
    return (
      <div className="empty">
        <h3>No genome loaded</h3>
        <p>Upload a file first to see your analysis.</p>
      </div>
    );
  }

  const callRatePct = (genome.call_rate * 100).toFixed(2);
  const lowQuality = genome.call_rate < 0.97;
  const activeModel = models.find((m) => m.id === selectedModelId);

  async function generate() {
    setErr(null);
    if (!hasApiKey) {
      setErr("Save a Morpheus API key in Settings first.");
      return;
    }
    setReport("");
    setReportInProgress(true);
    setView("report");
    try {
      await api.generateReport(genome!, selectedModelId);
    } catch (e: any) {
      setErr(String(e));
      setReportInProgress(false);
    }
  }

  return (
    <>
      <h1 className="page-title">Analysis</h1>
      <p className="page-sub">
        Parsed locally. Review the numbers, then generate your full report.
      </p>

      {lowQuality && (
        <div className="banner banner-warn">
          <AlertTriangle size={16} />
          <div>
            Call rate is {callRatePct}% — below the 97% threshold. Results may be less reliable.
          </div>
        </div>
      )}

      <div className="card">
        <h3 className="card-title">File summary</h3>
        <p className="card-sub">
          Format detected: <strong>{prettyFormat(genome.format)}</strong>
          {" · "}Sex inference: <strong>{genome.sex_inference}</strong>
        </p>
        <div className="stat-grid">
          <Stat label="Total SNPs" value={genome.total_snps.toLocaleString()} />
          <Stat label="Autosomal" value={genome.autosomal.toLocaleString()} />
          <Stat label="X / Y / MT" value={`${genome.x_chromosome} · ${genome.y_chromosome} · ${genome.mitochondrial}`} />
          <Stat label="Call rate" value={`${callRatePct}%`} />
        </div>
      </div>

      <div className="card">
        <h3 className="card-title">Matched curated markers</h3>
        <p className="card-sub">
          {genome.matched_markers.length} of your SNPs hit the local literature-backed marker set.
          These are what the agent will reason over.
        </p>
        {genome.matched_markers.length > 0 ? (
          <table className="markers-table">
            <thead>
              <tr>
                <th>rsID</th>
                <th>Genotype</th>
                <th>Category</th>
                <th>Trait</th>
                <th>Interpretation</th>
              </tr>
            </thead>
            <tbody>
              {genome.matched_markers.map((m) => (
                <tr key={m.rsid}>
                  <td><code>{m.rsid}</code></td>
                  <td><code>{m.genotype}</code></td>
                  <td>{m.category}</td>
                  <td>{m.trait_name}</td>
                  <td style={{ color: "var(--text-dim)" }}>{m.interpretation}</td>
                </tr>
              ))}
            </tbody>
          </table>
        ) : (
          <p style={{ color: "var(--text-mute)" }}>No curated markers matched. The report will still run but with limited content.</p>
        )}
      </div>

      <div className="card">
        <h3 className="card-title">Generate report</h3>
        <p className="card-sub">
          Using <strong>{activeModel?.name ?? selectedModelId}</strong>
          {teeOnly && <span className="pill pill-tee" style={{ marginLeft: 8 }}>TEE only</span>}
          . Change the active model in Settings.
        </p>
        {err && (
          <div className="banner banner-warn" style={{ marginBottom: 12 }}>
            <AlertTriangle size={16} />
            <div>{err}</div>
          </div>
        )}
        <button className="btn btn-primary" onClick={generate} disabled={reportInProgress}>
          <Play size={14} />
          {reportInProgress ? "Generating…" : "Generate Hermes report"}
        </button>
      </div>
    </>
  );
}

function Stat({ label, value }: { label: string; value: string }) {
  return (
    <div className="stat">
      <div className="stat-label">{label}</div>
      <div className="stat-value">{value}</div>
    </div>
  );
}

function prettyFormat(f: string): string {
  switch (f) {
    case "twenty_three_and_me": return "23andMe";
    case "ancestry_dna": return "AncestryDNA";
    case "my_heritage": return "MyHeritage";
    case "family_tree_dna": return "FamilyTreeDNA";
    case "vcf": return "VCF";
    case "rsid_tsv": return "rsID TSV";
    default: return "Unknown";
  }
}
