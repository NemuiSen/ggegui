use std::{cell::RefCell, rc::Rc};

use egui::ClippedMesh;
use ggez::graphics;

#[derive(Default, Clone)]
pub struct Painter {
	pub paint_jobs: Rc<RefCell<Vec<ClippedMesh>>>,
	ggez_image: Option<graphics::Image>,
	egui_texture_version: Option<u64>,
}

impl Painter {
	pub fn draw(&mut self, ctx: &mut ggez::Context, egui_texture: &egui::Texture, scale_factor: f32) -> ggez::GameResult {
		// upload egui texture
		if self.egui_texture_version != Some(egui_texture.version) {
			self.ggez_image = Some(egui_texture.into_image(ctx)?);
			self.egui_texture_version = Some(egui_texture.version);
		}

		// drawing meshes
		for egui::ClippedMesh(_clip_rect, egui_mesh) in self.paint_jobs.borrow_mut().as_slice() {
			let vertices = egui_mesh.vertices.iter().map(|v| {
				graphics::Vertex {
					pos: [v.pos.x, v.pos.y],
					uv: [v.uv.x, v.uv.y],
					color: egui::Rgba::from(v.color).to_array(),
				}
			}).collect::<Vec<_>>();

			let ggez_mesh = graphics::Mesh::from_raw(
				ctx,
				vertices.as_slice(),
				egui_mesh.indices.as_slice(),
				self.ggez_image.clone()
			)?;

			graphics::draw(
				ctx, &ggez_mesh,
				graphics::DrawParam::default().scale([scale_factor, scale_factor])
			)?;
		}

		Ok(())
	}
}

// Generate ggez Image from egui Texture
trait Image {
	fn into_image(&self, ctx: &mut ggez::Context) -> ggez::GameResult<graphics::Image>;
}

impl Image for egui::Texture {
	fn into_image(&self, ctx: &mut ggez::Context) -> ggez::GameResult<graphics::Image> {
		let mut pixels: Vec<u8> = Vec::with_capacity(self.pixels.len() * 4);

		for pixel in self.srgba_pixels(1.0) {
			pixels.extend(pixel.to_array());
		}

		graphics::Image::from_rgba8(
			ctx, 
			self.width  as u16,
			self.height as u16,
			pixels.as_slice()
		)
	}
}
