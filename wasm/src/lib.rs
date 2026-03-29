mod board;
mod game;
mod minimax;
mod types;

use game::MetaGameState;
use serde::{Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use types::{Cell, GamePlayer, MetaMove};
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct BoardPos {
    row: usize,
    col: usize,
}

#[derive(Clone)]
#[wasm_bindgen]
pub struct GameStateWrapper {
    inner: MetaGameState,
}

#[wasm_bindgen]
impl GameStateWrapper {
    pub fn new() -> GameStateWrapper {
        GameStateWrapper {
            inner: MetaGameState::new(),
        }
    }

    pub fn apply_json_move(&mut self, move_js: JsValue) -> bool {
        let mov: MetaMove = from_value(move_js).expect("Could not deserialize move");
        self.inner.apply_move(&mov)
    }

    pub fn get_board_state(&self) -> JsValue {
        let board: Vec<Vec<Vec<Vec<&str>>>> = self.inner.boards.iter().map(|row| {
            row.iter().map(|sub_board| {
                sub_board.cells.iter().map(|sub_row| {
                    sub_row.iter().map(|cell| match cell {
                        Cell::Empty => "",
                        Cell::X => "x",
                        Cell::O => "o",
                    }).collect()
                }).collect()
            }).collect()
        }).collect();
        to_value(&board).unwrap()
    }

    pub fn get_next_board(&self) -> JsValue {
        match self.inner.next_board {
            Some((r, c)) => to_value(&BoardPos { row: r, col: c }).unwrap(),
            None => JsValue::NULL,
        }
    }

    pub fn get_board_winners(&self) -> JsValue {
        let winners: Vec<Vec<&str>> = self.inner.boards.iter().map(|row| {
            row.iter().map(|b| match b.winner {
                Some(GamePlayer::X) => "x",
                Some(GamePlayer::O) => "o",
                None => "",
            }).collect()
        }).collect();
        to_value(&winners).unwrap()
    }

    pub fn get_global_winner(&self) -> JsValue {
        match self.inner.check_global_winner() {
            Some(GamePlayer::X) => JsValue::from_str("x"),
            Some(GamePlayer::O) => JsValue::from_str("o"),
            None => JsValue::NULL,
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.inner.is_game_over()
    }

    pub fn current_player(&self) -> i32 {
        match self.inner.current_player {
            GamePlayer::X => 1,
            GamePlayer::O => -1,
        }
    }
}

#[wasm_bindgen]
pub struct Minimax;

#[wasm_bindgen]
impl Minimax {
    pub fn find_best_move(game_state: &GameStateWrapper, depth: usize) -> JsValue {
        let mov = minimax::find_best_move(&game_state.inner, depth)
            .expect("No valid moves available");
        to_value(&mov).unwrap()
    }
}
