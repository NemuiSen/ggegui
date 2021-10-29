mod input;
mod painter;
pub use input::Input;
use painter::Painter;
use std::{cell::RefCell, ops::Deref, rc::Rc};
use ggez::graphics::{self, Drawable};
use egui::{ClippedMesh, CtxRef};

/// Contains a copy of [`CtxRef`] and a mutable reference for the paint_jobs vector from [`Painter`].
///
/// When is droped automatically will call [`CtxRef::end_frame`] function and update the paint_jobs
pub struct EguiContext {
	context: CtxRef,
	paint_jobs: Rc<RefCell<Vec<ClippedMesh>>>,
}

impl Deref for EguiContext {
	type Target = CtxRef;
	fn deref(&self) -> &Self::Target {
		&self.context
	}
}

impl Drop for EguiContext {
	fn drop(&mut self) {
		let (_output, shapes) = self.context.end_frame();
		*self.paint_jobs.borrow_mut() = self.context.tessellate(shapes);
	}
}

/// Contains and handles everything related to [`egui`]
#[derive(Default)]
pub struct EguiBackend {
	context: CtxRef,
	pub input: Input,
	painter: RefCell<Painter>,
}

impl EguiBackend {
	/// Create a [`EguiBackend`] with extra information for use the [`Input::set_scale_factor`]
	pub fn new(ctx: &ggez::Context) -> Self {
		let mut input = Input::default();
		let (w, h) = graphics::size(ctx);
		input.set_scale_factor(1.0, (w, h));
		Self {
			input,
			..Default::default()
		}
	}

	/// Return an [`EguiContext`] for update the gui
	pub fn ctx(&mut self) -> EguiContext {
		self.context.begin_frame(self.input.raw.take());
		EguiContext {
			context: self.context.clone(),
			paint_jobs: self.painter.borrow().paint_jobs.clone(),
		}
	}
}

impl Drawable for EguiBackend {
	/// this funtion comes from [`Drawable`] trait that allow the struct use the function [`ggez::graphics::draw`]
	/// * Example
	/// ```
	/// struct State {
	/// 	egui_backend: EguiBackend
	/// }
	/// 
	/// impl EventHandler<ggez::GameError> for State {
	/// 	fn draw(&mut self, ctx: &mut ggez::Context) -> {
	/// 		ggez::graphics::draw(ctx, &self.egui_backend, ([0.0, 0.0],))?;
	/// 		...
	/// 	} 
	/// 	...
	/// }
	/// ```
	fn draw(&self, ctx: &mut ggez::Context, _param: ggez::graphics::DrawParam) -> ggez::GameResult {
		self.painter.borrow_mut().draw(ctx, &self.context.texture(), self.input.scale_factor)
	}

	fn dimensions(&self, _ctx: &mut ggez::Context) -> Option<ggez::graphics::Rect> {
		None
	}

	fn blend_mode(&self) -> Option<ggez::graphics::BlendMode> {
		None
	}

	fn set_blend_mode(&mut self, _mode: Option<ggez::graphics::BlendMode>) {}
}
