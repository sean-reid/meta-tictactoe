# Meta Tic-Tac-Toe

A browser-based Meta (Ultimate) Tic-Tac-Toe game with an AI opponent powered by Rust/WebAssembly.

The AI uses minimax search with alpha-beta pruning at depth 3.

## How to Play

- Win three sub-boards in a row to win the game.
- Your move determines which sub-board your opponent must play in next.
- If sent to a won or full board, you may choose any board.

## Building

Requires [Rust](https://rustup.rs/) and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/).

```sh
cd wasm
wasm-pack build --target web --out-dir pkg
```

Then serve the project root with any static file server:

```sh
python3 -m http.server
```

## Project Structure

```
index.html          # Page shell
style.css           # Styles
game.js             # UI logic and rendering
wasm/
  src/
    lib.rs           # WASM bindings
    types.rs         # Cell, GamePlayer, MetaMove, shared constants
    board.rs         # Sub-board (MiniBoard) logic
    game.rs          # Meta game state and move validation
    minimax.rs       # AI search (minimax + alpha-beta pruning)
```
