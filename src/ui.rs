use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::engine as GameEngine;

pub fn start_game_in_ui() {
    let mut board = GameEngine::new_game();
    let stdin = stdin();

    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}Use the arrow keys to control the game.\r\n{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        GameEngine::to_str(board),
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
            Key::Left => board = GameEngine::move_left(board),
            Key::Right => board = GameEngine::move_right(board),
            Key::Up => board = GameEngine::move_up(board),
            Key::Down => board = GameEngine::move_down(board),
            _ => (),
        }

        if GameEngine::is_game_over(board) {
            break;
        };

        write!(stdout, "{}", GameEngine::to_str(board)).unwrap();
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
