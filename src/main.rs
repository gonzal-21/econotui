use std::{io, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
};

#[derive(Clone, Debug, PartialEq)]
struct Price {
    symbol: &'static str,
    name: &'static str,
    value: f64,
    change: f64,
}

impl Price {
    fn change_label(&self) -> String {
        format!("{:+.2}%", self.change)
    }
}

fn prices() -> Vec<Price> {
    vec![
        Price {
            symbol: "BTC",
            name: "Bitcoin",
            value: 67_420.50,
            change: 2.84,
        },
        Price {
            symbol: "ETH",
            name: "Ethereum",
            value: 3_540.12,
            change: -1.16,
        },
        Price {
            symbol: "SOL",
            name: "Solana",
            value: 178.43,
            change: 4.21,
        },
        Price {
            symbol: "GOLD",
            name: "Gold",
            value: 2_342.80,
            change: 0.37,
        },
    ]
}

fn draw(frame: &mut Frame, prices: &[Price], selected: usize) {
    let area = frame.area();
    let layout = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(5),
        Constraint::Length(2),
    ])
    .split(area);

    let title = Paragraph::new(" econotui  •  price board")
        .style(Style::default().fg(Color::Cyan).bold())
        .block(Block::default().borders(Borders::BOTTOM));
    frame.render_widget(title, layout[0]);

    let rows = prices.iter().map(|price| {
        let change_color = if price.change >= 0.0 {
            Color::Green
        } else {
            Color::Red
        };
        Row::new([
            Cell::from(price.symbol),
            Cell::from(price.name),
            Cell::from(format!("${:>12.2}", price.value)),
            Cell::from(price.change_label()).style(Style::default().fg(change_color)),
        ])
    });
    let table = Table::new(
        rows,
        [
            Constraint::Length(10),
            Constraint::Min(20),
            Constraint::Length(16),
            Constraint::Length(12),
        ],
    )
    .header(
        Row::new(["Symbol", "Asset", "Price", "24h"])
            .style(Style::default().fg(Color::Yellow).bold()),
    )
    .row_highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White))
    .highlight_symbol("› ")
    .block(Block::default().title(" Watchlist ").borders(Borders::ALL));
    let mut state = TableState::default();
    state.select(Some(selected));
    frame.render_stateful_widget(table, layout[1], &mut state);

    let footer = Paragraph::new(" ↑/↓ or j/k  move   •   q  quit")
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(footer, layout[2]);
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let prices = prices();
    let mut selected = 0;

    loop {
        terminal.draw(|frame| draw(frame, &prices, selected))?;
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Down | KeyCode::Char('j') => {
                        selected = (selected + 1).min(prices.len() - 1);
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        selected = selected.saturating_sub(1);
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_prices_are_presented_in_watchlist_order() {
        let list = prices();

        assert_eq!(list.len(), 4);
        assert_eq!(list[0].symbol, "BTC");
        assert_eq!(list[1].symbol, "ETH");
    }

    #[test]
    fn change_label_includes_sign_and_two_decimal_places() {
        assert_eq!(prices()[0].change_label(), "+2.84%");
        assert_eq!(prices()[1].change_label(), "-1.16%");
    }
}
