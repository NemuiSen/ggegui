[![latest version](https://img.shields.io/crates/v/ggez-egui)](https://crates.io/crates/egui-ggez)
# ggez_egui
An [egui](https://github.com/emilk/egui/) implementation for the [ggez](https://ggez.rs/) game framework

### basic project template
```rust
use ggez::{ContextBuilder, GameError, GameResult, event, graphics::{self, Color}};
use ggez_egui::EguiBackend;

fn main() -> GameResult {
	let (ctx, event_loop) = ContextBuilder::new("game_id", "author")
	.build()?;

	let my_game = MyGame::new();

	event::run(ctx, event_loop, my_game)
}

struct MyGame {
	egui_backend: EguiBackend
}

impl MyGame {
	fn new() -> Self {
		Self {
			egui_backend: EguiBackend::default()
		}
	}
}

impl event::EventHandler<GameError> for MyGame {
	fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
		let egui_ctx = self.egui_backend.get_context();
		egui::Window::new("egui-window").show(&egui_ctx, |ui| {
			ui.label("a very nice gui :3");
			if ui.button("quit").clicked() {
				event::quit(ctx);
			}
		});
		Ok(())
	}

	fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
		graphics::clear(ctx, Color::BLACK);
		graphics::draw(ctx, &self.egui_backend, ([0.0, 0.0],))?;
		graphics::present(ctx)
	}

	fn mouse_button_down_event(&mut self, _ctx: &mut ggez::Context, button: event::MouseButton, _x: f32, _y: f32) {
		self.egui_backend.input.mouse_button_down_event(button);
	}

	fn mouse_button_up_event(&mut self, _ctx: &mut ggez::Context, button: event::MouseButton, _x: f32, _y: f32) {
		self.egui_backend.input.mouse_button_up_event(button);
	}

	fn mouse_motion_event(&mut self, _ctx: &mut ggez::Context, x: f32, y: f32, _dx: f32, _dy: f32) {
		self.egui_backend.input.mouse_motion_event(x, y);
	}
}

```
there are a couple of examples to know how to use this implementation. 