#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]
#![feature(iter_from_generator)]
#![feature(generators)]

use egui::*;
use ggez::conf::WindowMode;
use ggez::event::{self, quit, EventHandler, MouseButton};
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::{self, draw, Canvas, Color, DrawParam, Drawable, Rect};
use ggez::input::keyboard::KeyCode;
use ggez::input::mouse::position;
use ggez::{timer, Context, ContextBuilder, GameError, GameResult};
use ggez_egui::EguiBackend;
use serde::{Deserialize, Serialize};
mod world;


fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_mode(WindowMode::default().resizable(true))
        .build()
        .expect("aieee, could not create ggez context!");
    let my_game = MyGame::new(&mut ctx);
    event::run(ctx, event_loop, my_game);
}
struct MyGame {
    egui_backend: EguiBackend,
    world: world::World,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        let sprite_batch = SpriteBatch::new(graphics::Image::solid(ctx, 1, Color::WHITE).unwrap());
        let mut me = MyGame {
            egui_backend: EguiBackend::new(ctx),
            world: world::World::new(),
        };
        me
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let delta_seconds = timer::delta(ctx).as_secs_f32();
        let egui_ctx = self.egui_backend.ctx();
        Window::new("UI").show(&egui_ctx, |ui| {
            ui.spacing();
            ui.label("All Maps:");
        });
        self.world.update(delta_seconds);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_screen_coordinates(
            ctx,
            Rect::new(
                0.0,
                0.0,
                graphics::drawable_size(ctx).0,
                graphics::drawable_size(ctx).1,
            ),
        )
            .unwrap();
        graphics::clear(ctx, Color::new(0.2, 0.2, 0.1, 1.0));

        draw(ctx, &self.egui_backend, ([0.0, 0.0],))?;

        self.world.draw(ctx);
        graphics::present(ctx)
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        self.egui_backend.input.mouse_button_down_event(button);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        self.egui_backend.input.mouse_button_up_event(button);
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.egui_backend.input.mouse_motion_event(x, y);
    }
}


