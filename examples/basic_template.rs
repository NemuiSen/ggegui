use ggez::{
	*,
	event::*,
};
use ggez_egui::{egui, EguiBackend};

fn main() -> GameResult {
	let (ctx, event_loop) = ContextBuilder::new("game_id", "author")
	.build()?;

	let my_game = MyGame {
		egui_backend: EguiBackend::default(),
	};

	event::run(ctx, event_loop, my_game)
}

struct MyGame {
	egui_backend: EguiBackend
}

impl EventHandler<GameError> for MyGame {
	fn update(&mut self, ctx: &mut Context) -> GameResult {
		let egui_ctx = self.egui_backend.ctx();
		egui::Window::new("egui-window").show(&egui_ctx, |ui| {
			ui.label("a very nice gui :3");
			if ui.button("print \"hello world\"").clicked() {
				println!("hello world");
			}
			if ui.button("quit").clicked() {
				request_quit(ctx);
			}
		});
		self.egui_backend.update(ctx);
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		let mut canvas = graphics::Canvas::from_frame(
			ctx,
			graphics::CanvasLoadOp::Clear([0.1, 0.1, 0.1, 1.0].into())
		);
		canvas.draw(&self.egui_backend, graphics::DrawParam::default());
		canvas.finish(ctx)
	}
}

