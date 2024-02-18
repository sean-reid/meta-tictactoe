/* tslint:disable */
/* eslint-disable */
/**
*/
export enum Cell {
  Empty = 0,
  X = 1,
  O = 2,
}
/**
*/
export enum GamePlayer {
  X = 1,
  O = 2,
}
/**
*/
export class GameStateWrapper {
  free(): void;
/**
* @returns {GameStateWrapper}
*/
  static new(): GameStateWrapper;
/**
* @returns {any}
*/
  get_possible_moves(): any;
/**
* @param {MetaMove} mov
* @returns {boolean}
*/
  apply_move(mov: MetaMove): boolean;
/**
* @param {MetaMove} mov
*/
  undo_move(mov: MetaMove): void;
/**
* @returns {number}
*/
  evaluate(): number;
/**
* @returns {boolean}
*/
  is_game_over(): boolean;
/**
* @returns {number}
*/
  current_player(): number;
/**
* @param {any} move_js
* @returns {boolean}
*/
  apply_json_move(move_js: any): boolean;
/**
* @returns {any}
*/
  get_board_state(): any;
}
/**
*/
export class MetaGameState {
  free(): void;
}
/**
*/
export class MetaMove {
  free(): void;
}
/**
*/
export class MiniBoard {
  free(): void;
}
/**
*/
export class Minimax {
  free(): void;
/**
* @param {GameStateWrapper} game_state
* @param {number} depth
* @returns {any}
*/
  static find_best_move(game_state: GameStateWrapper, depth: number): any;
/**
* @param {GameStateWrapper} game_state
* @param {number} depth
* @param {number} alpha
* @param {number} beta
* @param {number} maximizing_player
* @returns {number}
*/
  static minimax(game_state: GameStateWrapper, depth: number, alpha: number, beta: number, maximizing_player: number): number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_metamove_free: (a: number) => void;
  readonly __wbg_miniboard_free: (a: number) => void;
  readonly __wbg_gamestatewrapper_free: (a: number) => void;
  readonly gamestatewrapper_new: () => number;
  readonly gamestatewrapper_get_possible_moves: (a: number) => number;
  readonly gamestatewrapper_apply_move: (a: number, b: number) => number;
  readonly gamestatewrapper_undo_move: (a: number, b: number) => void;
  readonly gamestatewrapper_evaluate: (a: number) => number;
  readonly gamestatewrapper_is_game_over: (a: number) => number;
  readonly gamestatewrapper_current_player: (a: number) => number;
  readonly gamestatewrapper_apply_json_move: (a: number, b: number) => number;
  readonly gamestatewrapper_get_board_state: (a: number) => number;
  readonly __wbg_minimax_free: (a: number) => void;
  readonly minimax_find_best_move: (a: number, b: number) => number;
  readonly minimax_minimax: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly __wbg_metagamestate_free: (a: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
