use crate::ai::expectimax_old::Expectimaxx;
use crate::ai::snake::{
    attributes::Column, attributes::Corner, attributes::Row, ban_rules::BanMove,
    try_rules::TryMove, Snake,
};
use crate::ai::{AI, AII};
use crate::engine::{GameEngineNoStores, Move};

use crate::engine_unsafe as GameEngine;
use crate::engine_unsafe::Board;
use crate::engine_unsafe::Move as UnsafeMove;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmExpectimax;

#[wasm_bindgen]
impl WasmExpectimax {
    pub fn new() -> Self {
        GameEngine::new();
        Expectimaxx::new();
        WasmExpectimax
    }

    pub fn get_next_move(&self, board: Board) -> i32 {
        match Expectimaxx.get_next_move(board) {
            Some(direction) => match direction {
                UnsafeMove::Up => 0,
                UnsafeMove::Right => 1,
                UnsafeMove::Down => 2,
                UnsafeMove::Left => 3,
            },
            None => -1,
        }
    }
}

#[wasm_bindgen]
pub struct WasmSnake(Snake);

#[wasm_bindgen]
impl WasmSnake {
    pub fn new() -> Self {
        let ban_rules = vec![BanMove::IfColumnNotLocked(Move::Up, Column::Left)];
        let try_rules = vec![
            TryMove::IfMovesLargestTileToCorner(Move::Left, Corner::BottomLeft),
            TryMove::ProducesMerge(Move::Up),
            TryMove::ProducesMerge(Move::Down),
            TryMove::CreatesMonotonicRow(Move::Down, Row::MiddleTop),
        ];
        let fallback = vec![Move::Left, Move::Down, Move::Up, Move::Right];
        let snake = Snake::new(&ban_rules, &try_rules, &fallback).unwrap();
        WasmSnake(snake)
    }

    pub fn get_next_move(&mut self, board: Board) -> i32 {
        match self.0.get_next_move(&GameEngineNoStores, board) {
            Some(direction) => match direction {
                Move::Up => 0,
                Move::Right => 1,
                Move::Down => 2,
                Move::Left => 3,
            },
            None => -1,
        }
    }
}
