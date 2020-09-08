/*
 * TODO:
 * [x] is_move_possible
 * [x] is_left_column_locked
 * [x] does move produce left merge
 * [] improve evaluation framework for ai
 *      [x] need to be able to print rules for a record
 *      [x] make a rule trait, must implement fmt::Display, each rule will then be a struct that
 *      takes whatever, strategy will be a list of things that implement rule trait
 *      [x] adapt the sequence code so there is a generic function that takes a list of inputs and
 *      generates all possible mutations
 *      [] change strategy ai to have field for the backup case
 * [] automate generation of strategies
 * [] does left merge harm monotonicity
 * [] how to deal with a move right/up when not wanted
 */
use self::ban_rules::BanMove;
use self::ban_rules::BanRules;
use self::try_rules::TryMove;
use self::try_rules::TryRules;
use crate::ai::AI;
use crate::engine::Board;
use crate::engine::GameEngine;
use crate::engine::Move;
use std::fmt;

pub mod attributes;
pub mod ban_rules;
pub mod evaluate_strategies;
pub mod generate_strategies;
pub mod mann_whitney;
pub mod search;
pub mod try_rules;

#[derive(Clone, PartialEq, Debug)]
pub struct Strategy {
    ban_rules: BanRules,
    try_rules: TryRules,
    fallback_moves: Vec<Move>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Rule {
    Ban(BanMove),
    Try(TryMove),
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rule::Ban(ban_rule) => write!(f, "{}", ban_rule),
            Rule::Try(try_rule) => write!(f, "{}", try_rule),
        }
    }
}

pub type Rules = Vec<Rule>;

impl Strategy {
    pub fn new(
        ban_rules: &BanRules,
        try_rules: &TryRules,
        fallback_moves: &Vec<Move>,
    ) -> Option<Self> {
        // if last try move direction is same as first fallback move it is a redundant strategy
        if let Some(try_rule) = try_rules.last() {
            let last_try_direction = try_rule.get_move();
            if let Some(first_fallback_direction) = fallback_moves.first() {
                if last_try_direction == *first_fallback_direction {
                    return None;
                }
            }
        }

        // if ban rule direction is not in try sequence it is a redundant strategy
        //let try_rule_directions = try_rules
        //    .iter()
        //    .map(|&try_rule| try_rule.get_move())
        //    .collect::<Vec<_>>();
        //for ban_rule in ban_rules {
        //    let ban_direction = ban_rule.get_move();
        //    if !try_rule_directions.contains(&ban_direction) {
        //        return None;
        //    }
        //}

        // TODO: ensure no 2 rules are the same in try_rules or ban rules

        Some(Strategy {
            ban_rules: ban_rules.clone(),
            try_rules: try_rules.clone(),
            fallback_moves: fallback_moves.clone(),
        })
    }

    pub fn get_rules_ban_first(&self) -> Rules {
        let mut ban_rules = self
            .ban_rules
            .iter()
            .map(|&rule| Rule::Ban(rule))
            .collect::<Rules>();
        let mut try_rules = self
            .try_rules
            .iter()
            .map(|&rule| Rule::Try(rule))
            .collect::<Rules>();
        ban_rules.append(&mut try_rules);
        ban_rules
    }

    pub fn get_rules_try_first(&self) -> Rules {
        let mut try_rules = self
            .try_rules
            .iter()
            .map(|&rule| Rule::Try(rule))
            .collect::<Rules>();
        let mut ban_rules = self
            .ban_rules
            .iter()
            .map(|&rule| Rule::Ban(rule))
            .collect::<Rules>();
        try_rules.append(&mut ban_rules);
        try_rules
    }

    pub fn swap_ban_rule(&self, rule_to_swap: BanMove, new_rule: BanMove) -> Option<Self> {
        Strategy::new(
            &self
                .ban_rules
                .iter()
                .map(|&rule| if rule == rule_to_swap { new_rule } else { rule })
                .collect::<BanRules>(),
            &self.try_rules,
            &self.fallback_moves,
        )
    }

    pub fn swap_try_rule(&self, rule_to_swap: TryMove, new_rule: TryMove) -> Option<Self> {
        Strategy::new(
            &self.ban_rules,
            &self
                .try_rules
                .iter()
                .map(|&rule| if rule == rule_to_swap { new_rule } else { rule })
                .collect::<TryRules>(),
            &self.fallback_moves,
        )
    }
}

impl AI for Strategy {
    fn get_next_move<T: GameEngine>(&mut self, engine: &T, board: Board) -> Option<Move> {
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
                        if attributes::is_move_possible(engine, board, direction) {
                            return Some(direction);
                        }
                    }
                }
                None => (),
            }
        }

        // try to make the fallback moves before forcing them
        for &direction in self.fallback_moves.iter() {
            if !banned_moves.contains(&direction) {
                if attributes::is_move_possible(engine, board, direction) {
                    return Some(direction);
                }
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

impl fmt::Display for Strategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Ban Rules: {}\tTry Rules: {}\tFallback: {}",
            vec_to_string_for_csv(&self.ban_rules),
            vec_to_string_for_csv(&self.try_rules),
            vec_to_string_for_csv(&self.fallback_moves),
        )
    }
}

fn vec_to_string_for_csv<T: fmt::Display>(vec: &[T]) -> String {
    let mut vec_iter = vec.iter().peekable();
    let mut result = String::new();
    while let Some(item) = vec_iter.next() {
        result.push_str(&item.to_string());
        if vec_iter.peek().is_some() {
            result.push_str("->");
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::attributes::Column;
    use super::*;

    #[test]
    fn it_new_strategy() {
        let fallback_moves = vec![Move::Left, Move::Down, Move::Up, Move::Left];
        assert_eq!(
            Strategy::new(
                &vec![],
                &vec![TryMove::IfMergePossible(Move::Left)],
                &fallback_moves,
            ),
            None
        );
    }

    #[test]
    fn it_get_rules() {
        let strategy = Strategy::new(
            &vec![BanMove::IfColumnNotLocked(Move::Up, Column::Left)],
            &vec![
                TryMove::ProducesMerge(Move::Up),
                TryMove::ProducesMerge(Move::Down),
            ],
            &vec![Move::Left, Move::Down, Move::Up, Move::Right],
        )
        .unwrap();
        assert_eq!(
            strategy.get_rules_ban_first(),
            vec![
                Rule::Ban(BanMove::IfColumnNotLocked(Move::Up, Column::Left)),
                Rule::Try(TryMove::ProducesMerge(Move::Up)),
                Rule::Try(TryMove::ProducesMerge(Move::Down))
            ]
        );
    }

    #[test]
    fn it_swap() {
        let strategy = Strategy::new(
            &vec![BanMove::IfColumnNotLocked(Move::Up, Column::Left)],
            &vec![
                TryMove::ProducesMerge(Move::Up),
                TryMove::ProducesMerge(Move::Down),
            ],
            &vec![Move::Left, Move::Down, Move::Up, Move::Right],
        )
        .unwrap();

        let try_swap_strategy = Strategy::new(
            &vec![BanMove::IfColumnNotLocked(Move::Up, Column::Left)],
            &vec![
                TryMove::IfMergePossible(Move::Left),
                TryMove::ProducesMerge(Move::Down),
            ],
            &vec![Move::Left, Move::Down, Move::Up, Move::Right],
        )
        .unwrap();

        let ban_swap_strategy = Strategy::new(
            &vec![BanMove::IfColumnNotLocked(Move::Down, Column::Left)],
            &vec![
                TryMove::IfMergePossible(Move::Left),
                TryMove::ProducesMerge(Move::Down),
            ],
            &vec![Move::Left, Move::Down, Move::Up, Move::Right],
        )
        .unwrap();

        let strategy2 = strategy
            .swap_try_rule(
                TryMove::ProducesMerge(Move::Up),
                TryMove::IfMergePossible(Move::Left),
            )
            .unwrap();
        let strategy3 = strategy2
            .swap_ban_rule(
                BanMove::IfColumnNotLocked(Move::Up, Column::Left),
                BanMove::IfColumnNotLocked(Move::Down, Column::Left),
            )
            .unwrap();

        assert_eq!(try_swap_strategy, strategy2);
        assert_eq!(ban_swap_strategy, strategy3);
    }
}
