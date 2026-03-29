import init, { GameStateWrapper, Minimax } from './wasm/pkg/meta_tictactoe_wasm.js';

let game;

function indexToRowCol(index) {
    return { row: Math.floor(index / 3), col: index % 3 };
}

function renderBoard() {
    const boardContainer = document.getElementById('board');
    boardContainer.innerHTML = '';

    const boardState = game.get_board_state();
    const nextBoard = game.get_next_board();
    const globalWinners = game.get_board_winners();
    const gameOver = game.is_game_over();

    // Dim non-target boards when a specific board is required
    boardContainer.classList.toggle('has-target', nextBoard != null && !gameOver);

    boardState.forEach((subBoardsRow, subBoardRowIndex) => {
        subBoardsRow.forEach((subBoard, subBoardColIndex) => {
            const subBoardElement = document.createElement('div');
            subBoardElement.className = 'sub-board';

            const winner = globalWinners[subBoardRowIndex][subBoardColIndex];
            if (winner === 'x') subBoardElement.classList.add('won-x');
            if (winner === 'o') subBoardElement.classList.add('won-o');

            // Highlight active sub-board
            if (!gameOver && !winner) {
                if (nextBoard == null || (nextBoard.row === subBoardRowIndex && nextBoard.col === subBoardColIndex)) {
                    subBoardElement.classList.add('active');
                }
            }

            const subBoardIndex = subBoardRowIndex * 3 + subBoardColIndex;

            // If sub-board is won, show large winner label over cells
            if (winner) {
                const overlay = document.createElement('div');
                overlay.className = `sub-board-winner ${winner}`;
                overlay.textContent = winner.toUpperCase();
                subBoardElement.appendChild(overlay);
            }

            subBoard.forEach((row, rowIndex) => {
                row.forEach((cell, colIndex) => {
                    const cellElement = document.createElement('div');
                    cellElement.className = 'cell';
                    if (cell === 'x') {
                        cellElement.classList.add('x');
                        cellElement.textContent = 'X';
                    } else if (cell === 'o') {
                        cellElement.classList.add('o');
                        cellElement.textContent = 'O';
                    }
                    const cellIndex = rowIndex * 3 + colIndex;
                    cellElement.addEventListener('click', () => handleCellClick(subBoardIndex, cellIndex));
                    subBoardElement.appendChild(cellElement);
                });
            });

            boardContainer.appendChild(subBoardElement);
        });
    });

    updateStatus(gameOver);
}

function updateStatus(gameOver) {
    const turnIndicator = document.getElementById('turn-indicator');
    const bannerContainer = document.getElementById('game-over-banner');
    bannerContainer.innerHTML = '';

    if (gameOver) {
        const winner = game.get_global_winner();
        turnIndicator.textContent = 'Game over';

        const banner = document.createElement('div');
        banner.className = 'game-over-banner';

        if (winner === 'x') {
            banner.classList.add('win');
            banner.innerHTML = 'You win!';
        } else if (winner === 'o') {
            banner.classList.add('lose');
            banner.innerHTML = 'You lose!';
        } else {
            banner.classList.add('draw');
            banner.innerHTML = "It's a draw!";
        }

        const btn = document.createElement('button');
        btn.textContent = 'Play Again';
        btn.addEventListener('click', resetGame);
        banner.appendChild(document.createElement('br'));
        banner.appendChild(btn);
        bannerContainer.appendChild(banner);
    } else {
        const player = game.current_player() === 1 ? 'X' : 'O';
        const cls = player === 'X' ? 'player-x' : 'player-o';
        if (player === 'X') {
            turnIndicator.innerHTML = `Your turn (<span class="${cls}">${player}</span>)`;
        } else {
            turnIndicator.innerHTML = `AI thinking... (<span class="${cls}">${player}</span>)`;
        }
    }
}

function handleCellClick(subBoardIndex, cellIndex) {
    if (game.is_game_over()) return;

    const { row: subBoardRow, col: subBoardCol } = indexToRowCol(subBoardIndex);
    const { row: cellRow, col: cellCol } = indexToRowCol(cellIndex);

    const valid = game.apply_json_move({
        board_row: subBoardRow,
        board_col: subBoardCol,
        row: cellRow,
        col: cellCol,
    });

    if (valid && !game.is_game_over()) {
        renderBoard(); // Show player's move first
        // Small delay so the player sees their move before AI responds
        setTimeout(() => {
            const bestMove = Minimax.find_best_move(game, 3);
            game.apply_json_move(bestMove);
            renderBoard();
        }, 50);
    } else {
        renderBoard();
    }
}

function resetGame() {
    game = GameStateWrapper.new();
    renderBoard();
}

// Modal logic
function setupModal() {
    const overlay = document.getElementById('rules-overlay');
    const openBtn = document.getElementById('show-rules');
    const closeBtn = document.getElementById('close-rules');

    openBtn.addEventListener('click', (e) => {
        e.preventDefault();
        overlay.classList.add('visible');
    });

    closeBtn.addEventListener('click', () => {
        overlay.classList.remove('visible');
    });

    overlay.addEventListener('click', (e) => {
        if (e.target === overlay) overlay.classList.remove('visible');
    });
}

async function main() {
    await init();
    game = GameStateWrapper.new();
    setupModal();
    renderBoard();
}

main().catch(console.error);
