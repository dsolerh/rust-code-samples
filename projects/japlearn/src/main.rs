use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::prelude::*;
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::Widget;
use ratatui::{DefaultTerminal, Frame};

const HIRAGANA: &[(&str, &str)] = &[
    ("あ", "a"),  ("い", "i"),  ("う", "u"),  ("え", "e"),  ("お", "o"),
    ("か", "ka"), ("き", "ki"), ("く", "ku"), ("け", "ke"), ("こ", "ko"),
    ("さ", "sa"), ("し", "shi"),("す", "su"), ("せ", "se"), ("そ", "so"),
    ("た", "ta"), ("ち", "chi"),("つ", "tsu"),("て", "te"), ("と", "to"),
    ("な", "na"), ("に", "ni"), ("ぬ", "nu"), ("ね", "ne"), ("の", "no"),
    ("は", "ha"), ("ひ", "hi"), ("ふ", "fu"), ("へ", "he"), ("ほ", "ho"),
    ("ま", "ma"), ("み", "mi"), ("む", "mu"), ("め", "me"), ("も", "mo"),
    ("や", "ya"),              ("ゆ", "yu"),              ("よ", "yo"),
    ("ら", "ra"), ("り", "ri"), ("る", "ru"), ("れ", "re"), ("ろ", "ro"),
    ("わ", "wa"),              ("を", "wo"),
    ("ん", "n"),
];

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| Quiz::new().run(terminal))
}

struct Quiz {
    current: usize,
    input: StringField,
    correct: u16,
    wrong: u16,
    locked: bool,
    last_correct: bool,
    exit: bool,
}

impl Quiz {
    fn new() -> Self {
        Self {
            current: 0,
            input: StringField::new(),
            correct: 0,
            wrong: 0,
            locked: false,
            last_correct: false,
            exit: false,
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.code == KeyCode::Esc {
            self.exit = true;
            return;
        }

        // On the summary screen, Enter exits
        if self.is_finished() {
            if key_event.code == KeyCode::Enter {
                self.exit = true;
            }
            return;
        }

        if self.locked {
            if key_event.code == KeyCode::Enter {
                self.advance();
            }
            return;
        }

        // Only allow lowercase ascii, backspace, and Enter to confirm
        match key_event.code {
            KeyCode::Char(c) if c.is_ascii_lowercase() => {
                self.input.on_key_press(key_event);
            }
            KeyCode::Backspace => {
                self.input.on_key_press(key_event);
            }
            KeyCode::Enter if !self.input.value().is_empty() => {
                let expected = HIRAGANA[self.current].1;
                self.locked = true;
                self.last_correct = self.input.value() == expected;
                if self.last_correct {
                    self.correct += 1;
                } else {
                    self.wrong += 1;
                }
            }
            _ => {}
        }
    }

    fn advance(&mut self) {
        self.current += 1;
        self.input.clear();
        self.locked = false;
    }

    fn is_finished(&self) -> bool {
        self.current >= HIRAGANA.len()
    }
}

impl Widget for &Quiz {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.is_finished() {
            self.render_summary(area, buf);
            return;
        }

        let layout = Layout::vertical([
            Constraint::Length(2), // score
            Constraint::Fill(1),  // top padding
            Constraint::Length(1), // hiragana character
            Constraint::Length(1), // spacer
            Constraint::Length(1), // romaji input
            Constraint::Fill(1),  // bottom padding
            Constraint::Length(1), // status message
            Constraint::Length(1), // footer
        ]);
        let [score_area, _, char_area, _, input_area, _, status_area, footer_area] =
            area.layout(&layout);

        // Score
        let score = Line::from(format!(
            "Correct: {}  Wrong: {}",
            self.correct, self.wrong
        ))
        .alignment(Alignment::Center);
        score.render(score_area, buf);

        // Hiragana character
        let (hiragana, _romaji) = HIRAGANA[self.current];
        let char_line = Line::from(hiragana).bold().yellow().alignment(Alignment::Center);
        char_line.render(char_area, buf);

        // Romaji input
        if self.locked {
            let style = if self.last_correct {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Red)
            };
            let input_text = format!("Romaji: {}", self.input.value());
            let input_line = Line::from(input_text).style(style).alignment(Alignment::Center);
            input_line.render(input_area, buf);
        } else {
            // Center the StringField by rendering into a centered sub-area
            let input_text = format!("Romaji: {}", self.input.value());
            let input_line = Line::from(input_text).alignment(Alignment::Center);
            input_line.render(input_area, buf);
        }

        // Status message
        let status = if self.locked {
            if self.last_correct {
                Line::from("Correct! Press Enter to continue...".green())
            } else {
                let expected = HIRAGANA[self.current].1;
                Line::from(format!("Wrong! It was '{}'. Press Enter to continue...", expected).red())
            }
        } else {
            Line::from("Type the romaji and press Enter to confirm".dark_gray())
        };
        status.alignment(Alignment::Center).render(status_area, buf);

        // Footer
        let progress = format!("{}/{}", self.current + 1, HIRAGANA.len());
        let footer_layout = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]);
        let [left_footer, right_footer] = footer_area.layout(&footer_layout);
        Line::from(" Esc to quit".dark_gray()).render(left_footer, buf);
        Line::from(progress.dark_gray())
            .alignment(Alignment::Right)
            .render(right_footer, buf);
    }
}

impl Quiz {
    fn render_summary(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Fill(1),
        ]);
        let [_, title_area, _, score_area, exit_area, _] = area.layout(&layout);

        Line::from("Quiz Complete!")
            .bold()
            .yellow()
            .alignment(Alignment::Center)
            .render(title_area, buf);

        let total = HIRAGANA.len();
        Line::from(format!("{}/{} correct", self.correct, total))
            .bold()
            .alignment(Alignment::Center)
            .render(score_area, buf);

        Line::from("Press Esc to exit".dark_gray())
            .alignment(Alignment::Center)
            .render(exit_area, buf);
    }
}

struct StringField {
    value: String,
}

impl StringField {
    const fn new() -> Self {
        Self {
            value: String::new(),
        }
    }

    fn value(&self) -> &str {
        &self.value
    }

    fn clear(&mut self) {
        self.value.clear();
    }

    fn on_key_press(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char(c) => self.value.push(c),
            KeyCode::Backspace => {
                self.value.pop();
            }
            _ => {}
        }
    }
}
