use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use crate::app::{BrowseFocus, Mode, RepeatKind, ReminderField};

pub enum AppAction {
    Quit,
    ToggleHelp,
    MoveUp,
    MoveDown,
    FocusNotes,
    FocusReminders,

    NewNote,
    EditBody,
    RequestDeleteNote,
    ConfirmDelete,
    CancelDelete,

    AddReminder,
    EditSelectedReminder,
    RequestDeleteReminder,

    SaveEdit,
    CancelEdit,
    NextField,
    PrevField,
    InsertChar(char),
    Backspace,
    Newline,

    ReminderToggleScheduleType,
    ReminderCycleRepeatKind,
    ReminderToggleWeekday(usize),
}

pub fn poll(timeout: Duration) -> std::io::Result<Option<Event>> {
    if event::poll(timeout)? {
        Ok(Some(event::read()?))
    } else {
        Ok(None)
    }
}

pub fn map_key(key: KeyEvent, mode: &Mode) -> Option<AppAction> {
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
    match mode {
        Mode::Browse { focus } => match key.code {
            KeyCode::Char('q') | KeyCode::Esc => Some(AppAction::Quit),
            KeyCode::Char('?') => Some(AppAction::ToggleHelp),
            KeyCode::Char('n') => Some(AppAction::NewNote),
            KeyCode::Char('a') => Some(AppAction::AddReminder),
            KeyCode::Char('d') if matches!(focus, BrowseFocus::NoteList) => {
                Some(AppAction::RequestDeleteNote)
            }
            KeyCode::Char('r') if matches!(focus, BrowseFocus::Reminders) => {
                Some(AppAction::RequestDeleteReminder)
            }
            KeyCode::Tab | KeyCode::Right | KeyCode::Char('l') => Some(AppAction::FocusReminders),
            KeyCode::BackTab | KeyCode::Left | KeyCode::Char('h') => Some(AppAction::FocusNotes),
            KeyCode::Enter => match focus {
                BrowseFocus::NoteList => Some(AppAction::EditBody),
                BrowseFocus::Reminders => Some(AppAction::EditSelectedReminder),
            },
            KeyCode::Char('j') | KeyCode::Down => Some(AppAction::MoveDown),
            KeyCode::Char('k') | KeyCode::Up => Some(AppAction::MoveUp),
            _ => None,
        },
        Mode::EditingTitle { .. } => {
            if ctrl && matches!(key.code, KeyCode::Char('s')) {
                return Some(AppAction::SaveEdit);
            }
            match key.code {
                KeyCode::Esc => Some(AppAction::CancelEdit),
                KeyCode::Enter => Some(AppAction::SaveEdit),
                KeyCode::Backspace => Some(AppAction::Backspace),
                KeyCode::Char(c) => Some(AppAction::InsertChar(c)),
                _ => None,
            }
        }
        Mode::EditingBody { .. } => {
            if ctrl && matches!(key.code, KeyCode::Char('s')) {
                return Some(AppAction::SaveEdit);
            }
            match key.code {
                KeyCode::Esc => Some(AppAction::CancelEdit),
                KeyCode::Enter => Some(AppAction::Newline),
                KeyCode::Backspace => Some(AppAction::Backspace),
                KeyCode::Char(c) => Some(AppAction::InsertChar(c)),
                _ => None,
            }
        }
        Mode::ReminderEdit { fields, .. } => {
            if ctrl && matches!(key.code, KeyCode::Char('s')) {
                return Some(AppAction::SaveEdit);
            }
            match key.code {
                KeyCode::Esc => Some(AppAction::CancelEdit),
                KeyCode::Tab => Some(AppAction::NextField),
                KeyCode::BackTab => Some(AppAction::PrevField),
                KeyCode::Backspace => Some(AppAction::Backspace),
                KeyCode::Char(' ') => match fields.active_field {
                    ReminderField::ScheduleType => Some(AppAction::ReminderToggleScheduleType),
                    ReminderField::RepeatKind => Some(AppAction::ReminderCycleRepeatKind),
                    ReminderField::Weekdays => Some(AppAction::ReminderToggleWeekday(0)),
                    _ => Some(AppAction::InsertChar(' ')),
                },
                KeyCode::Left => match fields.active_field {
                    ReminderField::ScheduleType => Some(AppAction::ReminderToggleScheduleType),
                    ReminderField::RepeatKind => Some(AppAction::ReminderCycleRepeatKind),
                    _ => None,
                },
                KeyCode::Right => match fields.active_field {
                    ReminderField::ScheduleType => Some(AppAction::ReminderToggleScheduleType),
                    ReminderField::RepeatKind => Some(AppAction::ReminderCycleRepeatKind),
                    _ => None,
                },
                KeyCode::Char(c) if matches!(fields.active_field, ReminderField::Weekdays) => {
                    weekday_key_index(c, fields.repeat_kind).map(AppAction::ReminderToggleWeekday)
                }
                KeyCode::Enter => Some(AppAction::NextField),
                KeyCode::Char(c) => Some(AppAction::InsertChar(c)),
                _ => None,
            }
        }
        Mode::ConfirmDeleteNote { .. } | Mode::ConfirmDeleteReminder { .. } => match key.code {
            KeyCode::Char('y') | KeyCode::Enter => Some(AppAction::ConfirmDelete),
            KeyCode::Char('n') | KeyCode::Esc => Some(AppAction::CancelDelete),
            _ => None,
        },
        Mode::Help { .. } => match key.code {
            KeyCode::Char('?') | KeyCode::Esc | KeyCode::Char('q') => Some(AppAction::ToggleHelp),
            _ => None,
        },
    }
}

fn weekday_key_index(c: char, kind: RepeatKind) -> Option<usize> {
    if !matches!(kind, RepeatKind::Weekdays) {
        return None;
    }
    match c.to_ascii_lowercase() {
        '1' => Some(0),
        '2' => Some(1),
        '3' => Some(2),
        '4' => Some(3),
        '5' => Some(4),
        '6' => Some(5),
        '7' => Some(6),
        _ => None,
    }
}
