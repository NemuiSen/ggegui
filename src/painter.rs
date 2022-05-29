use std::collections::{HashMap, LinkedList};

use ggez::graphics;

#[derive(Default, Clone)]
pub struct Painter {
	pub(crate) paint_jobs: Vec<egui::ClippedPrimitive>,
	pub(crate) textures_delta: LinkedList<egui::TexturesDelta>,
	textures: HashMap<egui::TextureId, graphics::Image>,
}

impl Painter {
	pub fn draw(&mut self, ctx: &mut ggez::Context, scale_factor: f32) -> ggez::GameResult {
		// Create and free textures
		if let Some(textures_delta) = self.textures_delta.pop_front() {
			self.update_textures(ctx, textures_delta)?;
		}
		// drawing meshes
		for egui::ClippedPrimitive { primitive, .. } in self.paint_jobs.as_slice() {
			match primitive {
				egui::epaint::Primitive::Mesh(mesh) => {
					if mesh.vertices.len() < 3 { continue; }

					let vertices = mesh.vertices.iter().map(|v| graphics::Vertex {
						pos: [v.pos.x, v.pos.y],
						uv: [v.uv.x, v.uv.y],
						color: egui::Rgba::from(v.color).to_array(),
					}).collect::<Vec<_>>();

					let ggez_mesh = graphics::Mesh::from_raw(
						ctx,
						vertices.as_slice(),
						mesh.indices.as_slice(),
						self.textures.get(&mesh.texture_id).map(|t| t.clone())
					)?;

					graphics::draw(
						ctx, &ggez_mesh,
						graphics::DrawParam::default().scale([scale_factor, scale_factor])
					)?;
				}
				egui::epaint::Primitive::Callback(_) => {
					panic!("Custom rendering callbacks are not implemented yet");
				}
			}

		}

		Ok(())
	}

	pub fn update_textures(&mut self, ctx: &mut ggez::Context, textures_delta: egui::TexturesDelta) -> ggez::GameResult {
		// set textures
		for (id, delta) in &textures_delta.set {
			let image = match &delta.image {
				egui::ImageData::Color(image) => {
					image.into_image(ctx)
				}
				egui::ImageData::Font(image) => {
					image.into_image(ctx)
				}
			}?;

			self.textures.insert(*id, image);
		}

		// free textures
		for id in &textures_delta.free {
			self.textures.remove(id);
		}

		Ok(())
	}
}

// Generate ggez Image from egui Texture
trait Image {
	fn into_image(&self, ctx: &mut ggez::Context) -> ggez::GameResult<graphics::Image>;
}

impl Image for egui::ColorImage {
	fn into_image(&self, ctx: &mut ggez::Context) -> ggez::GameResult<graphics::Image> {
		assert_eq!(
			self.width() * self.height(),
			self.pixels.len(),
			"Mismatch between texture size and texel count"
		);

		let mut pixels: Vec<u8> = Vec::with_capacity(self.pixels.len() * 4);

		for pixel in &self.pixels {
			pixels.extend(pixel.to_array());
		}

		graphics::Image::from_rgba8(
			ctx,
			self.width() as u16,
			self.height() as u16,
			pixels.as_slice()
		)
	}
}

impl Image for egui::FontImage {
	fn into_image(&self, ctx: &mut ggez::Context) -> ggez::GameResult<graphics::Image> {
		assert_eq!(
			self.width() * self.height(),
			self.pixels.len(),
			"Mismatch between texture size and texel count"
		);

		let mut pixels: Vec<u8> = Vec::with_capacity(self.pixels.len() * 4);

		let gamma = 1.0;
		for pixel in self.srgba_pixels(gamma) {
			pixels.extend(pixel.to_array());
		}

		graphics::Image::from_rgba8(
			ctx, 
			self.width()  as u16,
			self.height() as u16,
			pixels.as_slice()
		)
	}
}
