#![allow(non_snake_case)]

use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use glam;
use ggez::event::{self, EventHandler};

const RACKET_HEIGHT: f32 = 100.0;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT / 2.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH / 2.0;



fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("Pong_01", "Roombabatata")
        .build()
        .expect("aieee, could not create ggez context!");

    let state = MainState::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, state);
}

struct MainState {
    player_1_pos: glam::Vec2,    
    player_2_pos: glam::Vec2,    
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> MainState {
        let (screen_widht, screen_height) = graphics::drawable_size(_ctx);
        let screen_height_half = screen_height / 2.0;
        MainState {
            player_1_pos : glam::Vec2::new(RACKET_WIDTH_HALF, screen_height_half),
            // player_1_pos : glam::Vec2::new(20.0, 20.0),
            player_2_pos : glam::Vec2::new(screen_widht - RACKET_WIDTH_HALF, screen_height_half),
        }
    }
}
impl EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);
        
        let racket_rect = graphics::Rect::new(-RACKET_WIDTH_HALF, -RACKET_HEIGHT_HALF, RACKET_WIDTH, RACKET_HEIGHT);
        let racket_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), racket_rect, Color::WHITE)?;

        graphics::draw(
            ctx,
            &racket_mesh,
            graphics::DrawParam::new()
                .dest(self.player_1_pos),
        )?;
        

        graphics::present(ctx)
    }
}