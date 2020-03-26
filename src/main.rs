use std::fmt;

fn main() {
    let game = GameState::from(81985529216486895);
    println!("{}", game);
}

struct GameState(u64);

impl GameState {
    fn new() -> GameState {
        let game = GameState(0);
        game.generate_random_tile();
        game.generate_random_tile();
        game
    }

    fn from(val: u64) -> GameState {
        GameState(val)
    }

    fn parse_board(&self) -> [Option<u32>; 16] {
        let board = (0..16).fold([None; 16], |mut arr, idx| {
            let num = self.extract_tile(idx);

            match num {
                0 => arr[idx] = None,
                _ => arr[idx] = Some((2 as u32).pow(num)),
            }

            arr
        });
        board
    }

    fn extract_tile(&self, idx: usize) -> u32 {
        ((self.0 >> ((15 - idx) * 4)) & 15) as u32
    }

    /// Function to generate random tiles
    /// The random tiles are either 2 or 4
    /// There is a 90% chance the tile generated is a 2 and 10% of a 4
    /// At the start of the game 2 tiles are generated randomly and after a tile is generated after
    /// a move has been completed
    fn generate_random_tile(&self) {}
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let board: Vec<String> = self.parse_board().iter().map(|x| format_val(x)).collect();
        let out = format!(
            "
        {}|{}|{}|{}
        --------------------------------
        {}|{}|{}|{}
        --------------------------------
        {}|{}|{}|{}
        --------------------------------
        {}|{}|{}|{}
        ",
            board[0],
            board[1],
            board[2],
            board[3],
            board[4],
            board[5],
            board[6],
            board[7],
            board[8],
            board[9],
            board[10],
            board[11],
            board[12],
            board[13],
            board[14],
            board[15]
        );
        write!(f, "{}", out)
    }
}

fn format_val(val: &Option<u32>) -> String {
    match val {
        None => return String::from("       "),
        Some(x) => {
            let mut x = x.to_string();
            while x.len() < 7 {
                match x.len() {
                    6 => x = format!(" {}", x),
                    _ => x = format!(" {} ", x),
                }
            }
            x
        }
    }
}
