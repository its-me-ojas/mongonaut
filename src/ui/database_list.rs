use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::state::{self, AppState};

pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    render_header(f, chunks[0], state);
    render_database_list(f, chunks[1], state);
    render_footer(f, chunks[2]);
}

fn render_header(f: &mut Frame, area: Rect, state: &AppState) {
    let title = if let Some(conn) = &state.connection {
        format!(
            "Mongonaut - Connected to {} (MongoDB {})",
            conn.server_info.host, conn.server_info.version
        )
    } else {
        "Mongonaut - Not connected".to_string()
    };
    let header = Paragraph::new(title)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(header, area);
}

fn render_database_list(f: &mut Frame, area: Rect, state: &AppState) {
    let items: Vec<ListItem> = state
        .databases
        .iter()
        .enumerate()
        .map(|(i, db)| {
            let content = format!(
                "{} ({} collections, {} bytes)",
                db.name, db.collection_count, db.size_on_disk
            );

            let style = if i == state.selected_db_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::DarkGray)
            } else {
                Style::default().fg(Color::White)
            };

            ListItem::new(Line::from(Span::styled(content, style)))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Databases (↑/↓ to navigate, Enter to select, q to quit)"),
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(list, area);
}

fn render_footer(f: &mut Frame, area: Rect) {
    let footer_text =
        "Press 'q' to quit | ↑/↓ to navigate | Enter to select database | 'r' to refresh";
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(footer, area);
}
