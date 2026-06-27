use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::{AppState, Mode, RepeatKind, ReminderEditFields, ReminderField, ScheduleType};

pub fn render(app: &mut AppState, frame: &mut Frame, area: Rect) {
    let fields = match &app.mode {
        Mode::ReminderEdit { fields, .. } => fields,
        Mode::Help { previous } => match previous.as_ref() {
            Mode::ReminderEdit { fields, .. } => fields,
            _ => return,
        },
        _ => return,
    };

    let is_new = matches!(
        &app.mode,
        Mode::ReminderEdit { reminder_index: None, .. }
    );
    let title = if is_new { " New Reminder " } else { " Edit Reminder " };

    let outer = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta));
    let inner = outer.inner(area);
    frame.render_widget(outer, area);

    let mut constraints = vec![
        Constraint::Length(3), // label
        Constraint::Length(3), // schedule type
        Constraint::Length(3), // date
        Constraint::Length(3), // time
    ];
    if matches!(fields.schedule_type, ScheduleType::Repeating) {
        constraints.push(Constraint::Length(3)); // kind
        if !matches!(fields.repeat_kind, RepeatKind::Weekdays) {
            constraints.push(Constraint::Length(3)); // interval
        } else {
            constraints.push(Constraint::Length(3)); // weekdays
        }
    }
    constraints.push(Constraint::Min(1)); // error/spacer

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner);

    let mut i = 0;

    frame.render_widget(
        text_field("Label", &fields.label, fields.active_field == ReminderField::Label),
        chunks[i],
    );
    i += 1;

    frame.render_widget(
        schedule_type_field(fields, fields.active_field == ReminderField::ScheduleType),
        chunks[i],
    );
    i += 1;

    frame.render_widget(
        text_field("Date (YYYY-MM-DD)", &fields.date_input, fields.active_field == ReminderField::Date),
        chunks[i],
    );
    i += 1;

    frame.render_widget(
        text_field("Time (HH:MM)", &fields.time_input, fields.active_field == ReminderField::Time),
        chunks[i],
    );
    i += 1;

    if matches!(fields.schedule_type, ScheduleType::Repeating) {
        frame.render_widget(
            repeat_kind_field(fields, fields.active_field == ReminderField::RepeatKind),
            chunks[i],
        );
        i += 1;
        if matches!(fields.repeat_kind, RepeatKind::Weekdays) {
            frame.render_widget(
                weekdays_field(fields, fields.active_field == ReminderField::Weekdays),
                chunks[i],
            );
            i += 1;
        } else {
            frame.render_widget(
                text_field(
                    "Interval (N)",
                    &fields.interval,
                    fields.active_field == ReminderField::Interval,
                ),
                chunks[i],
            );
            i += 1;
        }
    }

    if let Some(err) = &fields.error {
        let err_widget = Paragraph::new(err.as_str()).style(Style::default().fg(Color::Red));
        frame.render_widget(err_widget, chunks[i]);
    }
}

fn text_field<'a>(title: &'a str, value: &'a str, active: bool) -> Paragraph<'a> {
    let border_color = if active { Color::Yellow } else { Color::DarkGray };
    let text = if active {
        format!("{}_", value)
    } else {
        value.to_string()
    };
    Paragraph::new(text)
        .style(if active {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        })
        .block(
            Block::default()
                .title(format!(" {} ", title))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color)),
        )
}

fn schedule_type_field<'a>(fields: &'a ReminderEditFields, active: bool) -> Paragraph<'a> {
    let border_color = if active { Color::Yellow } else { Color::DarkGray };
    let one_selected = matches!(fields.schedule_type, ScheduleType::OneTime);
    let rep_selected = matches!(fields.schedule_type, ScheduleType::Repeating);
    let spans = vec![
        Span::styled(
            if one_selected { "(*) One-time  " } else { "( ) One-time  " },
            mark_style(one_selected),
        ),
        Span::styled(
            if rep_selected { "(*) Repeating" } else { "( ) Repeating" },
            mark_style(rep_selected),
        ),
    ];
    Paragraph::new(Line::from(spans)).block(
        Block::default()
            .title(" Type (space/←→ to toggle) ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color)),
    )
}

fn repeat_kind_field<'a>(fields: &'a ReminderEditFields, active: bool) -> Paragraph<'a> {
    let border_color = if active { Color::Yellow } else { Color::DarkGray };
    let kinds = [
        (RepeatKind::Days, "Days"),
        (RepeatKind::Weeks, "Weeks"),
        (RepeatKind::Months, "Months"),
        (RepeatKind::Weekdays, "Weekdays"),
    ];
    let spans: Vec<Span> = kinds
        .iter()
        .flat_map(|(kind, label)| {
            let selected = fields.repeat_kind == *kind;
            vec![
                Span::styled(
                    if selected { format!("[{}] ", label) } else { format!(" {}  ", label) },
                    mark_style(selected),
                ),
            ]
        })
        .collect();
    Paragraph::new(Line::from(spans)).block(
        Block::default()
            .title(" Repeat Kind (space to cycle) ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color)),
    )
}

fn weekdays_field<'a>(fields: &'a ReminderEditFields, active: bool) -> Paragraph<'a> {
    let border_color = if active { Color::Yellow } else { Color::DarkGray };
    let names = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let mut spans: Vec<Span> = Vec::new();
    for (i, name) in names.iter().enumerate() {
        let on = fields.weekdays[i];
        spans.push(Span::styled(
            format!("[{}]{} ", if on { "x" } else { " " }, name),
            mark_style(on),
        ));
    }
    Paragraph::new(Line::from(spans)).block(
        Block::default()
            .title(" Weekdays (press 1-7 to toggle) ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color)),
    )
}

fn mark_style(on: bool) -> Style {
    if on {
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    }
}
