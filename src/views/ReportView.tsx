import { useEffect, useMemo, useState } from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { Loader2, FileText, MessageSquare } from "lucide-react";
import { useApp } from "../store";

export default function ReportView() {
  const { report, reportInProgress, setView } = useApp();
  const [activeSection, setActiveSection] = useState<string>("");

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

  if (!report && !reportInProgress) {
    return (
      <div className="empty">
        <FileText />
        <h3>No report yet</h3>
        <p>Generate one from the Analysis tab.</p>
      </div>
    );
  }

  return (
    <>
      <div className="row" style={{ marginBottom: 24 }}>
        <div>
          <h1 className="page-title" style={{ marginBottom: 2 }}>Your Hermes report</h1>
          <p className="page-sub" style={{ marginBottom: 0 }}>
            {reportInProgress ? (
              <><Loader2 size={13} className="spin" /> Streaming tokens from Morpheus…</>
            ) : (
              "Streaming complete. Review any section below."
            )}
          </p>
        </div>
        <div className="spacer" />
        <button className="btn" onClick={() => setView("chat")}>
          <MessageSquare size={14} /> Follow-up questions
        </button>
      </div>

      <div className="report-layout">
        <article className="report">
          <ReactMarkdown
            remarkPlugins={[remarkGfm]}
            components={{
              h2: ({ children }) => {
                const text = String(children);
                return <h2 id={slug(text)}>{text}</h2>;
              },
            }}
          >
            {report || "_Waiting for the first token…_"}
          </ReactMarkdown>
        </article>
        {sections.length > 0 && (
          <aside className="toc">
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
          </aside>
        )}
      </div>
    </>
  );
}

function slug(s: string): string {
  return "s-" + s.toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "");
}
