pub struct Marker {
    pub rsid: &'static str,
    pub category: &'static str,
    pub trait_name: &'static str,
    pub genotype_map: &'static [(&'static str, &'static str, &'static str)],
}

impl Marker {
    pub fn interpret(&self, genotype: &str) -> Option<(String, &'static str)> {
        let g = sorted_genotype(genotype);
        for (gt, interp, conf) in self.genotype_map.iter() {
            if sorted_genotype(gt) == g {
                return Some((interp.to_string(), conf));
            }
        }
        None
    }
}

fn sorted_genotype(g: &str) -> String {
    let mut bytes: Vec<char> = g.chars().filter(|c| c.is_ascii_alphabetic()).collect();
    bytes.sort();
    bytes.into_iter().collect()
}

pub fn curated_markers() -> &'static [Marker] {
    &MARKERS
}

static MARKERS: [Marker; 28] = [
    Marker {
        rsid: "rs4680",
        category: "Neuropsychiatric",
        trait_name: "COMT — dopamine clearance ('warrior/worrier')",
        genotype_map: &[
            ("AA", "Val/Val — warrior variant, faster dopamine clearance, typically higher stress resilience, lower baseline working-memory in high-dopamine tasks.", "high"),
            ("AG", "Val/Met — balanced clearance.", "high"),
            ("GG", "Met/Met — worrier variant, higher synaptic dopamine, generally stronger working memory but more stress-sensitivity.", "high"),
        ],
    },
    Marker {
        rsid: "rs1815739",
        category: "Athletic",
        trait_name: "ACTN3 — power vs endurance",
        genotype_map: &[
            ("CC", "RR — fast-twitch optimized (power/sprint).", "high"),
            ("CT", "RX — mixed fiber type.", "high"),
            ("TT", "XX — no functional alpha-actinin-3, endurance-biased.", "high"),
        ],
    },
    Marker {
        rsid: "rs9939609",
        category: "Weight",
        trait_name: "FTO — obesity/appetite risk",
        genotype_map: &[
            ("TT", "Lower-risk genotype.", "high"),
            ("AT", "Intermediate risk, ~1.3x BMI increase effect.", "high"),
            ("AA", "Higher-risk genotype, ~1.7x BMI increase effect; diet composition matters more.", "high"),
        ],
    },
    Marker {
        rsid: "rs1801133",
        category: "Methylation",
        trait_name: "MTHFR C677T",
        genotype_map: &[
            ("GG", "Wild-type — full MTHFR activity.", "high"),
            ("AG", "Heterozygous — ~40% reduced activity, monitor folate.", "high"),
            ("AA", "Homozygous — ~70% reduced activity, consider methylated folate.", "high"),
        ],
    },
    Marker {
        rsid: "rs1801131",
        category: "Methylation",
        trait_name: "MTHFR A1298C",
        genotype_map: &[
            ("TT", "Wild-type.", "high"),
            ("GT", "Heterozygous — modest reduction.", "medium"),
            ("GG", "Homozygous — reduced methyl-folate synthesis.", "medium"),
        ],
    },
    Marker {
        rsid: "rs671",
        category: "Pharmacogenomics",
        trait_name: "ALDH2 — alcohol flush",
        genotype_map: &[
            ("GG", "Normal ALDH2 activity.", "high"),
            ("AG", "Reduced activity — flushing, higher esophageal cancer risk with alcohol.", "high"),
            ("AA", "Near-zero activity — strong flush, avoid alcohol.", "high"),
        ],
    },
    Marker {
        rsid: "rs4988235",
        category: "Nutrition",
        trait_name: "LCT — lactase persistence",
        genotype_map: &[
            ("AA", "Lactase persistence — can digest lactose as adult.", "high"),
            ("AG", "Lactase persistence (heterozygous).", "high"),
            ("GG", "Lactase non-persistence — likely lactose intolerance.", "high"),
        ],
    },
    Marker {
        rsid: "rs762551",
        category: "Pharmacogenomics",
        trait_name: "CYP1A2 — caffeine metabolism",
        genotype_map: &[
            ("AA", "Fast metabolizer.", "high"),
            ("AC", "Slow metabolizer — caffeine lingers; watch evening intake.", "medium"),
            ("CC", "Slow metabolizer — elevated cardiac risk with high caffeine.", "medium"),
        ],
    },
    Marker {
        rsid: "rs7903146",
        category: "Metabolic",
        trait_name: "TCF7L2 — type 2 diabetes risk",
        genotype_map: &[
            ("CC", "Baseline risk.", "high"),
            ("CT", "~1.4x T2D risk.", "high"),
            ("TT", "~2x T2D risk — metabolic discipline matters.", "high"),
        ],
    },
    Marker {
        rsid: "rs429358",
        category: "Neurodegenerative",
        trait_name: "APOE ε4 (with rs7412)",
        genotype_map: &[
            ("TT", "No ε4 at this site.", "high"),
            ("CT", "ε4 carrier — Alzheimer's risk modifier (pair with rs7412).", "high"),
            ("CC", "ε4/ε4 possible — significantly elevated Alzheimer's risk (pair with rs7412).", "high"),
        ],
    },
    Marker {
        rsid: "rs7412",
        category: "Neurodegenerative",
        trait_name: "APOE (with rs429358)",
        genotype_map: &[
            ("CC", "No ε2.", "high"),
            ("CT", "ε2 carrier.", "high"),
            ("TT", "ε2/ε2.", "high"),
        ],
    },
    Marker {
        rsid: "rs1229984",
        category: "Pharmacogenomics",
        trait_name: "ADH1B — alcohol metabolism",
        genotype_map: &[
            ("CC", "Typical metabolism.", "high"),
            ("CT", "Faster acetaldehyde production.", "high"),
            ("TT", "Very fast — less alcohol tolerance.", "high"),
        ],
    },
    Marker {
        rsid: "rs53576",
        category: "Behavioral",
        trait_name: "OXTR — empathy/stress",
        genotype_map: &[
            ("GG", "Higher self-reported empathy.", "medium"),
            ("AG", "Intermediate.", "medium"),
            ("AA", "Lower empathy tendency, more stress reactivity.", "medium"),
        ],
    },
    Marker {
        rsid: "rs6265",
        category: "Neuropsychiatric",
        trait_name: "BDNF Val66Met",
        genotype_map: &[
            ("GG", "Val/Val — typical BDNF secretion.", "high"),
            ("AG", "Val/Met — reduced activity-dependent BDNF.", "high"),
            ("AA", "Met/Met — larger reduction, memory differences.", "high"),
        ],
    },
    Marker {
        rsid: "rs17822931",
        category: "Traits",
        trait_name: "ABCC11 — earwax / body odor",
        genotype_map: &[
            ("CC", "Wet earwax, typical odor.", "high"),
            ("CT", "Wet earwax (carrier).", "high"),
            ("TT", "Dry earwax, minimal axillary odor.", "high"),
        ],
    },
    Marker {
        rsid: "rs12913832",
        category: "Traits",
        trait_name: "HERC2 — eye color",
        genotype_map: &[
            ("AA", "Brown eyes most likely.", "high"),
            ("AG", "Mixed/hazel/green likely.", "medium"),
            ("GG", "Blue eyes most likely.", "high"),
        ],
    },
    Marker {
        rsid: "rs1042713",
        category: "Pharmacogenomics",
        trait_name: "ADRB2 — beta-2 agonist response",
        genotype_map: &[
            ("AA", "Gly16Gly — reduced short-term bronchodilator response.", "medium"),
            ("AG", "Intermediate.", "medium"),
            ("GG", "Arg16Arg — typical response.", "medium"),
        ],
    },
    Marker {
        rsid: "rs4986893",
        category: "Pharmacogenomics",
        trait_name: "CYP2C19 *3 — clopidogrel",
        genotype_map: &[
            ("GG", "Normal CYP2C19.", "high"),
            ("AG", "Intermediate metabolizer.", "high"),
            ("AA", "Poor metabolizer — reduced clopidogrel activation.", "high"),
        ],
    },
    Marker {
        rsid: "rs4244285",
        category: "Pharmacogenomics",
        trait_name: "CYP2C19 *2",
        genotype_map: &[
            ("GG", "Normal.", "high"),
            ("AG", "Intermediate metabolizer.", "high"),
            ("AA", "Poor metabolizer.", "high"),
        ],
    },
    Marker {
        rsid: "rs1799752",
        category: "Cardiovascular",
        trait_name: "ACE I/D (proxy)",
        genotype_map: &[
            ("II", "Endurance-favorable I/I.", "medium"),
            ("ID", "Mixed.", "medium"),
            ("DD", "Power-favorable, slightly higher hypertension risk.", "medium"),
        ],
    },
    Marker {
        rsid: "rs3892097",
        category: "Pharmacogenomics",
        trait_name: "CYP2D6 *4 — many antidepressants/opioids",
        genotype_map: &[
            ("GG", "Normal metabolizer.", "high"),
            ("AG", "Intermediate metabolizer.", "high"),
            ("AA", "Poor metabolizer — dose reductions often needed.", "high"),
        ],
    },
    Marker {
        rsid: "rs2231142",
        category: "Metabolic",
        trait_name: "ABCG2 — urate/gout risk",
        genotype_map: &[
            ("GG", "Baseline.", "high"),
            ("GT", "Elevated urate, moderate gout risk.", "high"),
            ("TT", "High urate, elevated gout risk.", "high"),
        ],
    },
    Marker {
        rsid: "rs1695",
        category: "Detoxification",
        trait_name: "GSTP1 Ile105Val",
        genotype_map: &[
            ("AA", "Ile/Ile — typical activity.", "medium"),
            ("AG", "Ile/Val — reduced activity.", "medium"),
            ("GG", "Val/Val — lowest activity; antioxidant support helpful.", "medium"),
        ],
    },
    Marker {
        rsid: "rs1799983",
        category: "Cardiovascular",
        trait_name: "eNOS (NOS3) Glu298Asp",
        genotype_map: &[
            ("GG", "Typical NO synthesis.", "medium"),
            ("GT", "Intermediate.", "medium"),
            ("TT", "Reduced NO — slight CV risk modifier.", "medium"),
        ],
    },
    Marker {
        rsid: "rs2070744",
        category: "Cardiovascular",
        trait_name: "NOS3 -786T>C",
        genotype_map: &[
            ("TT", "Typical expression.", "medium"),
            ("CT", "Intermediate.", "medium"),
            ("CC", "Reduced expression — endothelial function watch.", "medium"),
        ],
    },
    Marker {
        rsid: "rs10757278",
        category: "Cardiovascular",
        trait_name: "9p21 — coronary artery disease",
        genotype_map: &[
            ("AA", "Baseline risk.", "high"),
            ("AG", "~1.25x CAD risk.", "high"),
            ("GG", "~1.6x CAD risk — lifestyle factors dominate control.", "high"),
        ],
    },
    Marker {
        rsid: "rs2802292",
        category: "Longevity",
        trait_name: "FOXO3 — longevity associated",
        genotype_map: &[
            ("TT", "Baseline.", "medium"),
            ("GT", "Associated with longevity outcomes in some cohorts.", "medium"),
            ("GG", "Stronger longevity association.", "medium"),
        ],
    },
    Marker {
        rsid: "rs4977574",
        category: "Cardiovascular",
        trait_name: "9p21 — MI risk",
        genotype_map: &[
            ("AA", "Baseline.", "high"),
            ("AG", "Elevated MI risk.", "high"),
            ("GG", "Further elevated MI risk.", "high"),
        ],
    },
];
