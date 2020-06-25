/*
 * TODO:
 * [x] is_move_possible
 * [x] is_left_column_locked
 * [] does move produce left merge
 */
use crate::ai::AI;
use crate::engine as GameEngine;
use crate::engine::shift;
use crate::engine::Move;

#[derive(Debug, PartialEq)]
pub enum RuleResult {
    Execute(Move),
    Ban(Move),
    Force(Move),
    Proceed,
}

impl RuleResult {
    fn handle(self, moves_allowed: Vec<Move>) -> (Vec<Move>, Option<Move>) {
        match self {
            RuleResult::Execute(direction) => match moves_allowed.contains(&direction) {
                true => (moves_allowed, Some(direction)),
                false => (moves_allowed, None),
            },
            RuleResult::Ban(direction) => (remove_item(moves_allowed, direction), None),
            RuleResult::Force(direction) => (moves_allowed, Some(direction)),
            RuleResult::Proceed => (moves_allowed, None),
        }
    }
}

fn remove_item(vec: Vec<Move>, direction: Move) -> Vec<Move> {
    vec.iter()
        .filter(|&&move_dir| move_dir != direction)
        .copied()
        .collect()
}

type Rules = Vec<fn(GameEngine::Board) -> RuleResult>;

pub struct Snake {
    rules: Rules,
}

impl Snake {
    pub fn new(rules: Rules) -> Self {
        Snake { rules }
    }
}

impl AI for Snake {
    fn get_next_move(&mut self, board: GameEngine::Board) -> Option<Move> {
        let mut moves_allowed = vec![Move::Up, Move::Down, Move::Left, Move::Right];
        for rule in self.rules.iter() {
            let res = rule(board).handle(moves_allowed.clone());
            moves_allowed = res.0;
            if res.1 != None {
                return res.1;
            }
        }
        None
    }
}

pub mod rules {
    use super::*;

    pub fn is_move_possible(board: GameEngine::Board, direction: Move) -> RuleResult {
        if board == GameEngine::shift(board, direction) {
            return RuleResult::Ban(direction);
        }
        RuleResult::Force(direction)
    }

    pub fn is_left_column_locked(board: GameEngine::Board) -> RuleResult {
        let mut previous_val = 0;
        for i in 0..4 {
            let val = GameEngine::get_tile(board, i, 0);
            if val == 0 || val == previous_val {
                return RuleResult::Ban(Move::Up);
            }
            previous_val = val
        }
        RuleResult::Proceed
    }

    pub fn does_move_produce_left_merge(board: GameEngine::Board, direction: Move) -> RuleResult {
        let new_board = GameEngine::shift(board, direction);
        if board == new_board {
            return RuleResult::Proceed;
        }
        match is_merge_left_possible(new_board) {
            RuleResult::Execute(Move::Left) => RuleResult::Execute(direction),
            _ => RuleResult::Proceed,
        }
    }

    pub fn is_merge_left_possible(board: GameEngine::Board) -> RuleResult {
        for row_idx in 0..4 {
            let mut previous_val = 0;
            for col_idx in 0..4 {
                let val = GameEngine::get_tile(board, row_idx, col_idx);
                if val == previous_val && val != 0 {
                    return RuleResult::Execute(Move::Left);
                }
                previous_val = val
            }
        }
        RuleResult::Proceed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_move_possible() {
        GameEngine::new_game();
        assert_eq!(
            rules::is_move_possible(0x1111222233334444, Move::Left),
            RuleResult::Execute(Move::Left)
        );
        assert_eq!(
            rules::is_move_possible(0x1234123412341234, Move::Left),
            RuleResult::Ban(Move::Left)
        );
        assert_eq!(
            rules::is_move_possible(0x1111123412341234, Move::Right),
            RuleResult::Execute(Move::Right)
        );
        assert_eq!(
            rules::is_move_possible(0x1234123412341234, Move::Right),
            RuleResult::Ban(Move::Right)
        );
        assert_eq!(
            rules::is_move_possible(0x1111123423452345, Move::Down),
            RuleResult::Execute(Move::Down)
        );
        assert_eq!(
            rules::is_move_possible(0x1111123423452345, Move::Up),
            RuleResult::Execute(Move::Up)
        );
        assert_eq!(
            rules::is_move_possible(0x1234432112344321, Move::Up),
            RuleResult::Ban(Move::Up)
        );
        assert_eq!(
            rules::is_move_possible(0x1234432112344321, Move::Down),
            RuleResult::Ban(Move::Down)
        );
    }

    #[test]
    fn it_is_left_column_locked() {
        GameEngine::new_game();
        assert_eq!(
            rules::is_left_column_locked(0x1000200030004000),
            RuleResult::Proceed
        );
        assert_eq!(
            rules::is_left_column_locked(0x1000100020003000),
            RuleResult::Ban(Move::Up)
        );
        assert_eq!(
            rules::is_left_column_locked(0x0000000000000000),
            RuleResult::Ban(Move::Up)
        );
    }

    #[test]
    fn it_does_move_produce_left_merge() {
        GameEngine::new_game();
        assert_eq!(
            rules::does_move_produce_left_merge(0x1000210032004300, Move::Up),
            RuleResult::Execute(Move::Up)
        );
        assert_eq!(
            rules::does_move_produce_left_merge(0x1000210032004300, Move::Down),
            RuleResult::Proceed
        );
        assert_eq!(
            rules::does_move_produce_left_merge(0x1200230034004000, Move::Down),
            RuleResult::Execute(Move::Down)
        );
        assert_eq!(
            rules::does_move_produce_left_merge(0x1200230034004000, Move::Up),
            RuleResult::Proceed
        );
        assert_eq!(
            rules::does_move_produce_left_merge(0x0000100021103221, Move::Down),
            RuleResult::Proceed
        );
    }
}
