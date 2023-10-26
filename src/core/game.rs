use ggez::{
    event::EventHandler,
    Context,
    GameResult,
    graphics::{
        self,
        Color,
        Drawable,
        DrawParam,
    },
};
use crate::{
    core::{
        input_context::InputContext,
        traits::auto_register::AutoRegister
    },
    game::player::Player
};
extern crate generational_arena;
use generational_arena::Arena;

pub struct Game {
    player: Player,
    arena: Arena<Box<dyn AutoRegister>>
}

impl Game {
    pub fn new(ctx: &mut Context) -> Self {
        Game {
            player: Player::new(ctx),
            arena: Arena::new()
        }
    }

    pub fn start(&self) {}

    fn process_input(&mut self, ctx: &mut Context) {
        self.player.process_input(&InputContext::new((
            &ctx.keyboard,
            &ctx.mouse,
            &ctx.time,
        )));
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