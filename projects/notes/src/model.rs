use chrono::{DateTime, Datelike, Duration, Local, Weekday};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub reminders: Vec<Reminder>,
}

impl Note {
    pub fn new(title: String, body: String) -> Self {
        let now = Local::now();
        Self {
            id: Uuid::new_v4(),
            title,
            body,
            created_at: now,
            updated_at: now,
            reminders: Vec::new(),
        }
    }

    pub fn has_overdue(&self) -> bool {
        self.reminders.iter().any(|r| r.is_overdue())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reminder {
    pub id: Uuid,
    pub label: String,
    pub schedule: Schedule,
    pub next_fire: DateTime<Local>,
    pub active: bool,
}

impl Reminder {
    pub fn new(label: String, schedule: Schedule) -> Self {
        let next_fire = match &schedule {
            Schedule::OneTime { at } => *at,
            Schedule::Repeating { start, .. } => *start,
        };
        Self {
            id: Uuid::new_v4(),
            label,
            schedule,
            next_fire,
            active: true,
        }
    }

    pub fn is_overdue(&self) -> bool {
        self.active && self.next_fire < Local::now()
    }

    pub fn fast_forward(&mut self) -> u32 {
        let mut fires = 0;
        let now = Local::now();
        while self.active && self.next_fire <= now {
            fires += 1;
            match &self.schedule {
                Schedule::OneTime { .. } => {
                    self.active = false;
                }
                Schedule::Repeating { rule, .. } => {
                    self.next_fire = rule.next_after(self.next_fire);
                }
            }
        }
        fires
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Schedule {
    OneTime { at: DateTime<Local> },
    Repeating {
        rule: RepeatRule,
        start: DateTime<Local>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RepeatRule {
    EveryNDays(u32),
    EveryNWeeks(u32),
    EveryNMonths(u32),
    WeeklyOn(Vec<Weekday>),
}

impl RepeatRule {
    pub fn next_after(&self, from: DateTime<Local>) -> DateTime<Local> {
        match self {
            RepeatRule::EveryNDays(n) => from + Duration::days(*n as i64),
            RepeatRule::EveryNWeeks(n) => from + Duration::weeks(*n as i64),
            RepeatRule::EveryNMonths(n) => add_months(from, *n),
            RepeatRule::WeeklyOn(days) => next_weekday_after(from, days),
        }
    }
}

fn add_months(dt: DateTime<Local>, months: u32) -> DateTime<Local> {
    let total_months = dt.month() as i32 - 1 + months as i32;
    let years_added = total_months / 12;
    let new_month = (total_months % 12) as u32 + 1;
    let new_year = dt.year() + years_added;
    let day = dt.day().min(days_in_month(new_year, new_month));
    dt.with_year(new_year)
        .and_then(|d| d.with_month(new_month))
        .and_then(|d| d.with_day(day))
        .unwrap_or(dt)
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

fn next_weekday_after(from: DateTime<Local>, days: &[Weekday]) -> DateTime<Local> {
    if days.is_empty() {
        return from + Duration::weeks(1);
    }
    for offset in 1..=7 {
        let candidate = from + Duration::days(offset);
        if days.contains(&candidate.weekday()) {
            return candidate;
        }
    }
    from + Duration::days(1)
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppData {
    pub notes: Vec<Note>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn roundtrip_appdata() {
        let mut data = AppData::default();
        let mut note = Note::new("Shop".into(), "Milk, eggs".into());
        let at = Local.with_ymd_and_hms(2099, 1, 1, 9, 0, 0).unwrap();
        note.reminders
            .push(Reminder::new("Morning".into(), Schedule::OneTime { at }));
        data.notes.push(note);

        let json = serde_json::to_string(&data).unwrap();
        let back: AppData = serde_json::from_str(&json).unwrap();
        assert_eq!(back.notes.len(), 1);
        assert_eq!(back.notes[0].reminders.len(), 1);
    }

    #[test]
    fn repeating_advance() {
        let start = Local.with_ymd_and_hms(2026, 1, 1, 9, 0, 0).unwrap();
        let rule = RepeatRule::EveryNDays(3);
        let next = rule.next_after(start);
        assert_eq!(next, Local.with_ymd_and_hms(2026, 1, 4, 9, 0, 0).unwrap());
    }

    #[test]
    fn add_months_handles_rollover() {
        let dt = Local.with_ymd_and_hms(2026, 11, 15, 9, 0, 0).unwrap();
        let after = add_months(dt, 3);
        assert_eq!(after.year(), 2027);
        assert_eq!(after.month(), 2);
    }
}
