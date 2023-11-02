use std::ops::{Deref, DerefMut};
use crate::{
    core::{input_context::InputContext, traits::game_object::GameObject},
    game::player::Player,
};
use ggez::{
    event::EventHandler,
    graphics::{self, Color, DrawParam, Drawable},
    Context, GameResult,
};

extern crate generational_arena;

use generational_arena::Arena;

pub struct Game {
    game_object_registry: Arena<GameObject>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            game_object_registry: Arena::new()
        }
    }

    pub fn start(&self) {
        //self.spawn(Box::new(Player::new(&mut self.ctx)));
    }

    pub fn spawn(&mut self, game_object: GameObject) {
        self.game_object_registry.insert(game_object);
    }

    fn process_input(&mut self, ctx: &mut Context) {
        for (idx, mut game_object) in &mut self.game_object_registry {
            game_object.deref_mut()
                .process_input(&InputContext::new((&ctx.keyboard, &ctx.mouse, &ctx.time)));
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.process_input(_ctx);
        //self.player.update();
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(_ctx, Color::CYAN);

        //self.player.draw(&mut canvas, DrawParam::default());

        for (idx, game_object) in &self.game_object_registry {
            game_object.draw(&mut canvas, DrawParam::default());
        }

        canvas.finish(_ctx)
    }
}
