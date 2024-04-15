use crate::tile_state::TileState;


/// Smaller than a pointer, no need not to clone or copy
#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) struct Tile {
	pub(crate) state: TileState,
	/// Time to live, will turn `state` to None when it reaches 0.
	///
	/// Should be `0` when `state` is `None`.
	pub(crate) ttl: u8
}


