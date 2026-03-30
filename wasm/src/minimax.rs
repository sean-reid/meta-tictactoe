use crate::game::MetaGameState;
use crate::types::MetaMove;

const TIME_BUDGET_MS: f64 = 1500.0;
const WIN_SCORE: i32 = 100_000;

/// Iterative deepening: searches depth 1, 2, 3... until time runs out.
/// Returns the best move from the deepest completed search.
pub fn find_best_move(state: &MetaGameState, max_depth: usize) -> Option<MetaMove> {
    let start = js_sys::Date::now();
    let mut moves = state.get_next_moves();
    if moves.is_empty() {
        return None;
    }
    if moves.len() == 1 {
        return Some(moves[0]);
    }

    order_moves(state, &mut moves);
    let maximizing = state.current_player as i32;

    let mut best_move = Some(moves[0]);

    for depth in 1..=max_depth {
        let elapsed = js_sys::Date::now() - start;
        if elapsed > TIME_BUDGET_MS * 0.5 {
            break; // not enough time for another full iteration
        }

        let mut best_value = i32::MIN;
        let mut best_this_depth = moves[0];
        let mut timed_out = false;

        for mov in &moves {
            let mut next = *state;
            next.apply_move(mov);
            let value = minimax(&next, depth - 1, i32::MIN, i32::MAX, maximizing, start, &mut timed_out);
            if timed_out {
                break;
            }
            if value > best_value {
                best_value = value;
                best_this_depth = *mov;
            }
        }

        if !timed_out {
            best_move = Some(best_this_depth);
            // If we found a forced win, stop searching
            if best_value >= WIN_SCORE - 100 {
                break;
            }
            // Move the best move to front for next iteration (PV move ordering)
            if let Some(idx) = moves.iter().position(|m| m == &best_this_depth) {
                moves.swap(0, idx);
            }
        }
    }

    best_move
}

fn minimax(
    state: &MetaGameState,
    depth: usize,
    mut alpha: i32,
    mut beta: i32,
    maximizing_player: i32,
    start: f64,
    timed_out: &mut bool,
) -> i32 {
    if *timed_out {
        return 0;
    }

    if depth == 0 || state.is_game_over() {
        return state.evaluate_for(maximizing_player);
    }

    // Periodic time check (every entry to avoid per-node overhead being too frequent)
    let elapsed = js_sys::Date::now() - start;
    if elapsed > TIME_BUDGET_MS {
        *timed_out = true;
        return 0;
    }

    let mut moves = state.get_next_moves();
    order_moves(state, &mut moves);

    if state.current_player as i32 == maximizing_player {
        let mut max_eval = i32::MIN;
        for mov in &moves {
            let mut next = *state;
            next.apply_move(mov);
            let eval = minimax(&next, depth - 1, alpha, beta, maximizing_player, start, timed_out);
            if *timed_out { return 0; }
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
            let mut next = *state;
            next.apply_move(mov);
            let eval = minimax(&next, depth - 1, alpha, beta, maximizing_player, start, timed_out);
            if *timed_out { return 0; }
            min_eval = min_eval.min(eval);
            beta = beta.min(eval);
            if beta <= alpha {
                break;
            }
        }
        min_eval
    }
}

fn move_priority(state: &MetaGameState, mov: &MetaMove) -> i32 {
    let mut score: i32 = 0;

    // Prefer center cell, then corners
    score += CELL_WEIGHT[mov.row][mov.col];

    // Prefer strategically important boards
    score += BOARD_WEIGHT[mov.board_row][mov.board_col] * 2;

    // Strongly prefer moves that win a sub-board
    let mut test = state.boards[mov.board_row][mov.board_col];
    test.apply_move(mov.row, mov.col, state.current_player);
    if test.winner.is_some() {
        score += 1000;
    }

    // Penalize sending opponent to a board where they can win immediately
    let target = &state.boards[mov.row][mov.col];
    if target.winner.is_none() && !target.is_full() {
        let opponent = state.current_player.switch();
        for r in 0..3 {
            for c in 0..3 {
                if target.cells[r][c] == crate::types::Cell::Empty {
                    let mut t = *target;
                    t.apply_move(r, c, opponent);
                    if t.winner.is_some() {
                        score -= 200;
                        break;
                    }
                }
            }
        }
    }

    // Prefer sending opponent to won/full board (gives us free choice)
    if target.winner.is_some() || target.is_full() {
        score += 50;
    }

    score
}

fn order_moves(state: &MetaGameState, moves: &mut [MetaMove]) {
    moves.sort_unstable_by(|a, b| move_priority(state, b).cmp(&move_priority(state, a)));
}

const CELL_WEIGHT: [[i32; 3]; 3] = [
    [3, 1, 3],
    [1, 5, 1],
    [3, 1, 3],
];

const BOARD_WEIGHT: [[i32; 3]; 3] = [
    [4, 2, 4],
    [2, 8, 2],
    [4, 2, 4],
];
