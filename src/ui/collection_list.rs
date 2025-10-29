use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::state::AppState;

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
    render_collection_list(f, chunks[1], state);
    render_footer(f, chunks[2]);
}

fn render_header(f: &mut Frame, area: Rect, state: &AppState) {
    let title = if let Some(db) = &state.current_database {
        format!("Database: {}", db)
    } else {
        "No database selected".to_string()
    };

    f.render_widget(title, area);
}

fn render_collection_list(f: &mut Frame, area: Rect, state: &AppState) {
    let items: Vec<ListItem> = state
        .collections
        .iter()
        .enumerate()
        .map(|(i, coll)| {
            let content = format!(
                "{} ({} documents, {} indexes)",
                coll.name,
                coll.document_count,
                coll.indexes.len()
            );

            let style = if i == state.selected_coll_index {
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
                .title("Collections (↑/↓ to navigate, Enter to select, Backspace to go back)"),
        )
        .style(Style::default().fg(Color::White));
    f.render_widget(list, area);
}

fn render_footer(f: &mut Frame, area: Rect) {
    let footer_text =
        "Press 'q' to quit | ↑/↓ to navigate | Enter to view documents | Backspace to go back";
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(footer, area);
}
