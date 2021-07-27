use std::{cell::RefCell, rc::Rc};

use egui::ClippedMesh;
use ggez::graphics;

#[derive(Default, Clone)]
pub struct Painter {
	pub paint_jobs: Rc<RefCell<Vec<ClippedMesh>>>,
	egui_texture: Option<graphics::Image>,
	egui_texture_version: Option<u64>,
}

impl Painter {
	pub fn draw(&mut self, ctx: &mut ggez::Context, egui_texture: &egui::Texture, scale_factor: f32) -> ggez::GameResult {
		self.upload_egui_texture(ctx, egui_texture)?;

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
				self.egui_texture.clone()
			)?;

			graphics::draw(
				ctx, &ggez_mesh,
				graphics::DrawParam::default().scale([scale_factor, scale_factor])
			)?;
		}

		Ok(())
	}

	fn upload_egui_texture(&mut self, ctx: &mut ggez::Context, egui_texture: &egui::Texture) -> ggez::GameResult {
		if self.egui_texture_version == Some(egui_texture.version) {
			return Ok(());
		}
		self.egui_texture = egui_texture_to_ggez(ctx, egui_texture)?;
		self.egui_texture_version = Some(egui_texture.version);
		Ok(())
	}
}

// Generate ggez Image from egui Texture
#[inline]
pub fn egui_texture_to_ggez(ctx: &mut ggez::Context, egui_texture: &egui::Texture) -> ggez::GameResult<Option<graphics::Image>> {
	let mut pixels: Vec<u8> = Vec::with_capacity(egui_texture.pixels.len() * 4);

	for srgba in egui_texture.srgba_pixels() {
		pixels.extend(srgba.to_array());
	}

	Ok(Some(graphics::Image::from_rgba8(
		ctx,
		egui_texture.width  as u16,
		egui_texture.height as u16,
		pixels.as_slice()
	)?))
}
