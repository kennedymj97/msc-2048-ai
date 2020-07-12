use super::attributes;
use crate::engine as GameEngine;
use crate::engine::Move;
use std::fmt;

#[derive(PartialEq)]
pub enum Result {
    Try(Move),
    Ban(Move),
    Force(Move),
    Proceed,
}

impl Result {
    pub fn handle(self, moves_allowed: Vec<Move>) -> (Vec<Move>, Option<Move>) {
        match self {
            Result::Try(direction) => match moves_allowed.contains(&direction) {
                true => (moves_allowed, Some(direction)),
                false => (moves_allowed, None),
            },
            Result::Ban(direction) => (remove_item(moves_allowed, direction), None),
            Result::Force(direction) => (moves_allowed, Some(direction)),
            Result::Proceed => (moves_allowed, None),
        }
    }
}

fn remove_item(vec: Vec<Move>, direction: Move) -> Vec<Move> {
    vec.iter()
        .filter(|&&move_dir| move_dir != direction)
        .copied()
        .collect()
}

pub type Strategy = Vec<Box<dyn Rule>>;

pub fn strategy_to_str(strategy: &Strategy) -> String {
    let mut strategy_str = String::new();
    let mut strategy_iter = strategy.iter().peekable();
    while let Some(rule) = strategy_iter.next() {
        strategy_str.push_str(&format!("{}", rule));
        if strategy_iter.peek().is_some() {
            strategy_str.push_str(" -> ");
        }
    }
    strategy_str
}

pub trait Rule: fmt::Display + RuleClone {
    fn execute(&self, board: GameEngine::Board) -> Result;
}

pub trait RuleClone {
    fn clone_box(&self) -> Box<dyn Rule>;
}

impl<T> RuleClone for T
where
    T: 'static + Rule + Clone,
{
    fn clone_box(&self) -> Box<dyn Rule> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Rule> {
    fn clone(&self) -> Box<dyn Rule> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct ForceMoveIfPossible {
    direction: Move,
}

impl ForceMoveIfPossible {
    pub fn new(direction: Move) -> Box<dyn Rule> {
        Box::new(ForceMoveIfPossible { direction })
    }
}

impl fmt::Display for ForceMoveIfPossible {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Force move in direction: {} if possible", self.direction)
    }
}

impl Rule for ForceMoveIfPossible {
    fn execute(&self, board: GameEngine::Board) -> Result {
        if attributes::is_move_possible(board, self.direction) {
            return Result::Force(self.direction);
        }
        Result::Ban(self.direction)
    }
}

#[derive(Clone)]
pub struct BanMoveIfLeftColumnLocked {
    direction: Move,
}

impl BanMoveIfLeftColumnLocked {
    pub fn new(direction: Move) -> Box<dyn Rule> {
        Box::new(BanMoveIfLeftColumnLocked { direction })
    }
}

impl fmt::Display for BanMoveIfLeftColumnLocked {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Ban move in direction: {} if left column locked",
            self.direction
        )
    }
}

impl Rule for BanMoveIfLeftColumnLocked {
    fn execute(&self, board: GameEngine::Board) -> Result {
        if attributes::is_column_locked(board, 0) {
            return Result::Proceed;
        }
        Result::Ban(self.direction)
    }
}

#[derive(Clone)]
pub struct TryMoveIfProducesLeftMerge {
    direction: Move,
}

impl TryMoveIfProducesLeftMerge {
    pub fn new(direction: Move) -> Box<dyn Rule> {
        Box::new(TryMoveIfProducesLeftMerge { direction })
    }
}

impl fmt::Display for TryMoveIfProducesLeftMerge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Try move in direction: {} if produces a left merge",
            self.direction
        )
    }
}

impl Rule for TryMoveIfProducesLeftMerge {
    fn execute(&self, board: GameEngine::Board) -> Result {
        if attributes::does_move_produce_merge_in_direction(board, self.direction, Move::Left) {
            return Result::Try(self.direction);
        }
        Result::Proceed
    }
}

#[derive(Clone)]
pub struct TryMoveIfMergePossible {
    direction: Move,
}

impl TryMoveIfMergePossible {
    pub fn new(direction: Move) -> Box<dyn Rule> {
        Box::new(TryMoveIfMergePossible { direction })
    }
}

impl fmt::Display for TryMoveIfMergePossible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Try move in direction: {} if merge possible",
            self.direction
        )
    }
}

impl Rule for TryMoveIfMergePossible {
    fn execute(&self, board: GameEngine::Board) -> Result {
        if attributes::is_merge_possible(board, self.direction) {
            return Result::Try(self.direction);
        }
        Result::Proceed
    }
}
