use std::fmt;

fn main() {
    let game = GameState::from(81985529216486895);
    println!("{}", game);
}

struct GameState(u64);

impl GameState {
    fn new() -> GameState {
        GameState(0)
    }

    fn from(val: u64) -> GameState {
        GameState(val)
    }

    fn to_binary(&self) -> String {
        let mut temp = self.0;
        let mut bits = String::new();
        while temp > 0 {
            if temp % 2 == 0 {
                bits.push('0')
            } else {
                bits.push('1')
            };

            temp /= 2;
        }
        while bits.len() < 64 {
            bits.push('0');
        }
        bits.chars().rev().collect::<String>()
    }

    fn parse_board(&self) -> [Option<u32>; 16] {
        let bin = self.to_binary();
        assert!(bin.len() == 64);
        let board = (0..16).fold([None; 16], |mut arr, idx| {
            let bin_idx = idx * 4;
            let num = binary_to_decimal(&bin[bin_idx..bin_idx + 4]);

            match num {
                0 => arr[idx] = None,
                _ => arr[idx] = Some((2 as u32).pow(num)),
            }

            arr
        });
        board
    }
}

fn binary_to_decimal(bin: &str) -> u32 {
    assert!(bin.len() == 4);
    let num = (0..).zip(bin.chars()).fold(0, |acc, (idx, val)| {
        let mut store = 0;
        if let Some(x) = val.to_digit(10) {
            store = x;
        }
        acc + store * (2 as u32).pow(bin.len() as u32 - (idx + 1))
    });
    num
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
