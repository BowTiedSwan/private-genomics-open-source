import { useState } from "react";
import { openUrl } from "@tauri-apps/plugin-opener";
import { CheckCircle, ExternalLink, Shield, Trash2 } from "lucide-react";
import { useApp } from "../store";
import { api } from "../api";

export default function SettingsView() {
  const {
    models,
    selectedModelId,
    setSelectedModelId,
    teeOnly,
    setTeeOnly,
    hasApiKey,
    setHasApiKey,
  } = useApp();
  const [keyInput, setKeyInput] = useState("");
  const [saving, setSaving] = useState(false);
  const [saved, setSaved] = useState(false);

  const visibleModels = teeOnly ? models.filter((m) => m.tee) : models;

  async function saveKey() {
    if (!keyInput.trim()) return;
    setSaving(true);
    try {
      await api.saveApiKey(keyInput.trim());
      setHasApiKey(true);
      setSaved(true);
      setKeyInput("");
      setTimeout(() => setSaved(false), 2000);
    } finally {
      setSaving(false);
    }
  }

  async function clearKey() {
    await api.clearApiKey();
    setHasApiKey(false);
  }

  return (
    <>
      <h1 className="page-title">Settings</h1>
      <p className="page-sub">Connect Morpheus and choose how private you want inference to be.</p>

      <div className="card">
        <h3 className="card-title">Morpheus API key</h3>
        <p className="card-sub">
          Sign in to Morpheus and create a key, then paste it here. It's stored in your macOS Keychain —
          never in plaintext config.
        </p>
        <div className="helper" style={{ marginBottom: 12 }}>
          To avoid macOS keychain prompts on startup, the app no longer checks for a saved key automatically when it boots.
        </div>

        <div className="row" style={{ marginBottom: 12 }}>
          <button
            className="btn"
            onClick={() => openUrl("https://app.mor.org")}
          >
            <ExternalLink size={14} /> Open app.mor.org
          </button>
          {hasApiKey && (
            <span className="pill pill-ok">
              <CheckCircle size={11} /> Key saved
            </span>
          )}
        </div>

        <label className="label">Paste key</label>
        <input
          className="input input-mono"
          type="password"
          placeholder="sk_mor_•••••••••••••••"
          value={keyInput}
          onChange={(e) => setKeyInput(e.target.value)}
        />
        <div className="row" style={{ marginTop: 12 }}>
          <button
            className="btn btn-primary"
            onClick={saveKey}
            disabled={saving || !keyInput.trim()}
          >
            {saving ? "Saving…" : saved ? "Saved ✓" : "Save key"}
          </button>
          {hasApiKey && (
            <button className="btn btn-ghost" onClick={clearKey}>
              <Trash2 size={14} /> Remove saved key
            </button>
          )}
        </div>
      </div>

      <div className="card">
        <div className="toggle-row">
          <div
            className={"toggle " + (teeOnly ? "on" : "")}
            onClick={() => setTeeOnly(!teeOnly)}
            onKeyDown={(e) => {
              if (e.key === "Enter" || e.key === " ") {
                e.preventDefault();
                setTeeOnly(!teeOnly);
              }
            }}
            role="switch"
            aria-checked={teeOnly}
            aria-label="TEE-only inference"
            tabIndex={0}
          />
          <div style={{ flex: 1 }}>
            <h4>
              <Shield size={14} style={{ verticalAlign: "text-bottom", marginRight: 6, color: "var(--tee)" }} />
              TEE-only inference
            </h4>
            <p>
              Route all Hermes calls through a{" "}
              <strong>Trusted Execution Environment</strong> provider on Morpheus.
              Your prompts + genomic context stay inside an attested enclave end-to-end —
              neither the provider node nor the Morpheus router can read them in cleartext.
              Fewer models are available in this mode.
            </p>
          </div>
        </div>
      </div>

      <div className="card">
        <h3 className="card-title">Inference model</h3>
        <p className="card-sub">
          Default is <strong>Kimi 2.6</strong> — high-quality, general-purpose reasoning.
          Switch to a TEE-backed model for stronger privacy at the cost of a smaller model.
        </p>

        {visibleModels.length === 0 && (
          <p style={{ color: "var(--text-mute)" }}>
            No TEE-enabled models currently active on the network. Disable TEE-only to see all models.
          </p>
        )}

        {visibleModels.map((m) => (
          <label
            key={m.id}
            className={
              "model-option " +
              (m.tee ? "tee " : "") +
              (selectedModelId === m.id ? "selected" : "")
            }
          >
            <input
              type="radio"
              name="model"
              checked={selectedModelId === m.id}
              onChange={() => setSelectedModelId(m.id)}
            />
            <div>
              <h4>
                {m.name}
                {m.tee && <span className="pill pill-tee" style={{ marginLeft: 8 }}>TEE</span>}
                {m.web && <span className="pill" style={{ marginLeft: 6 }}>web</span>}
              </h4>
              <p>{m.description}</p>
              <p style={{ fontFamily: "var(--mono)", fontSize: 11, color: "var(--text-mute)", marginTop: 4 }}>
                {m.id}
              </p>
            </div>
          </label>
        ))}
      </div>
    </>
  );
}
