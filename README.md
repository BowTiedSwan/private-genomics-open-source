# Personal Genomics — Hermes × Morpheus

A private macOS app that parses your raw consumer DNA export (23andMe, AncestryDNA,
MyHeritage, FamilyTreeDNA, VCF) entirely on-device, then generates a structured
genomic report using a **Hermes-style agent** powered by **Kimi 2.6** inference
from the **Morpheus** decentralized inference marketplace — with an optional
**TEE-only mode** that pins inference to attested Trusted Execution Environment
providers for end-to-end privacy.

Built with [Tauri 2](https://tauri.app) + React + TypeScript, inspired by
[wkyleg/personal-genomics](https://github.com/wkyleg/personal-genomics).

---

## How it works

```
┌───────────────────────────────────┐          ┌──────────────────────────────┐
│  Your Mac                         │          │  Morpheus (api.mor.org)      │
│                                   │          │                              │
│  .txt / .vcf / .csv               │          │   kimi-k2.6     (default)    │
│        │                          │          │   kimi-k2.6:web              │
│   parse_genome  ─── Rust          │          │   mistral-31-24b:tee  ←TEE   │
│        │                          │          │   hermes-3-llama-3.1-405b    │
│   match curated markers (local)   │          │   kimi-k2-thinking           │
│        │                          │          │                              │
│   build redacted context ─────────┼──HTTPS──▶│   /v1/chat/completions       │
│        │                          │          │   streaming SSE back         │
│   Hermes system prompt + SSE ◀────┼──────────┘                              │
│        │                          │
│   Markdown report + chat          │
└───────────────────────────────────┘
```

What leaves your device: a compact summary (SNP counts, call rate, and the
matched rsID+interpretation list from the local curated marker set).
**Raw SNP calls are never sent.**

In **TEE-only mode** even that summary is only sent to a Morpheus provider
running inside an attested Trusted Execution Environment, so the provider
host cannot read it in cleartext.

---

## Getting started

Requirements: macOS 12+, Rust (stable) + `pnpm`.

```bash
pnpm install
pnpm tauri dev       # hot-reload dev build
pnpm tauri build     # ship a .app + .dmg in src-tauri/target/release/bundle/
```

On first launch:

1. Open **Settings**.
2. Click **Open app.mor.org**, sign in, create an API key.
3. Paste the key. It is stored in your macOS Keychain (never in plaintext config).
4. Optionally flip **TEE-only inference** on for full end-to-end privacy.
5. Go to **Upload**, pick your raw DNA file, then **Generate Hermes report**.

---

## Project layout

```
src-tauri/src/
  lib.rs           Tauri command surface
  genomics.rs      Multi-format parser (23andMe, AncestryDNA, MyHeritage, VCF, …)
  markers.rs       Curated literature-backed marker panel (28 markers)
  morpheus.rs      OpenAI-compatible client for api.mor.org + streaming
  hermes.rs        Hermes-style system prompt + agent loop
  settings.rs      Keychain-backed API-key storage
  error.rs         Unified error surface

src/
  App.tsx          Shell + sidebar navigation
  store.ts         Zustand app state
  api.ts           Tauri invoke wrappers
  types.ts         Shared types mirroring Rust
  styles.css       Design tokens + layout
  views/
    UploadView.tsx     File picker + privacy explainer
    AnalyzeView.tsx    Parsed summary + matched markers + model status
    ReportView.tsx     Streaming markdown with section TOC
    ChatView.tsx       Follow-up Q&A grounded in genome context
    SettingsView.tsx   API key, TEE toggle, model picker
```

---

## Design notes

- **Hermes** is implemented as a Rust-side agent loop (system prompt +
  message assembly + streaming). It drives Morpheus via its OpenAI-compatible
  `/chat/completions` endpoint, so the same code works unchanged across all
  active models listed on `active.mor.org/active_models.json`.
- **Kimi 2.6** is the default model (`kimi-k2.6`); a `:web` variant adds
  web tool-use; `hermes-3-llama-3.1-405b` and `kimi-k2-thinking` are offered
  as alternates. `mistral-31-24b:tee` is the currently-active TEE-backed model.
- The UI is deliberately calm: sidebar, big titles, tight type hierarchy,
  respectful of system dark/light mode, no marketing chrome.
- All genotype interpretation uses a sorted-pair representation so `AG` and
  `GA` always match the same rule.
- Call rate below 97% triggers a visible quality banner both in Analysis and
  inside the report (the system prompt instructs Hermes to open with a
  data-quality caveat in that case).

---

## License

For personal use. Not a medical device, not a diagnosis. Consult a clinician
before acting on any finding.
