import { useState } from "react";
import { AlertTriangle, Download, Play, Printer } from "lucide-react";
import { useApp } from "../store";
import { api } from "../api";
import type { MedicationCheckResponse, StructuredFinding } from "../types";
import { exportPdfReport } from "../printReport";

export default function AnalyzeView() {
  const {
    analysis,
    models,
    selectedModelId,
    teeOnly,
    hasApiKey,
    setView,
    setAnalysis,
    setReport,
    setReportError,
    setReportInProgress,
    reportInProgress,
  } = useApp();
  const [err, setErr] = useState<string | null>(null);
  const [exportErr, setExportErr] = useState<string | null>(null);
  const [exportBusy, setExportBusy] = useState(false);
  const [pdfErr, setPdfErr] = useState<string | null>(null);
  const [pdfBusy, setPdfBusy] = useState(false);
  const [medicationInput, setMedicationInput] = useState("");
  const [medicationBusy, setMedicationBusy] = useState(false);
  const [medicationErr, setMedicationErr] = useState<string | null>(null);
  const [medicationResults, setMedicationResults] = useState<MedicationCheckResponse | null>(null);

  if (!analysis) {
    return (
      <div className="empty">
        <h3>No genome loaded</h3>
        <p>Upload a file first to see your analysis.</p>
      </div>
    );
  }

  const currentAnalysis = analysis;
  const genome = currentAnalysis.genome;
  const results = currentAnalysis.results;

  const callRatePct = results.quality.call_rate_percent.toFixed(2);
  const lowQuality = results.quality.call_rate < 0.97;
  const activeModel = models.find((m) => m.id === selectedModelId);
  const findingSections = [
    ["Pharmacogenomics", results.finding_groups.pharmacogenomics],
    ["Metabolic & Cardiovascular", results.finding_groups.metabolic_cardiovascular],
    ["Neuropsychiatric & Cognitive", results.finding_groups.neuropsychiatric_cognitive],
    ["Traits", results.finding_groups.traits],
  ] as const;
  const exportArtifacts = [
    ["Clinical export", currentAnalysis.artifacts.counselor_export_json_path],
    ["Apple Health export", currentAnalysis.artifacts.apple_health_export_json_path],
    ["API export", currentAnalysis.artifacts.api_export_json_path],
    ["API export (full)", currentAnalysis.artifacts.api_export_full_json_path],
    ["Integration hooks", currentAnalysis.artifacts.integration_hooks_json_path],
  ] as const;
  const groupedMedicationResults = medicationResults ? groupMedicationResults(medicationResults) : [];

  async function generate() {
    setErr(null);
    if (!hasApiKey) {
      setErr("Save a Morpheus API key in Settings first.");
      return;
    }
    setReport("");
    setReportError(null);
    setReportInProgress(true);
    setView("report");
    try {
      const updatedAnalysis = await api.generateReport(currentAnalysis.id, selectedModelId);
      if (updatedAnalysis.report.markdown.trim()) {
        setAnalysis(updatedAnalysis);
        setReport(updatedAnalysis.report.markdown);
      } else {
        setReportError(
          "Report generation finished without content. Check your API key, selected model, and network response, then try again."
        );
      }
    } catch (e: any) {
      setErr(String(e));
      setReportError(String(e));
    } finally {
      setReportInProgress(false);
    }
  }

  async function exportAll() {
    setExportErr(null);
    setExportBusy(true);
    try {
      const updatedAnalysis = await api.exportAnalysisFormats(currentAnalysis.id);
      setAnalysis(updatedAnalysis);
    } catch (e: any) {
      setExportErr(String(e));
    } finally {
      setExportBusy(false);
    }
  }

  async function exportPdf() {
    setPdfErr(null);
    setPdfBusy(true);
    try {
      const updatedAnalysis = await exportPdfReport(
        currentAnalysis.id,
        defaultPdfFileName(currentAnalysis.source.file_name),
      );
      if (updatedAnalysis) {
        setAnalysis(updatedAnalysis);
      }
    } catch (e: any) {
      setPdfErr(String(e));
    } finally {
      setPdfBusy(false);
    }
  }

  async function checkMedications() {
    const medications = medicationInput
      .split(/[\n,]/)
      .map((item) => item.trim())
      .filter(Boolean);

    if (medications.length === 0) {
      setMedicationErr("Enter one or more medication names separated by commas or new lines.");
      setMedicationResults(null);
      return;
    }

    setMedicationErr(null);
    setMedicationBusy(true);
    try {
      const result = await api.checkMedicationInteractions(currentAnalysis.id, medications);
      setMedicationResults(result);
    } catch (e: any) {
      setMedicationErr(String(e));
    } finally {
      setMedicationBusy(false);
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
          Format detected: <strong>{results.quality.format_label}</strong>
          {" · "}Sex inference: <strong>{results.quality.sex_inference}</strong>
        </p>
        <div className="stat-grid">
          <Stat label="Total SNPs" value={results.quality.total_snps.toLocaleString()} />
          <Stat label="Autosomal" value={genome.autosomal.toLocaleString()} />
          <Stat label="X / Y / MT" value={`${genome.x_chromosome} · ${genome.y_chromosome} · ${genome.mitochondrial}`} />
          <Stat label="Call rate" value={`${callRatePct}%`} />
        </div>
        {results.quality.caveats.length > 0 && (
          <div style={{ marginTop: 16 }}>
            {results.quality.caveats.map((caveat) => (
              <div key={caveat} className="helper" style={{ marginTop: 8 }}>
                • {caveat}
              </div>
            ))}
          </div>
        )}
      </div>

      <div className="card">
        <h3 className="card-title">Structured local analysis</h3>
        <p className="card-sub">
          Deterministic local findings grouped into the same families the report will summarize.
        </p>
        <div className="stat-grid" style={{ marginBottom: 16 }}>
          <Stat label="Matched markers" value={results.summary.matched_marker_count.toString()} />
          <Stat label="Actionable findings" value={results.summary.actionable_finding_count.toString()} />
          <Stat label="High-confidence" value={results.summary.high_confidence_finding_count.toString()} />
          <Stat label="Quality tier" value={titleCase(results.quality.quality_tier)} />
        </div>

        <div className="stat-grid" style={{ marginBottom: 16 }}>
          <Stat label="Pharmacogenomics" value={results.summary.family_counts.pharmacogenomics.toString()} />
          <Stat label="Metabolic/Cardio" value={results.summary.family_counts.metabolic_cardiovascular.toString()} />
          <Stat label="Traits" value={results.summary.family_counts.traits.toString()} />
          <Stat label="Neuro/Cognitive" value={results.summary.family_counts.neuropsychiatric_cognitive.toString()} />
        </div>

        {findingSections.map(([title, findings]) =>
          findings.length > 0 ? <FindingTable key={title} title={title} findings={findings} /> : null
        )}

        {results.summary.matched_marker_count === 0 && (
          <p style={{ color: "var(--text-mute)" }}>
            No curated markers matched. The report will still run but with limited content.
          </p>
        )}
      </div>

      <div className="card">
        <h3 className="card-title">Deterministic recommendations</h3>
        <p className="card-sub">
          These are generated locally from the structured findings before any LLM summarization.
        </p>
        <RecommendationList title="Priority actions" items={results.recommendations.priority_actions} />
        <RecommendationList
          title="Clinician discussion topics"
          items={results.recommendations.clinician_discussion_topics}
        />
        <RecommendationList title="Lifestyle focus" items={results.recommendations.lifestyle_focus} />
        <RecommendationList
          title="Informational notes"
          items={results.recommendations.informational_notes}
        />
      </div>

      <div className="card">
        <h3 className="card-title">Medication interaction checker</h3>
        <p className="card-sub">
          Local pharmacogenomics checker for common medication names using the saved analysis package. Start with examples like clopidogrel, sertraline, codeine, tramadol, metoprolol, or albuterol.
        </p>
        {medicationErr && (
          <div className="banner banner-warn" style={{ marginBottom: 12 }}>
            <AlertTriangle size={16} />
            <div>{medicationErr}</div>
          </div>
        )}
        <label className="label">Medication names</label>
        <textarea
          className="input"
          rows={4}
          placeholder="clopidogrel, sertraline, codeine"
          value={medicationInput}
          onChange={(e) => setMedicationInput(e.target.value)}
          style={{ resize: "vertical", minHeight: 96 }}
        />
        <div className="row" style={{ marginTop: 12, marginBottom: 12 }}>
          <button className="btn" onClick={() => void checkMedications()} disabled={medicationBusy}>
            {medicationBusy ? "Checking…" : "Check medications"}
          </button>
        </div>
        {medicationResults && (
          <>
            <div className="helper" style={{ marginBottom: 12 }}>
              Requested {medicationResults.requested_count} medication(s); matched {medicationResults.matched_count} to local pharmacogenomic rules.
            </div>
            {groupedMedicationResults.map((severityGroup) => (
              <div key={severityGroup.severity} style={{ marginBottom: 16 }}>
                <div className="row" style={{ alignItems: "center", gap: 10, marginBottom: 10 }}>
                  <h4 style={{ margin: 0 }}>{titleCase(severityGroup.severity)}</h4>
                  <span className="pill" style={pillStyleForSeverity(severityGroup.severity)}>
                    {severityGroup.results.length}
                  </span>
                </div>
                {severityGroup.categories.map((categoryGroup) => (
                  <div key={`${severityGroup.severity}-${categoryGroup.category}`} style={{ marginBottom: 12 }}>
                    <div className="helper" style={{ marginBottom: 8 }}>
                      <strong>{categoryGroup.category}</strong>
                    </div>
                    <div style={{ display: "grid", gap: 12 }}>
                      {categoryGroup.results.map((result) => (
                        <div key={`${result.requested_medication}-${result.gene}`} className="stat" style={{ padding: 16 }}>
                          <div className="row" style={{ alignItems: "flex-start", gap: 12, marginBottom: 8 }}>
                            <div>
                              <div className="card-title" style={{ marginBottom: 4 }}>
                                {titleCase(result.matched_medication)}
                              </div>
                              <div className="helper">
                                {result.gene} · {titleCase(result.phenotype)} · requested as “{result.requested_medication}”
                              </div>
                            </div>
                            <div className="spacer" />
                            <span className="pill" style={pillStyleForSeverity(result.severity)}>
                              {titleCase(result.severity)}
                            </span>
                          </div>
                          <div style={{ color: "var(--text)", lineHeight: 1.6, marginBottom: 8 }}>{result.summary}</div>
                          <div style={{ color: "var(--text-dim)", lineHeight: 1.6, marginBottom: 8 }}>{result.recommendation}</div>
                          <div className="helper">
                            Supporting rsIDs: {result.supporting_rsids.join(", ") || "N/A"} · {result.evidence_note}
                          </div>
                        </div>
                      ))}
                    </div>
                  </div>
                ))}
              </div>
            ))}
            {medicationResults.unmatched_medications.length > 0 && (
              <div className="helper">
                No local interaction rules yet for: {medicationResults.unmatched_medications.join(", ")}
              </div>
            )}
          </>
        )}
      </div>

      <div className="card">
        <h3 className="card-title">Export analysis package</h3>
        <p className="card-sub">
          Generate the Stage 3 JSON exports from the structured local analysis contract.
        </p>
        {exportErr && (
          <div className="banner banner-warn" style={{ marginBottom: 12 }}>
            <AlertTriangle size={16} />
            <div>{exportErr}</div>
          </div>
        )}
        <div className="row" style={{ marginBottom: 12 }}>
          <button className="btn" onClick={exportAll} disabled={exportBusy}>
            <Download size={14} /> {exportBusy ? "Exporting…" : "Export all JSON formats"}
          </button>
        </div>
        <div className="helper" style={{ marginBottom: 12 }}>
          Export directory: <code>{currentAnalysis.artifacts.exports_dir}</code>
        </div>
        {exportArtifacts.some(([, path]) => Boolean(path)) && (
          <ul style={{ margin: 0, paddingLeft: 20, color: "var(--text-dim)" }}>
            {exportArtifacts.map(([label, path]) =>
              path ? (
                <li key={label} style={{ marginBottom: 6 }}>
                  <strong>{label}:</strong> <code>{path}</code>
                </li>
              ) : null
            )}
          </ul>
        )}
      </div>

      <div className="card">
        <h3 className="card-title">Export PDF</h3>
        <p className="card-sub">
          Export a real PDF file to a location you choose.
        </p>
        {pdfErr && (
          <div className="banner banner-warn" style={{ marginBottom: 12 }}>
            <AlertTriangle size={16} />
            <div>{pdfErr}</div>
          </div>
        )}
        <div className="row" style={{ marginBottom: 12 }}>
          <button className="btn btn-primary" onClick={() => void exportPdf()} disabled={pdfBusy}>
            <Printer size={14} /> {pdfBusy ? "Preparing PDF…" : "Export PDF"}
          </button>
        </div>
        <div className="helper">
          Last exported PDF: <code>{currentAnalysis.artifacts.exported_pdf_path ?? "Not exported yet"}</code>
        </div>
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
          {reportInProgress ? "Generating…" : "Generate DNA report"}
        </button>
      </div>
    </>
  );
}

function FindingTable({ title, findings }: { title: string; findings: StructuredFinding[] }) {
  return (
    <div style={{ marginBottom: 20 }}>
      <h4 style={{ marginBottom: 10 }}>{title}</h4>
      <table className="markers-table">
        <thead>
          <tr>
            <th>rsID</th>
            <th>Genotype</th>
            <th>Trait</th>
            <th>Signal</th>
            <th>Actionability</th>
          </tr>
        </thead>
        <tbody>
          {findings.map((finding) => (
            <tr key={`${finding.family}-${finding.rsid}`}>
              <td><code>{finding.rsid}</code></td>
              <td><code>{finding.genotype}</code></td>
              <td>
                <div>{finding.trait_name}</div>
                <div style={{ color: "var(--text-mute)", fontSize: 12 }}>{finding.source_category}</div>
              </td>
              <td style={{ color: "var(--text-dim)" }}>{finding.interpretation}</td>
              <td>
                <span className="pill" style={pillStyleForActionability(finding.actionability)}>
                  {titleCase(finding.actionability)}
                </span>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

function RecommendationList({ title, items }: { title: string; items: string[] }) {
  if (items.length === 0) return null;

  return (
    <div style={{ marginBottom: 16 }}>
      <h4 style={{ marginBottom: 8 }}>{title}</h4>
      <ul style={{ margin: 0, paddingLeft: 20, color: "var(--text-dim)" }}>
        {items.map((item) => (
          <li key={item} style={{ marginBottom: 6 }}>
            {item}
          </li>
        ))}
      </ul>
    </div>
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

function pillStyleForActionability(actionability: string): React.CSSProperties {
  switch (actionability) {
    case "high":
      return { background: "rgba(229, 62, 62, 0.15)", color: "#feb2b2" };
    case "medium":
      return { background: "rgba(236, 201, 75, 0.14)", color: "#f6e05e" };
    default:
      return { background: "rgba(148, 163, 184, 0.16)", color: "var(--text-mute)" };
  }
}

function pillStyleForSeverity(severity: string): React.CSSProperties {
  switch (severity) {
    case "critical":
      return { background: "rgba(229, 62, 62, 0.16)", color: "#feb2b2" };
    case "serious":
      return { background: "rgba(245, 101, 101, 0.12)", color: "#fc8181" };
    case "moderate":
      return { background: "rgba(236, 201, 75, 0.14)", color: "#f6e05e" };
    default:
      return { background: "rgba(148, 163, 184, 0.16)", color: "var(--text-mute)" };
  }
}

function groupMedicationResults(response: MedicationCheckResponse) {
  const severityOrder = ["critical", "serious", "moderate", "informational"];

  return severityOrder
    .map((severity) => {
      const severityResults = response.results.filter((result) => result.severity === severity);
      if (severityResults.length === 0) {
        return null;
      }

      const categories = Array.from(new Set(severityResults.map((result) => result.category))).sort();

      return {
        severity,
        results: severityResults,
        categories: categories.map((category) => ({
          category,
          results: severityResults.filter((result) => result.category === category),
        })),
      };
    })
    .filter(Boolean) as Array<{
      severity: string;
      results: MedicationCheckResponse["results"];
      categories: Array<{
        category: string;
        results: MedicationCheckResponse["results"];
      }>;
    }>;
}

function titleCase(value: string): string {
  return value
    .split(/[_\s]+/)
    .filter(Boolean)
    .map((segment) => segment.charAt(0).toUpperCase() + segment.slice(1))
    .join(" ");
}

function defaultPdfFileName(sourceFileName: string): string {
  const base = sourceFileName.replace(/\.[^.]+$/, "");
  return `${base || "personal-genomics-report"}.pdf`;
}
