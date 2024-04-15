/// A single move by a single player.
/// It is defined only by the position of the move,
/// because the player that played this move can be inferred from the order of the moves.
/// (Players change moves and the first is always X)
#[derive(Copy, Clone, Debug)]
pub struct GameMove {
	pub(crate) x: usize,
	pub(crate) y: usize,
}
impl GameMove {
	pub fn to_tuple(self) -> (usize, usize) {
		(self.x, self.y)
	}
}