use ggez::{
	*,
	event::*,
	graphics::*
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
				quit(ctx);
			}
		});
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		clear(ctx, Color::BLACK);
		draw(ctx, &self.egui_backend, ([0.0, 0.0],))?;
		present(ctx)
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
