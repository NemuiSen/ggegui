use std::collections::{HashMap, LinkedList};

use ggez::graphics;

#[derive(Default, Clone)]
pub struct Painter {
	pub(crate) shapes: Vec<egui::ClippedPrimitive>,
	pub(crate) textures_delta: LinkedList<egui::TexturesDelta>,
	paint_jobs: Vec<(egui::TextureId, graphics::Mesh)>,
	textures: HashMap<egui::TextureId, graphics::Image>,
}

impl Painter {
	pub fn draw(&mut self, canvas: &mut graphics::Canvas, scale_factor: f32) {
		for (id, mesh) in self.paint_jobs.iter() {
			canvas.draw(
				mesh,
				graphics::DrawParam::default().scale([scale_factor, scale_factor])
			);
			/*canvas.draw_textured_mesh(
				mesh.clone(),
				self.textures[&id].clone(),
				graphics::DrawParam::default().scale([scale_factor, scale_factor])
			);*/
		}
		self.paint_jobs.clear();
	}

	pub fn update(&mut self, ctx: &mut ggez::Context) {
		// Create and free textures
		if let Some(textures_delta) = self.textures_delta.pop_front() {
			self.update_textures(ctx, textures_delta);
		}

		// generating meshes
		for egui::ClippedPrimitive { primitive, .. } in self.shapes.iter() {
			match primitive {
				egui::epaint::Primitive::Mesh(mesh) => {
					if mesh.vertices.len() < 3 { continue; }

					let vertices = mesh.vertices.iter().map(|v| graphics::Vertex {
						position: [v.pos.x, v.pos.y],
						uv: [v.uv.x, v.uv.y],
						color: egui::Rgba::from(v.color).to_array()
					}).collect::<Vec<_>>();

					self.paint_jobs.push((
						mesh.texture_id,
						graphics::Mesh::from_data(
							ctx,
							graphics::MeshData {
								vertices: vertices.as_slice(),
								indices: mesh.indices.as_slice()
							}
						)
					));
				},
				egui::epaint::Primitive::Callback(_) => {
					panic!("Custom rendering callbacks are not implemented yet");
				}
			}
		}
	}

	pub fn update_textures(&mut self, ctx: &mut ggez::Context, textures_delta: egui::TexturesDelta) {
		// set textures
		for (id, delta) in &textures_delta.set {
			let image = match &delta.image {
				egui::ImageData::Color(image) => {
					image.into_image(ctx)
				}
				egui::ImageData::Font(image) => {
					image.into_image(ctx)
				}
			};

			self.textures.insert(*id, image);
		}

		// free textures
		for id in &textures_delta.free {
			self.textures.remove(id);
		}
	}
}

// Generate ggez Image from egui Texture
trait Image {
	fn into_image(&self, ctx: &mut ggez::Context) -> graphics::Image;
}

impl Image for egui::ColorImage {
	fn into_image(&self, ctx: &mut ggez::Context) -> graphics::Image {
		assert_eq!(
			self.width() * self.height(),
			self.pixels.len(),
			"Mismatch between texture size and texel count"
		);

		let mut pixels: Vec<u8> = Vec::with_capacity(self.pixels.len() * 4);

		for pixel in &self.pixels {
			pixels.extend(pixel.to_array());
		}

		graphics::Image::from_pixels(
			ctx,
			pixels.as_slice(),
			graphics::ImageFormat::Rgba8UnormSrgb,
			self.width() as u32,
			self.height() as u32
		)
	}
}

impl Image for egui::FontImage {
	fn into_image(&self, ctx: &mut ggez::Context) -> graphics::Image {
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

		graphics::Image::from_pixels(
			ctx, 
			pixels.as_slice(),
			graphics::ImageFormat::Rgba8UnormSrgb,
			self.width() as u32,
			self.height() as u32,
		)
	}
}
