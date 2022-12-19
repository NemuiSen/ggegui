use ggez::{Context, ContextBuilder, GameResult, glam};
use ggez::graphics::{self, Color, DrawParam};
use ggez::event::{self, EventHandler};
use ggez_egui::{EguiBackend, egui};

fn main() {
	let (mut ctx, event_loop) = ContextBuilder::new("game_id", "author")
		.build()
		.expect("FATAL - Failed to create the window.s");

	let my_game = MyGame::new(&mut ctx);

	event::run(ctx, event_loop, my_game);
}

struct MyGame {
	egui_backend: EguiBackend,
}

impl MyGame {
	pub fn new(_ctx: &mut Context) -> MyGame {
		MyGame {
			egui_backend: EguiBackend::new(_ctx),
		}
	}
}

impl EventHandler for MyGame {
	fn update(&mut self, ctx: &mut Context) -> GameResult {

		let egui_ctx = self.egui_backend.ctx();

		egui::Window::new("egui-window").show(&egui_ctx, |ui| {
			ui.label("a very nice gui :3");
			if ui.button("print \"hello world\"").clicked() {
				println!("hello world");
			}
			if ui.button("quit").clicked() {
				ctx.request_quit();
			}
		});

		self.egui_backend.update(ctx);
		
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
		canvas.draw(&self.egui_backend, DrawParam::default().dest(glam::vec2(0.0, 0.0)));
		canvas.finish(ctx)
	}
}