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
use self::rules::BanRules;
use self::rules::TryRules;
use crate::ai::AI;
use crate::engine::Board;
use crate::engine::GameEngine;
use crate::engine::Move;
use std::fmt;

pub mod attributes;
pub mod evaluate_strategies;
pub mod generate_strategies;
pub mod mann_whitney;
pub mod rules;
pub mod run_strategies;

#[derive(Clone)]
pub struct Snake {
    ban_rules: BanRules,
    try_rules: TryRules,
    fallback_moves: Vec<Move>,
}

impl Snake {
    pub fn new(
        ban_rules: &BanRules,
        try_rules: &TryRules,
        fallback_moves: &Vec<Move>,
    ) -> Box<Self> {
        Box::new(Snake {
            ban_rules: ban_rules.clone(),
            try_rules: try_rules.clone(),
            fallback_moves: fallback_moves.clone(),
        })
    }
}

impl AI for Snake {
    fn get_next_move(&mut self, engine: &GameEngine, board: Board) -> Option<Move> {
        let mut banned_moves = Vec::new();
        for ban_rule in self.ban_rules.iter() {
            match ban_rule.execute(engine, board) {
                Some(direction) => banned_moves.push(direction),
                None => (),
            }
        }

        for try_rule in self.try_rules.iter() {
            match try_rule.execute(engine, board) {
                Some(direction) => {
                    if !banned_moves.contains(&direction) {
                        return Some(direction);
                    }
                }
                None => (),
            }
        }

        for &direction in self.fallback_moves.iter() {
            if attributes::is_move_possible(engine, board, direction) {
                return Some(direction);
            }
        }
        None
    }
}

impl fmt::Display for Snake {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Ban Rules: {:?}\tTry Rules: {:?}\tFallback: {:?}",
            self.ban_rules, self.try_rules, self.fallback_moves
        )
    }
}
