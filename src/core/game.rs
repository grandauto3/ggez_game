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
    player: Player,
    arena: Arena<Box<dyn GameObject>>,
}

impl Game {
    pub fn new(ctx: &mut Context) -> Self {
        Game {
            player: Player::new(ctx),
            arena: Arena::new(),
        }
    }

    pub fn start(&self) {}

    pub fn spawn(&self, game_object: Box<dyn GameObject>) {
        // self.arena.
    }

    fn process_input(&mut self, ctx: &mut Context) {
        self.player
            .process_input(&InputContext::new((&ctx.keyboard, &ctx.mouse, &ctx.time)));
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.process_input(_ctx);
        self.player.update();
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(_ctx, Color::CYAN);

        self.player.draw(&mut canvas, DrawParam::default());

        canvas.finish(_ctx)
    }
}
