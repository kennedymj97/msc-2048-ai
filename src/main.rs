use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use msc_2048_ai::game::GameState;

fn main() {
    let mut game = GameState::new();
    println!("{}", game);
    run_game(&mut game);
}

fn run_game(game: &mut GameState) {
    let stdin = stdin();

    let mut stdout = stdout().into_raw_mode().unwrap();

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

        let current_state = game.0;

        match c.unwrap() {
            Key::Ctrl('c') => break,
            Key::Left => game.move_left(),
            Key::Right => game.move_right(),
            Key::Up => game.move_up(),
            Key::Down => game.move_down(),
            _ => (),
        }

        let new_state = game.0;

        if current_state != new_state {
            game.generate_random_tile();
        }
        write!(stdout, "{}", game).unwrap();
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
