/// R&D Department — Phòng Tiến Hóa (Research & Development)
/// Scans GitHub/HuggingFace for new AI models and tracks technology trends.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCandidate {
    pub name: String,
    pub source: String, // "huggingface" | "github" | "cloudflare"
    pub description: String,
    pub size_gb: f64,
    pub score: u8, // 0-100 relevance score
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RdReport {
    pub new_models_found: usize,
    pub top_candidate: Option<ModelCandidate>,
    pub recommendation: String,
    pub iZcore_version: String,
}

pub struct RdAgent {
    pub discovered_models: Vec<ModelCandidate>,
}

impl RdAgent {
    pub fn new() -> Self {
        Self { discovered_models: Vec::new() }
    }

    /// Evaluate a discovered model and add if relevant
    pub fn evaluate_model(&mut self, candidate: ModelCandidate) {
        if candidate.score >= 60 {
            println!("[R&D] ✓ Model đáng chú ý: {} (score: {})", candidate.name, candidate.score);
            self.discovered_models.push(candidate);
        }
    }

    /// Sort and return top model candidates
    pub fn top_candidates(&self, limit: usize) -> Vec<&ModelCandidate> {
        let mut sorted: Vec<&ModelCandidate> = self.discovered_models.iter().collect();
        sorted.sort_by(|a, b| b.score.cmp(&a.score));
        sorted.into_iter().take(limit).collect()
    }

    /// Generate weekly R&D report for BA
    pub fn generate_report(&self) -> RdReport {
        let top = self.top_candidates(1).into_iter().next().cloned();
        let recommendation = if let Some(ref model) = top {
            format!(
                "Đề xuất thử nghiệm '{}' từ {} — size: {}GB, relevance: {}/100",
                model.name, model.source, model.size_gb, model.score
            )
        } else {
            "Không có model mới đáng chú ý tuần này. Tiếp tục dùng Cloudflare AI.".to_string()
        };

        RdReport {
            new_models_found: self.discovered_models.len(),
            top_candidate: top,
            recommendation,
            iZcore_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

impl Default for RdAgent {
    fn default() -> Self { Self::new() }
}
