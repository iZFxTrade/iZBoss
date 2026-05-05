/// Finance Department — Phòng Tài Chính
/// Manages wallets, P&L tracking, and budget allocation for iZ.Life BOSS.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub id: String,
    pub name: String,
    pub purpose: String,
    pub balance_usd: f64,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinanceReport {
    pub period: String,
    pub total_balance: f64,
    pub trading_pnl: f64,
    pub operating_cost: f64,
    pub boss_transfer: f64,
    pub net_profit: f64,
}

/// Finance Agent — tracks wallets and generates P&L reports
pub struct FinanceAgent {
    pub wallets: Vec<Wallet>,
}

impl FinanceAgent {
    pub fn new() -> Self {
        Self {
            wallets: vec![
                Wallet { id: "w-ops".into(), name: "📌 Vận hành".into(), purpose: "Chi phí cố định".into(), balance_usd: 0.0, status: "active".into() },
                Wallet { id: "w-inv".into(), name: "📈 Đầu tư".into(), purpose: "Tái đầu tư Trade/AI".into(), balance_usd: 0.0, status: "active".into() },
                Wallet { id: "w-sav".into(), name: "💰 Tích lũy".into(), purpose: "Quỹ dự phòng".into(), balance_usd: 0.0, status: "active".into() },
                Wallet { id: "w-boss".into(), name: "👑 Ví Sếp Hưng".into(), purpose: "2,000$/tháng".into(), balance_usd: 0.0, status: "active".into() },
                Wallet { id: "w-rnd".into(), name: "🧬 Quỹ R&D".into(), purpose: "Phát triển iZcore".into(), balance_usd: 0.0, status: "active".into() },
            ],
        }
    }

    /// Generate a monthly finance report
    pub fn generate_report(&self, period: &str, trading_pnl: f64, cost: f64) -> FinanceReport {
        let total = self.wallets.iter().map(|w| w.balance_usd).sum();
        let boss_transfer = 2000.0_f64.min(trading_pnl * 0.3);
        FinanceReport {
            period: period.to_string(),
            total_balance: total,
            trading_pnl,
            operating_cost: cost,
            boss_transfer,
            net_profit: trading_pnl - cost - boss_transfer,
        }
    }

    /// Allocate incoming profit across wallets by percentage rule
    pub fn allocate_profit(&mut self, profit: f64) {
        // Rule: 30% ops, 30% invest, 20% savings, 10% boss, 10% rnd
        let allocations = [("w-ops", 0.30), ("w-inv", 0.30), ("w-sav", 0.20), ("w-boss", 0.10), ("w-rnd", 0.10)];
        for (id, pct) in allocations {
            if let Some(w) = self.wallets.iter_mut().find(|w| w.id == id) {
                w.balance_usd += profit * pct;
            }
        }
    }
}

impl Default for FinanceAgent {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_profit_allocation() {
        let mut agent = FinanceAgent::new();
        agent.allocate_profit(1000.0);
        let ops = agent.wallets.iter().find(|w| w.id == "w-ops").unwrap();
        assert!((ops.balance_usd - 300.0).abs() < 0.01);
    }
}
