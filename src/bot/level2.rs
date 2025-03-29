use std::thread::sleep;
use std::time::Duration;
use crate::game::*;
use crate::game::board::*;

// level 2 bot
pub fn gitBot(board: &mut Board) -> Point {
    // if it can win, do it
    for row in (0..3) {
        for col in (0..3) {
            if board.get((row, col)) == ' ' {
                board.set((row, col), 'O');
                if board.checkWinner() == 'O' {
                    board.set((row, col), ' ');
                    return (row, col);
                }
                board.set((row, col), ' '); // undo
            }
        }
    }

    // 2. If the opponent can win - block
    for row in (0..3) {
        for col in (0..3) {
            if board.get((row, col)) == ' ' {
                board.set((row, col), 'X');
                if board.checkWinner() == 'X' {
                    board.set((row, col), ' ');
                    return (row, col);
                }
                board.set((row, col), ' ');
            }
        }
    }

    // 3. if the center is free, take it
    if board.get((1,1)) == ' ' { return (1, 1) }

    // 4. If a corner is free, take it
    let corners = [(0,0), (0,2), (2,0), (2,2)];
    for &(row, col) in &corners {
        if board.get((row, col)) == ' ' {
            return (row, col);
        }
    }

    // 5. random free move
    for row in (0..3) {
        for col in (0..3) {
            if board.get((row, col)) == ' ' {
                return (row, col);
            }
        }
    }

    return (0, 0);
}