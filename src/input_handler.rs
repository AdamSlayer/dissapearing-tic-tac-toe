use macroquad::input::{is_mouse_button_down, mouse_position, MouseButton};
use macroquad::miniquad::CursorIcon::Move;
use macroquad::prelude::is_mouse_button_pressed;
use macroquad::ui::Drag::No;
use crate::game_move::GameMove;

pub struct InputHandler {
	pub move_to_play: Option<GameMove>
}
impl InputHandler {
	pub fn init() -> Self {
		Self {
			move_to_play: None
		}
	}
	pub fn update(&mut self) {
		if is_mouse_button_pressed(MouseButton::Left) {
			let x = ((mouse_position().0/128.) as usize).clamp(0,2);
			let y = ((mouse_position().1/128.) as usize).clamp(0,2);
			self.move_to_play = Some(GameMove { x, y })
		}
	}
}
