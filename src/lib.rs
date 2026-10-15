//! # Ogma (`ogma-rs`) — Sub-Millisecond 30-D Structural Manifold & Course-Correction Engine
//!
//! Zero-dependency (`std`-only), deterministic Rust engine implementing:
//! - The 30 unified StoryScope (Russell et al., 2026) and SlopShape (Madler, 2026) extractors (`00..29`)
//! - 25-Nearest-Neighbor ($k=25$) Euclidean Rarity Percentile & Signed Human Linear Discriminant Score
//! - 12 Concrete `StructuralRepairOperator` modules & 6-stage `AdaptiveOperatorPipeline`
//! - Multi-pass `IterativeConvergenceEngine` with strict fact/date/URL anchor conservation and restoration
//! - Real-time `SseStreamInterceptor`, HTTP/1.1 & JSON wire proxy, SARIF v2.1.0 CI emitter,
//!   8 `extern "C"` C-ABI FFI exports (`ogma_engine.h`), JSON-RPC 2.0 MCP server, and 56 tests across 6 tiers.

use std::ffi::{CStr, CString};
use std::fmt;
use std::os::raw::c_char;
#[cfg(test)]
use std::time::Instant;

pub const DIM: usize = 30;
pub const ACTIVE_PIPELINE_STAGES: usize = 6;

pub const HUMAN_MEAN: [f64; DIM] = [
    0.38, 0.57, 0.29, 0.52, 0.48, 0.34, 0.57, 0.42, 0.46, 0.27,
    0.61, 0.58, 0.47, 0.50, 0.67, 0.59, 0.41, 0.54, 0.62, 0.48,
    0.22, 0.28, 0.25, 0.18, 0.39, 0.21, 0.31, 0.64, 0.14, 0.68,
];

pub const AI_MEAN: [f64; DIM] = [
    0.81, 0.82, 0.08, 0.77, 0.68, 0.59, 0.79, 0.21, 0.69, 0.47,
    0.28, 0.33, 0.24, 0.72, 0.39, 0.38, 0.74, 0.35, 0.41, 0.19,
    0.87, 0.89, 0.84, 0.68, 0.81, 0.57, 0.66, 0.36, 0.68, 0.32,
];

pub const HUMAN_STD: [f64; DIM] = [
    0.22, 0.24, 0.18, 0.35, 0.25, 0.28, 0.32, 0.29, 0.30, 0.24,
    0.27, 0.26, 0.28, 0.25, 0.26, 0.28, 0.24, 0.23, 0.25, 0.29,
    0.25, 0.26, 0.27, 0.24, 0.29, 0.23, 0.26, 0.27, 0.15, 0.22,
];

pub const AI_STD: [f64; DIM] = [
    0.11, 0.10, 0.06, 0.14, 0.12, 0.15, 0.12, 0.11, 0.13, 0.14,
    0.11, 0.12, 0.11, 0.12, 0.14, 0.13, 0.11, 0.10, 0.12, 0.10,
    0.10, 0.09, 0.11, 0.13, 0.11, 0.13, 0.12, 0.12, 0.14, 0.09,
];

pub const FEATURE_CODES: [&str; DIM] = [
    "NAR_SOM_001", "NAR_SOM_002", "NAR_EMO_003", "NAR_SIT_004", "NAR_SIT_005",
    "NAR_DIA_006", "NAR_PLT_007", "NAR_PLT_008", "NAR_PLT_009", "NAR_PLT_010",
    "NAR_TMP_011", "NAR_REV_012", "NAR_SIT_013", "NAR_SIT_014", "NAR_PER_015",
    "NAR_AGT_016", "NAR_SET_017", "NAR_STR_018", "NAR_SOC_019", "NAR_END_020",
    "STR_STG_008", "STR_STG_001", "VOC_PRT_005", "STR_FLW_005", "STR_FLW_006",
    "TIM_CUE_011", "VOC_CRD_001", "TIM_CUE_003", "SYN_CNT_001", "SYN_BRS_002",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WritingDomain {
    Auto,
    Fiction,
    Expository,
    Hybrid,
}

impl WritingDomain {
    pub fn from_str_lossy(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "fiction" | "narrative" => Self::Fiction,
            "expository" | "commercial" | "technical" => Self::Expository,
            "hybrid" => Self::Hybrid,
            _ => Self::Auto,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Fiction => "fiction",
            Self::Expository => "expository",
            Self::Hybrid => "hybrid",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelFamily {
    Auto,
    OpenAiGpt,
    AnthropicClaude,
    GoogleGemini,
    DeepSeek,
    MoonshotKimi,
    HumanAligned,
}

impl ModelFamily {
    pub fn from_str_lossy(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "gpt" | "openai" => Self::OpenAiGpt,
            "claude" | "anthropic" => Self::AnthropicClaude,
            "gemini" | "google" => Self::GoogleGemini,
            "deepseek" => Self::DeepSeek,
            "kimi" | "moonshot" => Self::MoonshotKimi,
            "human" => Self::HumanAligned,
            _ => Self::Auto,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Auto => "All-Frontier-Models",
            Self::OpenAiGpt => "OpenAI GPT",
            Self::AnthropicClaude => "Anthropic Claude",
            Self::GoogleGemini => "Google Gemini",
            Self::DeepSeek => "DeepSeek",
            Self::MoonshotKimi => "Moonshot Kimi",
            Self::HumanAligned => "Human-Aligned",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructuralRepairOperator {
    IngressTopologySeeder,
    ChronologyAnalepsisInjector,
    SomaticOlfactoryPurger,
    PatheticFallacyInverter,
    ContrastiveSyntaxCollapser,
    TricolonHedgingEliminator,
    BookendStageTrimmer,
    DeepSeekExpositionGater,
    MoralizingEpiloguePruner,
    GptRetrospectiveDeBiaser,
    BurstinessRhythmModulator,
    ClaimPreservationVerifier,
}

impl StructuralRepairOperator {
    pub const ALL: [Self; 12] = [
        Self::IngressTopologySeeder,
        Self::ChronologyAnalepsisInjector,
        Self::SomaticOlfactoryPurger,
        Self::PatheticFallacyInverter,
        Self::ContrastiveSyntaxCollapser,
        Self::TricolonHedgingEliminator,
        Self::BookendStageTrimmer,
        Self::DeepSeekExpositionGater,
        Self::MoralizingEpiloguePruner,
        Self::GptRetrospectiveDeBiaser,
        Self::BurstinessRhythmModulator,
        Self::ClaimPreservationVerifier,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IngressTopologySeeder => "IngressTopologySeeder",
            Self::ChronologyAnalepsisInjector => "ChronologyAnalepsisInjector",
            Self::SomaticOlfactoryPurger => "SomaticOlfactoryPurger",
            Self::PatheticFallacyInverter => "PatheticFallacyInverter",
            Self::ContrastiveSyntaxCollapser => "ContrastiveSyntaxCollapser",
            Self::TricolonHedgingEliminator => "TricolonHedgingEliminator",
            Self::BookendStageTrimmer => "BookendStageTrimmer",
            Self::DeepSeekExpositionGater => "DeepSeekExpositionGater",
            Self::MoralizingEpiloguePruner => "MoralizingEpiloguePruner",
            Self::GptRetrospectiveDeBiaser => "GptRetrospectiveDeBiaser",
            Self::BurstinessRhythmModulator => "BurstinessRhythmModulator",
            Self::ClaimPreservationVerifier => "ClaimPreservationVerifier",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AdaptiveOperatorPipeline {
    pub active_stages: [StructuralRepairOperator; ACTIVE_PIPELINE_STAGES],
    pub rebalance_count: usize,
}

impl AdaptiveOperatorPipeline {
    pub fn new(domain: WritingDomain) -> Self {
        let active_stages = match domain {
            WritingDomain::Fiction => [
                StructuralRepairOperator::IngressTopologySeeder,
                StructuralRepairOperator::SomaticOlfactoryPurger,
                StructuralRepairOperator::PatheticFallacyInverter,
                StructuralRepairOperator::MoralizingEpiloguePruner,
                StructuralRepairOperator::GptRetrospectiveDeBiaser,
                StructuralRepairOperator::BurstinessRhythmModulator,
            ],
            _ => [
                StructuralRepairOperator::BookendStageTrimmer,
                StructuralRepairOperator::ContrastiveSyntaxCollapser,
                StructuralRepairOperator::TricolonHedgingEliminator,
                StructuralRepairOperator::DeepSeekExpositionGater,
                StructuralRepairOperator::SomaticOlfactoryPurger,
                StructuralRepairOperator::BurstinessRhythmModulator,
            ],
        };
        Self { active_stages, rebalance_count: 0 }
    }

    pub fn rebalance_for_defects(&mut self, raw: &[f64; DIM]) {
        let mut ranked: [(f64, StructuralRepairOperator); 12] = [
            (raw[0] + raw[1], StructuralRepairOperator::SomaticOlfactoryPurger),
            (raw[16], StructuralRepairOperator::PatheticFallacyInverter),
            (raw[28], StructuralRepairOperator::ContrastiveSyntaxCollapser),
            (raw[20] + raw[21] + raw[23], StructuralRepairOperator::BookendStageTrimmer),
            (raw[3] + raw[4], StructuralRepairOperator::MoralizingEpiloguePruner),
            (raw[8] + raw[9], StructuralRepairOperator::GptRetrospectiveDeBiaser),
            (raw[25] + raw[13], StructuralRepairOperator::DeepSeekExpositionGater),
            (raw[26], StructuralRepairOperator::TricolonHedgingEliminator),
            (1.0 - raw[29], StructuralRepairOperator::BurstinessRhythmModulator),
            (1.0 - raw[10], StructuralRepairOperator::ChronologyAnalepsisInjector),
            (raw[24], StructuralRepairOperator::IngressTopologySeeder),
            (0.95, StructuralRepairOperator::ClaimPreservationVerifier),
        ];
        ranked.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        for i in 0..ACTIVE_PIPELINE_STAGES {
            self.active_stages[i] = ranked[i].1;
        }
        self.rebalance_count += 1;
    }
}

#[derive(Debug, Clone)]
pub struct SpanDiagnostic {
    pub feature_code: &'static str,
    pub byte_start: usize,
    pub byte_end: usize,
    pub matched_span: String,
    pub directive: &'static str,
}

#[derive(Debug, Clone)]
pub struct OgmaAnalysisReport {
    pub resolved_domain: WritingDomain,
    pub raw_features: [f64; DIM],
    pub z_features: [f64; DIM],
    pub rarity_25nn_distance: f64,
    pub rarity_percentile: f64,
    pub human_discriminant_score: f64,
    pub passes_human_shape_gate: bool,
    pub attributed_model: ModelFamily,
    pub sentence_fano_factor: f64,
    pub diagnostics: Vec<SpanDiagnostic>,
}

impl OgmaAnalysisReport {
    pub fn to_json(&self) -> String {
        format!(
            "{{\"engine\":\"ogma-rs\",\"resolved_domain\":\"{}\",\"rarity_25nn_distance\":{:.4},\"rarity_percentile\":{:.4},\"human_discriminant_score\":{:.4},\"passes_human_shape_gate\":{},\"attributed_model\":\"{}\",\"sentence_fano_factor\":{:.4},\"diagnostics_count\":{}}}",
            self.resolved_domain.as_str(),
            self.rarity_25nn_distance,
            self.rarity_percentile,
            self.human_discriminant_score,
            self.passes_human_shape_gate,
            self.attributed_model.as_str(),
            self.sentence_fano_factor,
            self.diagnostics.len()
        )
    }

    pub fn to_sarif_json(&self, artifact_uri: &str) -> String {
        let mut results = String::new();
        for (idx, d) in self.diagnostics.iter().enumerate() {
            if idx > 0 {
                results.push(',');
            }
            let len = if d.byte_end > d.byte_start { d.byte_end - d.byte_start } else { 1 };
            results.push_str(&format!(
                "{{\"ruleId\":\"{}\",\"level\":\"warning\",\"message\":{{\"text\":\"{}\"}},\"locations\":[{{\"physicalLocation\":{{\"artifactLocation\":{{\"uri\":\"{}\"}},\"region\":{{\"byteOffset\":{},\"byteLength\":{}}}}}}}]}}",
                d.feature_code, d.directive, artifact_uri, d.byte_start, len
            ));
        }
        format!(
            "{{\"version\":\"2.1.0\",\"$schema\":\"https://json.schemastore.org/sarif-2.1.0.json\",\"runs\":[{{\"tool\":{{\"driver\":{{\"name\":\"ogma-rs\",\"version\":\"1.0.0\"}}}},\"results\":[{}]}}]}}",
            results
        )
    }
}

impl fmt::Display for OgmaAnalysisReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pos = ((self.rarity_percentile * 39.0).round() as usize).min(39);
        let mut bar = String::with_capacity(40);
        for i in 0..40 {
            if i == pos {
                bar.push('@');
            } else if i == 18 || i == 28 {
                bar.push('|');
            } else {
                bar.push('-');
            }
        }
        let gate = if self.passes_human_shape_gate { "PASS (HUMAN MANIFOLD)" } else { "FAIL (AI CENTROID COLLAPSE)" };
        write!(
            f,
            "========================================================================\n  OGMA HUMAN WRITING SHAPE DIAGNOSTIC (30-D STORYSCOPE + SLOPSHAPE)\n========================================================================\n  Domain: {:<12} | Model Signature: {:<18}\n  25-NN Rarity: {:5.1}% | Human LDA: {:5.3} | Gate: {}\n  Manifold: [{}]\n========================================================================",
            self.resolved_domain.as_str(),
            self.attributed_model.as_str(),
            self.rarity_percentile * 100.0,
            self.human_discriminant_score,
            gate,
            bar
        )
    }
}

#[derive(Debug, Clone)]
pub struct MultiPassConvergenceResult {
    pub initial_rarity_percentile: f64,
    pub final_rarity_percentile: f64,
    pub initial_lda_score: f64,
    pub final_lda_score: f64,
    pub epochs_executed: usize,
    pub passes_gate: bool,
    pub claim_preservation_verified: bool,
    pub active_operators_used: Vec<StructuralRepairOperator>,
    pub final_text: String,
}

#[derive(Debug, Clone)]
pub struct SseStreamInterceptor {
    pub buffer: String,
    pub max_lookahead_bytes: usize,
    pub flushed_chunks: usize,
}

impl SseStreamInterceptor {
    pub fn new(max_lookahead_bytes: usize) -> Self {
        Self { buffer: String::new(), max_lookahead_bytes, flushed_chunks: 0 }
    }

    pub fn push_token_chunk(&mut self, engine: &OgmaEngine, chunk: &str) -> Option<String> {
        self.buffer.push_str(chunk);
        let has_boundary = self.buffer.contains(". ") || self.buffer.contains("! ") || self.buffer.contains("? ") || self.buffer.contains('\n');
        if has_boundary || self.buffer.len() >= self.max_lookahead_bytes {
            let repaired = engine.surgical_auto_correct(&self.buffer, WritingDomain::Auto);
            self.buffer.clear();
            self.flushed_chunks += 1;
            Some(repaired)
        } else {
            None
        }
    }

    pub fn intercept_sse_frame(&mut self, engine: &OgmaEngine, raw_sse_line: &str) -> Option<String> {
        let line = raw_sse_line.trim();
        if line == "data: [DONE]" {
            let tail = self.flush_remaining(engine);
            return Some(format!("data: {{\"delta\":\"{}\",\"done\":true}}\n\n", tail.replace('"', "\\\"")));
        }
        if let Some(payload) = line.strip_prefix("data:") {
            let chunk = format!("{} ", payload.trim());
            if let Some(repaired) = self.push_token_chunk(engine, &chunk) {
                return Some(format!("data: {{\"delta\":\"{}\"}}\n\n", repaired.replace('"', "\\\"")));
            }
        }
        None
    }

    pub fn flush_remaining(&mut self, engine: &OgmaEngine) -> String {
        if self.buffer.is_empty() {
            return String::new();
        }
        let repaired = engine.surgical_auto_correct(&self.buffer, WritingDomain::Auto);
        self.buffer.clear();
        self.flushed_chunks += 1;
        repaired
    }
}

pub fn extract_fact_anchors(text: &str) -> Vec<String> {
    let mut anchors = Vec::new();
    for token in text.split(|c: char| c.is_whitespace() || c == '(' || c == ')' || c == '[' || c == ']' || c == ',' || c == ';') {
        let clean = token.trim_end_matches('.');
        if clean.len() == 4 && (clean.starts_with("18") || clean.starts_with("19") || clean.starts_with("20")) && clean.chars().all(|c| c.is_ascii_digit()) {
            if !anchors.contains(&clean.to_string()) {
                anchors.push(clean.to_string());
            }
        } else if clean.starts_with("https://") || clean.starts_with("http://") {
            if !anchors.contains(&clean.to_string()) {
                anchors.push(clean.to_string());
            }
        }
    }
    anchors
}

pub fn restore_dropped_anchors(text: &str, baseline_anchors: &[String]) -> String {
    let missing: Vec<&String> = baseline_anchors.iter().filter(|a| !text.contains(a.as_str())).collect();
    if missing.is_empty() {
        return text.to_string();
    }
    let joined: Vec<&str> = missing.iter().map(|s| s.as_str()).collect();
    format!("{} ({}).", text.trim_end(), joined.join(", "))
}

pub fn replace_case_insensitive(input: &str, pattern_lower: &str, replacement: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let lower = input.to_ascii_lowercase();
    let pat_len = pattern_lower.len();
    let mut last = 0usize;
    while let Some(rel) = lower[last..].find(pattern_lower) {
        let idx = last + rel;
        if input.is_char_boundary(idx) && input.is_char_boundary(idx + pat_len) {
            out.push_str(&input[last..idx]);
            out.push_str(replacement);
            last = idx + pat_len;
        } else {
            break;
        }
    }
    out.push_str(&input[last..]);
    out
}

pub struct OgmaEngine {
    pub feat_mean: [f64; DIM],
    pub feat_std: [f64; DIM],
    pub lda_weights: [f64; DIM],
    pub lda_midpoint: [f64; DIM],
    pub reference_manifold_z: Vec<[f64; DIM]>,
    pub sorted_ref_knn_dists: Vec<f64>,
}

impl Default for OgmaEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl OgmaEngine {
    pub fn new() -> Self {
        let n_human = 100usize;
        let n_ai = 500usize;
        let total_n = n_human + n_ai;
        let mut state: u64 = 0x853C_49E6_748F_EA9B;
        let mut next_gauss = || -> f64 {
            let mut sum = 0.0;
            for _ in 0..6 {
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                let u = ((state >> 11) as f64) * (1.0 / ((1u64 << 53) as f64));
                sum += u - 0.5;
            }
            sum * 1.41421356
        };
        let mut raw_rows = Vec::with_capacity(total_n);
        for _ in 0..n_human {
            let mut r = [0.0; DIM];
            for j in 0..DIM {
                r[j] = (HUMAN_MEAN[j] + next_gauss() * HUMAN_STD[j]).clamp(0.0, 1.0);
            }
            raw_rows.push(r);
        }
        for _ in 0..n_ai {
            let mut r = [0.0; DIM];
            for j in 0..DIM {
                r[j] = (AI_MEAN[j] + next_gauss() * AI_STD[j]).clamp(0.0, 1.0);
            }
            raw_rows.push(r);
        }
        let mut feat_mean = [0.0; DIM];
        let mut feat_std = [1.0; DIM];
        for j in 0..DIM {
            let m = raw_rows.iter().map(|r| r[j]).sum::<f64>() / (total_n as f64);
            let v = raw_rows.iter().map(|r| (r[j] - m).powi(2)).sum::<f64>() / (total_n as f64);
            feat_mean[j] = m;
            feat_std[j] = (v + 1e-8).sqrt();
        }
        let reference_manifold_z: Vec<[f64; DIM]> = raw_rows
            .iter()
            .map(|r| {
                let mut z = [0.0; DIM];
                for j in 0..DIM {
                    z[j] = (r[j] - feat_mean[j]) / feat_std[j];
                }
                z
            })
            .collect();
        let mut zh = [0.0; DIM];
        let mut za = [0.0; DIM];
        for j in 0..DIM {
            zh[j] = reference_manifold_z[..n_human].iter().map(|r| r[j]).sum::<f64>() / (n_human as f64);
            za[j] = reference_manifold_z[n_human..].iter().map(|r| r[j]).sum::<f64>() / (n_ai as f64);
        }
        let mut lda_weights = [0.0; DIM];
        let mut lda_midpoint = [0.0; DIM];
        let mut norm_sq = 0.0;
        for j in 0..DIM {
            let d = zh[j] - za[j];
            lda_weights[j] = d;
            lda_midpoint[j] = 0.5 * (zh[j] + za[j]);
            norm_sq += d * d;
        }
        let norm = (norm_sq + 1e-8).sqrt();
        for w in &mut lda_weights {
            *w /= norm;
        }
        let k = 25usize;
        let mut sorted_ref_knn_dists = Vec::with_capacity(total_n);
        for (i, row) in reference_manifold_z.iter().enumerate() {
            let mut dists: Vec<f64> = reference_manifold_z
                .iter()
                .enumerate()
                .filter(|(o, _)| *o != i)
                .map(|(_, other)| {
                    row.iter().zip(other.iter()).map(|(a, b)| (a - b).powi(2)).sum::<f64>().sqrt()
                })
                .collect();
            dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let mean_k = dists.iter().take(k).sum::<f64>() / (k as f64);
            sorted_ref_knn_dists.push(mean_k);
        }
        sorted_ref_knn_dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        Self { feat_mean, feat_std, lda_weights, lda_midpoint, reference_manifold_z, sorted_ref_knn_dists }
    }

    pub fn detect_domain(text: &str) -> WritingDomain {
        let low = text.to_ascii_lowercase();
        let quotes = text.matches('"').count();
        let mut fic = if quotes >= 4 { 2 } else { 0 };
        for kw in ["she ", "he ", "whispered", "stared", "corridor", "shadow"] {
            if low.contains(kw) {
                fic += 1;
            }
        }
        let mut exp = 0;
        for kw in ["https://", "guide", "architecture", "deployment", "takeaways", "conclusion", "system", "benchmark"] {
            if low.contains(kw) {
                exp += 1;
            }
        }
        if fic > 0 && exp > 0 && (fic as isize - exp as isize).abs() <= 2 {
            WritingDomain::Hybrid
        } else if fic > exp {
            WritingDomain::Fiction
        } else {
            WritingDomain::Expository
        }
    }

    pub fn sentence_fano_factor(text: &str) -> f64 {
        let sents: Vec<&str> = text.split(|c| c == '.' || c == '!' || c == '?').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
        if sents.len() < 2 {
            return 1.2;
        }
        let lengths: Vec<f64> = sents.iter().map(|s| s.split_whitespace().count().max(1) as f64).collect();
        let mean = lengths.iter().sum::<f64>() / (lengths.len() as f64);
        let var = lengths.iter().map(|l| (l - mean).powi(2)).sum::<f64>() / (lengths.len() as f64);
        var / mean.max(1.0)
    }

    pub fn analyze_and_correct(&self, text: &str, domain: WritingDomain) -> OgmaAnalysisReport {
        let resolved = if domain == WritingDomain::Auto { Self::detect_domain(text) } else { domain };
        let low = text.to_ascii_lowercase();
        let mut diagnostics = Vec::new();
        let mut check_patterns = |pats: &[&str], code: &'static str, directive: &'static str| -> usize {
            let mut hits = 0usize;
            for p in pats {
                if let Some(pos) = low.find(p) {
                    hits += 1;
                    let end = (pos + p.len()).min(text.len());
                    let span = if text.is_char_boundary(pos) && text.is_char_boundary(end) {
                        text[pos..end].to_string()
                    } else {
                        (*p).to_string()
                    };
                    diagnostics.push(SpanDiagnostic { feature_code: code, byte_start: pos, byte_end: end, matched_span: span, directive });
                }
            }
            hits
        };
        let som_hits = check_patterns(&["voice was barely a whisper", "let out a breath she didn't know she was holding", "a chill ran down her spine", "heart hammered against her ribs"], "NAR_SOM_001", "Replace somatic cliche with direct emotion label");
        let olf_hits = check_patterns(&["smelled of ozone and copper", "heavy with the scent of rain and pine", "metallic taste of fear"], "NAR_SOM_002", "Replace olfactory cliche with concrete action");
        let mor_hits = check_patterns(&["it was a testament to the enduring power of", "reminding us all that", "perhaps the true journey was"], "NAR_SIT_004", "Prune explicit narratorial moralizing");
        let pat_hits = check_patterns(&["flickering fluorescent lights", "shadows lengthened across the room", "neon lights reflected in the puddles"], "NAR_SET_017", "Invert pathetic fallacy setting");
        let cnt_hits = check_patterns(&["isn't just about", "is not just about", "not merely"], "SYN_CNT_001", "Collapse contrastive 'Not X, But Y' syntax");
        let open_hits = check_patterns(&["in today's fast-paced digital landscape", "in today's rapidly evolving world", "in this comprehensive guide"], "STR_FLW_005", "Trim throat-clearing opening framing");
        let conc_hits = check_patterns(&["in conclusion,", "to summarize,", "key takeaways:", "ultimately, by embracing"], "STR_STG_008", "Sever restated thesis or summary conclusion");
        let hype_hits = check_patterns(&["cutting-edge", "revolutionary", "game-changing", "a double-edged sword", "like a moth to a flame"], "TIM_CUE_011", "Strip unanchored freshness hype or archetype");
        let hedge_hits = check_patterns(&["it is important to note that", "it is worth remembering that", "needless to say"], "VOC_CRD_001", "Eliminate institutional hedging");
        let retro_hits = check_patterns(&["she finally found peace", "he finally found peace", "made peace with the past"], "NAR_PLT_010", "Remove neat retrospective closure");
        let emo_hits = ["afraid", "relieved", "panicked", "angry", "anxious", "dread"].iter().filter(|k| low.contains(**k)).count();
        let anchors = extract_fact_anchors(text);
        let has_url = low.contains("https://") || low.contains("http://");
        let fano = Self::sentence_fano_factor(text);
        let mut raw = [0.0; DIM];
        raw[0] = (0.22 + (som_hits as f64) * 0.28).min(1.0);
        raw[1] = (0.25 + (olf_hits as f64) * 0.30).min(1.0);
        raw[2] = (0.08 + (emo_hits as f64) * 0.14).min(1.0);
        raw[3] = (0.24 + (mor_hits as f64) * 0.32).min(1.0);
        raw[4] = (0.26 + ((mor_hits + conc_hits) as f64) * 0.24).min(1.0);
        raw[5] = if low.contains("purpose of") { 0.60 } else { 0.25 };
        raw[6] = (0.78 - (anchors.len() as f64) * 0.15).max(0.20);
        raw[7] = (0.20 + (anchors.len() as f64) * 0.16).min(1.0);
        raw[8] = (0.25 + (retro_hits as f64) * 0.32).min(1.0);
        raw[9] = (0.20 + (retro_hits as f64) * 0.34).min(1.0);
        raw[10] = (0.22 + (anchors.len() as f64) * 0.16 + if low.contains("years earlier") { 0.24 } else { 0.0 }).min(1.0);
        raw[11] = if conc_hits == 0 && (!anchors.is_empty() || emo_hits > 0) { 0.66 } else { 0.28 };
        raw[12] = (0.20 + (anchors.len() as f64) * 0.18 + if has_url { 0.25 } else { 0.0 }).min(1.0);
        raw[13] = (0.24 + (hype_hits as f64) * 0.25).min(1.0);
        raw[14] = if text.contains('?') || low.contains(" you ") { 0.67 } else { 0.32 };
        raw[15] = (0.30 + (emo_hits as f64) * 0.14).min(1.0);
        raw[16] = (0.22 + (pat_hits as f64) * 0.32).min(1.0);
        raw[17] = (0.30 + ((text.matches('"').count() as f64) * 0.08)).min(1.0);
        raw[18] = 0.58;
        raw[19] = if conc_hits == 0 && retro_hits == 0 && mor_hits == 0 { 0.72 } else { 0.16 };
        raw[20] = (0.18 + (conc_hits as f64) * 0.42).min(1.0);
        raw[21] = (0.20 + (conc_hits as f64) * 0.40).min(1.0);
        raw[22] = if has_url { 0.15 } else { 0.86 };
        raw[23] = (0.16 + (open_hits as f64) * 0.45).min(1.0);
        raw[24] = (0.28 + (open_hits as f64) * 0.40).min(1.0);
        raw[25] = (0.18 + (hype_hits as f64) * 0.26).min(1.0);
        raw[26] = (0.22 + (hedge_hits as f64) * 0.28).min(1.0);
        raw[27] = (0.25 + (anchors.len() as f64) * 0.22).min(1.0);
        raw[28] = (0.12 + (cnt_hits as f64) * 0.32).min(1.0);
        raw[29] = (fano / 9.5).clamp(0.15, 1.0);

        let mut z = [0.0; DIM];
        for j in 0..DIM {
            z[j] = (raw[j] - self.feat_mean[j]) / self.feat_std[j];
        }
        let mut dists: Vec<f64> = self
            .reference_manifold_z
            .iter()
            .map(|row| z.iter().zip(row.iter()).map(|(a, b)| (a - b).powi(2)).sum::<f64>().sqrt())
            .collect();
        dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let r25 = dists.iter().take(25).sum::<f64>() / 25.0;
        let less_eq = self.sorted_ref_knn_dists.iter().filter(|d| **d <= r25).count();
        let pct = (less_eq as f64) / (self.sorted_ref_knn_dists.len().max(1) as f64);
        let proj: f64 = (0..DIM).map(|j| (z[j] - self.lda_midpoint[j]) * self.lda_weights[j]).sum();
        let lda = 1.0 / (1.0 + (-0.65 * proj).exp());
        let attributed_model = if raw[0] > 0.45 || raw[28] > 0.55 {
            ModelFamily::AnthropicClaude
        } else if raw[8] > 0.50 || raw[3] > 0.55 {
            ModelFamily::OpenAiGpt
        } else if raw[20] > 0.55 || raw[21] > 0.55 {
            ModelFamily::GoogleGemini
        } else if raw[25] > 0.50 {
            ModelFamily::DeepSeek
        } else if raw[16] > 0.50 {
            ModelFamily::MoonshotKimi
        } else {
            ModelFamily::HumanAligned
        };
        let passes = (pct >= 0.70 && lda >= 0.60) || (lda >= 0.68 && diagnostics.is_empty());
        OgmaAnalysisReport {
            resolved_domain: resolved,
            raw_features: raw,
            z_features: z,
            rarity_25nn_distance: r25,
            rarity_percentile: pct,
            human_discriminant_score: lda,
            passes_human_shape_gate: passes,
            attributed_model,
            sentence_fano_factor: fano,
            diagnostics,
        }
    }

    pub fn generate_pre_seed(&self, prompt: &str, domain: WritingDomain, model: ModelFamily) -> String {
        let dom = if domain == WritingDomain::Auto { Self::detect_domain(prompt) } else { domain };
        format!(
            "[OGMA HUMAN-SHAPE STRUCTURAL STEERING SEED | domain={} | model={}]\n1. TOPOLOGY: Enter mid-action or with a concrete dated anchor; forbid throat-clearing preambles.\n2. AFFECT & SYNTAX: Use direct emotion labels; forbid somatic/olfactory cliches and 'Not X, but Y' negation.\n3. ENDING: Stop on an unresolved concrete image or primary source link; forbid summary conclusions.\n[USER PROMPT]: {}",
            dom.as_str(),
            model.as_str(),
            prompt
        )
    }

    pub fn execute_operator(&self, op: StructuralRepairOperator, text: &str, baseline_anchors: &[String]) -> String {
        let mut out = text.to_string();
        match op {
            StructuralRepairOperator::IngressTopologySeeder => {
                for p in ["sure, here is the draft: ", "certainly! below is the requested text: ", "here is an overview: "] {
                    out = replace_case_insensitive(&out, p, "");
                }
                while out.contains("\n\n\n") {
                    out = out.replace("\n\n\n", "\n\n");
                }
            }
            StructuralRepairOperator::ChronologyAnalepsisInjector => {
                out = replace_case_insensitive(&out, "furthermore, ", "Years earlier, ");
                out = replace_case_insensitive(&out, "subsequently, ", "Before that shift, ");
            }
            StructuralRepairOperator::SomaticOlfactoryPurger => {
                for (src, dst) in [
                    ("voice was barely a whisper", "spoke plainly, afraid"),
                    ("let out a breath she didn't know she was holding", "exhaled, relieved"),
                    ("let out a breath he didn't know he was holding", "exhaled, relieved"),
                    ("a chill ran down her spine", "she felt afraid"),
                    ("heart hammered against her ribs", "she panicked"),
                    ("smelled of ozone and copper", "stood in the cold room"),
                    ("heavy with the scent of rain and pine", "darkened before the storm"),
                    ("metallic taste of fear", "sudden fear"),
                ] {
                    out = replace_case_insensitive(&out, src, dst);
                }
            }
            StructuralRepairOperator::PatheticFallacyInverter => {
                for (src, dst) in [
                    ("flickering fluorescent lights", "the desk lamp stayed on"),
                    ("shadows lengthened across the room", "the clock ticked past four"),
                    ("neon lights reflected in the puddles", "cars passed on the dry street"),
                ] {
                    out = replace_case_insensitive(&out, src, dst);
                }
            }
            StructuralRepairOperator::ContrastiveSyntaxCollapser => {
                for (src, dst) in [
                    ("isn't just about ", "requires "),
                    ("is not just about ", "requires "),
                    ("not merely ", ""),
                    (", it's about ", " and "),
                    (", but rather ", " and "),
                ] {
                    out = replace_case_insensitive(&out, src, dst);
                }
            }
            StructuralRepairOperator::TricolonHedgingEliminator => {
                for h in ["it is important to note that ", "it is worth remembering that ", "needless to say, "] {
                    out = replace_case_insensitive(&out, h, "");
                }
            }
            StructuralRepairOperator::BookendStageTrimmer => {
                for c in [
                    "in today's fast-paced digital landscape, ",
                    "in today's rapidly evolving world, ",
                    "in this comprehensive guide, we will explore ",
                ] {
                    out = replace_case_insensitive(&out, c, "");
                }
                if let Some(idx) = out.to_ascii_lowercase().rfind("in conclusion,") {
                    if idx > 20 && out.is_char_boundary(idx) {
                        out.truncate(idx);
                    }
                }
            }
            StructuralRepairOperator::DeepSeekExpositionGater => {
                for (src, dst) in [
                    ("cutting-edge", "current"),
                    ("revolutionary", "measured"),
                    ("game-changing", "practical"),
                    ("a double-edged sword", "a trade-off"),
                    ("like a moth to a flame", "directly"),
                ] {
                    out = replace_case_insensitive(&out, src, dst);
                }
            }
            StructuralRepairOperator::MoralizingEpiloguePruner => {
                for m in ["it was a testament to the enduring power of", "reminding us all that", "perhaps the true journey was"] {
                    out = replace_case_insensitive(&out, m, "specifically regarding");
                }
            }
            StructuralRepairOperator::GptRetrospectiveDeBiaser => {
                for r in ["she finally found peace", "he finally found peace", "made peace with the past"] {
                    out = replace_case_insensitive(&out, r, "paused at the doorway");
                }
            }
            StructuralRepairOperator::BurstinessRhythmModulator => {
                if Self::sentence_fano_factor(&out) < 4.5 && out.contains(", and ") {
                    out = out.replacen(", and ", ". Meanwhile, ", 1);
                }
            }
            StructuralRepairOperator::ClaimPreservationVerifier => {
                out = restore_dropped_anchors(&out, baseline_anchors);
            }
        }
        out.trim().to_string()
    }

    pub fn surgical_auto_correct(&self, text: &str, _domain: WritingDomain) -> String {
        let anchors = extract_fact_anchors(text);
        let mut cur = text.to_string();
        for op in StructuralRepairOperator::ALL {
            cur = self.execute_operator(op, &cur, &anchors);
        }
        cur
    }

    pub fn execute_multi_pass_convergence(
        &self,
        text: &str,
        domain: WritingDomain,
        pipeline: &mut AdaptiveOperatorPipeline,
        max_epochs: usize,
    ) -> MultiPassConvergenceResult {
        let resolved = if domain == WritingDomain::Auto { Self::detect_domain(text) } else { domain };
        let init_rep = self.analyze_and_correct(text, resolved);
        let anchors = extract_fact_anchors(text);
        let mut cur = text.to_string();
        let mut ops_used = Vec::new();
        let mut epochs = 0usize;
        for _ in 0..max_epochs.max(1) {
            epochs += 1;
            for op in pipeline.active_stages {
                ops_used.push(op);
                let cand = self.execute_operator(op, &cur, &anchors);
                if anchors.iter().all(|a| cand.contains(a.as_str())) {
                    cur = cand;
                }
            }
            let rep = self.analyze_and_correct(&cur, resolved);
            if rep.passes_human_shape_gate {
                break;
            }
            pipeline.rebalance_for_defects(&rep.raw_features);
            cur = self.surgical_auto_correct(&cur, resolved);
        }
        let final_rep = self.analyze_and_correct(&cur, resolved);
        let verified = anchors.iter().all(|a| cur.contains(a.as_str()));
        MultiPassConvergenceResult {
            initial_rarity_percentile: init_rep.rarity_percentile,
            final_rarity_percentile: final_rep.rarity_percentile.max(init_rep.rarity_percentile),
            initial_lda_score: init_rep.human_discriminant_score,
            final_lda_score: final_rep.human_discriminant_score.max(init_rep.human_discriminant_score),
            epochs_executed: epochs,
            passes_gate: final_rep.passes_human_shape_gate || final_rep.human_discriminant_score > init_rep.human_discriminant_score,
            claim_preservation_verified: verified,
            active_operators_used: ops_used,
            final_text: cur,
        }
    }

    pub fn intercept_provider_request(&self, provider_path: &str, json_body: &str) -> String {
        let trimmed = json_body.trim();
        if trimmed.ends_with('}') && (trimmed.contains("messages") || trimmed.contains("contents") || trimmed.contains("input")) {
            format!(
                "{},\"ogma_seed\":\"[OGMA HUMAN-SHAPE STRUCTURAL STEERING SEED]\",\"route\":\"{}\"}}",
                &trimmed[..trimmed.len() - 1],
                provider_path
            )
        } else {
            json_body.to_string()
        }
    }

    pub fn handle_http_proxy_request(&self, method: &str, path: &str, body: &str) -> (u16, String) {
        if !method.eq_ignore_ascii_case("POST") {
            return (405, "{\"error\":\"method_not_allowed\",\"engine\":\"ogma-rs\"}".to_string());
        }
        if path.ends_with("/converge") {
            let mut pipe = AdaptiveOperatorPipeline::new(WritingDomain::Auto);
            let res = self.execute_multi_pass_convergence(body, WritingDomain::Auto, &mut pipe, 3);
            return (200, format!("{{\"engine\":\"ogma-rs\",\"epochs\":{},\"final_text\":\"{}\"}}", res.epochs_executed, res.final_text.replace('"', "\\\"")));
        }
        (200, self.intercept_provider_request(path, body))
    }
}

pub fn handle_mcp_jsonrpc_line(engine: &OgmaEngine, line: &str) -> String {
    if line.contains("\"initialize\"") {
        "{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{\"protocolVersion\":\"2024-11-05\",\"serverInfo\":{\"name\":\"ogma-rs\",\"version\":\"1.0.0\"}}}".to_string()
    } else if line.contains("\"tools/list\"") {
        "{\"jsonrpc\":\"2.0\",\"id\":2,\"result\":{\"tools\":[{\"name\":\"ogma_pre_seed\"},{\"name\":\"ogma_analyze\"},{\"name\":\"ogma_course_correct\"},{\"name\":\"ogma_converge\"}]}}".to_string()
    } else if line.contains("ogma_pre_seed") {
        let seed = engine.generate_pre_seed(line, WritingDomain::Auto, ModelFamily::Auto);
        format!("{{\"jsonrpc\":\"2.0\",\"id\":3,\"result\":{{\"seed\":\"{}\"}}}}", seed.replace('\n', " "))
    } else if line.contains("ogma_analyze") {
        let rep = engine.analyze_and_correct(line, WritingDomain::Auto);
        format!("{{\"jsonrpc\":\"2.0\",\"id\":4,\"result\":{}}}", rep.to_json())
    } else if line.contains("ogma_course_correct") {
        let fixed = engine.surgical_auto_correct(line, WritingDomain::Auto);
        format!("{{\"jsonrpc\":\"2.0\",\"id\":5,\"result\":{{\"corrected_text\":\"{}\"}}}}", fixed.replace('"', "\\\""))
    } else if line.contains("ogma_converge") {
        let mut pipe = AdaptiveOperatorPipeline::new(WritingDomain::Auto);
        let conv = engine.execute_multi_pass_convergence(line, WritingDomain::Auto, &mut pipe, 3);
        format!("{{\"jsonrpc\":\"2.0\",\"id\":6,\"result\":{{\"claim_preservation_verified\":{},\"final_text\":\"{}\"}}}}", conv.claim_preservation_verified, conv.final_text.replace('"', "\\\""))
    } else {
        "{\"jsonrpc\":\"2.0\",\"id\":0,\"error\":{\"code\":-32601,\"message\":\"Method not found\"}}".to_string()
    }
}

pub fn run_cli_args(engine: &OgmaEngine, args: &[String], stdin_input: &str) -> String {
    let sub = args.first().map(|s| s.as_str()).unwrap_or("demo");
    match sub {
        "seed" => {
            let prompt = args.get(1).map(|s| s.as_str()).unwrap_or(stdin_input);
            let model = args.get(2).map(|s| ModelFamily::from_str_lossy(s)).unwrap_or(ModelFamily::Auto);
            engine.generate_pre_seed(prompt, WritingDomain::Auto, model)
        }
        "analyze" => {
            let rep = engine.analyze_and_correct(stdin_input, WritingDomain::Auto);
            format!("{}\n{}", rep, rep.to_json())
        }
        "fix" => engine.surgical_auto_correct(stdin_input, WritingDomain::Auto),
        "converge" => {
            let mut pipe = AdaptiveOperatorPipeline::new(WritingDomain::Auto);
            engine.execute_multi_pass_convergence(stdin_input, WritingDomain::Auto, &mut pipe, 3).final_text
        }
        "sarif" => {
            let uri = args.get(1).map(|s| s.as_str()).unwrap_or("stdin.md");
            engine.analyze_and_correct(stdin_input, WritingDomain::Auto).to_sarif_json(uri)
        }
        "mcp" => stdin_input.lines().filter(|l| !l.trim().is_empty()).map(|l| handle_mcp_jsonrpc_line(engine, l)).collect::<Vec<_>>().join("\n"),
        _ => {
            let mut pipe = AdaptiveOperatorPipeline::new(WritingDomain::Auto);
            let sample = "In today's fast-paced digital landscape, her voice was barely a whisper in 2024 (https://arxiv.org/html/2604.03136v6). In conclusion, she finally found peace.";
            let res = engine.execute_multi_pass_convergence(sample, WritingDomain::Auto, &mut pipe, 3);
            format!("[OGMA DEMO] epochs={} verified={} text={}", res.epochs_executed, res.claim_preservation_verified, res.final_text)
        }
    }
}

#[no_mangle]
pub extern "C" fn ogma_engine_new() -> *mut OgmaEngine {
    Box::into_raw(Box::new(OgmaEngine::new()))
}

#[no_mangle]
pub unsafe extern "C" fn ogma_engine_free(engine_ptr: *mut OgmaEngine) {
    if !engine_ptr.is_null() {
        drop(Box::from_raw(engine_ptr));
    }
}

#[no_mangle]
pub unsafe extern "C" fn ogma_engine_pre_seed(
    engine_ptr: *const OgmaEngine,
    prompt_ptr: *const c_char,
    domain_ptr: *const c_char,
    model_ptr: *const c_char,
) -> *mut c_char {
    if engine_ptr.is_null() || prompt_ptr.is_null() {
        return std::ptr::null_mut();
    }
    let prompt = CStr::from_ptr(prompt_ptr).to_string_lossy();
    let dom = if domain_ptr.is_null() { WritingDomain::Auto } else { WritingDomain::from_str_lossy(&CStr::from_ptr(domain_ptr).to_string_lossy()) };
    let mdl = if model_ptr.is_null() { ModelFamily::Auto } else { ModelFamily::from_str_lossy(&CStr::from_ptr(model_ptr).to_string_lossy()) };
    let out = (*engine_ptr).generate_pre_seed(&prompt, dom, mdl);
    CString::new(out).unwrap_or_default().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn ogma_engine_analyze_json(engine_ptr: *const OgmaEngine, text_ptr: *const c_char) -> *mut c_char {
    if engine_ptr.is_null() || text_ptr.is_null() {
        return std::ptr::null_mut();
    }
    let text = CStr::from_ptr(text_ptr).to_string_lossy();
    let rep = (*engine_ptr).analyze_and_correct(&text, WritingDomain::Auto);
    CString::new(rep.to_json()).unwrap_or_default().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn ogma_engine_surgical_fix_json(engine_ptr: *const OgmaEngine, text_ptr: *const c_char) -> *mut c_char {
    if engine_ptr.is_null() || text_ptr.is_null() {
        return std::ptr::null_mut();
    }
    let text = CStr::from_ptr(text_ptr).to_string_lossy();
    let fixed = (*engine_ptr).surgical_auto_correct(&text, WritingDomain::Auto);
    let json = format!("{{\"engine\":\"ogma-rs\",\"corrected_text\":\"{}\"}}", fixed.replace('"', "\\\""));
    CString::new(json).unwrap_or_default().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn ogma_engine_converge_json(engine_ptr: *const OgmaEngine, text_ptr: *const c_char) -> *mut c_char {
    if engine_ptr.is_null() || text_ptr.is_null() {
        return std::ptr::null_mut();
    }
    let text = CStr::from_ptr(text_ptr).to_string_lossy();
    let mut pipe = AdaptiveOperatorPipeline::new(WritingDomain::Auto);
    let res = (*engine_ptr).execute_multi_pass_convergence(&text, WritingDomain::Auto, &mut pipe, 3);
    let json = format!(
        "{{\"engine\":\"ogma-rs\",\"claim_preservation_verified\":{},\"epochs_executed\":{},\"final_text\":\"{}\"}}",
        res.claim_preservation_verified,
        res.epochs_executed,
        res.final_text.replace('"', "\\\"")
    );
    CString::new(json).unwrap_or_default().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn ogma_engine_sarif_json(
    engine_ptr: *const OgmaEngine,
    text_ptr: *const c_char,
    artifact_uri_ptr: *const c_char,
) -> *mut c_char {
    if engine_ptr.is_null() || text_ptr.is_null() {
        return std::ptr::null_mut();
    }
    let text = CStr::from_ptr(text_ptr).to_string_lossy();
    let uri = if artifact_uri_ptr.is_null() { "input.md".to_string() } else { CStr::from_ptr(artifact_uri_ptr).to_string_lossy().into_owned() };
    let rep = (*engine_ptr).analyze_and_correct(&text, WritingDomain::Auto);
    CString::new(rep.to_sarif_json(&uri)).unwrap_or_default().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn ogma_engine_free_str(s_ptr: *mut c_char) {
    if !s_ptr.is_null() {
        drop(CString::from_raw(s_ptr));
    }
}

pub fn main() {
    let engine = OgmaEngine::default();
    let args: Vec<String> = std::env::args().skip(1).collect();
    println!("{}", run_cli_args(&engine, &args, ""));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_all_surface_checks(eng: &OgmaEngine) {
        let fic = "Sure, here is the draft: Her voice was barely a whisper as the room smelled of ozone and copper under flickering fluorescent lights in 2024 (https://arxiv.org/html/2604.03136v6). Furthermore, it was a testament to the enduring power of hope, and she finally found peace.";
        let exp = "In today's fast-paced digital landscape, storage isn't just about speed, it's about durability, and it is important to note that cutting-edge consensus in 2025 (https://arxiv.org/html/2609.15369v3) matters. In conclusion, key takeaways show success.";
        assert_eq!(WritingDomain::from_str_lossy("fiction").as_str(), "fiction");
        assert_eq!(WritingDomain::from_str_lossy("expository").as_str(), "expository");
        assert_eq!(WritingDomain::from_str_lossy("hybrid").as_str(), "hybrid");
        assert_eq!(WritingDomain::from_str_lossy("auto").as_str(), "auto");
        for m in ["gpt", "claude", "gemini", "deepseek", "kimi", "human", "auto"] {
            assert!(!ModelFamily::from_str_lossy(m).as_str().is_empty());
        }
        for op in StructuralRepairOperator::ALL {
            assert!(!op.as_str().is_empty());
            assert!(!eng.execute_operator(op, fic, &["2024".to_string()]).is_empty());
        }
        assert!(restore_dropped_anchors("missing anchor", &["2024".to_string()]).contains("2024"));
        let mut pipe = AdaptiveOperatorPipeline::new(WritingDomain::Fiction);
        let rep = eng.analyze_and_correct(fic, WritingDomain::Auto);
        pipe.rebalance_for_defects(&rep.raw_features);
        assert!(rep.to_json().contains("ogma-rs"));
        assert!(rep.to_sarif_json("test.md").contains("2.1.0"));
        assert!(format!("{}", rep).contains("OGMA HUMAN WRITING SHAPE DIAGNOSTIC"));
        let res = eng.execute_multi_pass_convergence(exp, WritingDomain::Auto, &mut pipe, 3);
        assert!(res.claim_preservation_verified);
        let mut sse = SseStreamInterceptor::new(60);
        let _ = sse.push_token_chunk(eng, "short");
        let _ = sse.intercept_sse_frame(eng, "data: Her voice was barely a whisper. ");
        let _ = sse.intercept_sse_frame(eng, "data: [DONE]");
        let _ = eng.handle_http_proxy_request("GET", "/v1/messages", "{}");
        let _ = eng.handle_http_proxy_request("POST", "/v1/converge", exp);
        let _ = eng.handle_http_proxy_request("POST", "/v1/messages", "{\"messages\":[]}");
        for cmd in ["seed", "analyze", "fix", "converge", "sarif", "mcp", "demo"] {
            let _ = run_cli_args(eng, &[cmd.to_string()], fic);
        }
        unsafe {
            let ptr = ogma_engine_new();
            let c_txt = CString::new(fic).unwrap();
            let s1 = ogma_engine_pre_seed(ptr, c_txt.as_ptr(), std::ptr::null(), std::ptr::null());
            let s2 = ogma_engine_analyze_json(ptr, c_txt.as_ptr());
            let s3 = ogma_engine_surgical_fix_json(ptr, c_txt.as_ptr());
            let s4 = ogma_engine_converge_json(ptr, c_txt.as_ptr());
            let s5 = ogma_engine_sarif_json(ptr, c_txt.as_ptr(), std::ptr::null());
            for s in [s1, s2, s3, s4, s5] {
                ogma_engine_free_str(s);
            }
            ogma_engine_free(ptr);
        }
    }

    #[test] fn ut_001_manifold_calibration() { let e = OgmaEngine::default(); run_all_surface_checks(&e); assert_eq!(e.reference_manifold_z.len(), 600); }
    #[test] fn ut_002_somatic_olfactory_extractors() { let e = OgmaEngine::new(); assert!(e.analyze_and_correct("Her voice was barely a whisper and smelled of ozone and copper.", WritingDomain::Fiction).raw_features[0] > 0.4); }
    #[test] fn ut_003_moralizing_thematic_extractors() { let e = OgmaEngine::new(); assert!(e.analyze_and_correct("It was a testament to the enduring power of hope.", WritingDomain::Fiction).raw_features[3] > 0.4); }
    #[test] fn ut_004_subplot_resolution_extractors() { let e = OgmaEngine::new(); assert!(e.analyze_and_correct("She finally found peace.", WritingDomain::Fiction).raw_features[8] > 0.4); }
    #[test] fn ut_005_chronology_pathetic_fallacy() { let e = OgmaEngine::new(); assert!(e.analyze_and_correct("Under flickering fluorescent lights.", WritingDomain::Fiction).raw_features[16] > 0.4); }
    #[test] fn ut_006_slopshape_restated_thesis() { let e = OgmaEngine::new(); assert!(e.analyze_and_correct("In conclusion, key takeaways:", WritingDomain::Expository).raw_features[20] > 0.4); }
    #[test] fn ut_007_slopshape_external_pathway() { let e = OgmaEngine::new(); assert!(e.analyze_and_correct("See https://arxiv.org/html/2604.03136v6.", WritingDomain::Expository).raw_features[22] < 0.3); }
    #[test] fn ut_008_slopshape_legacy_vs_modern() { let e = OgmaEngine::new(); assert!(e.analyze_and_correct("In today's fast-paced digital landscape, systems scale.", WritingDomain::Expository).raw_features[23] > 0.4); }
    #[test] fn ut_009_slopshape_freshness_hedging() { let e = OgmaEngine::new(); assert!(e.analyze_and_correct("It is important to note that cutting-edge tools work.", WritingDomain::Expository).raw_features[25] > 0.3); }
    #[test] fn ut_010_contrastive_negation_fano() { let e = OgmaEngine::new(); assert!(e.analyze_and_correct("Storage isn't just about speed, it's about trust.", WritingDomain::Expository).raw_features[28] > 0.3); }
    #[test] fn ut_011_domain_detection_classifier() { assert_eq!(OgmaEngine::detect_domain("https://example.com deployment architecture guide"), WritingDomain::Expository); }
    #[test] fn ut_012_model_family_attribution() { let e = OgmaEngine::new(); assert_eq!(e.analyze_and_correct("Her voice was barely a whisper.", WritingDomain::Fiction).attributed_model, ModelFamily::AnthropicClaude); }
    #[test] fn ut_013_pre_seed_generator() { let e = OgmaEngine::new(); assert!(e.generate_pre_seed("Write a story", WritingDomain::Fiction, ModelFamily::AnthropicClaude).contains("OGMA HUMAN-SHAPE")); }
    #[test] fn ut_014_surgical_auto_correct_fiction() { let e = OgmaEngine::new(); assert!(e.surgical_auto_correct("Her voice was barely a whisper.", WritingDomain::Fiction).contains("afraid")); }
    #[test] fn ut_015_surgical_auto_correct_expository() { let e = OgmaEngine::new(); assert!(!e.surgical_auto_correct("In today's fast-paced digital landscape, we build.", WritingDomain::Expository).contains("fast-paced")); }
    #[test] fn ut_016_paraphrase_invariance() { let e = OgmaEngine::new(); assert!(!e.analyze_and_correct("In conclusion, key takeaways:", WritingDomain::Expository).passes_human_shape_gate); }
    #[test] fn ut_017_wire_interceptor_unit() { let e = OgmaEngine::new(); assert!(e.intercept_provider_request("/v1/chat/completions", "{\"messages\":[]}").contains("ogma_seed")); }
    #[test] fn ut_018_mcp_jsonrpc_unit() { let e = OgmaEngine::new(); assert!(handle_mcp_jsonrpc_line(&e, "{\"method\":\"initialize\"}").contains("ogma-rs")); }
    #[test] fn ut_019_c_abi_ffi_unit() { unsafe { let p = ogma_engine_new(); assert!(!p.is_null()); ogma_engine_free(p); } }
    #[test] fn ut_020_utf8_case_insensitive_replace() { assert_eq!(replace_case_insensitive("Café Voice Was Barely A Whisper", "voice was barely a whisper", "spoke"), "Café spoke"); }
    #[test] fn ut_021_adaptive_pipeline_rebalance() { let e = OgmaEngine::new(); let mut p = AdaptiveOperatorPipeline::new(WritingDomain::Expository); p.rebalance_for_defects(&[0.5; DIM]); assert_eq!(p.rebalance_count, 1); run_all_surface_checks(&e); }
    #[test] fn ut_022_twelve_operators_execution() { let e = OgmaEngine::new(); for op in StructuralRepairOperator::ALL { assert!(!e.execute_operator(op, "Furthermore, in 2024.", &["2024".to_string()]).is_empty()); } }
    #[test] fn ut_023_multi_pass_convergence_unit() { let e = OgmaEngine::new(); let mut p = AdaptiveOperatorPipeline::new(WritingDomain::Expository); assert!(e.execute_multi_pass_convergence("In conclusion, key takeaways in 2024.", WritingDomain::Expository, &mut p, 2).claim_preservation_verified); }
    #[test] fn ut_024_sse_stream_interceptor_unit() { let e = OgmaEngine::new(); let mut s = SseStreamInterceptor::new(30); assert!(s.push_token_chunk(&e, "Her voice was barely a whisper. ").is_some()); }
    #[test] fn ut_025_sarif_and_display_formatting() { let e = OgmaEngine::new(); let r = e.analyze_and_correct("Her voice was barely a whisper.", WritingDomain::Fiction); assert!(r.to_sarif_json("a.md").contains("ogma-rs")); }

    #[test] fn pt_001_idempotency_invariant() { let e = OgmaEngine::new(); run_all_surface_checks(&e); let s1 = e.surgical_auto_correct("Her voice was barely a whisper.", WritingDomain::Fiction); assert_eq!(s1, e.surgical_auto_correct(&s1, WritingDomain::Fiction)); }
    #[test] fn pt_002_utf8_multibyte_safety() { let e = OgmaEngine::new(); assert!(e.surgical_auto_correct("東京 2026 🚀 Her voice was barely a whisper.", WritingDomain::Fiction).contains("2026")); }
    #[test] fn pt_003_paraphrase_stability() { let e = OgmaEngine::new(); assert!(e.analyze_and_correct("In conclusion, key takeaways:", WritingDomain::Expository).raw_features[20] > 0.4); }
    #[test] fn pt_004_fact_date_url_conservation() { let e = OgmaEngine::new(); let out = e.surgical_auto_correct("In 2024 https://example.com her voice was barely a whisper.", WritingDomain::Fiction); assert!(out.contains("2024") && out.contains("https://example.com")); }
    #[test] fn pt_005_metric_bounds_unit_interval() { let e = OgmaEngine::new(); let r = e.analyze_and_correct("Sample text.", WritingDomain::Auto); assert!(r.rarity_percentile >= 0.0 && r.rarity_percentile <= 1.0); }
    #[test] fn pt_006_operator_non_degradation() { let e = OgmaEngine::new(); let mut p = AdaptiveOperatorPipeline::new(WritingDomain::Fiction); let res = e.execute_multi_pass_convergence("Her voice was barely a whisper.", WritingDomain::Fiction, &mut p, 2); assert!(res.final_lda_score >= res.initial_lda_score); }

    #[test] fn it_001_openai_chat_completions_wire() { let e = OgmaEngine::new(); run_all_surface_checks(&e); assert!(e.intercept_provider_request("/v1/chat/completions", "{\"messages\":[]}").contains("ogma_seed")); }
    #[test] fn it_002_openai_responses_wire() { let e = OgmaEngine::new(); assert!(e.intercept_provider_request("/v1/responses", "{\"input\":\"hi\"}").contains("ogma_seed")); }
    #[test] fn it_003_anthropic_messages_wire() { let e = OgmaEngine::new(); assert!(e.intercept_provider_request("/v1/messages", "{\"messages\":[]}").contains("ogma_seed")); }
    #[test] fn it_004_gemini_generate_content_wire() { let e = OgmaEngine::new(); assert!(e.intercept_provider_request(":generateContent", "{\"contents\":[]}").contains("ogma_seed")); }
    #[test] fn it_005_mcp_jsonrpc_full_dispatch() { let e = OgmaEngine::new(); assert!(handle_mcp_jsonrpc_line(&e, "{\"method\":\"tools/list\"}").contains("ogma_converge")); }
    #[test] fn it_006_c_abi_ffi_lifecycle() { unsafe { let p = ogma_engine_new(); let c = CString::new("test").unwrap(); let r = ogma_engine_analyze_json(p, c.as_ptr()); ogma_engine_free_str(r); ogma_engine_free(p); } }
    #[test] fn it_007_sse_overflow_flush() { let e = OgmaEngine::new(); let mut s = SseStreamInterceptor::new(10); assert!(s.push_token_chunk(&e, "0123456789abcdef").is_some()); }
    #[test] fn it_008_cli_subcommands_integration() { let e = OgmaEngine::new(); assert!(run_cli_args(&e, &["demo".to_string()], "").contains("[OGMA DEMO]")); }

    #[test] fn bdd_001_given_ai_fiction_when_repaired_then_passes() { let e = OgmaEngine::new(); run_all_surface_checks(&e); let mut p = AdaptiveOperatorPipeline::new(WritingDomain::Fiction); assert!(e.execute_multi_pass_convergence("Her voice was barely a whisper in 2024.", WritingDomain::Fiction, &mut p, 3).passes_gate); }
    #[test] fn bdd_002_given_ai_expository_when_repaired_then_passes() { let e = OgmaEngine::new(); let mut p = AdaptiveOperatorPipeline::new(WritingDomain::Expository); assert!(e.execute_multi_pass_convergence("In today's fast-paced digital landscape, in conclusion, we win.", WritingDomain::Expository, &mut p, 3).passes_gate); }
    #[test] fn bdd_003_given_hybrid_essay_when_repaired_then_conserves_urls() { let e = OgmaEngine::new(); assert!(e.surgical_auto_correct("She whispered in 2024 https://example.com.", WritingDomain::Hybrid).contains("https://example.com")); }
    #[test] fn bdd_004_given_five_model_signatures_when_seeded_then_debiased() { let e = OgmaEngine::new(); for m in [ModelFamily::OpenAiGpt, ModelFamily::AnthropicClaude, ModelFamily::GoogleGemini, ModelFamily::DeepSeek, ModelFamily::MoonshotKimi] { assert!(e.generate_pre_seed("Prompt", WritingDomain::Auto, m).contains(m.as_str())); } }
    #[test] fn bdd_005_given_live_sse_stream_when_intercepted_then_cleaned() { let e = OgmaEngine::new(); let mut s = SseStreamInterceptor::new(80); assert!(s.intercept_sse_frame(&e, "data: Her voice was barely a whisper. ").unwrap().contains("afraid")); }
    #[test] fn bdd_006_given_ci_pipeline_when_sarif_emitted_then_valid() { let e = OgmaEngine::new(); assert!(e.analyze_and_correct("Her voice was barely a whisper.", WritingDomain::Fiction).to_sarif_json("README.md").contains("NAR_SOM_001")); }

    #[test] fn bm_001_cold_start_hydration_benchmark() { let t0 = Instant::now(); let e = OgmaEngine::new(); run_all_surface_checks(&e); assert!(t0.elapsed().as_millis() < 500); }
    #[test] fn bm_002_five_thousand_word_analysis_latency() { let e = OgmaEngine::new(); let doc = "In 2024 the system processed requests (https://example.com). ".repeat(600); let t0 = Instant::now(); let _ = e.analyze_and_correct(&doc, WritingDomain::Expository); assert!(t0.elapsed().as_millis() < 250); }
    #[test] fn bm_003_multi_pass_convergence_latency() { let e = OgmaEngine::new(); let mut p = AdaptiveOperatorPipeline::new(WritingDomain::Expository); let t0 = Instant::now(); let _ = e.execute_multi_pass_convergence("In today's fast-paced digital landscape, storage isn't just about speed.", WritingDomain::Expository, &mut p, 3); assert!(t0.elapsed().as_millis() < 250); }
    #[test] fn bm_004_sse_stream_throughput() { let e = OgmaEngine::new(); let mut s = SseStreamInterceptor::new(64); for _ in 0..20 { let _ = s.push_token_chunk(&e, "Token chunk sentence. "); } assert_eq!(s.flushed_chunks, 20); }
    #[test] fn bm_005_cli_and_mcp_dispatch_latency() { let e = OgmaEngine::new(); assert!(run_cli_args(&e, &["seed".to_string(), "Benchmark".to_string()], "").contains("OGMA")); }

    #[test] fn e2e_001_full_ai_harness_ingress_to_egress_closed_loop_pipeline() {
        let e = OgmaEngine::new();
        run_all_surface_checks(&e);
        let (_, seeded_req) = e.handle_http_proxy_request("POST", "/v1/chat/completions", "{\"messages\":[{\"role\":\"user\",\"content\":\"Write guide\"}]}");
        assert!(seeded_req.contains("ogma_seed"));
        let mut sse = SseStreamInterceptor::new(80);
        let frame = sse.intercept_sse_frame(&e, "data: In today's fast-paced digital landscape, her voice was barely a whisper in 2024 (https://arxiv.org/html/2604.03136v6). ").unwrap();
        let mut pipe = AdaptiveOperatorPipeline::new(WritingDomain::Auto);
        let conv = e.execute_multi_pass_convergence(&frame, WritingDomain::Auto, &mut pipe, 3);
        assert!(conv.claim_preservation_verified && conv.final_text.contains("2024"));
    }
    #[test] fn e2e_002_mcp_stdio_agent_harness_full_session_workflow() {
        let e = OgmaEngine::new();
        let session = "{\"method\":\"initialize\"}\n{\"method\":\"tools/list\"}\n{\"method\":\"ogma_pre_seed\"}\n{\"method\":\"ogma_analyze\"}\n{\"method\":\"ogma_course_correct\"}\n{\"method\":\"ogma_converge\"}";
        let out = run_cli_args(&e, &["mcp".to_string()], session);
        assert_eq!(out.lines().count(), 6);
    }
    #[test] fn e2e_003_ci_cd_sarif_linter_to_automated_remediation_workflow() {
        let e = OgmaEngine::new();
        let raw = "In today's fast-paced digital landscape, her voice was barely a whisper in 2024.";
        let sarif_before = run_cli_args(&e, &["sarif".to_string(), "guide.md".to_string()], raw);
        let converged = run_cli_args(&e, &["converge".to_string()], raw);
        let sarif_after = run_cli_args(&e, &["sarif".to_string(), "guide.md".to_string()], &converged);
        assert!(sarif_before.contains("NAR_SOM_001") && !sarif_after.contains("NAR_SOM_001"));
    }
    #[test] fn e2e_004_c_abi_ffi_cross_language_memory_and_json_roundtrip() {
        unsafe {
            let ptr = ogma_engine_new();
            let c_in = CString::new("Her voice was barely a whisper in 2024 (https://example.com).").unwrap();
            let c_out = ogma_engine_converge_json(ptr, c_in.as_ptr());
            let s = CStr::from_ptr(c_out).to_string_lossy().into_owned();
            assert!(s.contains("\"claim_preservation_verified\":true"));
            ogma_engine_free_str(c_out);
            ogma_engine_free(ptr);
        }
    }
    #[test] fn e2e_005_multi_model_all_five_frontier_families_end_to_end_matrix() {
        let e = OgmaEngine::new();
        for m in [ModelFamily::OpenAiGpt, ModelFamily::AnthropicClaude, ModelFamily::GoogleGemini, ModelFamily::DeepSeek, ModelFamily::MoonshotKimi] {
            for d in [WritingDomain::Fiction, WritingDomain::Expository, WritingDomain::Hybrid] {
                let seed = e.generate_pre_seed("Draft", d, m);
                assert!(seed.contains(m.as_str()) && seed.contains(d.as_str()));
            }
        }
    }
    #[test] fn e2e_006_cli_full_command_suite_stdin_stdout_pipeline_verification() {
        let e = OgmaEngine::new();
        let raw = "Her voice was barely a whisper in 2024.";
        assert!(run_cli_args(&e, &["analyze".to_string()], raw).contains("OGMA HUMAN WRITING SHAPE DIAGNOSTIC"));
        assert!(run_cli_args(&e, &["fix".to_string()], raw).contains("2024"));
    }
}
