use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::app::state::AppState;

pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(chunks[0]);

    render_header(f, left_chunks[0], state);
    render_document_list(f, left_chunks[1], state);
    render_footer(f, left_chunks[2]);
    render_document_content(f, chunks[1], state);
}

fn render_header(f: &mut Frame, area: Rect, state: &AppState) {
    let title = if let (Some(db), Some(coll)) = (&state.current_database, &state.current_collection)
    {
        format!("{}.{}", db, coll)
    } else {
        "No collection selected".to_string()
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

fn render_document_list(f: &mut Frame, area: Rect, state: &AppState) {
    let items: Vec<ListItem> = state
        .documents
        .iter()
        .enumerate()
        .map(|(i, doc)| {
            let id = doc
                .get("_id")
                .map(|v| format!("{}", v))
                .unwrap_or_else(|| format!("Doc {}", i + 1));

            let content = if id.len() > 25 {
                format!("{}...", &id[..22])
            } else {
                id
            };

            let style = if i == state.selected_doc_index {
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

    let title = format!(
        "Documents ({}/{})",
        state.documents.len(),
        state.documents.len()
    );
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .style(Style::default().fg(Color::White));

    f.render_widget(list, area);
}

fn render_document_content(f: &mut Frame, area: Rect, state: &AppState) {
    let content = if let Some(doc) = state.get_selected_document() {
        // pretty print JSON with indentation
        match serde_json::to_string_pretty(&doc) {
            Ok(json) => json,
            Err(_) => format!("{:?}", doc),
        }
    } else {
        "No document selected".to_string()
    };

    // split into lines and apply scroll offset
    let lines: Vec<Line> = content
        .lines()
        .skip(state.doc_scroll_offset)
        .map(|line| Line::from(line.to_string()))
        .collect();

    let paragraph = Paragraph::new(lines)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Document Content (PgUp/PgDn to scroll)"),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}

fn render_footer(f: &mut Frame, area: Rect) {
    let footer_text =
        "q: quit | ↑/↓: navigate | Backspace: back | PgUp/PgDn: scroll | 'r': refresh";
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(footer, area);
}
