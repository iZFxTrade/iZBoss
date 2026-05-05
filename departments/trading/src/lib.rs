/// Trading Department — Phòng iZFx (Trading)
/// Monitors EA positions, tracks P&L, and manages trading fund.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub ticket: u64,
    pub symbol: String,
    pub volume: f64,
    pub open_price: f64,
    pub current_price: f64,
    pub sl: f64,
    pub tp: f64,
    pub floating_pnl: f64,
    pub direction: String, // "BUY" | "SELL"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradingReport {
    pub open_positions: usize,
    pub total_volume: f64,
    pub floating_pnl: f64,
    pub daily_pnl: f64,
    pub win_rate: f64,
    pub alert: Option<String>,
}

pub struct TradingAgent {
    pub positions: Vec<Position>,
    pub daily_closed_pnl: f64,
    pub total_trades_today: u32,
    pub winning_trades: u32,
}

impl TradingAgent {
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            daily_closed_pnl: 0.0,
            total_trades_today: 0,
            winning_trades: 0,
        }
    }

    /// Update open positions from cTrader/MT5 feed
    pub fn update_positions(&mut self, positions: Vec<Position>) {
        self.positions = positions;
    }

    /// Check SL/TP proximity and generate alerts
    pub fn check_risk_alerts(&self) -> Vec<String> {
        let mut alerts = Vec::new();
        for pos in &self.positions {
            let risk_pct = if pos.direction == "BUY" {
                (pos.current_price - pos.sl) / pos.current_price * 100.0
            } else {
                (pos.sl - pos.current_price) / pos.current_price * 100.0
            };
            if risk_pct < 0.5 {
                alerts.push(format!(
                    "⚠ {} #{}: SL gần ({:.1}%) — current: {:.5} | SL: {:.5}",
                    pos.symbol, pos.ticket, risk_pct, pos.current_price, pos.sl
                ));
            }
        }
        alerts
    }

    /// Generate trading summary report
    pub fn generate_report(&self) -> TradingReport {
        let floating_pnl: f64 = self.positions.iter().map(|p| p.floating_pnl).sum();
        let total_volume: f64 = self.positions.iter().map(|p| p.volume).sum();
        let win_rate = if self.total_trades_today > 0 {
            self.winning_trades as f64 / self.total_trades_today as f64 * 100.0
        } else { 0.0 };

        let alerts = self.check_risk_alerts();
        TradingReport {
            open_positions: self.positions.len(),
            total_volume,
            floating_pnl,
            daily_pnl: self.daily_closed_pnl,
            win_rate,
            alert: alerts.first().cloned(),
        }
    }
}

impl Default for TradingAgent {
    fn default() -> Self { Self::new() }
}
