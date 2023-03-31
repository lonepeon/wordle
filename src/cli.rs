use std::io;

use crate::{dictionary, game, word};
use tui::{layout, style, widgets};

const SCREEN_WIDTH: u16 = 27;
const SCREEN_HEIGHT: u16 = 20;

enum Letter {
    Green(char),
    Yellow(char),
    Gray(char),
}

impl Letter {
    fn from_guess(guess: &word::Guess) -> [Self; 5] {
        guess.guessed_letters().map(|h| match h {
            word::Hint::Misplaced(c) => Letter::Yellow(c),
            word::Hint::Exact(c) => Letter::Green(c),
            word::Hint::Absent(c) => Letter::Gray(c),
        })
    }

    fn to_tuple(&self) -> (&char, style::Color) {
        match self {
            Letter::Green(c) => (c, style::Color::Green),
            Letter::Yellow(c) => (c, style::Color::Yellow),
            Letter::Gray(c) => (c, style::Color::Reset),
        }
    }
}

pub fn play(mut wordle: dictionary::Wordle) {
    let mut game = game::Game::new(word::Word::new(wordle.split()));
    let mut input = String::new();

    crossterm::terminal::enable_raw_mode().expect("cannot set terminal to raw mode");
    let stdout = io::stdout();
    let backend = tui::backend::CrosstermBackend::new(stdout);
    let mut terminal =
        tui::terminal::Terminal::new(backend).expect("cannot setup terminal properly");

    terminal.clear().expect("cannot clear screen");
    loop {
        terminal
            .draw(|f| draw_ui(f, &wordle, &game, &input))
            .expect("can't draw UI");

        if game.state != game::State::InProgress {
            crossterm::terminal::disable_raw_mode().expect("cannot unset terminal raw mode");
            std::process::exit(0);
        }

        read_event(&mut input, &mut game)
    }
}

fn draw_ui<B: tui::backend::Backend>(
    f: &mut tui::Frame<B>,
    wordle: &dictionary::Wordle,
    game: &game::Game,
    input: &str,
) {
    let mut size = f.size();
    size.width = SCREEN_WIDTH;
    size.height = SCREEN_HEIGHT;

    let app_layout = layout::Layout::default()
        .direction(layout::Direction::Vertical)
        .constraints([layout::Constraint::Percentage(100)])
        .split(size);

    let main_block = widgets::Block::default()
        .title(format!("Wordle ({})", wordle.seed))
        .borders(widgets::Borders::ALL);
    f.render_widget(main_block, app_layout[0]);

    let guess_sections = layout::Layout::default()
        .direction(layout::Direction::Vertical)
        .margin(1)
        .constraints(
            [
                layout::Constraint::Length(3),
                layout::Constraint::Length(3),
                layout::Constraint::Length(3),
                layout::Constraint::Length(3),
                layout::Constraint::Length(3),
                layout::Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(app_layout[0]);

    game.history
        .iter()
        .enumerate()
        .for_each(|(i, guess)| display_guess(f, guess_sections[i], &Letter::from_guess(guess)));

    match game.state {
        game::State::Lost => {
            lost_popup(f, size, &wordle.word);
            f.set_cursor(size.left(), size.bottom());
        }
        game::State::Win => {
            win_popup(f, size, game.tries());
            f.set_cursor(size.left(), size.bottom());
        }
        game::State::InProgress => {
            display_current_input(f, guess_sections[game.history.len()], input);
        }
    }
}

fn read_event(input: &mut String, game: &mut game::Game) {
    if let crossterm::event::Event::Key(key) = crossterm::event::read().expect("read event") {
        match (key.modifiers, key.code) {
            (crossterm::event::KeyModifiers::CONTROL, crossterm::event::KeyCode::Char('c')) => {
                crossterm::terminal::disable_raw_mode().expect("cannot unset terminal raw mode");
                std::process::exit(0);
            }
            (_, crossterm::event::KeyCode::Enter) => {
                if input.len() == 5 {
                    let mut chars = input.chars();
                    let input_guess = [
                        chars.next().expect("cannot get character at position 0"),
                        chars.next().expect("cannot get character at position 1"),
                        chars.next().expect("cannot get character at position 2"),
                        chars.next().expect("cannot get character at position 3"),
                        chars.next().expect("cannot get character at position 4"),
                    ];

                    game.guess(input_guess);
                    input.clear();
                }
            }

            (_, crossterm::event::KeyCode::Char(c)) => {
                if input.len() == 5 {
                    input.pop();
                }

                input.push_str(&c.to_uppercase().to_string());
            }
            (_, crossterm::event::KeyCode::Backspace) => {
                input.pop();
            }
            _ => {}
        }
    }
}

fn win_popup<B: tui::backend::Backend>(f: &mut tui::Frame<B>, rect: layout::Rect, tries: usize) {
    popup(
        f,
        rect,
        format!("\n🎉\n\nyou found the word in {} tries", tries).as_ref(),
    );
}

fn lost_popup<B: tui::backend::Backend>(f: &mut tui::Frame<B>, rect: layout::Rect, response: &str) {
    popup(
        f,
        rect,
        format!(
            "🥺\n\nthe word was\n{}\nYou will do better next time",
            response
        )
        .as_ref(),
    );
}

fn display_current_input<B: tui::backend::Backend>(
    f: &mut tui::Frame<B>,
    parent: layout::Rect,
    input: &str,
) {
    let mut chars = input.chars();
    display_guess(
        f,
        parent,
        &[
            Letter::Gray(chars.next().unwrap_or(' ')),
            Letter::Gray(chars.next().unwrap_or(' ')),
            Letter::Gray(chars.next().unwrap_or(' ')),
            Letter::Gray(chars.next().unwrap_or(' ')),
            Letter::Gray(chars.next().unwrap_or(' ')),
        ],
    );
}

fn display_guess<B: tui::backend::Backend>(
    f: &mut tui::Frame<B>,
    parent: layout::Rect,
    letters: &[Letter; 5],
) {
    let chunks = layout::Layout::default()
        .direction(layout::Direction::Horizontal)
        .constraints(
            [
                layout::Constraint::Length(5),
                layout::Constraint::Length(5),
                layout::Constraint::Length(5),
                layout::Constraint::Length(5),
                layout::Constraint::Length(5),
            ]
            .as_ref(),
        )
        .split(parent);

    for (i, l) in letters.iter().enumerate() {
        let (letter, color) = l.to_tuple();
        f.render_widget(
            widgets::Paragraph::new(letter.to_string())
                .alignment(layout::Alignment::Center)
                .style(style::Style::default().fg(color))
                .block(
                    widgets::Block::default()
                        .border_type(widgets::BorderType::Double)
                        .borders(widgets::Borders::ALL),
                ),
            chunks[i],
        );
    }
}

fn popup<B: tui::backend::Backend>(f: &mut tui::Frame<B>, rect: layout::Rect, content: &str) {
    let block = widgets::Paragraph::new(content)
        .wrap(widgets::Wrap { trim: true })
        .alignment(layout::Alignment::Center)
        .block(widgets::Block::default().borders(widgets::Borders::ALL));

    let popup_layout = layout::Layout::default()
        .direction(layout::Direction::Vertical)
        .constraints(
            [
                layout::Constraint::Percentage(50 / 2),
                layout::Constraint::Percentage(50),
                layout::Constraint::Percentage(50 / 2),
            ]
            .as_ref(),
        )
        .split(rect);

    let area = layout::Layout::default()
        .direction(layout::Direction::Horizontal)
        .constraints(
            [
                layout::Constraint::Percentage(30 / 2),
                layout::Constraint::Percentage(70),
                layout::Constraint::Percentage(30 / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1];

    f.render_widget(widgets::Clear, area);
    f.render_widget(block, area);
}
