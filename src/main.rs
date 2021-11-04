#![allow(non_snake_case)]

use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("Pong_01", "Roombabatata")
        .build()
        .expect("aieee, could not create ggez context!");

    let state = MainState::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, state);
}

struct MainState {
    // Your state here...
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> MainState {
        // Load/create resources such as images here.
        MainState {
            // ...
        }
    }
}
impl EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);
        // Draw code here...
        graphics::present(ctx)
    }
}