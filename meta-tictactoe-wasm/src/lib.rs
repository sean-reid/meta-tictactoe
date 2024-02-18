use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

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
    X = 1, // Represents an X player occupying the cell
    O = 2, // Represents an O player occupying the cell
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy, Debug)]
struct MiniMove {
    row: usize,
    col: usize,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy, Debug)]
struct MetaMove {
    board_row: usize,
    board_col: usize,
    row: usize,
    col: usize,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct MiniBoard {
    cells: [[Cell; 3]; 3],
    winner: Option<GamePlayer>,
}

impl MiniBoard {
    pub fn new() -> Self {
        MiniBoard {
            cells: [[Cell::Empty; 3]; 3],
            winner: None,
        }
    }

    pub fn apply_move(&mut self, mov: MiniMove, player: GamePlayer) -> bool {
        if self.cells[mov.row][mov.col] == Cell::Empty && self.winner.is_none() {
            // Directly use the player enum value to set the cell
            self.cells[mov.row][mov.col] = match player {
                GamePlayer::X => Cell::X,
                GamePlayer::O => Cell::O,
            };
            self.winner = self.check_winner();
            true
        } else {
            false
        }
    }

    pub fn undo_move(&mut self, mov: MiniMove) {
        self.cells[mov.row][mov.col] = Cell::Empty;
        self.winner = self.check_winner(); // Recheck winner status after undoing a move
    }

    pub fn check_winner(&self) -> Option<GamePlayer> {
        // Check rows, columns, and diagonals for a win
        let lines = [
            // Rows
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)],
            [(2, 0), (2, 1), (2, 2)],
            // Columns
            [(0, 0), (1, 0), (2, 0)],
            [(0, 1), (1, 1), (2, 1)],
            [(0, 2), (1, 2), (2, 2)],
            // Diagonals
            [(0, 0), (1, 1), (2, 2)],
            [(0, 2), (1, 1), (2, 0)],
        ];

        for line in &lines {
            let first_cell = self.cells[line[0].0][line[0].1];
            if first_cell == Cell::X || first_cell == Cell::O {
                if line
                    .iter()
                    .all(|&(row, col)| self.cells[row][col] == first_cell)
                {
                    // Convert the Cell type back to GamePlayer type for the winner
                    return match first_cell {
                        Cell::X => Some(GamePlayer::X),
                        Cell::O => Some(GamePlayer::O),
                        _ => None, // This case should not happen
                    };
                }
            }
        }

        None
    }

    pub fn is_full(&self) -> bool {
        self.cells
            .iter()
            .all(|row| row.iter().all(|&cell| cell != Cell::Empty))
    }

    pub fn evaluate(&self, player: GamePlayer) -> i32 {
        let mut player_score = 0;
        let mut opponent_score = 0;
        let lines = [
            // Rows
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)],
            [(2, 0), (2, 1), (2, 2)],
            // Columns
            [(0, 0), (1, 0), (2, 0)],
            [(0, 1), (1, 1), (2, 1)],
            [(0, 2), (1, 2), (2, 2)],
            // Diagonals
            [(0, 0), (1, 1), (2, 2)],
            [(2, 0), (1, 1), (0, 2)],
        ];

        for line in &lines {
            let (player_line_score, opponent_line_score) = self.evaluate_line(line, player);
            player_score += player_line_score;
            opponent_score += opponent_line_score;
        }

        player_score - opponent_score
    }

    pub fn evaluate_line(&self, line: &[(usize, usize)], player: GamePlayer) -> (i32, i32) {
        let mut player_count = 0;
        let mut opponent_count = 0;

        // Convert GamePlayer to the corresponding Cell variant for comparison
        let player_cell = match player {
            GamePlayer::X => Cell::X,
            GamePlayer::O => Cell::O,
        };

        for &(row, col) in line {
            match self.cells[row][col] {
                cell if cell == player_cell => player_count += 1,
                Cell::Empty => (),
                _ => opponent_count += 1, // If the cell is not empty and not the player's cell, it's the opponent's
            }
        }

        let player_line_score = if opponent_count > 0 {
            0
        } else {
            10 - player_count // Adjust the scoring as needed
        };
        let opponent_line_score = if player_count > 0 {
            0
        } else {
            10 - opponent_count // Adjust the scoring as needed
        };

        (player_line_score, opponent_line_score)
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct MetaGameState {
    boards: [[MiniBoard; 3]; 3],
    current_player: GamePlayer,
    next_board: Option<(usize, usize)>, // Specifies which board to play in next, or None if any board is allowed
}

impl MetaGameState {
    pub fn new() -> Self {
        MetaGameState {
            boards: [[MiniBoard::new(); 3]; 3],
            current_player: GamePlayer::X, // X starts the game
            next_board: None,
        }
    }

    pub fn get_next_moves(&self) -> Vec<MetaMove> {
        let mut moves = vec![];
        match self.next_board {
            Some((br, bc)) if self.boards[br][bc].winner.is_none() => {
                for r in 0..3 {
                    for c in 0..3 {
                        if matches!(self.boards[br][bc].cells[r][c], Cell::Empty) {
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
            _ => {
                for br in 0..3 {
                    for bc in 0..3 {
                        if self.boards[br][bc].winner.is_none() {
                            for r in 0..3 {
                                for c in 0..3 {
                                    if matches!(self.boards[br][bc].cells[r][c], Cell::Empty) {
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
                    }
                }
            }
        }
        moves
    }

    pub fn apply_move(&mut self, mov: &MetaMove) -> bool {
        let board_row = (*mov).board_row;
        let board_col = (*mov).board_col;
        let cell_row = (*mov).row;
        let cell_col = (*mov).col;

        // Check if the next move must be made on a specific board
        if let Some((next_board_row, next_board_col)) = self.next_board {
            if next_board_row != board_row || next_board_col != board_col {
                return false; // Move must be made on the specified next board
            }
        }

        // Delegate to the specific SmallBoard's apply_move method
        let valid = self.boards[board_row][board_col].apply_move(
            MiniMove {
                row: cell_row,
                col: cell_col,
            },
            self.current_player,
        );
        if valid {
            self.current_player = self.current_player.switch();
            self.next_board = if self.boards[cell_row][cell_col].winner.is_some() || self.boards[cell_row][cell_col].is_full() {
                None // Player can choose any board if the targeted board is completed
            } else {
                Some((cell_row, cell_col)) // Next move must be made in the same mini-board
            };
        }
        valid
    }

    pub fn undo_move(&mut self, mov: &MetaMove) {
        let board_row = (*mov).board_row;
        let board_col = (*mov).board_col;
        let cell_row = (*mov).row;
        let cell_col = (*mov).col;
        self.boards[board_row][board_col].undo_move(MiniMove {
            row: cell_row,
            col: cell_col,
        });
        self.current_player = self.current_player.switch();
        // Undoing next_board logic requires tracking move history or a more complex state management
    }

    pub fn check_global_winner(&self) -> Option<GamePlayer> {
        // Define lines to check - rows, columns, and diagonals
        let lines = [
            // Rows
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)],
            [(2, 0), (2, 1), (2, 2)],
            // Columns
            [(0, 0), (1, 0), (2, 0)],
            [(0, 1), (1, 1), (2, 1)],
            [(0, 2), (1, 2), (2, 2)],
            // Diagonals
            [(0, 0), (1, 1), (2, 2)],
            [(2, 0), (1, 1), (0, 2)],
        ];

        for line in &lines {
            if let Some(first_winner) = self.boards[line[0].0][line[0].1].winner {
                // Check if all boards in the line have the same winner
                let is_line_winner = line
                    .iter()
                    .all(|&(r, c)| self.boards[r][c].winner == Some(first_winner));

                if is_line_winner {
                    return Some(first_winner);
                }
            }
        }

        None
    }

    pub fn is_global_board_full(&self) -> bool {
        self.boards.iter().all(|row| {
            row.iter()
                .all(|board| board.winner.is_some() || board.is_full())
        })
    }

    pub fn evaluate_global(&self) -> i32 {
        let mut global_evaluation_score = 0;

        // Define lines to check - rows, columns, and diagonals on the global board
        let lines = [
            // Rows
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)],
            [(2, 0), (2, 1), (2, 2)],
            // Columns
            [(0, 0), (1, 0), (2, 0)],
            [(0, 1), (1, 1), (2, 1)],
            [(0, 2), (1, 2), (2, 2)],
            // Diagonals
            [(0, 0), (1, 1), (2, 2)],
            [(2, 0), (1, 1), (0, 2)],
        ];

        // Iterate through each line to evaluate
        for line in &lines {
            let mut player_count = 0;
            let mut opponent_count = 0;
            let mut line_score = 0;

            // Check each MiniBoard in the line
            for &(row, col) in line {
                match self.boards[row][col].winner {
                    Some(winner) if winner == self.current_player => player_count += 1,
                    Some(_) => opponent_count += 1,
                    None => {
                        // Evaluate the mini-board if it doesn't have a winner yet
                        line_score += self.boards[row][col].evaluate(self.current_player);
                    }
                }
            }

            // Update the global evaluation score based on the line evaluation
            global_evaluation_score += self.score_line(player_count, opponent_count);
            global_evaluation_score += line_score;
        }

        global_evaluation_score
    }

    // This helper function scores a line based on the counts of player and opponent wins
    pub fn score_line(&self, player_count: i32, opponent_count: i32) -> i32 {
        // Customize these scores as needed for your evaluation strategy
        match (player_count, opponent_count) {
            (3, _) => 1000,  // Winning condition
            (_, 3) => -1000, // Losing condition
            (2, 0) => 100,   // One move away from winning
            (0, 2) => -100,  // Opponent one move away from winning
            (1, 0) => 10,    // Two moves away from winning
            (0, 1) => -10,   // Opponent two moves away from winning
            _ => -5,         // No advantage or blocked line
        }
    }
}

trait GameState: Copy {
    type Move: Clone;
    fn get_possible_moves(&self) -> Vec<Self::Move>;
    fn apply_move(&mut self, mov: &Self::Move) -> ();
    fn undo_move(&mut self, mov: &Self::Move) -> ();
    fn evaluate(&self) -> i32;
    fn is_game_over(&self) -> bool;
    fn current_player(&self) -> i32;
}

impl GameState for MetaGameState {
    type Move = MetaMove; // ((board_row, board_col), (cell_row, cell_col))

    fn get_possible_moves(&self) -> Vec<Self::Move> {
        self.get_next_moves()
    }

    fn apply_move(&mut self, mov: &Self::Move) {
        self.apply_move(mov);
    }

    fn undo_move(&mut self, mov: &Self::Move) {
        self.undo_move(mov);
    }

    fn evaluate(&self) -> i32 {
        self.evaluate_global()
    }

    fn is_game_over(&self) -> bool {
        self.check_global_winner().is_some() || self.is_global_board_full()
    }

    fn current_player(&self) -> i32 {
        match self.current_player {
            GamePlayer::X => 1,
            GamePlayer::O => -1,
        }
    }
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

    pub fn get_possible_moves(&self) -> JsValue {
        // Convert the moves to a format that JavaScript can understand
        JsValue::from_serde(&self.inner.get_possible_moves()).unwrap()
    }

    pub fn apply_move(&mut self, mov: &MetaMove) -> bool {
        self.inner.apply_move(mov)
    }

    pub fn undo_move(&mut self, mov: &MetaMove) {
        self.inner.undo_move(mov);
    }

    pub fn evaluate(&self) -> i32 {
        self.inner.evaluate()
    }

    pub fn is_game_over(&self) -> bool {
        self.inner.is_game_over()
    }

    pub fn current_player(&self) -> i32 {
        self.inner.current_player()
    }

    pub fn apply_json_move(&mut self, move_js: JsValue) -> bool {
        let mov: MetaMove = from_value(move_js).expect("Could not deserialize move");
        self.inner.apply_move(&mov)
    }


    pub fn get_board_state(&self) -> JsValue {
        let board = self.inner.boards.iter().map(|row| {
            row.iter().map(|sub_board| {
                sub_board.cells.iter().map(|sub_row| {
                    sub_row.iter().map(|&cell| {
                        match cell {
                            Cell::Empty => "",
                            Cell::X => "x",
                            Cell::O => "o",
                        }.into()
                    }).collect::<Vec<String>>()
                }).collect::<Vec<Vec<String>>>()
            }).collect::<Vec<Vec<Vec<String>>>>()
        }).collect::<Vec<Vec<Vec<Vec<String>>>>>();

        JsValue::from_serde(&board).unwrap()
    }
}

#[wasm_bindgen]
pub struct Minimax;

#[wasm_bindgen]
impl Minimax {
    #[wasm_bindgen]
    pub fn find_best_move(game_state: &GameStateWrapper, depth: usize) -> JsValue {
        let mut best_move: Option<MetaMove> = None;
        let mut best_value = i32::MIN;
        let player = game_state.current_player();

        let possible_moves = game_state.get_possible_moves();
        let moves: Vec<MetaMove> = possible_moves.into_serde().unwrap();

        for mov in &moves {
            let mut new_game_state = game_state.clone();
            new_game_state.apply_move(mov);
            let value = Minimax::minimax(&new_game_state, depth - 1, i32::MIN, i32::MAX, player);
            if value > best_value {
                best_value = value;
                best_move = Some(*mov);
            }
        }

        JsValue::from_serde(&best_move.expect("No valid moves available")).unwrap()
    }

    #[wasm_bindgen]
    pub fn minimax(
        game_state: &GameStateWrapper,
        depth: usize,
        alpha: i32,
        beta: i32,
        maximizing_player: i32,
    ) -> i32 {
        if depth == 0 || game_state.is_game_over() {
            return game_state.evaluate();
        }

        let mut alpha = alpha;
        let mut beta = beta;
        let possible_moves = game_state.get_possible_moves();
        let moves: Vec<MetaMove> = possible_moves.into_serde().unwrap();

        if game_state.current_player() == maximizing_player {
            let mut max_eval = i32::MIN;
            for mov in &moves {
                let mut new_game_state = game_state.clone();
                new_game_state.apply_move(mov);
                let eval =
                    Minimax::minimax(&new_game_state, depth - 1, alpha, beta, maximizing_player);
                max_eval = max_eval.max(eval);
                alpha = alpha.max(eval);
                if beta <= alpha {
                    break;
                }
            }
            max_eval
        } else {
            let mut min_eval = i32::MAX;
            for mov in &moves {
                let mut new_game_state = game_state.clone();
                new_game_state.apply_move(mov);
                let eval =
                    Minimax::minimax(&new_game_state, depth - 1, alpha, beta, maximizing_player);
                min_eval = min_eval.min(eval);
                beta = beta.min(eval);
                if beta <= alpha {
                    break;
                }
            }
            min_eval
        }
    }
}
