use crate::board::MiniBoard;
use crate::types::{Cell, GamePlayer, MetaMove, LINES};

#[derive(Clone, Copy, Debug)]
pub struct MetaGameState {
    pub boards: [[MiniBoard; 3]; 3],
    pub current_player: GamePlayer,
    pub next_board: Option<(usize, usize)>,
}

const BOARD_WEIGHT: [[i32; 3]; 3] = [
    [3, 2, 3],
    [2, 5, 2],
    [3, 2, 3],
];

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
                    if self.boards[br][bc].cells[r][c] == Cell::Empty {
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

    /// Evaluate from `player`'s perspective. Positive = good for `player`.
    pub fn evaluate_for(&self, player_id: i32) -> i32 {
        let player = if player_id == GamePlayer::X as i32 { GamePlayer::X } else { GamePlayer::O };
        let opponent = player.switch();

        // Terminal
        if let Some(winner) = self.check_global_winner() {
            return if winner == player { 100_000 } else { -100_000 };
        }

        let mut score: i32 = 0;

        // --- Global line analysis ---
        for line in &LINES {
            let mut p_won = 0i32;
            let mut o_won = 0i32;
            let mut p_threats = 0i32; // boards where player is close to winning
            let mut o_threats = 0i32;
            let mut contested_score = 0i32;

            for &(r, c) in line {
                let weight = BOARD_WEIGHT[r][c];
                let board = &self.boards[r][c];

                match board.winner {
                    Some(w) if w == player => {
                        p_won += 1;
                        score += weight * 40;
                    }
                    Some(_) => {
                        o_won += 1;
                        score -= weight * 40;
                    }
                    None => {
                        let board_eval = board.evaluate_detailed(player);
                        contested_score += board_eval * weight;

                        // Check if player/opponent has a near-win on this sub-board
                        if board.has_two_in_row(player) { p_threats += 1; }
                        if board.has_two_in_row(opponent) { o_threats += 1; }
                    }
                }
            }

            score += global_line_score(p_won, o_won);

            // Threats along global lines amplify the line's value
            if o_won == 0 {
                score += p_threats * 30;
            }
            if p_won == 0 {
                score -= o_threats * 30;
            }

            score += contested_score;
        }

        // --- Routing analysis ---
        // Penalize states where we're forced to a board that's bad for us
        if self.current_player == player {
            if let Some((nr, nc)) = self.next_board {
                let target = &self.boards[nr][nc];
                if target.winner.is_none() && !target.is_full() {
                    // If opponent can win the target board immediately, that's very bad
                    if target.has_winning_move(opponent) {
                        score -= 300;
                    }
                }
            }
        } else {
            // Opponent is to move; if they're forced somewhere bad for them, good for us
            if let Some((nr, nc)) = self.next_board {
                let target = &self.boards[nr][nc];
                if target.winner.is_none() && !target.is_full() {
                    if target.has_winning_move(player) {
                        score += 300;
                    }
                }
            }
        }

        score
    }
}

fn global_line_score(player: i32, opponent: i32) -> i32 {
    match (player, opponent) {
        (3, _) => 10_000,
        (_, 3) => -10_000,
        (2, 0) => 800,
        (0, 2) => -800,
        (1, 0) => 60,
        (0, 1) => -60,
        _ => 0,
    }
}
