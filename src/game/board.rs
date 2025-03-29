use crate::game::*;
use colored::*;

pub struct Board([[char; 3]; 3]);

impl Board {
    pub fn new() -> Board {
        return Board([
            [' ', ' ', ' '],
            [' ', ' ', ' '],
            [' ', ' ', ' '],
        ]);
    }

    pub fn isEmpty(&mut self) -> bool {
        return self.0 == Board::new().0;
    }

    pub fn set(&mut self, pos: Point, ch: char) {
        self.0[pos.0][pos.1] = ch;
    }

    pub fn get(&mut self, pos: Point) -> char {
        return self.0[pos.0][pos.1];
    }

    pub fn print(&mut self) {
        for row in (0..3) {
            for col in (0..3) {
                if self.0[row][col] == 'O' {
                    print!(" {} ", "O".red());
                } else if self.0[row][col] == 'X' {
                    print!(" {} ", "X".green());
                } else {
                    print!("   ");
                }
                if col < 2 {
                    print!("|");
                }
            }
            println!();
            if row < 2 {
                println!("---+---+---");
            }
        }
        println!()
    }

    pub fn isFull(&mut self) -> bool {
        for row in (0..3) {
            for col in (0..3) {
                if self.0[row][col] == ' ' {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn checkWinner(&self) -> char {
        // Check rows and columns
        for i in (0..3) {
            if self.0[i][0] != ' ' && self.0[i][0] == self.0[i][1] && self.0[i][1] == self.0[i][2] {
                return self.0[i][0];
            }
            if self.0[0][i] != ' ' && self.0[0][i] == self.0[1][i] && self.0[1][i] == self.0[2][i] {
                return self.0[0][i];
            }
        }

        // Check diagonals
        if self.0[1][1] != ' ' {
            if self.0[0][0] == self.0[1][1] && self.0[1][1] == self.0[2][2] {
                return self.0[1][1];
            }
            if self.0[0][2] == self.0[1][1] && self.0[1][1] == self.0[2][0] {
                return self.0[1][1];
            }
        }

        return ' ';
    }

    pub fn clone(&self) -> Board {
        let mut new = Board::new();
        for row in (0..3) {
            for col in (0..3) {
                new.set((row, col), self.0[row][col]);
            }
        }
        return new;
    }
}