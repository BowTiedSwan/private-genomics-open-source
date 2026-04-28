use crate::analysis::{save, AnalysisPackage};
use crate::error::AppResult;
use crate::local_analysis::StructuredFinding;
use printpdf::*;
use std::time::{SystemTime, UNIX_EPOCH};

const PAGE_WIDTH_MM: f32 = 210.0;
const PAGE_HEIGHT_MM: f32 = 297.0;
const LEFT_MARGIN_MM: f32 = 18.0;
const RIGHT_MARGIN_MM: f32 = 18.0;
const TOP_MARGIN_MM: f32 = 20.0;
const BOTTOM_MARGIN_MM: f32 = 18.0;
const BODY_FONT_SIZE: f32 = 10.5;
const SMALL_FONT_SIZE: f32 = 9.0;
const H1_FONT_SIZE: f32 = 24.0;
const H2_FONT_SIZE: f32 = 15.0;
const H3_FONT_SIZE: f32 = 11.5;
const LINE_HEIGHT_MM: f32 = 5.5;
const SMALL_LINE_HEIGHT_MM: f32 = 4.6;
const SECTION_GAP_MM: f32 = 4.0;
const CHAR_WIDTH_FACTOR: f32 = 0.46;

pub fn export_pdf(package: &mut AnalysisPackage, output_path: &str) -> AppResult<()> {
    let pdf_bytes = build_pdf(package);
    std::fs::write(output_path, pdf_bytes)?;
    package.artifacts.exported_pdf_path = Some(output_path.to_string());
    package.updated_at_unix_ms = now_unix_ms()?;
    save(package)
}

fn build_pdf(package: &AnalysisPackage) -> Vec<u8> {
    let mut doc = PdfDocument::new("Personal Genomics Report");
    let mut pages: Vec<PdfPage> = Vec::new();
    let mut ops: Vec<Op> = Vec::new();
    let mut cursor_y = PAGE_HEIGHT_MM - TOP_MARGIN_MM;

    begin_text_section(&mut ops);

    write_wrapped_paragraph(
        &mut pages,
        &mut ops,
        &mut cursor_y,
        &mut doc,
        "Personal Genomics Report",
        H1_FONT_SIZE,
        true,
    );
    write_wrapped_paragraph(
        &mut pages,
        &mut ops,
        &mut cursor_y,
        &mut doc,
        "Private, on-device genomic summary generated from the saved local analysis package.",
        SMALL_FONT_SIZE,
        false,
    );
    write_wrapped_paragraph(
        &mut pages,
        &mut ops,
        &mut cursor_y,
        &mut doc,
        "This document is designed to read like a compact report: start with quality and actionable items, then move into structured findings and the saved narrative summary.",
        SMALL_FONT_SIZE,
        false,
    );

    write_section_heading(&mut pages, &mut ops, &mut cursor_y, &mut doc, "Summary");
    write_bullet(
        &mut pages,
        &mut ops,
        &mut cursor_y,
        &mut doc,
        &format!("Source file: {}", package.source.file_name),
        BODY_FONT_SIZE,
    );
    write_bullet(
        &mut pages,
        &mut ops,
        &mut cursor_y,
        &mut doc,
        &format!("Format: {}", package.results.quality.format_label),
        BODY_FONT_SIZE,
    );
    write_bullet(
        &mut pages,
        &mut ops,
        &mut cursor_y,
        &mut doc,
        &format!(
            "Call rate: {:.2}%",
            package.results.quality.call_rate_percent
        ),
        BODY_FONT_SIZE,
    );
    write_bullet(
        &mut pages,
        &mut ops,
        &mut cursor_y,
        &mut doc,
        &format!(
            "Actionable findings: {} · High-confidence findings: {}",
            package.results.summary.actionable_finding_count,
            package.results.summary.high_confidence_finding_count
        ),
        BODY_FONT_SIZE,
    );
    write_bullet(
        &mut pages,
        &mut ops,
        &mut cursor_y,
        &mut doc,
        &format!(
            "Family coverage — PGx: {} · Met/Cardio: {} · Neuro/Cognitive: {} · Traits: {}",
            package.results.summary.family_counts.pharmacogenomics,
            package
                .results
                .summary
                .family_counts
                .metabolic_cardiovascular,
            package
                .results
                .summary
                .family_counts
                .neuropsychiatric_cognitive,
            package.results.summary.family_counts.traits,
        ),
        BODY_FONT_SIZE,
    );

    if !package.results.quality.caveats.is_empty() {
        write_section_heading(
            &mut pages,
            &mut ops,
            &mut cursor_y,
            &mut doc,
            "Quality Caveats",
        );
        for caveat in &package.results.quality.caveats {
            write_bullet(
                &mut pages,
                &mut ops,
                &mut cursor_y,
                &mut doc,
                caveat,
                BODY_FONT_SIZE,
            );
        }
    }

    write_section_heading(
        &mut pages,
        &mut ops,
        &mut cursor_y,
        &mut doc,
        "Deterministic Recommendations",
    );
    write_subheading_and_bullets(
        &mut pages,
        &mut ops,
        &mut cursor_y,
        &mut doc,
        "Priority Actions",
        &package.results.recommendations.priority_actions,
    );
    write_subheading_and_bullets(
        &mut pages,
        &mut ops,
        &mut cursor_y,
        &mut doc,
        "Clinician Discussion Topics",
        &package.results.recommendations.clinician_discussion_topics,
    );
    write_subheading_and_bullets(
        &mut pages,
        &mut ops,
        &mut cursor_y,
        &mut doc,
        "Lifestyle Focus",
        &package.results.recommendations.lifestyle_focus,
    );
    write_subheading_and_bullets(
        &mut pages,
        &mut ops,
        &mut cursor_y,
        &mut doc,
        "Informational Notes",
        &package.results.recommendations.informational_notes,
    );

    write_structured_section(
        &mut pages,
        &mut ops,
        &mut cursor_y,
        &mut doc,
        "Pharmacogenomics",
        &package.results.finding_groups.pharmacogenomics,
    );
    write_structured_section(
        &mut pages,
        &mut ops,
        &mut cursor_y,
        &mut doc,
        "Metabolic & Cardiovascular",
        &package.results.finding_groups.metabolic_cardiovascular,
    );
    write_structured_section(
        &mut pages,
        &mut ops,
        &mut cursor_y,
        &mut doc,
        "Neuropsychiatric & Cognitive",
        &package.results.finding_groups.neuropsychiatric_cognitive,
    );
    write_structured_section(
        &mut pages,
        &mut ops,
        &mut cursor_y,
        &mut doc,
        "Traits",
        &package.results.finding_groups.traits,
    );

    if !package.report.markdown.trim().is_empty() {
        write_section_heading(
            &mut pages,
            &mut ops,
            &mut cursor_y,
            &mut doc,
            "Saved Narrative Report",
        );
        for paragraph in narrative_paragraphs(&package.report.markdown) {
            write_wrapped_paragraph(
                &mut pages,
                &mut ops,
                &mut cursor_y,
                &mut doc,
                &paragraph,
                BODY_FONT_SIZE,
                false,
            );
        }
    }

    end_text_section(&mut ops);
    pages.push(PdfPage::new(Mm(PAGE_WIDTH_MM), Mm(PAGE_HEIGHT_MM), ops));
    append_page_footers(&mut pages, &package.source.file_name);

    doc.with_pages(pages)
        .save(&PdfSaveOptions::default(), &mut Vec::new())
}

fn write_structured_section(
    pages: &mut Vec<PdfPage>,
    ops: &mut Vec<Op>,
    cursor_y: &mut f32,
    doc: &mut PdfDocument,
    title: &str,
    findings: &[StructuredFinding],
) {
    if findings.is_empty() {
        return;
    }

    write_section_heading(pages, ops, cursor_y, doc, title);
    for finding in findings {
        write_wrapped_paragraph(
            pages,
            ops,
            cursor_y,
            doc,
            &format!(
                "{} ({}, {}): {}",
                finding.trait_name, finding.rsid, finding.genotype, finding.interpretation
            ),
            BODY_FONT_SIZE,
            false,
        );
    }
}

fn write_subheading_and_bullets(
    pages: &mut Vec<PdfPage>,
    ops: &mut Vec<Op>,
    cursor_y: &mut f32,
    doc: &mut PdfDocument,
    heading: &str,
    items: &[String],
) {
    if items.is_empty() {
        return;
    }

    write_wrapped_paragraph(pages, ops, cursor_y, doc, heading, H3_FONT_SIZE, true);
    for item in items {
        write_bullet(pages, ops, cursor_y, doc, item, BODY_FONT_SIZE);
    }
}

fn write_section_heading(
    pages: &mut Vec<PdfPage>,
    ops: &mut Vec<Op>,
    cursor_y: &mut f32,
    doc: &mut PdfDocument,
    text: &str,
) {
    ensure_space(pages, ops, cursor_y, doc, 18.0);
    *cursor_y -= SECTION_GAP_MM;
    write_wrapped_paragraph(pages, ops, cursor_y, doc, text, H2_FONT_SIZE, true);
}

fn write_bullet(
    pages: &mut Vec<PdfPage>,
    ops: &mut Vec<Op>,
    cursor_y: &mut f32,
    doc: &mut PdfDocument,
    text: &str,
    size: f32,
) {
    let bullet_text = format!("- {}", text);
    write_wrapped_paragraph(pages, ops, cursor_y, doc, &bullet_text, size, false);
}

fn write_wrapped_paragraph(
    pages: &mut Vec<PdfPage>,
    ops: &mut Vec<Op>,
    cursor_y: &mut f32,
    doc: &mut PdfDocument,
    text: &str,
    size: f32,
    bold: bool,
) {
    let sanitized = sanitize_pdf_text(text);
    let line_height = if size <= SMALL_FONT_SIZE {
        SMALL_LINE_HEIGHT_MM
    } else {
        LINE_HEIGHT_MM.max(size * 0.48)
    };
    let max_chars = max_chars_for_size(size);

    for line in wrap_text(&sanitized, max_chars) {
        ensure_space(pages, ops, cursor_y, doc, line_height + 1.2);
        write_line(ops, *cursor_y, &line, size, bold);
        *cursor_y -= line_height;
    }

    *cursor_y -= 1.4;
}

fn write_line(ops: &mut Vec<Op>, y_mm: f32, text: &str, size: f32, bold: bool) {
    let font = if bold {
        BuiltinFont::HelveticaBold
    } else {
        BuiltinFont::Helvetica
    };

    ops.push(Op::StartTextSection);
    ops.push(Op::SetTextCursor {
        pos: Point::new(Mm(LEFT_MARGIN_MM), Mm(y_mm)),
    });
    ops.push(Op::SetFont {
        font: PdfFontHandle::Builtin(font),
        size: Pt(size),
    });
    ops.push(Op::ShowText {
        items: vec![TextItem::Text(text.to_string())],
    });
    ops.push(Op::EndTextSection);
}

fn ensure_space(
    pages: &mut Vec<PdfPage>,
    ops: &mut Vec<Op>,
    cursor_y: &mut f32,
    _doc: &mut PdfDocument,
    needed_mm: f32,
) {
    if *cursor_y - needed_mm < BOTTOM_MARGIN_MM {
        pages.push(PdfPage::new(
            Mm(PAGE_WIDTH_MM),
            Mm(PAGE_HEIGHT_MM),
            std::mem::take(ops),
        ));
        *cursor_y = PAGE_HEIGHT_MM - TOP_MARGIN_MM;
    }
}

fn narrative_paragraphs(markdown: &str) -> Vec<String> {
    markdown
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.trim_start_matches("## ")
                .trim_start_matches("### ")
                .trim_start_matches("- ")
                .trim_start_matches("* ")
                .replace('`', "")
                .replace("**", "")
                .replace('*', "")
        })
        .collect()
}

fn sanitize_pdf_text(text: &str) -> String {
    text.replace('•', "-")
        .replace('·', ";")
        .replace('—', "-")
        .replace('–', "-")
        .replace('−', "-")
        .replace('“', "\"")
        .replace('”', "\"")
        .replace('’', "'")
        .replace('‘', "'")
        .replace('…', "...")
        .replace('ε', "epsilon")
}

fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        if current.is_empty() {
            current.push_str(word);
            continue;
        }

        if current.len() + 1 + word.len() <= max_chars {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(current);
            current = word.to_string();
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}

fn max_chars_for_size(size: f32) -> usize {
    let usable_width = PAGE_WIDTH_MM - LEFT_MARGIN_MM - RIGHT_MARGIN_MM;
    ((usable_width / (size * CHAR_WIDTH_FACTOR)).floor() as usize).max(36)
}

fn append_page_footers(pages: &mut [PdfPage], source_file_name: &str) {
    let total_pages = pages.len();

    for (index, page) in pages.iter_mut().enumerate() {
        let footer_text = format!(
            "{} · page {} of {}",
            source_file_name,
            index + 1,
            total_pages
        );
        let y_mm = BOTTOM_MARGIN_MM - 5.0;
        let font = BuiltinFont::Helvetica;

        page.ops.push(Op::StartTextSection);
        page.ops.push(Op::SetTextCursor {
            pos: Point::new(Mm(LEFT_MARGIN_MM), Mm(y_mm)),
        });
        page.ops.push(Op::SetFont {
            font: PdfFontHandle::Builtin(font),
            size: Pt(8.5),
        });
        page.ops.push(Op::ShowText {
            items: vec![TextItem::Text(footer_text)],
        });
        page.ops.push(Op::EndTextSection);
    }
}

fn begin_text_section(_ops: &mut Vec<Op>) {}

fn end_text_section(_ops: &mut Vec<Op>) {}

fn now_unix_ms() -> AppResult<u64> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| {
            crate::error::AppError::Other(format!("system clock is before unix epoch: {e}"))
        })?
        .as_millis() as u64)
}
