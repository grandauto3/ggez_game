use ggez::{ContextBuilder, event};
use crate::core::game::Game;

pub struct App;

impl App {
    pub fn new() -> App {
        App
    }

    pub fn run(&self) {
        let (mut ctx, event_loop) = ContextBuilder::new("new_game", "Game Author")
            .build()
            .expect("Could not create context");

        let game = Game::new();
        game.start();

        event::run(ctx, event_loop, game);
    }
}