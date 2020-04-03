use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::engine::initial::GameState;

pub fn start_game_in_ui() {
    let stdin = stdin();

    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut game = GameState::new();

    write!(
        stdout,
        "{}{}Use the arrow keys to control the game.\r\n{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        game,
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
            Key::Left => game.move_left(),
            Key::Right => game.move_right(),
            Key::Up => game.move_up(),
            Key::Down => game.move_down(),
            _ => (),
        }

        write!(stdout, "{}", game).unwrap();
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
