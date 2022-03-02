use ggez::{event::{EventHandler, quit, run}, graphics::*};
use ggez_egui::{egui, EguiBackend};

struct State {
	egui_backend: EguiBackend,
	scale_factor: f32,
	text: String,
}

impl State {
	fn new(ctx: &ggez::Context) -> Self {
		Self {
			egui_backend: EguiBackend::new(ctx),
			scale_factor: 1.0,
			text: String::new(),
		}
	}
}

impl EventHandler<ggez::GameError> for State {
	fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
		let egui_ctx = self.egui_backend.ctx();
		egui::Window::new("egui-window").show(&egui_ctx, |ui| {
			ui.group(|ui| {
				ui.label("scale_factor");
				ui.horizontal(|ui| {
					ui.add(egui::Slider::new(&mut self.scale_factor, 0.5..=1.5));
					if ui.button("update scale_factor").clicked() {
						let (w, h) = size(ctx);
						self.egui_backend.input.set_scale_factor(self.scale_factor, (w, h));
					}
				});
			});
			ui.add(egui::TextEdit::multiline(&mut self.text).hint_text("text test:"));
			if ui.button("print text test").clicked() {
				println!("{}", self.text);
			}
			if ui.button("close button").clicked() {
				quit(ctx);
			}
		});
		Ok(())
	}

	fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
		clear(ctx, Color::BLACK);
		let mesh = MeshBuilder::new().rectangle(
			DrawMode::fill(),
			Rect::new(300.0, 300.0, 100.0, 100.0),
			Color::WHITE
		)?.build(ctx)?;
		draw(ctx, &mesh, DrawParam::default())?;
		draw(ctx, &self.egui_backend, ([0.0, 0.0],))?;
		present(ctx)
	}

	fn resize_event(&mut self, ctx: &mut ggez::Context, width: f32, height: f32) {	
		self.egui_backend.input.resize_event(width, height);
		let rect = ggez::graphics::Rect::new(0.0, 0.0, width, height);
		ggez::graphics::set_screen_coordinates(ctx, rect).unwrap();
	}

	fn mouse_button_up_event(&mut self, _ctx: &mut ggez::Context, button: ggez::event::MouseButton, _x: f32, _y: f32) {
		self.egui_backend.input.mouse_button_up_event(button);
	}

	fn mouse_button_down_event(&mut self, _ctx: &mut ggez::Context, button: ggez::event::MouseButton, _x: f32, _y: f32) {
		self.egui_backend.input.mouse_button_down_event(button);
	}

	fn mouse_wheel_event(&mut self, _ctx: &mut ggez::Context, x: f32, y: f32) {
		self.egui_backend.input.mouse_wheel_event(x, y);
	}

	fn mouse_motion_event(&mut self, _ctx: &mut ggez::Context, x: f32, y: f32, _dx: f32, _dy: f32) {
		self.egui_backend.input.mouse_motion_event(x, y);
	}

	fn key_down_event(&mut self, _ctx: &mut ggez::Context, keycode: ggez::event::KeyCode, keymods: ggez::event::KeyMods, _repeat: bool) {
		self.egui_backend.input.key_down_event(keycode, keymods);
	}

	fn text_input_event(&mut self, _ctx: &mut ggez::Context, character: char) {
		self.egui_backend.input.text_input_event(character);
	}
}

fn main() -> ggez::GameResult {
	let cb = ggez::ContextBuilder::new("game_id", "author");
	let (mut ctx, event_loop) = cb.build()?;
	ggez::graphics::set_resizable(&mut ctx, true)?;
	let state = State::new(&ctx);
	run(ctx, event_loop, state);
}
