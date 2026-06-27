mod app;
mod event;
mod model;
mod storage;
mod ui;

use std::io;
use std::time::Duration;

use crossterm::event::Event;
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use crate::app::AppState;
use crate::event::{AppAction, map_key, poll};

fn main() -> io::Result<()> {
    let data_path = match storage::data_path() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("failed to resolve data path: {}", e);
            std::process::exit(1);
        }
    };
    let mut data = match storage::load(&data_path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("failed to load data: {}", e);
            std::process::exit(1);
        }
    };

    let mut startup_fired = 0u32;
    for note in &mut data.notes {
        for rem in &mut note.reminders {
            startup_fired += rem.fast_forward();
        }
    }

    let mut app = AppState::new(data, data_path);
    if startup_fired > 0 {
        app.mark_dirty();
        app.set_status(format!("{} reminder(s) fired while away", startup_fired));
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_loop(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    app.save_if_dirty();

    result
}

fn run_loop<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut AppState,
) -> io::Result<()> {
    let tick = Duration::from_millis(250);
    loop {
        terminal.draw(|f| ui::render(app, f))?;

        if let Some(event) = poll(tick)? {
            if let Event::Key(key) = event {
                if key.kind != crossterm::event::KeyEventKind::Press {
                    continue;
                }
                if let Some(action) = map_key(key, &app.mode) {
                    apply_action(app, action);
                }
            }
        }

        app.tick();

        if app.should_quit {
            break;
        }
    }
    Ok(())
}

fn apply_action(app: &mut AppState, action: AppAction) {
    match action {
        AppAction::Quit => app.quit(),
        AppAction::ToggleHelp => app.toggle_help(),
        AppAction::MoveUp => app.move_selection(-1),
        AppAction::MoveDown => app.move_selection(1),
        AppAction::FocusNotes => app.focus_notes(),
        AppAction::FocusReminders => app.focus_reminders(),
        AppAction::NewNote => app.start_new_note(),
        AppAction::EditBody => app.start_edit_body(),
        AppAction::RequestDeleteNote => app.confirm_delete_note(),
        AppAction::ConfirmDelete => {
            if matches!(app.mode, crate::app::Mode::ConfirmDeleteNote { .. }) {
                app.delete_current_note();
            } else if matches!(app.mode, crate::app::Mode::ConfirmDeleteReminder { .. }) {
                app.delete_current_reminder();
            }
        }
        AppAction::CancelDelete => {
            let default = crate::app::Mode::Browse {
                focus: crate::app::BrowseFocus::NoteList,
            };
            app.mode = match std::mem::replace(&mut app.mode, default) {
                crate::app::Mode::ConfirmDeleteNote { .. } => crate::app::Mode::Browse {
                    focus: crate::app::BrowseFocus::NoteList,
                },
                crate::app::Mode::ConfirmDeleteReminder { .. } => crate::app::Mode::Browse {
                    focus: crate::app::BrowseFocus::Reminders,
                },
                other => other,
            };
        }
        AppAction::AddReminder => app.start_new_reminder(),
        AppAction::EditSelectedReminder => app.start_edit_selected_reminder(),
        AppAction::RequestDeleteReminder => app.confirm_delete_reminder(),
        AppAction::SaveEdit => {
            if matches!(app.mode, crate::app::Mode::EditingTitle { .. }) {
                app.confirm_title();
            } else if matches!(app.mode, crate::app::Mode::EditingBody { .. }) {
                app.save_body();
            } else if matches!(app.mode, crate::app::Mode::ReminderEdit { .. }) {
                app.save_reminder_edit();
            }
        }
        AppAction::CancelEdit => app.cancel_edit(),
        AppAction::NextField => app.next_field(),
        AppAction::PrevField => app.prev_field(),
        AppAction::InsertChar(c) => app.insert_char(c),
        AppAction::Backspace => app.backspace(),
        AppAction::Newline => app.insert_newline(),
        AppAction::ReminderToggleScheduleType => app.reminder_toggle_schedule_type(),
        AppAction::ReminderCycleRepeatKind => app.reminder_cycle_repeat_kind(),
        AppAction::ReminderToggleWeekday(i) => app.reminder_toggle_weekday(i),
    }
}
