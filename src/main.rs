#![allow(non_snake_case)]

use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use glam;
use ggez::event::{self, EventHandler};
use ggez::input::keyboard::{self, KeyCode};

const RACKET_HEIGHT: f32 = 100.0;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT / 2.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH / 2.0;
const BALL_SIZE: f32 = 30.0;
const BALL_SIZE_HALF: f32 = 30.0 / 2.0;
const PLAYER_SPEED: f32 = 600.0;


fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("Pong_01", "Roombabatata")
        .build()
        .expect("aieee, could not create ggez context!");

    let state = MainState::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, state);
}




fn clamp(value: &mut f32, low: f32, height: f32) {
    if *value < low {
        *value = low;
    } else if *value > height {
        *value = height;
    }
}

struct MainState {
    player_1_pos: glam::Vec2,    
    player_2_pos: glam::Vec2,    
    ball_pos: glam::Vec2,
}
fn movement(ctx: &mut Context, delta: f32, object: &mut glam::Vec2, keycode: [KeyCode; 2]) {

    if keyboard::is_key_pressed(ctx, keycode[0]) {
        object.y += -PLAYER_SPEED * delta;
    } if keyboard::is_key_pressed(ctx, keycode[1]) {
        object.y += PLAYER_SPEED * delta;
    }

}

impl MainState {
    pub fn new(_ctx: &mut Context) -> MainState {
        let (screen_widht, screen_height) = graphics::drawable_size(_ctx);
        let (screen_widht_half, screen_height_half) = (screen_widht / 2.0, screen_height / 2.0);
        MainState {
            player_1_pos : glam::Vec2::new(RACKET_WIDTH_HALF, screen_height_half),
            player_2_pos : glam::Vec2::new(screen_widht - RACKET_WIDTH_HALF, screen_height_half),
            ball_pos : glam::Vec2::new(screen_widht_half, screen_height_half),
        }
    }
}
impl EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        
        let delta = ggez::timer::delta(_ctx).as_secs_f32();
        let screen_h = graphics::drawable_size(_ctx).1;


        movement(_ctx, delta, &mut self.player_1_pos, [KeyCode::W, KeyCode::S]);
        movement(_ctx, delta, &mut self.player_2_pos, [KeyCode::Up, KeyCode::Down]);

        clamp(&mut self.player_1_pos.y, RACKET_HEIGHT_HALF, screen_h-RACKET_HEIGHT_HALF);
        clamp(&mut self.player_2_pos.y, RACKET_HEIGHT_HALF, screen_h-RACKET_HEIGHT_HALF);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);
        
        let racket_rect = graphics::Rect::new(-RACKET_WIDTH_HALF, -RACKET_HEIGHT_HALF, RACKET_WIDTH, RACKET_HEIGHT);
        let racket_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), racket_rect, Color::WHITE)?;

        let ball_rect = graphics::Rect::new(-BALL_SIZE_HALF, -BALL_SIZE, BALL_SIZE, BALL_SIZE);
        let ball_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), ball_rect, Color::WHITE)?;

        // Player 1
        graphics::draw(ctx, &racket_mesh,
            graphics::DrawParam::new()
                .dest(self.player_1_pos),
        )?;
        
        // Player 2
        graphics::draw(ctx, &racket_mesh,
            graphics::DrawParam::new()
                .dest(self.player_2_pos),
        )?;
        
        // The Ball
        graphics::draw(ctx, &ball_mesh,
            graphics::DrawParam::new()
                .dest(self.ball_pos),
        )?;
        

        graphics::present(ctx)
    }
}