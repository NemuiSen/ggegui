[![crates.io](https://img.shields.io/crates/v/ggegui)](https://crates.io/crates/ggegui)
[![docs.rs](https://img.shields.io/docsrs/ggegui)](https://docs.rs/ggegui/)
# ggegui
An [egui](https://github.com/emilk/egui/) implementation for the [ggez](https://ggez.rs/) game framework

## Ultra minimal example
```rust
use ggegui::{egui, Gui};
use ggez::{
	ContextBuilder, Context, GameResult, glam,
	event::{ self, EventHandler}, 
	graphics::{ self, DrawParam, Color }
};

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
		Self { 
			gui: Gui::new(ctx),
		}
	} 
} 

impl EventHandler for State {
	fn update(&mut self, ctx: &mut Context) -> GameResult {
		let gui_ctx = self.gui.ctx();

		egui::Window::new("Title").show(&gui_ctx, |ui| {
			ui.label("label");
			if ui.button("button").clicked() {
				println!("button clicked");
			}
		});
		self.gui.update(ctx);
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
		canvas.draw(
			&self.gui, 
			DrawParam::default().dest(glam::Vec2::ZERO),
		);
		canvas.finish(ctx)
	}
}
```
> **__NOTE:__** due to how ggez is made currently there is no way to access the raw events emited by eventloop so instead to get the events using the functions and the context of EventHandler, as shown in the example of adove inside `EventHandler::update` is called `Gui::update` this function extract the information inside the context to send it to egui and draws everything. To make use of text input you must call `Input::text_input_event` inside of `EventHandler::text_input_event` like this `self.gui.input.text_input_event(character)`, this also applies to `Input::mouse_wheel_event` and `Input::resize_event`, you just call then inside of their respective functions exposed by `EventHandler`.

there are a few [examples](./examples/) to show how to use this implementation.
