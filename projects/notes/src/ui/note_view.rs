use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};

use crate::app::{AppState, format_local, humanize_until};
use crate::model::{RepeatRule, Schedule};

pub fn render(
    app: &mut AppState,
    frame: &mut Frame,
    area: Rect,
    reminders_focused: bool,
    editing_body: bool,
) {
    let note_index = match app.list_state.selected() {
        Some(i) if i < app.data.notes.len() => i,
        _ => {
            let placeholder = Paragraph::new("No note selected.")
                .style(Style::default().fg(Color::DarkGray))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::DarkGray)),
                );
            frame.render_widget(placeholder, area);
            return;
        }
    };
    let note = &app.data.notes[note_index];

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(10)])
        .split(area);

    let body_text = if editing_body {
        format!("{}▌", note.body)
    } else if note.body.is_empty() {
        "(empty)".to_string()
    } else {
        note.body.clone()
    };
    let body_style = if editing_body {
        Style::default().fg(Color::Yellow)
    } else if note.body.is_empty() {
        Style::default().fg(Color::DarkGray)
    } else {
        Style::default()
    };
    let body_border = if editing_body {
        Color::Yellow
    } else if reminders_focused {
        Color::DarkGray
    } else {
        Color::Cyan
    };
    let title = format!(" {} ", note.title);
    let body_block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(body_border));
    let body = Paragraph::new(body_text)
        .style(body_style)
        .wrap(Wrap { trim: false })
        .block(body_block);
    frame.render_widget(body, chunks[0]);

    let items: Vec<ListItem> = note
        .reminders
        .iter()
        .map(|rem| {
            let schedule_str = describe_schedule(&rem.schedule);
            let mut spans: Vec<Span> = Vec::new();
            if !rem.active {
                spans.push(Span::styled("[done] ", Style::default().fg(Color::DarkGray)));
            } else if rem.is_overdue() {
                spans.push(Span::styled(
                    "[!] ",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ));
            } else {
                spans.push(Span::raw("    "));
            }
            let label = if rem.label.is_empty() {
                "reminder".to_string()
            } else {
                rem.label.clone()
            };
            spans.push(Span::styled(
                label,
                Style::default().add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::raw("  "));
            spans.push(Span::styled(schedule_str, Style::default().fg(Color::Cyan)));
            spans.push(Span::raw("  "));
            spans.push(Span::styled(
                format!("next: {} ({})", format_local(&rem.next_fire), humanize_until(&rem.next_fire)),
                Style::default().fg(Color::DarkGray),
            ));
            ListItem::new(Line::from(spans))
        })
        .collect();

    let rem_border = if reminders_focused { Color::Magenta } else { Color::DarkGray };
    let rem_block = Block::default()
        .title(" Reminders ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(rem_border));
    if note.reminders.is_empty() {
        let empty = Paragraph::new("No reminders. Press 'a' to add one.")
            .style(Style::default().fg(Color::DarkGray))
            .block(rem_block);
        frame.render_widget(empty, chunks[1]);
    } else {
        let highlight_style = if reminders_focused {
            Style::default()
                .bg(Color::Magenta)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD)
        };
        let list = List::new(items)
            .block(rem_block)
            .highlight_style(highlight_style)
            .highlight_symbol(if reminders_focused { "> " } else { "  " });
        frame.render_stateful_widget(list, chunks[1], &mut app.reminder_list_state);
    }
}

fn describe_schedule(s: &Schedule) -> String {
    match s {
        Schedule::OneTime { at } => format!("once at {}", format_local(at)),
        Schedule::Repeating { rule, .. } => match rule {
            RepeatRule::EveryNDays(n) => format!("every {} day{}", n, if *n == 1 { "" } else { "s" }),
            RepeatRule::EveryNWeeks(n) => format!("every {} week{}", n, if *n == 1 { "" } else { "s" }),
            RepeatRule::EveryNMonths(n) => format!("every {} month{}", n, if *n == 1 { "" } else { "s" }),
            RepeatRule::WeeklyOn(days) => {
                let names: Vec<&str> = days.iter().map(weekday_short).collect();
                format!("weekly on {}", names.join(","))
            }
        },
    }
}

fn weekday_short(w: &chrono::Weekday) -> &'static str {
    match w {
        chrono::Weekday::Mon => "Mon",
        chrono::Weekday::Tue => "Tue",
        chrono::Weekday::Wed => "Wed",
        chrono::Weekday::Thu => "Thu",
        chrono::Weekday::Fri => "Fri",
        chrono::Weekday::Sat => "Sat",
        chrono::Weekday::Sun => "Sun",
    }
}
