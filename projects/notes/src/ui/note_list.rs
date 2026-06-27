use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

use crate::app::AppState;

pub fn render(
    app: &mut AppState,
    frame: &mut Frame,
    area: Rect,
    focused: bool,
    editing_title: bool,
) {
    let border_color = if editing_title {
        Color::Yellow
    } else if focused {
        Color::Cyan
    } else {
        Color::DarkGray
    };
    if app.data.notes.is_empty() {
        let block = Block::default()
            .title(" Notes ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color));
        let empty = Paragraph::new("No notes yet.\nPress 'n' to create one.")
            .style(Style::default().fg(Color::DarkGray))
            .block(block);
        frame.render_widget(empty, area);
        return;
    }

    let selected = app.list_state.selected();
    let items: Vec<ListItem> = app
        .data
        .notes
        .iter()
        .enumerate()
        .map(|(i, note)| {
            let is_editing_this = editing_title && Some(i) == selected;
            let count = note.reminders.len();
            let overdue = note.has_overdue();
            let title_text = if is_editing_this {
                format!("{}▌", note.title)
            } else {
                truncate(&note.title, 26)
            };
            let title_style = if is_editing_this {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().add_modifier(Modifier::BOLD)
            };
            let mut first = vec![Span::styled(title_text, title_style)];
            if overdue {
                first.push(Span::raw(" "));
                first.push(Span::styled(
                    "!",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ));
            }
            let sub = Span::styled(
                format!("  {} reminder{}", count, if count == 1 { "" } else { "s" }),
                Style::default().fg(Color::DarkGray),
            );
            ListItem::new(vec![Line::from(first), Line::from(sub)])
        })
        .collect();

    let highlight_style = if editing_title {
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD)
    } else if focused {
        Style::default()
            .bg(Color::Cyan)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD)
    };

    let symbol = if editing_title {
        "> "
    } else if focused {
        "> "
    } else {
        "  "
    };

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Notes ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color)),
        )
        .highlight_style(highlight_style)
        .highlight_symbol(symbol);

    frame.render_stateful_widget(list, area, &mut app.list_state);
}

fn truncate(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        s.to_string()
    } else {
        let t: String = s.chars().take(n).collect();
        format!("{}…", t)
    }
}
