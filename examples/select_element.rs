use ggegui::{egui, Gui};
use ggez::{
	context::Has,
	event::{self, EventHandler},
	graphics::{self, *},
	Context, ContextBuilder, GameError,
};
use std::{collections::HashMap, f32::consts::TAU};

#[derive(Clone)]
struct Element {
	pos: [f32; 2],
	size: [f32; 2],
	angle: f32,
	mesh: Mesh,
}

impl Element {
	fn new(ctx: &mut Context) -> Element {
		Element {
			pos: [300.0, 500.0],
			size: [100.0, 50.0],
			angle: 0.0,
			mesh: Mesh::new_rectangle(
				ctx,
				DrawMode::fill(),
				Rect::new(-1.0, -1.0, 1.0, 1.0),
				Color::WHITE,
			)
			.unwrap(),
		}
	}
}

impl Drawable for Element {
	fn draw(&self, canvas: &mut Canvas, _param: impl Into<DrawParam>) {
		canvas.draw(
			&self.mesh,
			DrawParam::default()
				.dest(self.pos)
				.offset([-0.5, -0.5])
				.scale(self.size)
				.rotation(self.angle),
		)
	}

	fn dimensions(&self, _gfx: &impl Has<GraphicsContext>) -> Option<graphics::Rect> {
		None
	}
}

fn main() {
	let (ctx, event_loop) = ContextBuilder::new("game_id", "author")
		.build()
		.expect("FATAL - Failed to create the window.s");

	let my_game = MyGame::default();
	event::run(ctx, event_loop, my_game);
}

#[derive(Default)]
struct MyGame {
	gui: Gui,
	elements: HashMap<String, Element>,
	curret_element: String,
}

impl EventHandler<ggez::GameError> for MyGame {
	fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
		let gui_ctx = self.gui.ctx();
		egui::Window::new("Editor").show(&gui_ctx, |ui| {
			ui.add(
				egui::TextEdit::singleline(&mut self.curret_element).hint_text("put an id here"),
			);
			let selected = self.elements.get(&self.curret_element).is_some()
				&& !self.curret_element.is_empty();
			if ui
				.add_enabled(!selected, egui::Button::new("add element"))
				.clicked()
			{
				self.elements
					.insert(self.curret_element.clone(), Element::new(ctx));
				self.curret_element = String::new();
			}

			if !self.elements.is_empty() {
				let elements = self.elements.clone();
				for id in elements.keys() {
					ui.horizontal(|ui| {
						let selected = self.curret_element == *id;
						if ui
							.add_enabled(!selected, egui::Button::new("select"))
							.clicked()
						{
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
							ui.add(
								egui::Slider::new(&mut element.angle, 0.0..=TAU).prefix("angle: "),
							);
						});
					}
				});
			}
		});
		self.gui.update(ctx);
		Ok(())
	}

	fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
		let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
		for element in self.elements.values() {
			canvas.draw(element, DrawParam::default());
		}
		canvas.draw(&self.gui, DrawParam::default());
		canvas.finish(ctx)
	}

	fn text_input_event(
		&mut self,
		_ctx: &mut ggez::Context,
		character: char,
	) -> Result<(), GameError> {
		self.gui.input.text_input_event(character);
		Ok(())
	}
}
