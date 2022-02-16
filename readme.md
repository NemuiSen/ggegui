[![latest version](https://img.shields.io/crates/v/ggez-egui)](https://crates.io/crates/ggez-egui)
# ggez_egui
An [egui](https://github.com/emilk/egui/) implementation for the [ggez](https://ggez.rs/) game framework

## First steps
To make use of this implementation you must first add the "EguiBackend" inside your game structure and initialize it. Ej:
```rust
struct MyGame {
	egui_backend: EguiBackend,
	...
}
...
let game = MyGame {
	egui_backend: EguiBackend::default(), //or EguiBackend::new(ctx)
	...
}
```

later, in the part of the "EventHandler", inside the "update" we will add the structure and logic of the ui, inside the "draw" we will send to draw the ui, and at the end we will detect the user input. Ej:
```rust
impl EventHandler<GameResult> for MyGame {
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

	// Input

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
```
In this case we only handle the mouse input so we only use those three functions (mouse_button_down_event, mouse_button_up_event, mouse_motion_event), it is not very necessary to explain them because their names are already very descriptive. If we need to manage other things in the window such as the keyboard, the size, or even the scale factor, there are also respective functions for that, check out the [Input Documentation](https://docs.rs/ggez-egui/latest/ggez_egui/struct.Input.html).

there are a few [examples](./examples/) to know how to use this implementation.
