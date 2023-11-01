use std::ops::Deref;
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

pub struct Game<'a> {
    game_object_arena: Arena<Box<dyn GameObject>>,
    drawable_arena: Arena<Box<dyn Drawable>>,
    ctx: &'a Context,
}

impl Game<'_> {
    pub fn new(ctx: &mut Context) -> Self {
        Game {
            game_object_arena: Arena::new(),
            drawable_arena: Arena::new(),
            ctx,
        }
    }

    pub fn start(&mut self) {
        self.spawn(Box::new(Player::new(&mut self.ctx)));
    }

    pub fn spawn(&mut self, game_object: Box<dyn GameObject>) {
        self.game_object_arena.insert(game_object);
    }

    fn process_input(&mut self, ctx: &mut Context) {
        for (idx, game_object) in &self.game_object_arena {
            game_object.deref()
                       .process_input(&InputContext::new((&ctx.keyboard, &ctx.mouse, &ctx.time)));
        }
    }
}

impl EventHandler for Game<'_> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.process_input(_ctx);
        self.player.update();
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(_ctx, Color::CYAN);

        self.player.draw(&mut canvas, DrawParam::default());

        for (idx, game_object) in &self.game_object_arena {
            game_object.deref().draw
        }

        canvas.finish(_ctx)
    }
}
