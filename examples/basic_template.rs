use ggegui::{egui, Gui};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam};
use ggez::{glam, Context, ContextBuilder, GameResult};

fn main() {
	let (mut ctx, event_loop) = ContextBuilder::new("game_id", "author").build().unwrap();
	let state = State::new(&mut ctx);
	event::run(ctx, event_loop, state);
}

struct State {
	gui: Gui,
}

impl State {
	pub fn new(ctx: &mut Context) -> Self {
		Self { gui: Gui::new(ctx) }
	}
}

impl EventHandler for State {
	fn update(&mut self, ctx: &mut Context) -> GameResult {
		let gui_ctx = self.gui.ctx();

		egui::Window::new("UI").show(&gui_ctx, |ui| {
			ui.label("a very nice gui :3");
			if ui.button("print \"hello world\"").clicked() {
				println!("hello world");
			}
			if ui.button("quit").clicked() {
				ctx.request_quit();
			}
		});
		self.gui.update(ctx);
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
		canvas.draw(&self.gui, DrawParam::default().dest(glam::Vec2::ZERO));
		canvas.finish(ctx)
	}
}
