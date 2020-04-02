use crate::game::GameState;
use rand::Rng;

pub fn run_ai() {
    let mut game = GameState::new();
    let mut rng = rand::thread_rng();
    loop {
        let old_game_state = game.0;
        let num = rng.gen_range(0, 4);
        match num {
            0 => game.move_left(),
            1 => game.move_right(),
            2 => game.move_up(),
            3 => game.move_down(),
            _ => panic!("The number generated to select the move was invalid"),
        }
        let new_game_state = game.0;
        if old_game_state == new_game_state {
            break;
        }
    }
    println!("{}", game);
}
