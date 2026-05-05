/// Marketing Department — Phòng Marketing
/// Automates content creation, scheduling, and social media presence.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentPiece {
    pub id: String,
    pub platform: String, // "telegram" | "facebook" | "zalo"
    pub content: String,
    pub hashtags: Vec<String>,
    pub scheduled_at: Option<String>,
    pub status: ContentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContentStatus {
    Draft,
    Scheduled,
    Published,
    Failed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketingReport {
    pub total_posts: usize,
    pub published: usize,
    pub scheduled: usize,
    pub reach_estimate: u64,
    pub top_platform: String,
}

pub struct MarketingAgent {
    pub content_queue: Vec<ContentPiece>,
    pub published_count: usize,
}

impl MarketingAgent {
    pub fn new() -> Self {
        Self { content_queue: Vec::new(), published_count: 0 }
    }

    /// Create a new content piece for scheduling
    pub fn create_content(&mut self, platform: &str, content: &str, hashtags: Vec<String>) -> String {
        let id = format!("post-{}-{}", platform, self.content_queue.len() + 1);
        self.content_queue.push(ContentPiece {
            id: id.clone(),
            platform: platform.to_string(),
            content: content.to_string(),
            hashtags,
            scheduled_at: None,
            status: ContentStatus::Draft,
        });
        println!("[Marketing] ✓ Đã tạo nội dung '{}' cho {}", id, platform);
        id
    }

    /// Schedule content for publishing
    pub fn schedule(&mut self, post_id: &str, datetime: &str) {
        if let Some(post) = self.content_queue.iter_mut().find(|p| p.id == post_id) {
            post.scheduled_at = Some(datetime.to_string());
            post.status = ContentStatus::Scheduled;
            println!("[Marketing] 📅 '{}' scheduled: {}", post_id, datetime);
        }
    }

    /// Mark a post as published
    pub fn mark_published(&mut self, post_id: &str) {
        if let Some(post) = self.content_queue.iter_mut().find(|p| p.id == post_id) {
            post.status = ContentStatus::Published;
            self.published_count += 1;
        }
    }

    /// Generate marketing performance report
    pub fn generate_report(&self) -> MarketingReport {
        let published = self.content_queue.iter().filter(|p| p.status == ContentStatus::Published).count();
        let scheduled = self.content_queue.iter().filter(|p| p.status == ContentStatus::Scheduled).count();
        MarketingReport {
            total_posts: self.content_queue.len(),
            published,
            scheduled,
            reach_estimate: (published as u64) * 500, // estimate 500 reach per post
            top_platform: "telegram".to_string(),
        }
    }
}

impl Default for MarketingAgent {
    fn default() -> Self { Self::new() }
}
