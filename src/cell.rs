use crate::vertex;
use rand::Rng;
use crate::texture;

#[derive(Copy, Clone)]
pub struct Cell {
	pub is_cleared: bool,
	pub has_flag: bool,
	pub has_mine: bool,
	pub mine_neighbours: u8,
	pub not_mine_guaranteed: bool,
}

impl Cell {
	pub fn new_blank() -> Self {
		Self {
			is_cleared: false, has_flag: false, has_mine: false, mine_neighbours: 0, not_mine_guaranteed: false,
		}
	}

	pub fn new(pos: [usize; 2]) -> Self {
		let mut rng = rand::thread_rng();
		let not_mine_guaranteed = pos == [0, 0];
		let has_mine = match not_mine_guaranteed {
			true => false,
			false => rng.gen_bool(0.1),
		};
		Self {
			is_cleared: false, has_flag: false, has_mine: has_mine, mine_neighbours: 0, not_mine_guaranteed: not_mine_guaranteed,
		}
	}

	pub fn draw(self: &Self, pos: [u16; 2]) -> Vec<vertex::Vertex> {
		// Vertex buffer to be returned
		// Draw cell without content
		let mut out = match self.is_cleared {
			false => match self.not_mine_guaranteed {
				false => texture::Texture::Cell,
				true => texture::Texture::CellNotMineGuaranteed,
			},
			true => texture::Texture::CellCleared,
		}.generate_tris(pos).to_vec();
		// Draw cell content
		match self.is_cleared {
			false => {
				if self.has_flag {
					out.extend(texture::Texture::Flag.generate_tris(pos));
				}
			},
			true => {
				if self.has_mine {
					out.extend(texture::Texture::Mine.generate_tris(pos));
				}
				else if self.mine_neighbours != 0 {
					out.extend(texture::Texture::Nearby(self.mine_neighbours).generate_tris(pos));
				}
			},
		}
		// Return vertex buffer
		out
	}
}