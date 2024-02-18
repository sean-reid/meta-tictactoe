/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export function __wbg_metamove_free(a: number): void;
export function __wbg_miniboard_free(a: number): void;
export function __wbg_gamestatewrapper_free(a: number): void;
export function gamestatewrapper_new(): number;
export function gamestatewrapper_get_possible_moves(a: number): number;
export function gamestatewrapper_apply_move(a: number, b: number): number;
export function gamestatewrapper_undo_move(a: number, b: number): void;
export function gamestatewrapper_evaluate(a: number): number;
export function gamestatewrapper_is_game_over(a: number): number;
export function gamestatewrapper_current_player(a: number): number;
export function gamestatewrapper_apply_json_move(a: number, b: number): number;
export function gamestatewrapper_get_board_state(a: number): number;
export function __wbg_minimax_free(a: number): void;
export function minimax_find_best_move(a: number, b: number): number;
export function minimax_minimax(a: number, b: number, c: number, d: number, e: number): number;
export function __wbg_metagamestate_free(a: number): void;
export function __wbindgen_malloc(a: number, b: number): number;
export function __wbindgen_realloc(a: number, b: number, c: number, d: number): number;
export function __wbindgen_exn_store(a: number): void;
