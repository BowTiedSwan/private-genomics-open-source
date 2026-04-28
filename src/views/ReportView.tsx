import { Children, isValidElement, useEffect, useMemo, useState } from "react";
import ReactMarkdown from "react-markdown";
import type { Components } from "react-markdown";
import remarkGfm from "remark-gfm";
import { Loader2, FileText, MessageSquare, AlertTriangle, CircleHelp, Printer, X } from "lucide-react";
import { useApp } from "../store";
import { api } from "../api";
import { exportPdfReport } from "../printReport";

const REPORT_EXPLAIN_MODEL = "glm-4.7-flash";

export default function ReportView() {
  const { report, reportError, reportInProgress, setView, setAnalysis, analysis } = useApp();
  const [activeSection, setActiveSection] = useState<string>("");
  const [explainingFinding, setExplainingFinding] = useState<string | null>(null);
  const [findingExplanation, setFindingExplanation] = useState<string>("");
  const [findingError, setFindingError] = useState<string | null>(null);
  const [findingBusy, setFindingBusy] = useState(false);
  const [printErr, setPrintErr] = useState<string | null>(null);
  const [printBusy, setPrintBusy] = useState(false);

  const sections = useMemo(() => {
    const out: { id: string; title: string }[] = [];
    const re = /^##\s+(.+?)\s*$/gm;
    let m: RegExpExecArray | null;
    while ((m = re.exec(report)) !== null) {
      const title = m[1].trim();
      out.push({ id: slug(title), title });
    }
    return out;
  }, [report]);

  useEffect(() => {
    if (!sections.length) return;
    const onScroll = () => {
      for (let i = sections.length - 1; i >= 0; i--) {
        const el = document.getElementById(sections[i].id);
        if (el && el.getBoundingClientRect().top < 120) {
          setActiveSection(sections[i].id);
          return;
        }
      }
      setActiveSection(sections[0].id);
    };
    const main = document.querySelector(".main");
    main?.addEventListener("scroll", onScroll);
    onScroll();
    return () => main?.removeEventListener("scroll", onScroll);
  }, [sections]);

  if (!report && !reportInProgress && !reportError) {
    return (
      <div className="empty">
        <FileText />
        <h3>No report yet</h3>
        <p>Generate one from the Analysis tab.</p>
      </div>
    );
  }

  async function explainFinding(finding: string) {
    if (findingBusy) return;
    setExplainingFinding(finding);
    setFindingExplanation("");
    setFindingError(null);
    setFindingBusy(true);
    try {
      const explanation = await api.explainMarker({
        model: REPORT_EXPLAIN_MODEL,
        genome: analysis?.genome ?? null,
        finding,
      });
      setFindingExplanation(explanation.trim());
    } catch (e) {
      setFindingError(String(e));
    } finally {
      setFindingBusy(false);
    }
  }

  function closeFindingExplanation() {
    setExplainingFinding(null);
    setFindingExplanation("");
    setFindingError(null);
    setFindingBusy(false);
  }

  async function exportPdf() {
    if (!analysis) return;
    setPrintErr(null);
    setPrintBusy(true);
    try {
      const updatedAnalysis = await exportPdfReport(
        analysis.id,
        defaultPdfFileName(analysis.source.file_name),
      );
      if (updatedAnalysis) {
        setAnalysis(updatedAnalysis);
      }
    } catch (e) {
      setPrintErr(String(e));
    } finally {
      setPrintBusy(false);
    }
  }

  const markdownComponents: Components = {
    h2: ({ children }) => {
      const text = String(children);
      return <h2 id={slug(text)}>{text}</h2>;
    },
    li: ({ children }) => {
      const text = flattenText(children).trim();
      const hasSnpReference = /\brs\d+\b/i.test(text);
      const isActiveFinding = explainingFinding === text;

      if (!hasSnpReference) {
        return <li>{children}</li>;
      }

      return (
        <li className="report-snp-li">
          <div className="report-snp-row">
            <span className="report-snp-content">{children}</span>
            <button
              className="report-snp-ask"
              type="button"
              aria-label="Explain this SNP finding in simpler language"
              onClick={() => void explainFinding(text)}
              disabled={findingBusy && !isActiveFinding}
            >
              <CircleHelp size={14} />
            </button>
          </div>
        </li>
      );
    },
  };

  return (
    <>
      {reportError && (
        <div className="banner banner-warn" style={{ marginBottom: 16 }}>
          <AlertTriangle size={16} />
          <div>{reportError}</div>
        </div>
      )}
      <div className="row" style={{ marginBottom: 24 }}>
        <div>
          <h1 className="page-title" style={{ marginBottom: 2 }}>Your DNA report</h1>
          <p className="page-sub" style={{ marginBottom: 0 }}>
            {reportInProgress ? (
              <><Loader2 size={13} className="spin" /> Streaming tokens from Morpheus…</>
            ) : (
              reportError ? "Report generation stopped before content was produced." : "Streaming complete. Review any section below."
            )}
          </p>
        </div>
        <div className="spacer" />
        <button className="btn btn-primary" onClick={() => void exportPdf()} disabled={printBusy || !analysis}>
          <Printer size={14} /> {printBusy ? "Preparing PDF…" : "Export PDF"}
        </button>
        <button className="btn" onClick={() => setView("chat")}>
          <MessageSquare size={14} /> Follow-up questions
        </button>
      </div>

      {printErr && (
        <div className="banner banner-warn" style={{ marginBottom: 16 }}>
          <AlertTriangle size={16} />
          <div>{printErr}</div>
        </div>
      )}

      <div className="helper" style={{ marginBottom: 16 }}>
        Last exported PDF: <code>{analysis?.artifacts.exported_pdf_path ?? "Not exported yet"}</code>
      </div>

      <div className="report-layout">
        <article className="report">
          <ReactMarkdown remarkPlugins={[remarkGfm]} components={markdownComponents}>
            {report || (reportError ? "**Report generation failed before any content was produced.**\n\nPlease check the error banner above, then verify your API key, selected model, and network access before trying again." : "_Waiting for the first token…_")}
          </ReactMarkdown>
        </article>
        {sections.length > 0 && (
          <aside className="report-sidebar-stack">
            <div className="toc">
              <div className="toc-title">Sections</div>
              {sections.map((s) => (
                <a
                  key={s.id}
                  className={"toc-item " + (activeSection === s.id ? "active" : "")}
                  onClick={() => {
                    document.getElementById(s.id)?.scrollIntoView({ behavior: "smooth", block: "start" });
                  }}
                >
                  {s.title}
                </a>
              ))}
            </div>
            {explainingFinding && (
              <div className="toc report-explainer-panel">
                <div className="report-explainer-header">
                  <div className="toc-title">Explain this finding</div>
                  <button
                    type="button"
                    className="report-explainer-close"
                    onClick={closeFindingExplanation}
                    aria-label="Close finding explanation"
                  >
                    <X size={14} />
                  </button>
                </div>
                <div className="report-explainer-finding">{explainingFinding}</div>
                {findingBusy ? (
                  <div className="chat-thinking" aria-live="polite">
                    <Loader2 size={14} className="spin" /> Generating a plain-English explanation…
                  </div>
                ) : findingError ? (
                  <div className="banner banner-warn" style={{ marginBottom: 0 }}>
                    <AlertTriangle size={16} />
                    <div>{findingError}</div>
                  </div>
                ) : (
                  <div className="report-explainer-body" role="region" aria-live="polite">
                    {findingExplanation}
                  </div>
                )}
              </div>
            )}
          </aside>
        )}
      </div>
    </>
  );
}

function slug(s: string): string {
  return "s-" + s.toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "");
}

function flattenText(children: React.ReactNode): string {
  return Children.toArray(children)
    .map((child) => {
      if (typeof child === "string" || typeof child === "number") {
        return String(child);
      }
      if (isValidElement<{ children?: React.ReactNode }>(child)) {
        return flattenText(child.props.children);
      }
      return "";
    })
    .join("");
}

function defaultPdfFileName(sourceFileName: string): string {
  const base = sourceFileName.replace(/\.[^.]+$/, "");
  return `${base || "personal-genomics-report"}.pdf`;
}
