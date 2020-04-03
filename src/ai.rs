use crate::engine::GameEngine;
use rand::Rng;

pub fn run_ai(engine: &mut impl GameEngine) {
    let mut rng = rand::thread_rng();
    loop {
        let num = rng.gen_range(0, 4);
        match num {
            0 => engine.move_left(),
            1 => engine.move_right(),
            2 => engine.move_up(),
            3 => engine.move_down(),
            _ => panic!("The number generated to select the move was invalid"),
        }
        if engine.is_game_over() {
            break;
        }
    }
    println!("{}", engine);
}
