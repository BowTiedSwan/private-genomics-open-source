import { useEffect } from "react";
import { ArrowRight, CheckCircle2, Download, ExternalLink, GitBranch, LockKeyhole, Microscope, ShieldCheck } from "lucide-react";
import BrandHelix from "../components/BrandHelix";
import appIconUrl from "../../src-tauri/icons/icon.svg?url";
import { initAnalytics, trackAction } from "../analytics";
import "./landing.css";

const OPEN_SOURCE_REPO_URL = "https://github.com/BowTiedSwan/private-genomics";
const MORPHEUS_URL =
  "https://mor.org/inference-api?utm_source=private_genomics_website&utm_medium=referral&utm_campaign=powered_by_morpheus_inference&utm_content=landing_page";

const ctas = {
  githubHero: {
    action: "open_source_repo_clicked",
    label: "View the open-source repo",
    location: "hero",
    destination: OPEN_SOURCE_REPO_URL,
  },
  downloadHero: {
    action: "mac_download_placeholder_clicked",
    label: "Mac app placeholder",
    location: "hero",
    destination: "#download",
  },
  morpheus: {
    action: "morpheus_inference_clicked",
    label: "Powered by Morpheus Inference",
    location: "inference_section",
    destination: MORPHEUS_URL,
  },
  githubFooter: {
    action: "open_source_repo_clicked",
    label: "Inspect the code",
    location: "final_cta",
    destination: OPEN_SOURCE_REPO_URL,
  },
};

const localBenefits = [
  "Raw DNA files stay on your Mac",
  "Deterministic marker analysis happens locally",
  "Only redacted context is sent when inference is enabled",
];

const workflow = [
  ["Load your file", "Import 23andMe, AncestryDNA, MyHeritage, FamilyTreeDNA, VCF, or CSV exports."],
  ["Review local findings", "GeneVault checks data quality and maps curated pharmacogenomic and trait markers."],
  ["Generate a report", "Helix turns the local analysis package into a plain-English report you can review."],
  ["Ask follow-ups", "Chat with the saved genome context without sending your raw SNP calls."],
];

const proof = ["Tauri Mac app", "Open source", "Local-first analysis", "Powered by Morpheus Inference"];

export default function LandingPage() {
  useEffect(() => {
    initAnalytics();
  }, []);

  return (
    <main className="landing-shell">
      <nav className="landing-nav" aria-label="Main navigation">
        <a
          className="landing-brand"
          href="#top"
          data-track-action="nav_brand_clicked"
          onClick={() => trackAction({ action: "nav_brand_clicked", label: "GeneVault", location: "nav", destination: "#top" })}
        >
          <BrandHelix />
          <span>GeneVault</span>
        </a>
        <div className="nav-links">
          <a href="#privacy" onClick={() => trackAction({ action: "nav_link_clicked", label: "Privacy", location: "nav", destination: "#privacy" })}>Privacy</a>
          <a href="#download" onClick={() => trackAction({ action: "nav_link_clicked", label: "Download", location: "nav", destination: "#download" })}>Download</a>
          <a href={OPEN_SOURCE_REPO_URL} onClick={() => trackAction({ ...ctas.githubHero, location: "nav" })}>GitHub</a>
        </div>
      </nav>

      <section id="top" className="hero-section">
        <div className="hero-copy">
          <p className="eyebrow">Private genomics for people who read the fine print</p>
          <h1>Understand your raw DNA file without handing it to another cloud dashboard.</h1>
          <p className="hero-subcopy">
            GeneVault is an open-source Mac app for local-first personal genomics. It parses common consumer DNA exports on-device, checks curated markers, and creates a structured report with Helix.
          </p>
          <div className="hero-actions">
            <a className="button primary" href={OPEN_SOURCE_REPO_URL} onClick={() => trackAction(ctas.githubHero)}>
              View the open-source repo <GitBranch aria-hidden="true" />
            </a>
            <a className="button secondary" href="#download" onClick={() => trackAction(ctas.downloadHero)}>
              Mac app download <Download aria-hidden="true" />
            </a>
          </div>
        </div>

        <div className="hero-card" aria-label="GeneVault app summary">
          <div className="hero-icon-wrap">
            <img src={appIconUrl} alt="GeneVault helix app icon" />
          </div>
          <div className="scan-panel">
            <span>Genome file</span>
            <strong>local_analysis_package.json</strong>
          </div>
          <div className="marker-grid">
            <span>Pharmacogenomics</span>
            <span>Sleep profile</span>
            <span>Traits</span>
            <span>Quality checks</span>
          </div>
        </div>
      </section>

      <section className="proof-strip" aria-label="Project proof points">
        {proof.map((item) => (
          <span key={item}>{item}</span>
        ))}
      </section>

      <section id="privacy" className="split-section">
        <div>
          <p className="eyebrow">The privacy problem</p>
          <h2>Your genome is not just another upload.</h2>
        </div>
        <div className="copy-stack">
          <p>
            Most DNA tools start by asking for the most sensitive file you own. GeneVault flips that flow: parse first, understand what is being summarized, and decide when inference is worth using.
          </p>
          <div className="benefit-list">
            {localBenefits.map((benefit) => (
              <div key={benefit}>
                <CheckCircle2 aria-hidden="true" />
                <span>{benefit}</span>
              </div>
            ))}
          </div>
        </div>
      </section>

      <section className="cards-section" aria-label="GeneVault workflow">
        <div className="section-heading">
          <p className="eyebrow">How it works</p>
          <h2>A clean workflow for messy DNA exports.</h2>
        </div>
        <div className="workflow-grid">
          {workflow.map(([title, body], index) => (
            <article className="workflow-card" key={title}>
              <span className="step-index">0{index + 1}</span>
              <h3>{title}</h3>
              <p>{body}</p>
            </article>
          ))}
        </div>
      </section>

      <section className="inference-section">
        <div className="inference-card">
          <Microscope aria-hidden="true" />
          <div>
            <p className="eyebrow">Inference layer</p>
            <h2>Powered by Morpheus Inference</h2>
            <p>
              When you choose to generate a report, GeneVault can send a compact, redacted analysis package to Morpheus-compatible models. Raw SNP calls stay out of the request.
            </p>
            <a href={MORPHEUS_URL} onClick={() => trackAction(ctas.morpheus)}>
              Learn about Morpheus Inference <ExternalLink aria-hidden="true" />
            </a>
          </div>
        </div>
      </section>

      <section id="download" className="download-section">
        <div>
          <p className="eyebrow">Downloads</p>
          <h2>Mac app download</h2>
          <p>
            The public Mac build will appear here when the release artifact is ready. For now, use the repository to build the app locally from source.
          </p>
        </div>
        <div className="download-card">
          <LockKeyhole aria-hidden="true" />
          <strong>Download placeholder</strong>
          <span>No `.dmg` file has been published yet.</span>
          <a className="button secondary" href={OPEN_SOURCE_REPO_URL} onClick={() => trackAction({ action: "build_from_source_clicked", label: "Build from source", location: "download", destination: OPEN_SOURCE_REPO_URL })}>
            Build from source <ArrowRight aria-hidden="true" />
          </a>
        </div>
      </section>

      <section className="open-source-section">
        <div className="repo-panel">
          <ShieldCheck aria-hidden="true" />
          <h2>Open source so the trust boundary is visible.</h2>
          <p>
            Inspect the parser, marker rules, report prompt, and Morpheus client. The app is built for people who want to know what leaves their machine before they run an analysis.
          </p>
          <a className="button primary" href={OPEN_SOURCE_REPO_URL} onClick={() => trackAction(ctas.githubFooter)}>
            Inspect the code <GitBranch aria-hidden="true" />
          </a>
        </div>
      </section>

      <footer className="landing-footer">
        <span>GeneVault</span>
        <span>Private genomics, local first.</span>
        <a href={MORPHEUS_URL} onClick={() => trackAction({ ...ctas.morpheus, location: "footer" })}>Powered by Morpheus Inference</a>
      </footer>
    </main>
  );
}
