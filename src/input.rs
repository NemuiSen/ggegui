use egui::{Key, PointerButton, Pos2, RawInput, pos2, vec2};
use ggez::event::*;

/// Contains and manages everything related to the [`egui`] input
/// 
/// such as the location of the mouse or the pressed keys
pub struct Input {
	pointer_pos: Pos2,
	pub(crate) raw: RawInput,
	pub(crate) scale_factor: f32,
}

impl Default for Input {
	/// scale_factor: 1.0
	fn default() -> Self {
		Self {
			pointer_pos: Default::default(),
			raw: Default::default(),
			scale_factor: 1.0,
		}
	}
}

impl Input {
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

	/*======================= Mouse =======================*/

	/// lets know which key is pressed on the mouse
	pub fn mouse_button_down_event(&mut self, button: MouseButton) {
		self.raw.events.push(egui::Event::PointerButton {
			button: match button {
				MouseButton::Left => PointerButton::Primary,
				MouseButton::Right => PointerButton::Secondary,
				MouseButton::Middle => PointerButton::Middle,
				_ => unreachable!()
			},
			modifiers: Default::default(),
			pos: self.pointer_pos,
			pressed: true
		});
	}

	/// lets know which key was released on the mouse
	pub fn mouse_button_up_event(&mut self, button: MouseButton) {
		self.raw.events.push(egui::Event::PointerButton {
			button: match button {
				MouseButton::Left => PointerButton::Primary,
				MouseButton::Right => PointerButton::Secondary,
				MouseButton::Middle => PointerButton::Middle,
				_ => unreachable!()
			},
			modifiers: Default::default(),
			pos: self.pointer_pos,
			pressed: false
		});
	}

	/// lets you know the rotation of the mouse wheel
	pub fn mouse_wheel_event(&mut self, x: f32, y: f32) {
		self.raw.scroll_delta = vec2(x, y);
	}

	/// lets know the location of the mouse
	pub fn mouse_motion_event(&mut self, x: f32, y: f32) {
		self.pointer_pos = pos2(
			x / self.scale_factor,
			y / self.scale_factor
		);
		self.raw.events.push(egui::Event::PointerMoved(self.pointer_pos));
	}

	/*======================= Keyboard =======================*/

	/// lets know which key is pressed on the keyboard
	pub fn key_down_event(&mut self, keycode: KeyCode, keymods: KeyMods) {
		if keymods.intersects(KeyMods::CTRL) {
			match keycode {
				KeyCode::C => self.raw.events.push(egui::Event::Copy),
				KeyCode::X => self.raw.events.push(egui::Event::Cut),
				#[cfg(feature = "clipboard")]
				KeyCode::V => unimplemented!(),
				_ =>  {
					if let Some(key) = winit_to_egui_key_code(keycode) {
						self.raw.events.push(egui::Event::Key {
							key,
							pressed: true,
							modifiers: ggez_to_egui_modifiers(keymods),
						});
					}
				}
			}
		} else {
			if let Some(key) = winit_to_egui_key_code(keycode) {
				self.raw.events.push(egui::Event::Key {
					key,
					pressed: true,
					modifiers: ggez_to_egui_modifiers(keymods),
				});
			}
		}
	}

	/// lets know what character is pressed on the keyboard
	pub fn text_input_event(&mut self, ch: char) {
		if is_printable(ch) {
			self.raw.events.push(egui::Event::Text(ch.to_string()));
		}
	}
}

#[inline]
fn winit_to_egui_key_code(key: KeyCode) -> Option<egui::Key> {
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
fn ggez_to_egui_modifiers(keymods: KeyMods) -> egui::Modifiers {
	egui::Modifiers {
        alt: keymods.intersects(KeyMods::ALT),
        ctrl: keymods.intersects(KeyMods::CTRL),
        shift: keymods.intersects(KeyMods::SHIFT),
        #[cfg(target_os = "macos")]
        mac_cmd: keymods.intersects(KeyMods::LOGO),
        #[cfg(target_os = "macos")]
        command: keymods.intersects(KeyMods::LOGO),
        #[cfg(not(target_os = "macos"))]
        mac_cmd: false,
        #[cfg(not(target_os = "macos"))]
        command: keymods.intersects(KeyMods::CTRL),
    }
}

#[inline]
fn is_printable(chr: char) -> bool {
    let is_in_private_use_area = '\u{e000}' <= chr && chr <= '\u{f8ff}'
        || '\u{f0000}' <= chr && chr <= '\u{ffffd}'
        || '\u{100000}' <= chr && chr <= '\u{10fffd}';

    !is_in_private_use_area && !chr.is_ascii_control()
}