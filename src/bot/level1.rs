use crate::game::*;
use crate::game::board::*;

// level 1 bot
pub fn idioticBot(board: &mut Board) -> Point {
    pub fn score(board: &mut Board, isMaximizing: bool, depth: i32, alpha: &mut i32, beta: &mut i32) -> i32 {
        let winner = board.checkWinner();
        if winner != ' ' {
           if winner == 'O' { return  10 - depth }
           else             { return -10 - depth };
        }

        if board.isFull() { return 0 }
        if depth > 9      { return 0 }

        let mut bestScore = if isMaximizing { i32::MIN } else { i32::MAX };
        let marker = if isMaximizing { 'O' } else { 'X' };

        for row in (0..3) {
            for col in (0..3) {
                if board.get((row, col)) == ' ' {
                    board.set((row, col), marker);
                    let score = score(board, !isMaximizing, depth + 1, alpha, beta);
                    board.set((row, col), ' ');

                    if isMaximizing {
                        bestScore = bestScore.max(score);
                        *alpha = (*alpha).max(bestScore);
                    } else {
                        bestScore = bestScore.min(score);
                        *beta = (*beta).min(bestScore);
                    }

                    // stop farther checking as it is pointless
                    if *beta <= *alpha { break }
                }
            }
        }

        return bestScore;
    }

    const BOT_MARKER: char = 'O';
    let mut bestScore = i32::MIN;
    let mut bestMove: Point = (0, 0);
    let mut alpha = i32::MIN;
    let mut beta = i32::MAX;

    for row in (0..3) {
        for col in (0..3) {
            if board.get((row, col)) == ' ' {
                board.set((row, col), BOT_MARKER);
                let score = score(board, false, 0, &mut alpha, &mut beta);
                board.set((row, col), ' ');

                if score > bestScore {
                    bestScore = score;
                    bestMove = (row, col);
                }
            }
        }
    }

    return bestMove;
}