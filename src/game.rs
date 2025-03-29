#![allow(non_snake_case)]
#![allow(dead_code)]

pub mod board;
pub mod playerController;
mod utils;

use std::io;

pub use crate::bot::bot::*;
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;
use utils::*;

use crate::randElm;
pub use board::*;
pub use playerController::*;

use crossterm::style::Attribute;
use crossterm::{
    queue,
    style::{Color, Print, ResetColor, SetAttribute, SetForegroundColor},
};
use inquire::Select;

pub type Point = (usize, usize);

pub enum Player {
    Player,
    Bot,
}

impl Player {
    pub fn next(&mut self) {
        *self = match self {
            Player::Player => Player::Bot,
            Player::Bot => Player::Player,
        };
    }
}

pub struct TicTacToe {
    pub board: Box<Board>,
    pub turn: Player,

    pub playerController: Controller,
    pub currentBot: usize,

    pub bots: Vec<Bot>,
}

impl TicTacToe {
    pub fn new(playerController: Controller) -> TicTacToe {
        return TicTacToe {
            board: Box::new(Board::new()),
            turn: Player::Player,

            playerController: playerController,
            currentBot: 1,
            bots: Vec::new(),
        };
    }

    pub fn registerBot(&mut self, bot: Bot) {
        self.bots.push(bot);
    }

    pub fn menu(&mut self) {
        let colors = [
            Color::Cyan,
            Color::Blue,
            Color::Green,
            Color::DarkGreen,
            Color::Red,
        ];
        for i in 0..self.bots.len() {
            queue!(
                io::stdout(),
                SetForegroundColor(colors[self.bots[i].level as usize]),
                SetAttribute(Attribute::Bold),
                Print(format!("{} ", self.bots[i].name)),
                SetAttribute(Attribute::Reset),
                SetForegroundColor(colors[self.bots[i].level as usize]),
                Print(format!(
                    "(level {}): {}",
                    self.bots[i].level, self.bots[i].description
                )),
                ResetColor,
            )
            .unwrap();
            println!();
        }
        let mut botsNames: Vec<String> = vec![];
        for i in 0..self.bots.len() {
            botsNames.push(self.bots[i].name.clone());
        }
        let ans = Select::new("choose a bot to play with:", botsNames.clone())
            .prompt()
            .expect("Error");
        for i in 0..botsNames.len() {
            if self.bots[i].name == ans {
                self.currentBot = i;
            }
        }
    }

    pub fn start(&mut self) {
        self.board.print();
        println!("Plaing with: {}", self.bots[self.currentBot].name);

        loop {
            let playerMove = (self.playerController)(&mut self.board);
            self.board.set(playerMove, 'X');

            println!();
            self.board.print();
            if self.checkWinner() {
                break;
            }

            let msgs = [
                "{bot} thinks over the move...",
                "{bot} is thinking about the next move...",
                "{bot} contemplates next move...",
            ];
            print!(
                "{}",
                randElm!(msgs).replace("{bot}", &*self.bots[self.currentBot].name.to_string())
            );
            flush();

            let botMove = (self.bots[self.currentBot]).play(&mut self.board);
            sleep(Duration::from_millis(rand(100, 1000)));

            println!(" {} x {}", botMove.0 + 1, botMove.1 + 1);
            self.board.set(botMove, 'O');
            self.board.print();
            if self.checkWinner() {
                break;
            }
        }
    }

    pub fn checkWinner(&mut self) -> bool {
        match self.board.checkWinner() {
            'X' => {
                queue!(
                    io::stdout(),
                    SetForegroundColor(Color::Green),
                    SetAttribute(Attribute::Bold),
                    Print("Congratulations! You have won!"),
                    ResetColor,
                )
                .unwrap();
                println!();
                return true;
            }
            'O' => {
                queue!(
                    io::stdout(),
                    SetForegroundColor(Color::Red),
                    SetAttribute(Attribute::Bold),
                    Print("You've lost"),
                    ResetColor,
                )
                .unwrap();
                println!();
                return true;
            }
            ' ' => {
                if self.board.isFull() {
                    queue!(
                        io::stdout(),
                        SetForegroundColor(Color::Blue),
                        SetAttribute(Attribute::Bold),
                        Print("Tie!"),
                        ResetColor,
                    )
                    .unwrap();
                    println!();
                    return true;
                }
                return false;
            }
            _ => {
                return false;
            }
        }
    }
}
