use crate::users::{User, UserRole};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs, Gauge, Table, Row, Cell},
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::time::{Duration, Instant};

pub struct App<'a> {
    pub user: &'a User,
    pub tabs: Vec<&'a str>,
    pub index: usize,
    pub should_quit: bool,
}

impl<'a> App<'a> {
    pub fn new(user: &'a User) -> App<'a> {
        App {
            user,
            tabs: vec!["Fleet", "Workforce", "LLM Matrix", "Finance", "Users", "System"],
            index: 0,
            should_quit: false,
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.tabs.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.tabs.len() - 1;
        }
    }
}

pub async fn run_dashboard(user: &User) -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(user);
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => app.should_quit = true,
                    KeyCode::Right | KeyCode::Char('l') => app.next(),
                    KeyCode::Left | KeyCode::Char('h') => app.previous(),
                    _ => {}
                }
            }
        }

        if app.should_quit {
            break;
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Tabs
            Constraint::Min(0),    // Main Content
            Constraint::Length(3), // Footer/Status
        ])
        .split(size);

    // 1. Header with Tabs
    let titles = app.tabs.iter().map(|t| Line::from(Span::styled(*t, Style::default().fg(Color::Cyan)))).collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(format!(" iZCore Terminal Dashboard | User: {} ", app.user.id)))
        .select(app.index)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
    f.render_widget(tabs, chunks[0]);

    // 2. Main Content based on Tab
    match app.index {
        0 => draw_fleet_tab(f, chunks[1]),
        1 => draw_workforce_tab(f, chunks[1]),
        2 => draw_llm_tab(f, chunks[1]),
        3 => draw_finance_tab(f, chunks[1]),
        4 => draw_users_tab(f, chunks[1]),
        5 => draw_system_tab(f, chunks[1]),
        _ => {}
    }

    // 3. Footer
    let footer = Paragraph::new(" [Q] Quit | [Tab/Arrows] Switch Tabs | iZ.Life BOSS Sovereign Matrix v25.5 Platinum ")
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::TOP));
    f.render_widget(footer, chunks[2]);
}

fn draw_fleet_tab(f: &mut ratatui::Frame, area: Rect) {
    let rows = vec![
        Row::new(vec!["iznode-82fa1", "FPT PlayBox", "online", "linux-armv7", "Active"]),
        Row::new(vec!["iznode-c9012", "MacBook M1", "online", "macos-aarch64", "Idle"]),
        Row::new(vec!["iznode-f1234", "Win-Workstation", "offline", "windows-x86_64", "Inactive"]),
    ];
    let table = Table::new(rows, [
        Constraint::Length(15),
        Constraint::Length(20),
        Constraint::Length(10),
        Constraint::Length(15),
        Constraint::Length(10),
    ])
    .header(Row::new(vec!["Node ID", "Name", "Status", "Platform", "Load"]).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)))
    .block(Block::default().borders(Borders::ALL).title(" Infrastructure Fleet "));
    f.render_widget(table, area);
}

fn draw_workforce_tab(f: &mut ratatui::Frame, area: Rect) {
    let agents = vec![
        ListItem::new("🤖 Agent Alpha [Market Scanner] - Processing Symbol: GOLD"),
        ListItem::new("🤖 Agent Beta [Sentiment Analyzer] - Monitoring X (Twitter)"),
        ListItem::new("🤖 Agent Gamma [Order Manager] - Idle"),
    ];
    let list = List::new(agents)
        .block(Block::default().borders(Borders::ALL).title(" Active Workforce Matrix "));
    f.render_widget(list, area);
}

fn draw_llm_tab(f: &mut ratatui::Frame, area: Rect) {
    let models = Paragraph::new(
        "🧠 Gemini 1.5 Pro (Primary) | TPM: 1M | Status: Active\n\
         🧠 Claude 3.5 Sonnet (Fallback) | TPM: 200K | Status: Standby\n\
         🧠 DeepSeek Coder V2 (Internal) | Status: Training..."
    )
    .block(Block::default().borders(Borders::ALL).title(" LLM Matrix (DNA Brains) "));
    f.render_widget(models, area);
}

fn draw_finance_tab(f: &mut ratatui::Frame, area: Rect) {
    let wallets = Table::new(vec![
        Row::new(vec!["Main Treasury", "0x82f...a1c", "4.52 BTC"]),
        Row::new(vec!["Node Incentives", "0x12a...b3d", "12,450 IZL"]),
        Row::new(vec!["Operational", "0x99f...f12", "$5,200.00"]),
    ], [
        Constraint::Percentage(30),
        Constraint::Percentage(40),
        Constraint::Percentage(30),
    ])
    .header(Row::new(vec!["Wallet", "Address", "Balance"]).style(Style::default().fg(Color::Green)))
    .block(Block::default().borders(Borders::ALL).title(" Financial Sovereignty "));
    f.render_widget(wallets, area);
}

fn draw_users_tab(f: &mut ratatui::Frame, area: Rect) {
    let users = Table::new(vec![
        Row::new(vec!["iZFxTrade", "Root", "Verified"]),
        Row::new(vec!["FxBlueNet", "Root", "Verified"]),
        Row::new(vec!["admin_01", "Admin", "Verified"]),
        Row::new(vec!["mod_alpha", "Mod", "Verified"]),
    ], [
        Constraint::Percentage(33),
        Constraint::Percentage(33),
        Constraint::Percentage(33),
    ])
    .header(Row::new(vec!["User ID", "Role", "Status"]).style(Style::default().fg(Color::Magenta)))
    .block(Block::default().borders(Borders::ALL).title(" RBAC & Permissions "));
    f.render_widget(users, area);
}

fn draw_system_tab(f: &mut ratatui::Frame, area: Rect) {
    let evolution = crate::evolution::get_current_state();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // CPU
            Constraint::Length(3), // RAM
            Constraint::Length(6), // Evolution
            Constraint::Min(0)     // Info
        ])
        .split(area);

    let cpu = Gauge::default()
        .block(Block::default().title(" CPU Load ").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Cyan))
        .percent(12);
    f.render_widget(cpu, chunks[0]);

    let ram = Gauge::default()
        .block(Block::default().title(" RAM Usage ").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Green))
        .percent(45);
    f.render_widget(ram, chunks[1]);
    
    let evo_info = Paragraph::new(format!(
        "🧬 Generation: Gen-{}\n\
         🧠 Collective Intelligence: {:.0}%\n\
         ⭐ Fitness Score: {:.2}\n\
         ⚡ Last Mutation: {}",
        evolution.generation,
        evolution.collective_intelligence_level * 100.0,
        evolution.fitness_score,
        evolution.last_mutation
    ))
    .block(Block::default().borders(Borders::ALL).title(" Autonomous Evolution State ").style(Style::default().fg(Color::Yellow)));
    f.render_widget(evo_info, chunks[2]);

    let info = Paragraph::new("OS: Linux 6.1.0-21-amd64\nKernel: iZCore v0.1.2\nUptime: 4d 12h 30m\nNode ID: iznode-primary")
        .block(Block::default().borders(Borders::ALL).title(" System Diagnostics "));
    f.render_widget(info, chunks[3]);
}
