use crate::game::*;
use crate::game::board::*;

pub type Controller = fn(&mut Board) -> Point;

pub struct Bot {
    pub handler: Controller,
    pub name:    String,
    pub level:   i32,

    pub description: String,
}

impl Bot {
    pub fn new(handler: Controller, name: String, description: String, level: i32) -> Bot {
        return Bot {
            handler: handler,
            name:    name,
            level:   level,

            description: description,
        };
    }

    pub fn play(&mut self, board: &mut Board) -> Point {
        return (self.handler)(board);
    }
}