use crate::game::MetaGameState;
use crate::types::MetaMove;

pub fn find_best_move(state: &MetaGameState, depth: usize) -> Option<MetaMove> {
    let moves = state.get_next_moves();
    let maximizing = state.current_player as i32;

    let mut best_move = None;
    let mut best_value = i32::MIN;

    for mov in &moves {
        let mut next = *state;
        next.apply_move(mov);
        let value = minimax(&next, depth - 1, i32::MIN, i32::MAX, maximizing);
        if value > best_value {
            best_value = value;
            best_move = Some(*mov);
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
) -> i32 {
    if depth == 0 || state.is_game_over() {
        return state.evaluate_global();
    }

    let moves = state.get_next_moves();

    if state.current_player as i32 == maximizing_player {
        let mut max_eval = i32::MIN;
        for mov in &moves {
            let mut next = *state;
            next.apply_move(mov);
            let eval = minimax(&next, depth - 1, alpha, beta, maximizing_player);
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
            let eval = minimax(&next, depth - 1, alpha, beta, maximizing_player);
            min_eval = min_eval.min(eval);
            beta = beta.min(eval);
            if beta <= alpha {
                break;
            }
        }
        min_eval
    }
}
