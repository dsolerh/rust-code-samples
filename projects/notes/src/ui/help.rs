use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use super::popup::centered_rect;

pub fn render_help(frame: &mut Frame, area: Rect) {
    let popup_area = centered_rect(60, 70, area);
    frame.render_widget(Clear, popup_area);

    let entries = [
        ("n", "new note (type title inline)"),
        ("Enter (notes)", "edit body of selected note"),
        ("Enter (rems)", "edit selected reminder"),
        ("d", "delete note (notes focus)"),
        ("a", "add reminder to current note"),
        ("r", "delete reminder (rems focus)"),
        ("Tab / → / l", "focus reminders"),
        ("Shift+Tab / ← / h", "focus notes"),
        ("j / ↓", "move down"),
        ("k / ↑", "move up"),
        ("Enter (title)", "confirm title → body"),
        ("Ctrl+S / Esc (body)", "save body"),
        ("Esc (title)", "cancel creation"),
        ("Space / ←→", "toggle (reminder form)"),
        ("1-7", "toggle weekday"),
        ("?", "toggle help"),
        ("q / Esc", "quit"),
    ];

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(Span::styled(
        " Keybindings ",
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));
    for (key, desc) in entries {
        lines.push(Line::from(vec![
            Span::styled(format!("  {:<8}", key), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(desc.to_string()),
        ]));
    }

    let p = Paragraph::new(lines).block(
        Block::default()
            .title(" Help ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow)),
    );
    frame.render_widget(p, popup_area);
}
