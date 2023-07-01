use egui_demo_lib::DemoWindows;
use ggegui::Gui;
use ggez::conf::WindowMode;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam};
use ggez::{glam, Context, ContextBuilder, GameResult};

fn main() {
	let (mut ctx, event_loop) = ContextBuilder::new("game_id", "author")
		.window_mode(WindowMode {
			maximized: true,
			resizable: true,
			..Default::default()
		})
		.build()
		.unwrap();
	ctx.gfx.window().set_resizable(true);
	let state = State::new(&mut ctx);
	event::run(ctx, event_loop, state);
}

struct State {
	gui: Gui,
	demo: DemoWindows,
}

impl State {
	pub fn new(ctx: &mut Context) -> Self {
		Self {
			gui: Gui::new(ctx),
			demo: DemoWindows::default(),
		}
	}
}

impl EventHandler for State {
	fn update(&mut self, ctx: &mut Context) -> GameResult {
		self.gui.update(ctx);
		let gui_ctx = self.gui.ctx();
		self.demo.ui(&gui_ctx);
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
		canvas.draw(&self.gui, DrawParam::default().dest(glam::Vec2::ZERO));
		canvas.finish(ctx)
	}

	fn text_input_event(&mut self, _ctx: &mut Context, character: char) -> GameResult {
		self.gui.input.text_input_event(character);
		Ok(())
	}
}
