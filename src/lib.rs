mod input;
mod painter;

pub use egui;
use ggez::{
	context::Has,
	graphics::{self, Canvas, DrawParam, Drawable, GraphicsContext},
};
pub use input::Input;
use painter::Painter;
use std::{
	ops::Deref,
	sync::{Arc, Mutex},
};

/// Contains the [`egui::Context`] and the [`Painter`].
///
/// When is droped automatically will call [`egui::Context::end_frame`] function and update the [`Painter`]
pub struct GuiContext {
	context: egui::Context,
	painter: Arc<Mutex<Painter>>,
}

impl Deref for GuiContext {
	type Target = egui::Context;
	fn deref(&self) -> &Self::Target {
		&self.context
	}
}

impl Drop for GuiContext {
	fn drop(&mut self) {
		let egui::FullOutput {
			textures_delta,
			shapes,
			pixels_per_point
			..
		} = self.context.end_frame();

		let mut painter = self.painter.lock().unwrap();
		painter.shapes = self.context.tessellate(shapes);
		painter.textures_delta.push_front(textures_delta, pixels_per_point);
	}
}

/// Acts as an intermediary between [`ggez`] and [`egui`]
/// ```
/// use ggegui::{egui, Gui};
/// struct State {
///     gui: Gui,
/// }
///
/// impl State {
///     pub fn new(ctx: &mut Context) -> Self {
///         Self {
///             gui: Gui::new(ctx),
///         }
///     }
/// }
///
/// impl EventHandler for State {
///     fn update(&mut self, ctx: &mut Context) -> GameResult {
///         let gui_ctx = self.gui.ctx();
///
///         egui::Window::new("Title").show(&gui_ctx, |ui| {
///             ui.label("label");
///             if ui.button("button").clicked() {
///                 println!("button clicked");
///             }
///         });
///            self.gui.update(ctx);
///         Ok(())
///     }
///
///     fn draw(&mut self, ctx: &mut Context) -> GameResult {
///         let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
///         canvas.draw(
///             &self.egui_backend,
///             DrawParam::default().dest(glam::Vec2::ZERO),
///         );
///         canvas.finish(ctx)
///     }
/// }
/// ```
#[derive(Default)]
pub struct Gui {
	context: egui::Context,
	pub input: Input,
	painter: Arc<Mutex<Painter>>,
}

impl Gui {
	/// Create a [`Gui`] with extra information for use the [`Input::set_scale_factor`]
	pub fn new(ctx: &ggez::Context) -> Self {
		let mut input = Input::default();
		let (w, h) = ctx.gfx.size();
		input.set_scale_factor(1.0, (w, h));
		Self {
			input,
			..Default::default()
		}
	}

	pub fn update(&mut self, ctx: &mut ggez::Context) {
		self.input.update(ctx);
		self.painter
			.lock()
			.unwrap()
			.update(ctx, self.input.scale_factor);
		// self.input.set_scale_factor(1.0, ctx.gfx.size());
	}

	/// Return an [`EguiContext`] for update the gui
	pub fn ctx(&mut self) -> GuiContext {
		self.context.begin_frame(self.input.take());
		GuiContext {
			context: self.context.clone(),
			painter: self.painter.clone(),
		}
	}
}

impl Drawable for Gui {
	fn draw(&self, canvas: &mut Canvas, _param: impl Into<DrawParam>) {
		self.painter
			.lock()
			.unwrap()
			.draw(canvas, self.input.scale_factor);
	}

	fn dimensions(&self, _gfx: &impl Has<GraphicsContext>) -> Option<graphics::Rect> {
		None
	}
}
