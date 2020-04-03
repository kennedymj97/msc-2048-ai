use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::engine::GameEngine;

pub fn start_game_in_ui(engine: &mut impl GameEngine) {
    let stdin = stdin();

    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}Use the arrow keys to control the game.\r\n{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        engine,
        termion::cursor::Hide
    )
    .unwrap();

    stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All,
        )
        .unwrap();

        match c.unwrap() {
            Key::Ctrl('c') => break,
            Key::Left => engine.move_left(),
            Key::Right => engine.move_right(),
            Key::Up => engine.move_up(),
            Key::Down => engine.move_down(),
            _ => (),
        }

        if engine.is_game_over() {
            break;
        };

        write!(stdout, "{}", engine).unwrap();
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
