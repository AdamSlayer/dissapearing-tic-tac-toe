use std::cmp::PartialEq;
use macroquad::color::WHITE;
use macroquad::math::Vec2;
use macroquad::prelude::{draw_texture, draw_texture_ex, DrawTextureParams};
use crate::game_move::GameMove;
use crate::play_result::PlayResult;
use crate::play_result::PlayResult::{Draw, InvalidMove, NoWinYet, Winner};
use crate::player::Player;
use crate::render_tools::RenderTools;
use crate::tile::Tile;
use crate::tile_state::TileState;
use crate::tile_state::TileState::Empty;

#[derive(PartialEq, Eq)]
pub(crate) struct GameState {
	field: [[Tile;3];3],
	pub turn: Player
}

impl GameState {
	pub fn init() -> Self {
		let empty_tile = Tile {
			state: TileState::Empty,
			ttl: 0
		};
		Self {
			field: [[empty_tile;3];3],
			turn: Player::X,
		}
	}
	
	pub async fn render(&self, render_tools: &mut RenderTools) {
		for x in 0..3 {
			for y in 0..3 {
				let tile = self.field[x][y].clone();
				let tex_path = tile.state.get_texture_path();
				// Separate scope for mutable borrow
				let tex = {
					render_tools.get_texture(tex_path).await
				};
				// Use `render_tools` immutably here
				let mut color = WHITE;
				color.a = tile.ttl as f32 * 0.4 + 0.2;
				draw_texture_ex(
					&tex,
					x as f32 * 128.,
					y as f32 * 128.,
					color,
					DrawTextureParams {
						dest_size: Some(Vec2 {
							x: 128.,
							y: 128.
						}),
						source: None,
						rotation: 0.0,
						flip_x: false,
						flip_y: false,
						pivot: None,
					}
				);
			}
		}
	}
	
	/// Make a move as the `pos` as the player whose turn it currently is.
	pub fn play(&mut self, game_move: GameMove) -> PlayResult {
		let (x, y) = game_move.to_tuple();
		// check if move is valid
		if self.field[x][y].state != Empty {
			return InvalidMove
		}
		
		// decrease ttl for all tiles
		for x in 0..3 {
			for y in 0..3 {
				let tile = &mut self.field[x][y];
				if tile.state == self.turn.get_tile_state() {
					if tile.ttl == 0 {
						tile.state = Empty;
						break
					}
					tile.ttl -= 1
				}
			}
		}
		// place the tile and switch turn
		self.field[x][y] = Tile {
			state: self.turn.get_tile_state(),
			ttl: 2,
		};
		self.turn = self.turn.next();
		// check for a winner
		match self.check_winner() {
			Some(winner) => Winner(winner),
			None => if self.is_draw() {
				Draw
			} else {
				NoWinYet
			},
		}
	}
	
	fn check_winner(&self) -> Option<Player> {
		let lines = [
			// Rows
			(self.field[0][0].state, self.field[0][1].state, self.field[0][2].state),
			(self.field[1][0].state, self.field[1][1].state, self.field[1][2].state),
			(self.field[2][0].state, self.field[2][1].state, self.field[2][2].state),
			// Columns
			(self.field[0][0].state, self.field[1][0].state, self.field[2][0].state),
			(self.field[0][1].state, self.field[1][1].state, self.field[2][1].state),
			(self.field[0][2].state, self.field[1][2].state, self.field[2][2].state),
			// Diagonals
			(self.field[0][0].state, self.field[1][1].state, self.field[2][2].state),
			(self.field[0][2].state, self.field[1][1].state, self.field[2][0].state),
		];
		
		for line in &lines {
			if line.0 == line.1 && line.1 == line.2 && line.0 != Empty {
				return Some(line.0.get_player().unwrap());
			}
		}
		None
	}
	
	// todo
	fn is_draw(&self) -> bool {
		false
	}
}

