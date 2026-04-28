# Personal Genomics parity roadmap

## Executive summary

- **Current app status:** solid Tauri shell with local file parsing, a small curated marker panel, streamed markdown report generation, follow-up chat, model selection, and keychain-backed Morpheus credentials.
- **Hermes status:** this is **not** a real Hermes integration in the official Nous sense. It is a **Hermes-style prompt + simple chat loop** in `src-tauri/src/hermes.rs` over `src-tauri/src/morpheus.rs`.
- **Best implementation strategy:** copy the **upstream feature contracts and product behavior**, not the Python implementations. Build around a canonical local `analysis_results` package first, then exports, then offline dashboard, then PDF/print, then larger dataset-driven features.

---

## Current implementation audit

### Implemented now

#### App shell and UX
- React + Tauri desktop app shell: `src/App.tsx`
- Views for upload / analyze / report / chat / settings: `src/views/*`
- Zustand app state: `src/store.ts`
- Calm offline-first desktop UI with animated DNA background: `src/components/DnaHelixBackground.tsx`

#### File ingestion and local parsing
- Supported inputs implemented in parser and README:
  - 23andMe
  - AncestryDNA
  - MyHeritage
  - FamilyTreeDNA
  - VCF
  - rsID TSV
- Parser implementation: `src-tauri/src/genomics.rs`
- Format detection, chromosome normalization, no-call counting, call-rate calculation, sex inference, sample SNP extraction all implemented.

#### Local genomic interpretation
- Curated local marker panel exists: `src-tauri/src/markers.rs`
- Current panel size: **28 markers**
- Marker matching pipeline implemented locally in Rust: `match_known_markers()` in `src-tauri/src/genomics.rs`
- Analysis summary screen implemented: `src/views/AnalyzeView.tsx`

#### Report and chat
- Markdown report generation via LLM implemented: `generate_report()` in `src-tauri/src/hermes.rs`
- Streaming SSE report updates implemented: `chat_stream()` in `src-tauri/src/morpheus.rs`
- Report rendering with table of contents and per-finding explanation affordance: `src/views/ReportView.tsx`
- Follow-up chat grounded in genome context implemented: `src/views/ChatView.tsx`

#### Morpheus + privacy
- Morpheus API client implemented: `src-tauri/src/morpheus.rs`
- Curated model list implemented, including TEE-tagged model metadata.
- Keychain-backed API key storage implemented: `src-tauri/src/settings.rs`
- TEE-only model filter UX implemented: `src/views/SettingsView.tsx`

#### Quality handling
- Call-rate threshold warning implemented in UI.
- Low quality warning also wired into the report prompt.

### Implemented partially / thinly

#### Hermes
- Current code uses the name Hermes, but architecturally it is a **prompt wrapper**, not a Hermes agent runtime.
- What exists:
  - fixed system prompt
  - message assembly
  - streaming wrapper
- What does **not** exist:
  - `AIAgent`-style orchestration
  - tool registry
  - memory files
  - session database / search
  - skills system
  - MCP integration
  - delegation / subagents
  - persistent agent state

#### Persistence
- Only durable secret storage exists today: API key in Keychain.
- No persisted analyses, report packages, exports, dashboard artifacts, or history.

---

## Upstream feature parity map

Legend:
- **Implemented** = present and usable now
- **Partial** = some equivalent exists, but much thinner than upstream
- **Missing** = not present

### v3 baseline and core outputs

| Upstream feature | Local status | Notes |
|---|---:|---|
| Multi-format DNA import | Implemented | Parser supports major consumer formats in `genomics.rs`. |
| Core genetic analysis | Partial | Local app has only 28 curated markers, not broad comprehensive analysis. |
| Pharmacogenomics | Partial | Only a handful of PGx markers inside the 28-marker set. No dedicated PGx pipeline. |
| Polygenic risk scores | Missing | No PRS engine, no percentile scoring. |
| Carrier status screening | Missing | No carrier module. |
| Health risks assessment | Partial | Limited marker-based narrative only. |
| Traits analysis | Partial | Limited traits within current marker set. |
| Nutrition markers | Partial | A few nutrition-related markers only. |
| Fitness markers | Partial | A few fitness-related markers only. |
| Neurogenetics / cognition | Partial | Some markers only. |
| Longevity markers | Partial | Very limited. |
| Immunity markers | Missing | No dedicated HLA/immunity module. |
| Rare disease panel | Missing | No rare disease analysis. |
| Mental health markers | Partial | Only thin marker coverage. |
| Dermatology markers | Missing | No dedicated skin/UV module. |
| Vision & hearing | Missing | No dedicated module. |
| Fertility markers | Missing | No dedicated module. |
| Agent-friendly JSON output | Missing | No canonical analysis bundle or export JSON. |
| Full analysis JSON | Missing | No persisted machine-readable report package. |
| Human-readable text report | Partial | Markdown report exists, but no stable exported text artifact. |

### v4.0 feature family

| Upstream feature | Local status | Notes |
|---|---:|---|
| Haplogroup analysis | Missing | No mtDNA / Y-DNA logic or reference data. |
| Ancestry composition | Missing | No AIMs/admixture/population comparison. |
| Hereditary cancer panel | Missing | No structured cancer-gene analysis. |
| Autoimmune HLA analysis | Missing | No HLA/immunity pipeline. |
| Pain sensitivity module | Missing | No dedicated module. |
| Professional PDF report | Missing | No PDF generation path at all. |
| Data quality metrics suite | Partial | Call rate / no-calls / chromosome counts exist; no full quality report, confidence scoring, or platform-grade QA package. |
| Genetic counselor clinical export (ACMG-style) | Missing | No clinical export schema or file. |
| Apple Health compatible format | Missing | No Apple Health export structure. |
| API-ready JSON structure | Missing | No canonical JSON schema. |
| Integration hooks for health trackers | Missing | No webhook/hooks/integration layer. |

### v4.1 feature family

| Upstream feature | Local status | Notes |
|---|---:|---|
| Medication interaction checker | Missing | No medication name input or interaction severity layer. |
| Sleep optimization profile | Missing | No sleep/chronotype package. |
| Dietary interaction matrix | Missing | No dedicated matrix/export. |
| Athletic performance profiling | Missing | No structured athletic scoring model. |
| UV sensitivity calculator | Missing | No Fitzpatrick / UV / melanoma logic. |
| Natural-language explanations | Partial | LLM report/chat exist, but no structured explanation layer for all findings. |
| Telomere length estimation | Missing | No telomere model. |
| Research variant flagging | Missing | No evidence tiering between established vs emerging. |

### v4.2 feature family

| Upstream feature | Local status | Notes |
|---|---:|---|
| Interactive web dashboard | Missing | No generated dashboard artifact. |
| Responsive HTML visualization | Missing | No generated HTML report/dashboard. |
| Auto-generated with every analysis | Missing | No artifact generation pipeline. |
| No external dependencies / works offline | Missing | Current app UI is offline-capable, but no standalone generated offline dashboard. |
| Dark mode, search/filter, collapsible sections | Missing | Could be implemented in generated HTML artifact. |
| Dashboard sections for pharmacogenomics / PRS / traits / ancestry / carrier / sleep / athletic / UV / dietary | Missing | Depends on structured analysis modules. |
| PDF export from dashboard print flow | Missing | No print-to-PDF surface. |
| Drag & drop JSON loading for standalone dashboard | Missing | No saved analysis JSON format exists yet. |
| Programmatic entrypoints / dashboard generator CLI equivalents | Missing | No durable analysis package or generator surface. |

### v4.4 / v4.4.1 / v5.0 feature family

| Upstream feature | Local status | Notes |
|---|---:|---|
| Ancient DNA matching | Missing | No ancient genome reference data or matching logic. |
| Population comparison | Missing | No 1000G / HGDP / SGDP comparison engine. |
| Ancient ancestry signals | Missing | No ancient marker panels/timeline. |
| Neanderthal / Denisovan reporting | Missing | No archaic introgression module. |
| Nine local reference databases | Missing | No local reference dataset layer at all. |
| Enhanced visualizations | Missing | No structured dashboard visualization system. |
| Runs of homozygosity detection | Missing | No ROH engine. |
| Larger testing + typed analysis contracts | Missing | No robust typed analysis schema or test suite around feature families. |

---

## Integration & export parity checklist

The user explicitly called these out. Current status:

| Feature | Status | What is missing |
|---|---:|---|
| Genetic counselor clinical export (ACMG-style) | Missing | Structured clinical variant classification, actionability buckets, inheritance metadata, counselor-facing JSON/PDF surface. |
| Apple Health compatible format | Missing | Export schema only is needed first; direct Apple Health write is not required initially. |
| API-ready JSON structure | Missing | Canonical `analysis_results` package plus versioned export schema. |
| Integration hooks for health trackers | Missing | Webhook/event payload generator and stable identifiers. |

---

## Database / reference-data gaps

### Missing app persistence layer
- No saved analysis history
- No persisted report package
- No export registry
- No dashboard artifact storage
- No session history for chat/report provenance

### Missing genomics reference datasets
- No local ancestry reference datasets
- No haplogroup reference tables
- No ClinVar / PharmGKB / GWAS / PGS / gnomAD ingestion layer
- No ancient DNA reference panels
- No population frequency databases

### Recommended persistence model

#### Stage 1 persistence
- **Filesystem-backed analysis packages** in Tauri app data directory
- Each run should persist:
  - uploaded file metadata
  - parser summary
  - normalized genotype summary
  - structured findings
  - generated report markdown
  - dashboard HTML path
  - exports generated

#### Stage 2 persistence
- Add a lightweight local database for indexing/search/history.
- Preferred use cases:
  - recent analyses
  - saved exports
  - report/dashboard lookup
  - future agent/session history

#### Candidate options
- **SQLite** is the right default.
  - Good fit for Tauri desktop
  - Enough for analysis metadata, artifact indexes, and future search/history
- Keep large genomic/reference assets on disk, indexed by SQLite metadata.

---

## PDF report assessment

### Current local repo
- No PDF generation crates in `src-tauri/Cargo.toml`
- No PDF command surface in `src-tauri/src/lib.rs`
- No print/export UI in React
- No structured report data model beyond markdown text + parsed genome object

### Upstream `pdf_report.py`
- **Better than current implementation** in scope and professionalism.
- It already models the concept of a physician-shareable artifact:
  - executive summary
  - critical findings
  - pharmacogenomics tables
  - health risks
  - ancestry
  - recommendations
  - disclaimers
- But it is **not** a direct port candidate.

### Recommendation
- Use upstream PDF module as **product inspiration and section spec**.
- Do **not** port ReportLab/Python design literally into this repo.
- In Tauri, the better path is:
  1. create stable structured `analysis_results`
  2. generate a print-optimized HTML report from it
  3. use webview / OS print-to-PDF flow
- Only create a dedicated native PDF generator later if print CSS cannot achieve clinician-grade fidelity.

---

## Offline dashboard assessment

### Current local repo
- No dashboard generator
- No saved analysis JSON contract
- No embedded offline artifact flow
- No dedicated webview for generated report/dashboard assets

### Upstream `generate_dashboard.py`
- **Strong inspiration candidate**.
- Best upstream idea to copy:
  - generate a **self-contained HTML artifact** from analysis JSON
  - no network dependency
  - reusable for in-app view, reopen-from-disk, sharing, and print

### Recommendation for this Tauri app
- Implement the dashboard as a **generated artifact**, not a second live app architecture.
- Requirements:
  - inline CSS/JS/SVG
  - no remote fonts, CDN scripts, or API calls
  - stored in app data directory alongside the analysis package
  - load in a dedicated Tauri webview via local file/custom protocol
- This should become the basis for:
  - offline visualization in-app
  - standalone artifact on disk
  - print-to-PDF

---

## Hermes assessment

### Current code is a simple loop, not official Hermes

Evidence in local repo:
- `src-tauri/src/hermes.rs` = fixed system prompt + message assembly
- `src-tauri/src/morpheus.rs` = direct OpenAI-compatible chat requests + SSE stream parsing
- `src-tauri/src/lib.rs` = direct commands for parse/report/chat

Missing official Hermes markers from docs:
- `AIAgent` orchestration layer
- tool registry
- callback surfaces
- memory files
- session DB with search
- skills system
- MCP integration
- delegation/subagents
- persistent state and lineage

### Decision
- **Do not make “real Hermes” the next major milestone.**
- First build the local structured analysis and export foundation.
- Revisit true Hermes integration only after saved analysis packages, exports, dashboard artifacts, and session persistence exist.

---

## Staged implementation order

## Stage 0 — Rename the architecture honestly

### Goal
Stop treating the current app as if Hermes parity already exists.

### Deliverables
- Internal docs note: current implementation is a Morpheus chat/report adapter with Hermes-style prompting.
- Audit any UX copy that overstates Hermes capability.

### Why first
- Prevents architecture drift.
- Keeps future work focused on data/results instead of branding.

---

## Stage 1 — Create the canonical local analysis package

### Goal
Introduce a versioned `analysis_results` contract and persist each analysis locally.

### Deliverables
- Stable Rust-side analysis schema beyond `ParsedGenome`
- Separate types for:
  - input metadata
  - quality metrics
  - finding categories
  - pharmacogenomics findings
  - trait findings
  - recommendations
  - provenance/evidence
  - generated artifacts
- Save package to app data directory after each analysis

### Includes
- artifact directory structure
- schema version field
- timestamps
- model/report provenance

### Depends on
- nothing except current parser and marker matcher

### Why now
- Every missing upstream feature depends on this.

---

## Stage 2 — Expand deterministic local analysis modules

### Goal
Move from “28 markers + LLM narrative” toward structured local analysis families.

### Deliverables
- Refactor current marker matching into category modules
- Add structured categories first for highest leverage:
  1. pharmacogenomics
  2. metabolic/cardiovascular
  3. traits
  4. data quality
- Preserve LLM report generation as downstream summarization, not primary analysis logic

### Why this order
- Exports and dashboard need structured data, not raw markdown.

---

## Stage 3 — Export layer parity

### Goal
Implement the upstream export family around the local analysis package.

### Deliverables
- Genetic counselor clinical export JSON
- Apple Health-compatible JSON export
- API-ready JSON export
- integration/webhook payload hooks
- export-all command surface

### Implementation guidance
- Upstream `exports.py` can be used as a **schema target**.
- Reimplement in Rust/TS for this app.

### Why before dashboard/PDF
- Exports clarify the data contract.
- Dashboard and PDF should consume the same structured package.

---

## Stage 4 — Offline dashboard artifact

### Goal
Generate a standalone, self-contained HTML dashboard with every analysis.

### Deliverables
- `dashboard.html` artifact generated locally
- sections driven by available structured result families
- local open/view action in Tauri
- artifact reopening from saved package

### Tauri-specific requirements
- store in app data dir
- load via local file or custom protocol in a dedicated webview
- keep all CSS/JS inline or bundled locally
- no runtime network fetches

### Why here
- The dashboard becomes the single richest artifact for offline visualization and future PDF printing.

---

## Stage 5 — Print/PDF export

### Goal
Add physician-shareable report export.

### Deliverables
- print-optimized HTML report template
- print-to-PDF flow from dashboard/report artifact
- section fidelity inspired by upstream `pdf_report.py`

### Recommendation
- Start with HTML/print.
- Only add dedicated native PDF generation if layout fidelity proves insufficient.

---

## Stage 6 — Full data-quality package

### Goal
Reach upstream-style quality reporting and trust boundaries.

### Deliverables
- call rate and no-call summary already exist; extend with:
  - platform confidence
  - chromosome coverage summary
  - parser anomalies
  - missingness explanations
  - confidence / caveat fields per module

### Why here
- Important for clinician/shareable artifacts and future dataset-heavy modules.

---

## Stage 7 — High-value feature families from upstream v4.0/v4.1

### Priority order
1. medication interaction checker
2. sleep optimization profile
3. dietary interaction matrix
4. athletic performance profiling
5. UV sensitivity calculator
6. research variant flagging
7. telomere estimate
8. hereditary cancer panel
9. autoimmune HLA
10. pain sensitivity

### Why this order
- Highest user value before the very large reference-dataset work.

---

## Stage 8 — Reference-dataset foundation

### Goal
Introduce the missing local databases that unlock ancestry, PRS, ancient DNA, and population comparison.

### Deliverables
- on-disk reference dataset layout
- metadata index in SQLite
- ingestion/versioning pipeline
- provenance documentation for each dataset

### Candidate dataset families
- ClinVar
- PharmGKB-derived lookup material
- GWAS catalog subsets
- PGS catalog subsets
- ancestry AIM panels
- haplogroup marker tables
- 1000 Genomes / HGDP / SGDP summaries
- ancient DNA marker panels

---

## Stage 9 — Ancestry / population / ancient DNA suite

### Goal
Match the upstream’s most ambitious genomics surface area.

### Deliverables
- haplogroups
- ancestry composition
- population comparison
- ancient DNA signals
- ancient match cards
- neanderthal/denisovan summaries

### Note
- This stage should only start after Stage 8 is stable.

---

## Stage 10 — Decide whether to add real Hermes

### Goal
Only after the local analysis system is rich enough, decide whether true Hermes architecture adds value.

### Consider adding if needed
- saved session history/search
- tool-backed exports/dashboard access
- richer multi-step clinical explanation workflows
- MCP / memory / skills / delegation

### If not needed
- Keep the app simpler and treat Hermes as product voice, not runtime substrate.

---

## File-level impact map for upcoming work

### Existing files likely to evolve
- `src-tauri/src/genomics.rs`
- `src-tauri/src/markers.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/hermes.rs`
- `src/types.ts`
- `src/api.ts`
- `src/store.ts`
- `src/views/AnalyzeView.tsx`
- `src/views/ReportView.tsx`
- `src/views/SettingsView.tsx`

### Likely new backend files
- `src-tauri/src/analysis.rs`
- `src-tauri/src/exports.rs`
- `src-tauri/src/dashboard.rs`
- `src-tauri/src/artifacts.rs`
- `src-tauri/src/storage.rs`
- `src-tauri/src/db.rs` or equivalent
- `src-tauri/src/modules/*` for feature families
- `src-tauri/src/reference_data/*`

### Likely new frontend files
- dashboard viewer route/view
- export center / artifact history UI
- saved analyses UI
- richer report template/render helpers

---

## Bottom-line decisions

### Do this
- Build a **canonical saved analysis package** first.
- Port **export schemas** early.
- Use upstream **offline HTML dashboard** as a strong design model.
- Use upstream **PDF structure** as inspiration, but implement through HTML/print first.
- Add **SQLite + filesystem artifacts** before dataset-heavy work.

### Do not do this yet
- Do not chase true Hermes runtime parity first.
- Do not port ReportLab directly.
- Do not build the dashboard as a second dynamic frontend app before the analysis package exists.
