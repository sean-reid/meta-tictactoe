use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Win-check lines: rows, columns, diagonals for a 3x3 grid.
pub const LINES: [[(usize, usize); 3]; 8] = [
    [(0, 0), (0, 1), (0, 2)],
    [(1, 0), (1, 1), (1, 2)],
    [(2, 0), (2, 1), (2, 2)],
    [(0, 0), (1, 0), (2, 0)],
    [(0, 1), (1, 1), (2, 1)],
    [(0, 2), (1, 2), (2, 2)],
    [(0, 0), (1, 1), (2, 2)],
    [(2, 0), (1, 1), (0, 2)],
];

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GamePlayer {
    X = 1,
    O = 2,
}

impl GamePlayer {
    pub fn switch(self) -> GamePlayer {
        match self {
            GamePlayer::X => GamePlayer::O,
            GamePlayer::O => GamePlayer::X,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cell {
    Empty = 0,
    X = 1,
    O = 2,
}

impl Cell {
    pub fn from_player(player: GamePlayer) -> Cell {
        match player {
            GamePlayer::X => Cell::X,
            GamePlayer::O => Cell::O,
        }
    }

    pub fn to_player(self) -> Option<GamePlayer> {
        match self {
            Cell::X => Some(GamePlayer::X),
            Cell::O => Some(GamePlayer::O),
            Cell::Empty => None,
        }
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct MetaMove {
    pub board_row: usize,
    pub board_col: usize,
    pub row: usize,
    pub col: usize,
}
