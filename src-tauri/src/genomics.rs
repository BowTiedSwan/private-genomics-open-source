use crate::error::{AppError, AppResult};
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GenomicFormat {
    TwentyThreeAndMe,
    AncestryDna,
    MyHeritage,
    FamilyTreeDna,
    Vcf,
    RsidTsv,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snp {
    pub rsid: String,
    pub chromosome: String,
    pub position: u64,
    pub genotype: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedGenome {
    pub format: GenomicFormat,
    pub total_snps: usize,
    pub autosomal: usize,
    pub x_chromosome: usize,
    pub y_chromosome: usize,
    pub mitochondrial: usize,
    pub no_calls: usize,
    pub call_rate: f64,
    pub chromosomes: HashMap<String, usize>,
    pub sample_snps: Vec<Snp>,
    pub matched_markers: Vec<MatchedMarker>,
    pub sex_inference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchedMarker {
    pub rsid: String,
    pub genotype: String,
    pub category: String,
    pub trait_name: String,
    pub interpretation: String,
    pub confidence: String,
}

fn open_maybe_gzipped(path: &Path) -> AppResult<Box<dyn Read>> {
    let file = File::open(path)?;
    let is_gz = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.eq_ignore_ascii_case("gz"))
        .unwrap_or(false);
    if is_gz {
        Ok(Box::new(GzDecoder::new(file)))
    } else {
        Ok(Box::new(file))
    }
}

pub fn detect_format(path: &Path) -> AppResult<GenomicFormat> {
    let reader = open_maybe_gzipped(path)?;
    let buf = BufReader::new(reader);
    for (i, line) in buf.lines().enumerate() {
        if i > 60 {
            break;
        }
        let line = line?;
        let lower = line.to_ascii_lowercase();
        if lower.contains("23andme") {
            return Ok(GenomicFormat::TwentyThreeAndMe);
        }
        if lower.contains("ancestrydna") || lower.contains("ancestry.com") {
            return Ok(GenomicFormat::AncestryDna);
        }
        if lower.contains("myheritage") {
            return Ok(GenomicFormat::MyHeritage);
        }
        if lower.contains("familytreedna") || lower.contains("ftdna") {
            return Ok(GenomicFormat::FamilyTreeDna);
        }
        if line.starts_with("##fileformat=VCF") || line.starts_with("#CHROM") {
            return Ok(GenomicFormat::Vcf);
        }
        if !line.starts_with('#') && !line.trim().is_empty() {
            let cols: Vec<&str> = line.split('\t').collect();
            if cols.len() >= 4 && cols[0].starts_with("rs") {
                return Ok(GenomicFormat::RsidTsv);
            }
        }
    }
    Ok(GenomicFormat::Unknown)
}

fn normalize_chrom(c: &str) -> String {
    let c = c.trim_start_matches("chr").to_ascii_uppercase();
    match c.as_str() {
        "MT" | "M" => "MT".into(),
        "23" => "X".into(),
        "24" => "Y".into(),
        "25" => "MT".into(),
        _ => c,
    }
}

fn is_no_call(g: &str) -> bool {
    g.is_empty() || g.contains("--") || g.contains("00") || g == "." || g.contains("./.")
}

pub fn parse(path: &Path) -> AppResult<ParsedGenome> {
    let format = detect_format(path)?;
    let reader = open_maybe_gzipped(path)?;
    let buf = BufReader::new(reader);

    let mut snps: Vec<Snp> = Vec::with_capacity(600_000);
    let mut chromosomes: HashMap<String, usize> = HashMap::new();
    let mut no_calls = 0usize;

    for line in buf.lines() {
        let line = line?;
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = line.split('\t').collect();
        let snp = match format {
            GenomicFormat::TwentyThreeAndMe | GenomicFormat::MyHeritage | GenomicFormat::FamilyTreeDna | GenomicFormat::RsidTsv => {
                if cols.len() < 4 { continue; }
                Snp {
                    rsid: cols[0].trim_matches('"').to_string(),
                    chromosome: normalize_chrom(cols[1].trim_matches('"')),
                    position: cols[2].trim_matches('"').parse().unwrap_or(0),
                    genotype: cols[3].trim_matches('"').to_uppercase(),
                }
            }
            GenomicFormat::AncestryDna => {
                if cols.len() < 5 { continue; }
                let a1 = cols[3].trim_matches('"');
                let a2 = cols[4].trim_matches('"');
                Snp {
                    rsid: cols[0].trim_matches('"').to_string(),
                    chromosome: normalize_chrom(cols[1].trim_matches('"')),
                    position: cols[2].trim_matches('"').parse().unwrap_or(0),
                    genotype: format!("{}{}", a1, a2).to_uppercase(),
                }
            }
            GenomicFormat::Vcf => {
                if cols.len() < 10 { continue; }
                let ref_a = cols[3];
                let alt_a = cols[4];
                let fmt = cols[8];
                let sample = cols[9];
                let gt_idx = fmt.split(':').position(|f| f == "GT").unwrap_or(0);
                let gt_raw = sample.split(':').nth(gt_idx).unwrap_or("./.");
                let alleles: Vec<&str> = gt_raw.split(|c| c == '|' || c == '/').collect();
                let genotype: String = alleles.iter().map(|a| match *a {
                    "0" => ref_a,
                    "1" => alt_a,
                    _ => "N",
                }).collect::<Vec<_>>().join("").to_uppercase();
                Snp {
                    rsid: cols[2].to_string(),
                    chromosome: normalize_chrom(cols[0]),
                    position: cols[1].parse().unwrap_or(0),
                    genotype,
                }
            }
            GenomicFormat::Unknown => {
                return Err(AppError::Parse("unrecognized genomic file format".into()));
            }
        };
        if is_no_call(&snp.genotype) {
            no_calls += 1;
        }
        *chromosomes.entry(snp.chromosome.clone()).or_insert(0) += 1;
        snps.push(snp);
    }

    if snps.is_empty() {
        return Err(AppError::Parse("no SNPs parsed from file".into()));
    }

    let autosomal: usize = chromosomes
        .iter()
        .filter(|(k, _)| k.parse::<u32>().is_ok())
        .map(|(_, v)| *v)
        .sum();
    let x_chromosome = *chromosomes.get("X").unwrap_or(&0);
    let y_chromosome = *chromosomes.get("Y").unwrap_or(&0);
    let mitochondrial = *chromosomes.get("MT").unwrap_or(&0);

    let call_rate = 1.0 - (no_calls as f64 / snps.len() as f64);
    let sex_inference = if y_chromosome > 100 { "XY (male)".into() } else if x_chromosome > 0 { "XX (female)".into() } else { "unknown".into() };

    let matched_markers = match_known_markers(&snps);
    let sample_snps: Vec<Snp> = snps.iter().take(20).cloned().collect();

    Ok(ParsedGenome {
        format,
        total_snps: snps.len(),
        autosomal,
        x_chromosome,
        y_chromosome,
        mitochondrial,
        no_calls,
        call_rate,
        chromosomes,
        sample_snps,
        matched_markers,
        sex_inference,
    })
}

fn match_known_markers(snps: &[Snp]) -> Vec<MatchedMarker> {
    let db = crate::markers::curated_markers();
    let lookup: HashMap<&str, &Snp> = snps.iter().map(|s| (s.rsid.as_str(), s)).collect();
    let mut out = Vec::new();
    for m in db.iter() {
        if let Some(snp) = lookup.get(m.rsid) {
            let interp = m.interpret(&snp.genotype);
            if let Some((interpretation, confidence)) = interp {
                out.push(MatchedMarker {
                    rsid: m.rsid.to_string(),
                    genotype: snp.genotype.clone(),
                    category: m.category.to_string(),
                    trait_name: m.trait_name.to_string(),
                    interpretation,
                    confidence: confidence.to_string(),
                });
            }
        }
    }
    out
}
