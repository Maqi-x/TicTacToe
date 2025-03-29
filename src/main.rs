#![feature(unboxed_closures)]

mod game;
mod bot;
mod config;

use inquire::Select;
use game::*;

use std::io;

pub use crate::bot::{
    bot::*,
    level1::*,
    level2::*,
    level3::*,
};

use crossterm::{queue, execute, terminal as term, cursor};

fn main() {
    let mut menu: bool = false;
    loop {
        queue!(
            io::stdout(),
            term::Clear(term::ClearType::All),
            cursor::MoveTo(0, 0),
        );
        let mut game = TicTacToe::new(
            playerController,
        );

        game.registerBot(
            Bot::new(
                idioticBot,
                String::from("Idiotic Bot"),
                String::from("A simple enough premitigated bot - ideal for beginners"),
                1,
            ),
        );
        game.registerBot(
            Bot::new(
                gitBot,
                String::from("Git Bot"),
                String::from("Quite a smart bot, plays similar to the average person"),
                2,
            ),
        );

        game.registerBot(
            Bot::new(
                minimax,
                String::from("Maxi Bot"),
                String::from("An advanced bot running on the minimax algorithm. UNBEATABLE"),
                3,
            ),
        );

        if menu { game.menu(); }
        game.start();

        let inp = Select::new("What do you want to do now?", vec!["Play again", "Change difficulty level", "Quit"])
            .prompt()
            .expect("Error");
        match inp {
            "Play again" =>              { menu = false; }
            "Change difficulty level" => { menu = true;  }
            "Quit" =>                    { return        }
            _ => {}
        }
    }

}