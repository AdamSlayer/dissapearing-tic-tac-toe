use macroquad::input::KeyCode::P;
use crate::player::Player;

#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) enum TileState {
	Empty,
	O,
	X
}
impl TileState {
	pub(crate) fn get_texture_path(self) -> String {
		format!("./assets/{}", match self {
			TileState::Empty => "empty.png",
			TileState::O => "o.png",
			TileState::X => "x.png",
		})
	}
	
	pub(crate) fn get_player(self) -> Option<Player> {
		match self {
			TileState::Empty => None,
			TileState::O => Some(Player::O),
			TileState::X => Some(Player::X),
		}
	}
}