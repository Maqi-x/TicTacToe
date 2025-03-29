use crate::game::*;

pub type Controller = fn(&mut Board) -> Point;

pub struct Bot {
    pub handler: Controller,
    pub name: String,
    pub level: i32,

    pub description: String,
}

impl Bot {
    pub fn new(handler: Controller, name: String, description: String, level: i32) -> Bot {
        return Bot {
            handler,
            name,
            level,
            description,
        };
    }

    pub fn play(&mut self, board: &mut Board) -> Point {
        return (self.handler)(board);
    }
}
