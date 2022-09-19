use crate::vertex;

pub fn generate_tris_from_id(id: u8, pos: [u16; 2]) -> [vertex::Vertex; 6] {
	let x_start = pos[0] as f32;
	let x_end = (pos[0] + 32) as f32;
	let y_start = pos[1] as f32;
	let y_end = (pos[1] + 32) as f32;

	let texture_column = id % 16;
	let texture_row = id >> 4;
	let texture_x_start = (texture_column as f32) / 16.;
	let texture_y_start = 1. - ((texture_row + 1) as f32) / 16.;
	let texture_x_end = ((texture_column + 1) as f32) / 16.;
	let texture_y_end = 1. - (texture_row as f32) / 16.;

	[vertex::Vertex {
		position: [x_start, y_start],
		texture_position: [texture_x_start, texture_y_end]
	},
	vertex::Vertex {
		position: [x_end, y_start],
		texture_position: [texture_x_end, texture_y_end]
	},
	vertex::Vertex {
		position: [x_start, y_end],
		texture_position: [texture_x_start, texture_y_start]
	},
	vertex::Vertex {
		position: [x_end, y_start],
		texture_position: [texture_x_end, texture_y_end]
	},
	vertex::Vertex {
		position: [x_end, y_end],
		texture_position: [texture_x_end, texture_y_start]
	},
	vertex::Vertex {
		position: [x_start, y_end],
		texture_position: [texture_x_start, texture_y_start]
	}]
}

#[derive(Copy, Clone)]
pub enum Texture {
	Cell,
	CellCleared,
	Mine,
	Flag,
	Nearby(u8),
	CellNotMineGuaranteed,
	Reset,
}

impl Texture {
	fn get_texture_id(self) -> u8 {
		match self {
			Self::Cell => 0,
			Self::CellCleared => 1,
			Self::Mine => 2,
			Self::Flag => 3,
			Self::Nearby(nearby) => 3 + nearby,
			Self::CellNotMineGuaranteed => 0xB,
			Self::Reset => 0xC,
		}
	}

	pub fn generate_tris(self, pos: [u16; 2]) -> [vertex::Vertex; 6] {
		generate_tris_from_id(self.get_texture_id(), pos)
	}
}