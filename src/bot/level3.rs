use crate::game::*;
use crate::game::board::*;

// level 3 bot
pub fn minimax(board: &mut Board) -> Point {
    pub fn score(board: &mut Board, isMaximizing: bool, depth: i32) -> i32 {
        let winner = board.checkWinner();
        if winner == 'O' { return 10 - depth }
        if winner == 'X' { return depth - 10 }
        if board.isFull() {
            return 0; // Draw
        }

        let mut bestScore = if isMaximizing { i32::MIN } else { i32::MAX };
        let marker: char;

        if isMaximizing { marker = 'O' }
        else { marker = 'X' }

        for row in (0..3) {
            for col in (0..3) {
                if board.get((row, col)) == ' ' {
                    board.set((row, col), marker);
                    let currentScore = score(board, !isMaximizing, depth + 1);
                    board.set((row, col), ' '); // Undo move

                    if isMaximizing {
                        bestScore = bestScore.max(currentScore);
                    } else {
                        bestScore = bestScore.min(currentScore);
                    }
                }
            }
        }

        return bestScore;
    }

    const BOT_MARKER: char = 'O';
    let mut bestScore = i32::MIN;
    let mut bestMove: Point = (0, 0);

    for row in (0..3) {
        for col in (0..3) {
            if board.get((row, col)) == ' ' {
                board.set((row, col), BOT_MARKER);
                let moveScore = score(board, false, 0);
                board.set((row, col), ' ');

                if moveScore > bestScore {
                    bestScore = moveScore;
                    bestMove = (row, col);
                }
            }
        }
    }

    return bestMove;
}
