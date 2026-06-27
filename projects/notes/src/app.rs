use std::path::PathBuf;
use std::time::Instant;

use chrono::{DateTime, Duration, Local, TimeZone, Weekday};
use ratatui::widgets::ListState;

use crate::model::{AppData, Note, RepeatRule, Reminder, Schedule};
use crate::storage;

pub enum Mode {
    Browse {
        focus: BrowseFocus,
    },
    EditingTitle {
        note_index: usize,
    },
    EditingBody {
        note_index: usize,
    },
    ReminderEdit {
        note_index: usize,
        reminder_index: Option<usize>,
        fields: ReminderEditFields,
    },
    ConfirmDeleteNote {
        note_index: usize,
    },
    ConfirmDeleteReminder {
        note_index: usize,
        reminder_index: usize,
    },
    Help {
        previous: Box<Mode>,
    },
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BrowseFocus {
    NoteList,
    Reminders,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ScheduleType {
    OneTime,
    Repeating,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RepeatKind {
    Days,
    Weeks,
    Months,
    Weekdays,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ReminderField {
    Label,
    ScheduleType,
    Date,
    Time,
    RepeatKind,
    Interval,
    Weekdays,
}

pub struct ReminderEditFields {
    pub label: String,
    pub schedule_type: ScheduleType,
    pub date_input: String,
    pub time_input: String,
    pub repeat_kind: RepeatKind,
    pub interval: String,
    pub weekdays: [bool; 7],
    pub active_field: ReminderField,
    pub error: Option<String>,
}

impl ReminderEditFields {
    pub fn empty() -> Self {
        let now = Local::now();
        Self {
            label: String::new(),
            schedule_type: ScheduleType::OneTime,
            date_input: now.format("%Y-%m-%d").to_string(),
            time_input: now.format("%H:%M").to_string(),
            repeat_kind: RepeatKind::Days,
            interval: "1".to_string(),
            weekdays: [false; 7],
            active_field: ReminderField::Label,
            error: None,
        }
    }

    pub fn from_reminder(rem: &Reminder) -> Self {
        let mut fields = Self::empty();
        fields.label = rem.label.clone();
        match &rem.schedule {
            Schedule::OneTime { at } => {
                fields.schedule_type = ScheduleType::OneTime;
                fields.date_input = at.format("%Y-%m-%d").to_string();
                fields.time_input = at.format("%H:%M").to_string();
            }
            Schedule::Repeating { rule, start } => {
                fields.schedule_type = ScheduleType::Repeating;
                fields.date_input = start.format("%Y-%m-%d").to_string();
                fields.time_input = start.format("%H:%M").to_string();
                match rule {
                    RepeatRule::EveryNDays(n) => {
                        fields.repeat_kind = RepeatKind::Days;
                        fields.interval = n.to_string();
                    }
                    RepeatRule::EveryNWeeks(n) => {
                        fields.repeat_kind = RepeatKind::Weeks;
                        fields.interval = n.to_string();
                    }
                    RepeatRule::EveryNMonths(n) => {
                        fields.repeat_kind = RepeatKind::Months;
                        fields.interval = n.to_string();
                    }
                    RepeatRule::WeeklyOn(days) => {
                        fields.repeat_kind = RepeatKind::Weekdays;
                        for d in days {
                            let idx = weekday_index(*d);
                            fields.weekdays[idx] = true;
                        }
                    }
                }
            }
        }
        fields
    }

    pub fn build(&self) -> Result<(String, Schedule), String> {
        let date = chrono::NaiveDate::parse_from_str(&self.date_input, "%Y-%m-%d")
            .map_err(|_| "invalid date (YYYY-MM-DD)".to_string())?;
        let time = chrono::NaiveTime::parse_from_str(&self.time_input, "%H:%M")
            .map_err(|_| "invalid time (HH:MM)".to_string())?;
        let naive = date.and_time(time);
        let dt: DateTime<Local> = Local
            .from_local_datetime(&naive)
            .single()
            .ok_or_else(|| "ambiguous local time".to_string())?;

        let schedule = match self.schedule_type {
            ScheduleType::OneTime => Schedule::OneTime { at: dt },
            ScheduleType::Repeating => {
                let rule = match self.repeat_kind {
                    RepeatKind::Days => {
                        let n: u32 = self.interval.parse().map_err(|_| "invalid interval".to_string())?;
                        if n == 0 { return Err("interval must be >= 1".into()); }
                        RepeatRule::EveryNDays(n)
                    }
                    RepeatKind::Weeks => {
                        let n: u32 = self.interval.parse().map_err(|_| "invalid interval".to_string())?;
                        if n == 0 { return Err("interval must be >= 1".into()); }
                        RepeatRule::EveryNWeeks(n)
                    }
                    RepeatKind::Months => {
                        let n: u32 = self.interval.parse().map_err(|_| "invalid interval".to_string())?;
                        if n == 0 { return Err("interval must be >= 1".into()); }
                        RepeatRule::EveryNMonths(n)
                    }
                    RepeatKind::Weekdays => {
                        let days: Vec<Weekday> = self
                            .weekdays
                            .iter()
                            .enumerate()
                            .filter_map(|(i, on)| if *on { Some(weekday_from_index(i)) } else { None })
                            .collect();
                        if days.is_empty() {
                            return Err("pick at least one weekday".into());
                        }
                        RepeatRule::WeeklyOn(days)
                    }
                };
                Schedule::Repeating { rule, start: dt }
            }
        };
        Ok((self.label.clone(), schedule))
    }
}

fn weekday_index(w: Weekday) -> usize {
    match w {
        Weekday::Mon => 0,
        Weekday::Tue => 1,
        Weekday::Wed => 2,
        Weekday::Thu => 3,
        Weekday::Fri => 4,
        Weekday::Sat => 5,
        Weekday::Sun => 6,
    }
}

fn weekday_from_index(i: usize) -> Weekday {
    match i {
        0 => Weekday::Mon,
        1 => Weekday::Tue,
        2 => Weekday::Wed,
        3 => Weekday::Thu,
        4 => Weekday::Fri,
        5 => Weekday::Sat,
        _ => Weekday::Sun,
    }
}

pub struct AppState {
    pub mode: Mode,
    pub data: AppData,
    pub data_path: PathBuf,
    pub list_state: ListState,
    pub reminder_list_state: ListState,
    pub should_quit: bool,
    pub status_message: Option<(String, Instant)>,
    pub dirty: bool,
    pub last_edit: Option<Instant>,
}

impl AppState {
    pub fn new(data: AppData, data_path: PathBuf) -> Self {
        let mut list_state = ListState::default();
        let mut reminder_list_state = ListState::default();
        if !data.notes.is_empty() {
            list_state.select(Some(0));
            if !data.notes[0].reminders.is_empty() {
                reminder_list_state.select(Some(0));
            }
        }
        Self {
            mode: Mode::Browse {
                focus: BrowseFocus::NoteList,
            },
            data,
            data_path,
            list_state,
            reminder_list_state,
            should_quit: false,
            status_message: None,
            dirty: false,
            last_edit: None,
        }
    }

    fn sync_reminder_selection(&mut self) {
        let Some(idx) = self.list_state.selected() else {
            self.reminder_list_state.select(None);
            return;
        };
        let Some(note) = self.data.notes.get(idx) else {
            self.reminder_list_state.select(None);
            return;
        };
        if note.reminders.is_empty() {
            self.reminder_list_state.select(None);
        } else {
            let cur = self.reminder_list_state.selected().unwrap_or(0);
            self.reminder_list_state
                .select(Some(cur.min(note.reminders.len() - 1)));
        }
    }

    pub fn selected_note_index(&self) -> Option<usize> {
        self.list_state.selected()
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
        self.last_edit = Some(Instant::now());
    }

    pub fn set_status(&mut self, msg: impl Into<String>) {
        self.status_message = Some((msg.into(), Instant::now()));
    }

    pub fn tick(&mut self) {
        if let Some((_, t)) = self.status_message {
            if t.elapsed() >= std::time::Duration::from_secs(3) {
                self.status_message = None;
            }
        }

        let mut fired = 0u32;
        for note in &mut self.data.notes {
            for rem in &mut note.reminders {
                let n = rem.fast_forward();
                fired += n;
            }
        }
        if fired > 0 {
            self.mark_dirty();
            self.set_status(format!("{} reminder(s) fired", fired));
        }

        if self.dirty {
            let due = match self.last_edit {
                Some(t) => t.elapsed() >= std::time::Duration::from_secs(2),
                None => true,
            };
            if due {
                self.save_if_dirty();
            }
        }
    }

    pub fn save_if_dirty(&mut self) {
        if !self.dirty {
            return;
        }
        match storage::save(&self.data_path, &self.data) {
            Ok(()) => {
                self.dirty = false;
                self.last_edit = None;
            }
            Err(e) => {
                self.set_status(format!("save failed: {}", e));
            }
        }
    }

    pub fn move_selection(&mut self, delta: i32) {
        if let Mode::Browse { focus } = &self.mode {
            match focus {
                BrowseFocus::NoteList => {
                    let len = self.data.notes.len();
                    step_list(&mut self.list_state, len, delta);
                    self.sync_reminder_selection();
                }
                BrowseFocus::Reminders => {
                    let Some(idx) = self.list_state.selected() else { return };
                    let Some(note) = self.data.notes.get(idx) else { return };
                    let len = note.reminders.len();
                    step_list(&mut self.reminder_list_state, len, delta);
                }
            }
        }
    }

    pub fn focus_notes(&mut self) {
        if let Mode::Browse { focus } = &mut self.mode {
            *focus = BrowseFocus::NoteList;
        }
    }

    pub fn focus_reminders(&mut self) {
        if let Mode::Browse { focus } = &mut self.mode {
            if let Some(idx) = self.list_state.selected() {
                if self.data.notes.get(idx).map_or(false, |n| !n.reminders.is_empty()) {
                    if self.reminder_list_state.selected().is_none() {
                        self.reminder_list_state.select(Some(0));
                    }
                    *focus = BrowseFocus::Reminders;
                }
            }
        }
    }

    pub fn start_new_note(&mut self) {
        let note = Note::new(String::new(), String::new());
        self.data.notes.push(note);
        let idx = self.data.notes.len() - 1;
        self.list_state.select(Some(idx));
        self.reminder_list_state.select(None);
        self.mark_dirty();
        self.mode = Mode::EditingTitle { note_index: idx };
    }

    pub fn start_edit_body(&mut self) {
        let Some(idx) = self.list_state.selected() else { return };
        if idx < self.data.notes.len() {
            self.mode = Mode::EditingBody { note_index: idx };
        }
    }

    pub fn confirm_title(&mut self) {
        let idx = match &self.mode {
            Mode::EditingTitle { note_index } => *note_index,
            _ => return,
        };
        if let Some(note) = self.data.notes.get_mut(idx) {
            if note.title.trim().is_empty() {
                note.title = "Untitled".to_string();
            }
            note.updated_at = Local::now();
        }
        self.mark_dirty();
        self.mode = Mode::EditingBody { note_index: idx };
    }

    pub fn cancel_title(&mut self) {
        let idx = match &self.mode {
            Mode::EditingTitle { note_index } => *note_index,
            _ => return,
        };
        if idx < self.data.notes.len() {
            self.data.notes.remove(idx);
            let new_len = self.data.notes.len();
            if new_len == 0 {
                self.list_state.select(None);
            } else {
                self.list_state.select(Some(idx.min(new_len - 1)));
            }
            self.sync_reminder_selection();
            self.mark_dirty();
            self.set_status("creation canceled");
        }
        self.mode = Mode::Browse {
            focus: BrowseFocus::NoteList,
        };
    }

    pub fn save_body(&mut self) {
        let idx = match &self.mode {
            Mode::EditingBody { note_index } => *note_index,
            _ => return,
        };
        if let Some(note) = self.data.notes.get_mut(idx) {
            note.updated_at = Local::now();
        }
        self.mark_dirty();
        self.set_status("note saved");
        self.mode = Mode::Browse {
            focus: BrowseFocus::NoteList,
        };
    }

    pub fn cancel_edit(&mut self) {
        if matches!(self.mode, Mode::EditingTitle { .. }) {
            self.cancel_title();
        } else if matches!(self.mode, Mode::EditingBody { .. }) {
            self.save_body();
        } else if matches!(self.mode, Mode::ReminderEdit { .. }) {
            self.mode = Mode::Browse {
                focus: BrowseFocus::Reminders,
            };
        }
    }

    pub fn confirm_delete_note(&mut self) {
        if let Some(idx) = self.selected_note_index() {
            if idx < self.data.notes.len() {
                self.mode = Mode::ConfirmDeleteNote { note_index: idx };
            }
        }
    }

    pub fn delete_current_note(&mut self) {
        if let Mode::ConfirmDeleteNote { note_index } = self.mode {
            if note_index < self.data.notes.len() {
                self.data.notes.remove(note_index);
                let new_len = self.data.notes.len();
                if new_len == 0 {
                    self.list_state.select(None);
                } else {
                    self.list_state.select(Some(note_index.min(new_len - 1)));
                }
                self.sync_reminder_selection();
                self.mark_dirty();
                self.set_status("note deleted");
            }
        }
        self.mode = Mode::Browse {
            focus: BrowseFocus::NoteList,
        };
    }

    pub fn start_new_reminder(&mut self) {
        let Some(idx) = self.list_state.selected() else { return };
        if idx >= self.data.notes.len() { return; }
        self.mode = Mode::ReminderEdit {
            note_index: idx,
            reminder_index: None,
            fields: ReminderEditFields::empty(),
        };
    }

    pub fn start_edit_selected_reminder(&mut self) {
        let Some(idx) = self.list_state.selected() else { return };
        let Some(note) = self.data.notes.get(idx) else { return };
        let Some(rem_idx) = self.reminder_list_state.selected() else { return };
        let Some(rem) = note.reminders.get(rem_idx) else { return };
        self.mode = Mode::ReminderEdit {
            note_index: idx,
            reminder_index: Some(rem_idx),
            fields: ReminderEditFields::from_reminder(rem),
        };
    }

    pub fn save_reminder_edit(&mut self) {
        let (note_index, reminder_index, built) = {
            let (n, r, fields) = match &mut self.mode {
                Mode::ReminderEdit { note_index, reminder_index, fields } => {
                    (*note_index, *reminder_index, fields)
                }
                _ => return,
            };
            match fields.build() {
                Ok(v) => (n, r, v),
                Err(e) => {
                    fields.error = Some(e);
                    return;
                }
            }
        };
        let (label, schedule) = built;
        let note = &mut self.data.notes[note_index];
        let new_selection = match reminder_index {
            Some(idx) => {
                let rem = &mut note.reminders[idx];
                rem.label = label;
                let next_fire = match &schedule {
                    Schedule::OneTime { at } => *at,
                    Schedule::Repeating { start, .. } => *start,
                };
                rem.schedule = schedule;
                rem.next_fire = next_fire;
                rem.active = true;
                idx
            }
            None => {
                note.reminders.push(Reminder::new(label, schedule));
                note.reminders.len() - 1
            }
        };
        note.updated_at = Local::now();
        self.list_state.select(Some(note_index));
        self.reminder_list_state.select(Some(new_selection));
        self.mark_dirty();
        self.set_status("reminder saved");
        self.mode = Mode::Browse {
            focus: BrowseFocus::Reminders,
        };
    }

    pub fn confirm_delete_reminder(&mut self) {
        let Some(idx) = self.list_state.selected() else { return };
        let Some(note) = self.data.notes.get(idx) else { return };
        let Some(rem_idx) = self.reminder_list_state.selected() else { return };
        if note.reminders.get(rem_idx).is_some() {
            self.mode = Mode::ConfirmDeleteReminder {
                note_index: idx,
                reminder_index: rem_idx,
            };
        }
    }

    pub fn delete_current_reminder(&mut self) {
        if let Mode::ConfirmDeleteReminder { note_index, reminder_index } = self.mode {
            let note = &mut self.data.notes[note_index];
            if reminder_index < note.reminders.len() {
                note.reminders.remove(reminder_index);
                note.updated_at = Local::now();
                let new_len = note.reminders.len();
                let focus = if new_len == 0 {
                    self.reminder_list_state.select(None);
                    BrowseFocus::NoteList
                } else {
                    self.reminder_list_state
                        .select(Some(reminder_index.min(new_len - 1)));
                    BrowseFocus::Reminders
                };
                self.mark_dirty();
                self.set_status("reminder deleted");
                self.mode = Mode::Browse { focus };
                return;
            }
            self.mode = Mode::Browse {
                focus: BrowseFocus::Reminders,
            };
        }
    }

    pub fn toggle_help(&mut self) {
        let placeholder = Mode::Browse {
            focus: BrowseFocus::NoteList,
        };
        let current = std::mem::replace(&mut self.mode, placeholder);
        match current {
            Mode::Help { previous } => {
                self.mode = *previous;
            }
            other => {
                self.mode = Mode::Help { previous: Box::new(other) };
            }
        }
    }

    pub fn quit(&mut self) {
        self.save_if_dirty();
        self.should_quit = true;
    }

    pub fn next_field(&mut self) {
        if let Mode::ReminderEdit { fields, .. } = &mut self.mode {
            fields.active_field = next_reminder_field(
                fields.active_field,
                fields.schedule_type,
                fields.repeat_kind,
                true,
            );
        }
    }

    pub fn prev_field(&mut self) {
        if let Mode::ReminderEdit { fields, .. } = &mut self.mode {
            fields.active_field = next_reminder_field(
                fields.active_field,
                fields.schedule_type,
                fields.repeat_kind,
                false,
            );
        }
    }

    pub fn insert_char(&mut self, c: char) {
        if let Some((idx, is_title)) = inline_edit_target(&self.mode) {
            if let Some(note) = self.data.notes.get_mut(idx) {
                if is_title {
                    note.title.push(c);
                } else {
                    note.body.push(c);
                }
                note.updated_at = Local::now();
            }
            self.mark_dirty();
            return;
        }
        if let Mode::ReminderEdit { fields, .. } = &mut self.mode {
            fields.error = None;
            match fields.active_field {
                ReminderField::Label => fields.label.push(c),
                ReminderField::Date => fields.date_input.push(c),
                ReminderField::Time => fields.time_input.push(c),
                ReminderField::Interval => {
                    if c.is_ascii_digit() {
                        fields.interval.push(c);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn backspace(&mut self) {
        if let Some((idx, is_title)) = inline_edit_target(&self.mode) {
            if let Some(note) = self.data.notes.get_mut(idx) {
                if is_title {
                    note.title.pop();
                } else {
                    note.body.pop();
                }
                note.updated_at = Local::now();
            }
            self.mark_dirty();
            return;
        }
        if let Mode::ReminderEdit { fields, .. } = &mut self.mode {
            fields.error = None;
            match fields.active_field {
                ReminderField::Label => {
                    fields.label.pop();
                }
                ReminderField::Date => {
                    fields.date_input.pop();
                }
                ReminderField::Time => {
                    fields.time_input.pop();
                }
                ReminderField::Interval => {
                    fields.interval.pop();
                }
                _ => {}
            }
        }
    }

    pub fn insert_newline(&mut self) {
        if let Mode::EditingBody { note_index } = &self.mode {
            let idx = *note_index;
            if let Some(note) = self.data.notes.get_mut(idx) {
                note.body.push('\n');
                note.updated_at = Local::now();
            }
            self.mark_dirty();
        }
    }

    pub fn reminder_toggle_schedule_type(&mut self) {
        if let Mode::ReminderEdit { fields, .. } = &mut self.mode {
            fields.schedule_type = match fields.schedule_type {
                ScheduleType::OneTime => ScheduleType::Repeating,
                ScheduleType::Repeating => ScheduleType::OneTime,
            };
            fields.error = None;
        }
    }

    pub fn reminder_cycle_repeat_kind(&mut self) {
        if let Mode::ReminderEdit { fields, .. } = &mut self.mode {
            fields.repeat_kind = match fields.repeat_kind {
                RepeatKind::Days => RepeatKind::Weeks,
                RepeatKind::Weeks => RepeatKind::Months,
                RepeatKind::Months => RepeatKind::Weekdays,
                RepeatKind::Weekdays => RepeatKind::Days,
            };
            fields.error = None;
        }
    }

    pub fn reminder_toggle_weekday(&mut self, idx: usize) {
        if let Mode::ReminderEdit { fields, .. } = &mut self.mode {
            if idx < 7 {
                fields.weekdays[idx] = !fields.weekdays[idx];
                fields.error = None;
            }
        }
    }
}

fn next_reminder_field(current: ReminderField, st: ScheduleType, rk: RepeatKind, forward: bool) -> ReminderField {
    let mut order: Vec<ReminderField> = vec![
        ReminderField::Label,
        ReminderField::ScheduleType,
        ReminderField::Date,
        ReminderField::Time,
    ];
    if matches!(st, ScheduleType::Repeating) {
        order.push(ReminderField::RepeatKind);
        if matches!(rk, RepeatKind::Weekdays) {
            order.push(ReminderField::Weekdays);
        } else {
            order.push(ReminderField::Interval);
        }
    }
    let idx = order.iter().position(|f| *f == current).unwrap_or(0);
    let next_idx = if forward {
        (idx + 1) % order.len()
    } else {
        (idx + order.len() - 1) % order.len()
    };
    order[next_idx]
}

pub fn inline_edit_target(mode: &Mode) -> Option<(usize, bool)> {
    match mode {
        Mode::EditingTitle { note_index } => Some((*note_index, true)),
        Mode::EditingBody { note_index } => Some((*note_index, false)),
        _ => None,
    }
}

fn step_list(state: &mut ListState, len: usize, delta: i32) {
    if len == 0 {
        state.select(None);
        return;
    }
    let current = state.selected().unwrap_or(0) as i32;
    let next = (current + delta).rem_euclid(len as i32);
    state.select(Some(next as usize));
}

pub fn format_local(dt: &DateTime<Local>) -> String {
    dt.format("%Y-%m-%d %H:%M").to_string()
}

pub fn humanize_until(dt: &DateTime<Local>) -> String {
    let now = Local::now();
    let diff: Duration = *dt - now;
    if diff.num_seconds() < 0 {
        return "overdue".to_string();
    }
    let total_mins = diff.num_minutes();
    if total_mins < 60 {
        return format!("in {}m", total_mins.max(1));
    }
    let hours = diff.num_hours();
    if hours < 24 {
        return format!("in {}h", hours);
    }
    let days = diff.num_days();
    if days < 30 {
        return format!("in {}d", days);
    }
    format_local(dt)
}
