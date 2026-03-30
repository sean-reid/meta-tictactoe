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

    /// Detailed evaluation of a sub-board from `player`'s perspective.
    pub fn evaluate_detailed(&self, player: GamePlayer) -> i32 {
        let player_cell = Cell::from_player(player);
        let mut score = 0;

        for line in &LINES {
            let mut p = 0;
            let mut o = 0;

            for &(r, c) in line {
                match self.cells[r][c] {
                    c if c == player_cell => p += 1,
                    Cell::Empty => (),
                    _ => o += 1,
                }
            }

            // Only score lines that aren't blocked
            match (p, o) {
                (3, 0) => score += 100, // won line (shouldn't normally reach here since winner is cached)
                (2, 0) => score += 12,  // one move from winning
                (1, 0) => score += 2,   // some presence
                (0, 3) => score -= 100,
                (0, 2) => score -= 12,
                (0, 1) => score -= 2,
                _ => (),                // blocked line, worthless
            }
        }

        // Center control bonus
        if self.cells[1][1] == player_cell {
            score += 4;
        } else if self.cells[1][1] != Cell::Empty {
            score -= 4;
        }

        score
    }

    /// Does `player` have two in a row with the third cell empty on any line?
    pub fn has_two_in_row(&self, player: GamePlayer) -> bool {
        let player_cell = Cell::from_player(player);
        for line in &LINES {
            let mut p = 0;
            let mut empty = 0;
            for &(r, c) in line {
                match self.cells[r][c] {
                    c if c == player_cell => p += 1,
                    Cell::Empty => empty += 1,
                    _ => { p = 0; break; } // blocked
                }
            }
            if p == 2 && empty == 1 {
                return true;
            }
        }
        false
    }

    /// Does `player` have an immediate winning move on this board?
    pub fn has_winning_move(&self, player: GamePlayer) -> bool {
        if self.winner.is_some() {
            return false;
        }
        let player_cell = Cell::from_player(player);
        for line in &LINES {
            let mut p = 0;
            let mut empty_pos = None;
            let mut blocked = false;
            for &(r, c) in line {
                match self.cells[r][c] {
                    c if c == player_cell => p += 1,
                    Cell::Empty => empty_pos = Some((r, c)),
                    _ => { blocked = true; break; }
                }
            }
            if !blocked && p == 2 && empty_pos.is_some() {
                return true;
            }
        }
        false
    }

}
