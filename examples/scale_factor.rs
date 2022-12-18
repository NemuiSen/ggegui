use ggez::{Context, ContextBuilder, GameResult, glam};
use ggez::graphics::{self, Color, DrawParam};
use ggez::event::{self, EventHandler};
use ggez_egui::{EguiBackend, egui};

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("game_id", "author")
        .build()
        .expect("FATAL - Failed to create the window.");
    let my_game = MyGame::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    egui_backend: EguiBackend,
    scale_factor: f32
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        MyGame {
            egui_backend: EguiBackend::new(_ctx),
            scale_factor: 1.0
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {

        let egui_ctx = self.egui_backend.ctx();

		egui::Window::new("egui-window").show(&egui_ctx, |ui| {
			ui.group(|ui| {
				ui.label("scale_factor");
				ui.horizontal(|ui| {
					ui.add(egui::Slider::new(&mut self.scale_factor, 0.5..=1.5));
					if ui.button("update scale_factor").clicked() {
						let (w, h) = ctx.gfx.size();
						self.egui_backend.input.set_scale_factor(self.scale_factor, (w, h));
					}
				});
			});
			ui.add(egui::TextEdit::multiline(&mut "Test!").hint_text("text test:"));
			if ui.button("print text test").clicked() {
				println!("Test!");
			}
			if ui.button("close button").clicked() {
				ctx.request_quit();
			}
		});

        self.egui_backend.update(ctx);
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        canvas.draw(&self.egui_backend, DrawParam::default().dest(glam::vec2(0.0, 0.0)));
        canvas.finish(ctx)
    }
}