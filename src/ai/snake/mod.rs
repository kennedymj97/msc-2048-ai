/*
 * TODO:
 * [x] is_move_possible
 * [x] is_left_column_locked
 * [x] does move produce left merge
 * [] improve evaluation framework for ai
 *      [x] need to be able to print rules for a record
 *      [x] make a rule trait, must implement fmt::Display, each rule will then be a struct that
 *      takes whatever, snake will be a list of things that implement rule trait
 *      [x] adapt the sequence code so there is a generic function that takes a list of inputs and
 *      generates all possible mutations
 *      [] change snake ai to have field for the backup case
 * [] automate generation of strategies
 * [] does left merge harm monotonicity
 * [] how to deal with a move right/up when not wanted
 */
use self::rules::Rules;
use crate::ai::AI;
use crate::engine as GameEngine;
use crate::engine::Move;

pub mod attributes;
pub mod evaluate_strategies;
pub mod generate_strategies;
pub mod mann_whitney;
pub mod rules;
pub mod run_strategies;

#[derive(Debug)]
pub struct Snake {
    rules: Rules,
    fallback: Rules,
}

impl Snake {
    pub fn new(rules: Rules, fallback: Rules) -> Box<Self> {
        Box::new(Snake { rules, fallback })
    }
}

impl AI for Snake {
    fn get_next_move(&mut self, board: GameEngine::Board) -> Option<Move> {
        let mut moves_allowed = vec![Move::Up, Move::Down, Move::Left, Move::Right];
        let mut rules = self.rules.clone();
        let mut fallback = self.fallback.clone();
        rules.append(&mut fallback);
        for rule in rules.iter() {
            let res = rule.execute(board).handle(moves_allowed.clone());
            moves_allowed = res.0;
            if res.1 != None {
                return res.1;
            }
        }
        None
    }
}
