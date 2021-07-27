use std::{collections::HashMap, f32::consts::TAU};

use ggez::{ContextBuilder, event::{EventHandler, run}, graphics::*};
use ggez_egui::EguiBackend;

#[derive(Clone)]
struct Element {
	pos: [f32; 2],
	size: [f32; 2],
	angle: f32,
}

impl Default for Element {
	fn default() -> Self {
		Self {
			pos: [300.0, 500.0],
			size: [100.0, 50.0],
			angle: 0.0,
		}
	}
}

impl Drawable for Element {
	fn draw(&self, ctx: &mut ggez::Context, _param: DrawParam) -> ggez::GameResult {
		let mesh = Mesh::new_rectangle(
			ctx,
			DrawMode::fill(),
			Rect::new(-1.0, -1.0, 1.0, 1.0),
			Color::WHITE
		)?;

		draw(
			ctx, &mesh,
			DrawParam::default()
			.dest(self.pos)
			.offset([-0.5, -0.5])
			.scale(self.size)
			.rotation(self.angle)
		)
	}

	fn dimensions(&self, _ctx: &mut ggez::Context) -> Option<Rect> {
		None
	}

	fn blend_mode(&self) -> Option<BlendMode> {
		None
	}

	fn set_blend_mode(&mut self, _mode: Option<BlendMode>) {}
}

#[derive(Default)]
struct State {
	egui_backend: EguiBackend,
	elements: HashMap<String, Element>,
	curret_element: String,
}

impl EventHandler<ggez::GameError> for State {
	fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
		let egui_ctx = self.egui_backend.get_context();
		egui::Window::new("Editor").show(&egui_ctx, |ui| {
			ui.add(
				egui::TextEdit::singleline(&mut self.curret_element).hint_text("put an id here")
			);
			let selected = self.elements.get(&self.curret_element).is_some() && !self.curret_element.is_empty();
			if ui.add(egui::Button::new("add element").enabled(!selected)).clicked() {
				self.elements.insert(self.curret_element.clone(), Element::default());
				self.curret_element = String::new();
			}

			if self.elements.len() > 0 {
				let elements = self.elements.clone();
				for (id, _element) in &elements.clone() {
					ui.horizontal(|ui| {
						let selected = self.curret_element == *id;
						if ui.add(egui::Button::new("select").enabled(!selected)).clicked() {
							self.curret_element = (*id).clone();
						}
						if ui.button("remove").clicked() {
							self.elements.remove(id);
						}
						ui.label(id.to_string());
					});
				}
			}

			if selected {
				ui.group(|ui| {
					if let Some(element) = self.elements.get_mut(&self.curret_element) {
						ui.label("config");
						ui.group(|ui| {
							ui.label("position");
							ui.horizontal(|ui| {
								ui.add(egui::DragValue::new(&mut element.pos[0]).prefix("x: "));
								ui.add(egui::DragValue::new(&mut element.pos[1]).prefix("y: "));
							});
						});
						ui.group(|ui| {
							ui.label("size");
							ui.horizontal(|ui| {
								ui.add(egui::DragValue::new(&mut element.size[0]).prefix("w: "));
								ui.add(egui::DragValue::new(&mut element.size[1]).prefix("h: "));
							});
						});
						ui.group(|ui| {
							ui.label("angle");
							ui.add(egui::Slider::new(&mut element.angle, 0.0..=TAU).prefix("angle: "));
						});
					}
				});
			}
		});
		Ok(())
	}

	fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
		clear(ctx, Color::BLACK);
		for (_, element) in &self.elements {
			draw(ctx, element, DrawParam::default())?;
		}
		draw(ctx, &self.egui_backend, DrawParam::default())?;
		present(ctx)
	}

	fn mouse_button_down_event(&mut self, _ctx: &mut ggez::Context, button: ggez::event::MouseButton, _x: f32, _y: f32) {
		self.egui_backend.input.mouse_button_down_event(button);
	}

	fn mouse_button_up_event(&mut self, _ctx: &mut ggez::Context, button: ggez::event::MouseButton, _x: f32, _y: f32) {
		self.egui_backend.input.mouse_button_up_event(button);
	}

	fn mouse_wheel_event(&mut self, _ctx: &mut ggez::Context, x: f32, y: f32) {
		self.egui_backend.input.mouse_wheel_event(x, y)
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
	let cb = ContextBuilder::new("game_id", "NemuiSen");
	let (ctx, events_loop) = cb.build()?;
	let state = State::default();
	run(ctx, events_loop, state);
}