mod help;
mod note_list;
mod note_view;
mod popup;
mod reminder_edit;

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::Paragraph;

use crate::app::{AppState, BrowseFocus, Mode};

pub fn render(app: &mut AppState, frame: &mut Frame) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);

    let body = chunks[0];
    let status = chunks[1];

    render_body(app, frame, body);
    render_status(app, frame, status);

    match &app.mode {
        Mode::ConfirmDeleteNote { .. } => popup::render_confirm(frame, area, "Delete this note? (y/n)"),
        Mode::ConfirmDeleteReminder { .. } => {
            popup::render_confirm(frame, area, "Delete this reminder? (y/n)")
        }
        Mode::Help { .. } => help::render_help(frame, area),
        _ => {}
    }
}

fn render_body(app: &mut AppState, frame: &mut Frame, area: Rect) {
    if matches!(app.mode, Mode::ReminderEdit { .. }) {
        reminder_edit::render(app, frame, area);
        return;
    }
    render_browse(app, frame, area);
}

fn render_browse(app: &mut AppState, frame: &mut Frame, area: Rect) {
    let split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(32), Constraint::Min(20)])
        .split(area);
    let focus = current_focus(&app.mode);
    let editing_title = matches!(effective_mode(&app.mode), Mode::EditingTitle { .. });
    let editing_body = matches!(effective_mode(&app.mode), Mode::EditingBody { .. });
    let list_focused = matches!(focus, BrowseFocus::NoteList) && !editing_body;
    let reminders_focused = matches!(focus, BrowseFocus::Reminders) && !editing_title && !editing_body;
    note_list::render(app, frame, split[0], list_focused, editing_title);
    note_view::render(app, frame, split[1], reminders_focused, editing_body);
}

fn current_focus(mode: &Mode) -> BrowseFocus {
    match mode {
        Mode::Browse { focus } => *focus,
        Mode::EditingTitle { .. } => BrowseFocus::NoteList,
        Mode::EditingBody { .. } => BrowseFocus::NoteList,
        Mode::ReminderEdit { .. } => BrowseFocus::Reminders,
        Mode::ConfirmDeleteNote { .. } => BrowseFocus::NoteList,
        Mode::ConfirmDeleteReminder { .. } => BrowseFocus::Reminders,
        Mode::Help { previous } => current_focus(previous),
    }
}

fn effective_mode(mode: &Mode) -> &Mode {
    match mode {
        Mode::Help { previous } => effective_mode(previous),
        other => other,
    }
}

fn render_status(app: &AppState, frame: &mut Frame, area: Rect) {
    let (hint, hint_style) = match &app.mode {
        Mode::Browse { focus } => {
            let text = match focus {
                BrowseFocus::NoteList => {
                    "[n]ew  [Enter]edit body  [d]elete  [a]dd rem  [Tab/→]reminders  [?]help  [q]uit"
                }
                BrowseFocus::Reminders => {
                    "[a]dd  [Enter]edit  [r]emove  [Shift+Tab/←]notes  [q]uit"
                }
            };
            (text, Style::default().fg(Color::DarkGray))
        }
        Mode::EditingTitle { .. } => (
            "type title  [Enter] → body  [Esc]cancel",
            Style::default().fg(Color::Yellow),
        ),
        Mode::EditingBody { .. } => (
            "type body  [Enter]newline  [Ctrl+S/Esc]save",
            Style::default().fg(Color::Yellow),
        ),
        Mode::ReminderEdit { .. } => (
            "[Tab]field  [Space/Arrows]toggle  [Ctrl+S]save  [Esc]cancel",
            Style::default().fg(Color::DarkGray),
        ),
        Mode::ConfirmDeleteNote { .. } | Mode::ConfirmDeleteReminder { .. } => {
            ("[y]confirm  [n]cancel", Style::default().fg(Color::Yellow))
        }
        Mode::Help { .. } => ("[?/Esc]close help", Style::default().fg(Color::DarkGray)),
    };

    let text = match &app.status_message {
        Some((msg, _)) => format!("  {}   {}", hint, msg),
        None => format!("  {}", hint),
    };
    let para = Paragraph::new(Line::from(text)).style(hint_style);
    frame.render_widget(para, area);
}
