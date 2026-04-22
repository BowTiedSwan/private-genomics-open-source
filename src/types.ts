export type GenomicFormat =
  | "twenty_three_and_me"
  | "ancestry_dna"
  | "my_heritage"
  | "family_tree_dna"
  | "vcf"
  | "rsid_tsv"
  | "unknown";

export interface Snp {
  rsid: string;
  chromosome: string;
  position: number;
  genotype: string;
}

export interface MatchedMarker {
  rsid: string;
  genotype: string;
  category: string;
  trait_name: string;
  interpretation: string;
  confidence: string;
}

export interface ParsedGenome {
  format: GenomicFormat;
  total_snps: number;
  autosomal: number;
  x_chromosome: number;
  y_chromosome: number;
  mitochondrial: number;
  no_calls: number;
  call_rate: number;
  chromosomes: Record<string, number>;
  sample_snps: Snp[];
  matched_markers: MatchedMarker[];
  sex_inference: string;
}

export interface ModelInfo {
  id: string;
  name: string;
  tee: boolean;
  web: boolean;
  description: string;
}

export interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}
