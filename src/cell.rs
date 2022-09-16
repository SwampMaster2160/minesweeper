use crate::vertex;
use rand::Rng;
use crate::texture;

#[derive(Copy, Clone)]
pub struct Cell {
	is_cleared: bool,
	has_flag: bool,
	has_mine: bool,
	nearby: u8
}

impl Cell {
	pub fn new_blank() -> Self {
		Self {
			is_cleared: false, has_flag: false, has_mine: false, nearby: 0,
		}
	}

	pub fn new() -> Self {
		let mut rng = rand::thread_rng();
		Self {
			is_cleared: false, has_flag: false, has_mine: rng.gen_bool(0.1), nearby: 0,
		}
	}

	pub fn draw(self: &Self, pos: [u16; 2]) -> Vec<vertex::Vertex> {
		let mut out = Vec::new();
		out.extend(match self.is_cleared {
			false => texture::Texture::Cell,
			true => texture::Texture::CellCleared,
		}.generate_tris(pos));
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
				else if self.nearby != 0 {
					out.extend(texture::Texture::Nearby(self.nearby).generate_tris(pos));
				}
			},
		}
		out
	}
}