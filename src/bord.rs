use crate::{vertex, cell};

pub struct Bord {
	cells: ndarray::Array2<cell::Cell>,
}

impl Bord {
	pub fn new(size: [u16; 2]) -> Self {
		let mut out = Self {
			cells: ndarray::Array2::from_elem([size[0] as usize, size[1] as usize], cell::Cell::new_blank())
		};
		for x in 0..size[0] {
			for y in 0..size[1] {
				out.cells[[x as usize, y as usize]] = cell::Cell::new();
			}
		}
		out
	}

	pub fn draw(self: &Self, bord_start_y: u16) -> Vec<vertex::Vertex> {
		let mut out = Vec::new();
		for (y, column) in self.cells.axis_iter(ndarray::Axis(1)).enumerate() {
			for (x, tile) in column.iter().enumerate() {
				out.extend(tile.draw([x as u16 * 32, y as u16 * 32 + bord_start_y]));
			}
		}
		out
	}

	pub fn get_size_in_pixels(self: &Self) -> [u16; 2] {
		[
			self.cells.shape()[0] as u16 * 32,
			self.cells.shape()[1] as u16 * 32,
		]
	}
}