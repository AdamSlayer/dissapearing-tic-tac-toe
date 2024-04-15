use crate::player::Player;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PlayResult {
	InvalidMove,
	NoWinYet,
	Winner(Player),
	Draw,
}