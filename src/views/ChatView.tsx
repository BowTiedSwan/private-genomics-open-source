import { useEffect, useRef, useState } from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { Loader2, Send } from "lucide-react";
import { useApp } from "../store";
import { api } from "../api";

export default function ChatView() {
  const { chat, appendChat, updateLastChat, analysis, selectedModelId, hasApiKey } = useApp();
  const [input, setInput] = useState("");
  const [busy, setBusy] = useState(false);
  const scrollRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    scrollRef.current?.scrollTo({ top: scrollRef.current.scrollHeight });
  }, [chat]);

  async function send() {
    const text = input.trim();
    if (!text || busy) return;
    if (!hasApiKey) {
      alert("Save a Morpheus API key in Settings first.");
      return;
    }
    const history = chat.slice(-10);
    appendChat({ role: "user", content: text });
    appendChat({ role: "assistant", content: "" });
    setInput("");
    setBusy(true);
    try {
      await api.chat({
        model: selectedModelId,
        genome: analysis?.genome ?? null,
        history,
        message: text,
      });
    } catch (e: any) {
      updateLastChat(`\n\n_Error: ${e}_`);
    } finally {
      setBusy(false);
    }
  }

  return (
    <>
      <h1 className="page-title">Ask Hermes</h1>
      <p className="page-sub">
        Follow-up questions grounded in your genome context.
      </p>

      <div className="chat-view">
        <div className="chat-scroll" ref={scrollRef}>
          {chat.length === 0 ? (
            <div className="empty" style={{ paddingTop: 20 }}>
              <h3>Ask anything</h3>
              <p>
                Examples: "What should I know before taking clopidogrel?",
                "Summarise my caffeine metabolism", "Explain my APOE status".
              </p>
            </div>
          ) : (
            chat.map((m, i) => (
              <div key={i} className={"chat-msg " + m.role}>
                <small>{m.role === "user" ? "You" : "Hermes"}</small>
                <div className="chat-bubble">
                  {m.content ? (
                    <ReactMarkdown remarkPlugins={[remarkGfm]}>{m.content}</ReactMarkdown>
                  ) : (
                    <div className="chat-thinking" aria-live="polite">
                      <Loader2 size={14} className="spin" /> Hermes is thinking…
                    </div>
                  )}
                </div>
              </div>
            ))
          )}
        </div>

        <div className="chat-input-row">
          <textarea
            placeholder="Ask a follow-up… (⌘+Enter to send)"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter" && (e.metaKey || e.ctrlKey)) {
                e.preventDefault();
                send();
              }
            }}
          />
          <button className="btn btn-primary" onClick={send} disabled={busy || !input.trim()}>
            <Send size={14} /> Send
          </button>
        </div>
      </div>
    </>
  );
}
