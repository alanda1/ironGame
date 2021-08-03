use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Color};
use ggez::input::keyboard;
use ggez::{Context, GameResult};
use glam::*;

struct MainState {
    pos_x: f32,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState { pos_x: 0.0 };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if keyboard::is_key_pressed(ctx, KeyCode::Right) {
            self.pos_x += 0.5;
        } else if keyboard::is_key_pressed(ctx, KeyCode::Left) {
            self.pos_x -= 0.5;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            glam::Vec2::new(0.0, 0.0),
            100.0,
            2.0,
            Color::WHITE,
        )?;
        graphics::draw(ctx, &circle, (Vec2::new(self.pos_x, 380.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
