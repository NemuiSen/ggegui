use std::time::Instant;

use egui::{pos2, vec2, Key, PointerButton, Pos2, RawInput};
use ggez::{
	event::MouseButton,
	input::keyboard::{KeyCode, KeyMods},
};

/// Contains and manages everything related to the [`egui`] input
///
/// such as the location of the mouse or the pressed keys
pub struct Input {
	dt: Instant,
	pointer_pos: Pos2,
	pub(crate) raw: RawInput,
	pub(crate) scale_factor: f32,
}

impl Default for Input {
	/// scale_factor: 1.0
	fn default() -> Self {
		Self {
			dt: Instant::now(),
			pointer_pos: Default::default(),
			raw: Default::default(),
			scale_factor: 1.0,
		}
	}
}

impl Input {
	pub(crate) fn take(&mut self) -> RawInput {
		self.raw.predicted_dt = self.dt.elapsed().as_secs_f32();
		self.dt = Instant::now();
		self.raw.take()
	}

	/// It updates egui of what is happening in the input (keys pressed, mouse position, etc), but it doesn't updates
	/// the information of the pressed characters, to update that information you have to
	/// use the function [text_input_event](Input:: text_input_event)
	pub fn update(&mut self, ctx: &ggez::Context) {
		/*======================= Keyboard =======================*/
		for key in ctx.keyboard.pressed_keys() {
			let pressed = ctx.keyboard.is_key_just_pressed(*key);
			if let Some(key) = translate_keycode(*key) {
				self.raw.events.push(egui::Event::Key {
					key,
					pressed,
					repeat: false,
					modifiers: translate_modifier(ctx.keyboard.active_mods()),
				})
			}
		}

		/*======================= Mouse =======================*/
		let ggez::mint::Point2 { x, y } = ctx.mouse.position();
		self.pointer_pos = pos2(x / self.scale_factor, y / self.scale_factor);
		self.raw
			.events
			.push(egui::Event::PointerMoved(self.pointer_pos));

		for button in [MouseButton::Left, MouseButton::Middle, MouseButton::Right] {
			if ctx.mouse.button_just_pressed(button) {
				self.raw.events.push(egui::Event::PointerButton {
					button: match button {
						MouseButton::Left => PointerButton::Primary,
						MouseButton::Right => PointerButton::Secondary,
						MouseButton::Middle => PointerButton::Middle,
						_ => unreachable!(),
					},
					pos: self.pointer_pos,
					pressed: true,
					modifiers: translate_modifier(ctx.keyboard.active_mods()),
				});
			} else if ctx.mouse.button_just_released(button) {
				self.raw.events.push(egui::Event::PointerButton {
					button: match button {
						MouseButton::Left => PointerButton::Primary,
						MouseButton::Right => PointerButton::Secondary,
						MouseButton::Middle => PointerButton::Middle,
						_ => unreachable!(),
					},
					pos: self.pointer_pos,
					pressed: false,
					modifiers: translate_modifier(ctx.keyboard.active_mods()),
				});
			}
		}
	}

	/// Set the scale_factor and update the screen_rect
	pub fn set_scale_factor(&mut self, scale_factor: f32, (w, h): (f32, f32)) {
		self.scale_factor = scale_factor;
		self.raw.pixels_per_point = Some(scale_factor);
		self.resize_event(w, h);
	}

	/// Update screen_rect data with window size
	pub fn resize_event(&mut self, w: f32, h: f32) {
		self.raw.screen_rect = Some(egui::Rect::from_min_size(
			Default::default(),
			vec2(w, h) / self.scale_factor,
		));
	}

	/// lets you know the rotation of the mouse wheel
	pub fn mouse_wheel_event(&mut self, x: f32, y: f32) {
		self.raw.events.push(egui::Event::Scroll(vec2(x, y)));
	}

	/// lets know what character is pressed on the keyboard
	pub fn text_input_event(&mut self, ch: char) {
		if is_printable(ch) {
			self.raw.events.push(egui::Event::Text(ch.to_string()));
		}
	}
}

#[inline]
fn translate_keycode(key: KeyCode) -> Option<egui::Key> {
	Some(match key {
		KeyCode::Escape => Key::Escape,
		KeyCode::Insert => Key::Insert,
		KeyCode::Home => Key::Home,
		KeyCode::Delete => Key::Delete,
		KeyCode::End => Key::End,
		KeyCode::PageDown => Key::PageDown,
		KeyCode::PageUp => Key::PageUp,
		KeyCode::Left => Key::ArrowLeft,
		KeyCode::Up => Key::ArrowUp,
		KeyCode::Right => Key::ArrowRight,
		KeyCode::Down => Key::ArrowDown,
		KeyCode::Back => Key::Backspace,
		KeyCode::Return => Key::Enter,
		KeyCode::Tab => Key::Tab,
		KeyCode::Space => Key::Space,

		KeyCode::A => Key::A,
		KeyCode::K => Key::K,
		KeyCode::U => Key::U,
		KeyCode::W => Key::W,
		KeyCode::Z => Key::Z,

		_ => {
			return None;
		}
	})
}

#[inline]
fn translate_modifier(keymods: KeyMods) -> egui::Modifiers {
	egui::Modifiers {
		alt: keymods.intersects(KeyMods::ALT),
		ctrl: keymods.intersects(KeyMods::CTRL),
		shift: keymods.intersects(KeyMods::SHIFT),

		#[cfg(not(target_os = "macos"))]
		mac_cmd: false,
		#[cfg(not(target_os = "macos"))]
		command: keymods.intersects(KeyMods::CTRL),

		#[cfg(target_os = "macos")]
		mac_cmd: keymods.intersects(KeyMods::LOGO),
		#[cfg(target_os = "macos")]
		command: keymods.intersects(KeyMods::LOGO),
	}
}

#[inline]
fn is_printable(chr: char) -> bool {
	let is_in_private_use_area = '\u{e000}' <= chr && chr <= '\u{f8ff}'
		|| '\u{f0000}' <= chr && chr <= '\u{ffffd}'
		|| '\u{100000}' <= chr && chr <= '\u{10fffd}';

	!is_in_private_use_area && !chr.is_ascii_control()
}
