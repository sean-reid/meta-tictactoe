use crate::types::{Cell, GamePlayer, LINES};

#[derive(Clone, Copy, Debug)]
pub struct MiniBoard {
    pub cells: [[Cell; 3]; 3],
    pub winner: Option<GamePlayer>,
}

impl MiniBoard {
    pub fn new() -> Self {
        MiniBoard {
            cells: [[Cell::Empty; 3]; 3],
            winner: None,
        }
    }

    pub fn apply_move(&mut self, row: usize, col: usize, player: GamePlayer) -> bool {
        if self.cells[row][col] != Cell::Empty || self.winner.is_some() {
            return false;
        }
        self.cells[row][col] = Cell::from_player(player);
        self.winner = self.check_winner();
        true
    }

    pub fn check_winner(&self) -> Option<GamePlayer> {
        for line in &LINES {
            let first = self.cells[line[0].0][line[0].1];
            if first == Cell::Empty {
                continue;
            }
            if line.iter().all(|&(r, c)| self.cells[r][c] == first) {
                return first.to_player();
            }
        }
        None
    }

    pub fn is_full(&self) -> bool {
        self.cells.iter().all(|row| row.iter().all(|&c| c != Cell::Empty))
    }

    pub fn evaluate(&self, player: GamePlayer) -> i32 {
        let player_cell = Cell::from_player(player);
        let mut score = 0;

        for line in &LINES {
            let mut player_count = 0;
            let mut opponent_count = 0;

            for &(r, c) in line {
                match self.cells[r][c] {
                    c if c == player_cell => player_count += 1,
                    Cell::Empty => (),
                    _ => opponent_count += 1,
                }
            }

            if opponent_count == 0 {
                score += 10 - player_count;
            }
            if player_count == 0 {
                score -= 10 - opponent_count;
            }
        }

        score
    }
}
