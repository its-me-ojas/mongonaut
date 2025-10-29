use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::state::AppState;

pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    render_title(f, chunks[0]);
    render_instructions(f, chunks[1]);
    render_input(f, chunks[2], state);
    render_footer(f, chunks[4], state);
}

fn render_title(f: &mut Frame, area: Rect) {
    let title = Paragraph::new("Mongonaut - MonogoDB TUI Client")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(title, area);
}

fn render_instructions(f: &mut Frame, area: Rect) {
    let text = "Enter MongoDB URI and press Enter to connect\nExamples: mongodb://localhost:27017 or mongodb+srv://...";
    let instructions = Paragraph::new(text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title("Instructions"));
    f.render_widget(instructions, area);
}

fn render_input(f: &mut Frame, area: Rect, state: &AppState) {
    let input_style = if state.input_mode {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };
    let input = Paragraph::new(state.connection_input.as_str())
        .style(input_style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Connection URI"),
        );
    f.render_widget(input, area);
}

fn render_footer(f: &mut Frame, area: Rect, state: &AppState) {
    let text = if state.error.is_some() {
        format!(
            "Error: {} | Press Esc to clear",
            state.error.as_ref().unwrap()
        )
    } else if state.loading {
        "Connecting...".to_string()
    } else {
        "Enter: Connect | Ctrl+V: Paste | Esc: Clear | Ctrl+C/Ctrl+Q: Quit".to_string()
    };
    let footer = Paragraph::new(text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, area);
}
