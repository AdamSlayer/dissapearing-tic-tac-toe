use crate::tile_state::TileState;
use crate::tile_state::TileState::{O, X};

/// One of the two players, O or X
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(crate) enum Player {
	O,
	X
}
impl Player {
	/// The tile state this player will change the tile to when they play (O is O and X is X)
	pub(crate) fn get_tile_state(self) -> TileState {
		match self {
			Player::O => O,
			Player::X => X,
		}
	}
	
	pub(crate) fn next(self) -> Player {
		match self {
			Player::O => Player::X,
			Player::X => Player::O,
		}
	}
}