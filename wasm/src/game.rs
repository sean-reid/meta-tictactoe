use crate::board::MiniBoard;
use crate::types::{GamePlayer, MetaMove, LINES};

#[derive(Clone, Copy, Debug)]
pub struct MetaGameState {
    pub boards: [[MiniBoard; 3]; 3],
    pub current_player: GamePlayer,
    pub next_board: Option<(usize, usize)>,
}

impl MetaGameState {
    pub fn new() -> Self {
        MetaGameState {
            boards: [[MiniBoard::new(); 3]; 3],
            current_player: GamePlayer::X,
            next_board: None,
        }
    }

    pub fn get_next_moves(&self) -> Vec<MetaMove> {
        let mut moves = Vec::new();

        let boards_to_check: Vec<(usize, usize)> = match self.next_board {
            Some((br, bc)) if self.boards[br][bc].winner.is_none() && !self.boards[br][bc].is_full() => {
                vec![(br, bc)]
            }
            _ => {
                let mut v = Vec::new();
                for br in 0..3 {
                    for bc in 0..3 {
                        if self.boards[br][bc].winner.is_none() && !self.boards[br][bc].is_full() {
                            v.push((br, bc));
                        }
                    }
                }
                v
            }
        };

        for (br, bc) in boards_to_check {
            for r in 0..3 {
                for c in 0..3 {
                    if self.boards[br][bc].cells[r][c] == crate::types::Cell::Empty {
                        moves.push(MetaMove {
                            board_row: br,
                            board_col: bc,
                            row: r,
                            col: c,
                        });
                    }
                }
            }
        }

        moves
    }

    pub fn apply_move(&mut self, mov: &MetaMove) -> bool {
        if let Some((nbr, nbc)) = self.next_board {
            if nbr != mov.board_row || nbc != mov.board_col {
                // If the forced board is still playable, reject
                if self.boards[nbr][nbc].winner.is_none() && !self.boards[nbr][nbc].is_full() {
                    return false;
                }
            }
        }

        let valid = self.boards[mov.board_row][mov.board_col]
            .apply_move(mov.row, mov.col, self.current_player);

        if valid {
            self.current_player = self.current_player.switch();
            let target = &self.boards[mov.row][mov.col];
            self.next_board = if target.winner.is_some() || target.is_full() {
                None
            } else {
                Some((mov.row, mov.col))
            };
        }
        valid
    }

    pub fn check_global_winner(&self) -> Option<GamePlayer> {
        for line in &LINES {
            if let Some(first_winner) = self.boards[line[0].0][line[0].1].winner {
                if line.iter().all(|&(r, c)| self.boards[r][c].winner == Some(first_winner)) {
                    return Some(first_winner);
                }
            }
        }
        None
    }

    pub fn is_global_board_full(&self) -> bool {
        self.boards.iter().all(|row| {
            row.iter().all(|b| b.winner.is_some() || b.is_full())
        })
    }

    pub fn is_game_over(&self) -> bool {
        self.check_global_winner().is_some() || self.is_global_board_full()
    }

    pub fn evaluate_global(&self) -> i32 {
        let mut score = 0;

        for line in &LINES {
            let mut player_count = 0;
            let mut opponent_count = 0;
            let mut line_score = 0;

            for &(row, col) in line {
                match self.boards[row][col].winner {
                    Some(w) if w == self.current_player => player_count += 1,
                    Some(_) => opponent_count += 1,
                    None => {
                        line_score += self.boards[row][col].evaluate(self.current_player);
                    }
                }
            }

            score += score_line(player_count, opponent_count);
            score += line_score;
        }

        score
    }
}

fn score_line(player_count: i32, opponent_count: i32) -> i32 {
    match (player_count, opponent_count) {
        (3, _) => 1000,
        (_, 3) => -1000,
        (2, 0) => 100,
        (0, 2) => -100,
        (1, 0) => 10,
        (0, 1) => -10,
        _ => -5,
    }
}
