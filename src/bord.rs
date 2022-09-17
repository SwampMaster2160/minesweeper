use glium::glutin::event;

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
		// For each cell calculate how many neighbours have mines
		for x in 0..(size[0] as usize) {
			for y in 0..(size[1] as usize) {
				let mut mine_neighbours = 0u8;
				for x_1 in (x.max(1) - 1)..=(x + 1).min((size[0] - 1) as usize) {
					for y_1 in (y.max(1) - 1)..=(y + 1).min((size[1] - 1) as usize) {
						if x == x_1 && y == y_1 {
							continue;
						}
						if out.cells[[x_1, y_1]].has_mine {
							mine_neighbours += 1;
						}
					}
				}
				out.cells[[x, y]].mine_neighbours = mine_neighbours;
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

	pub fn click(self: &mut Self, pos: [u16; 2], button: event::MouseButton) -> bool {
		let mut game_over = false;
		let cell = self.cells.get_mut([pos[0] as usize / 32, pos[1] as usize / 32]);
		match cell {
			Some(valid_cell) => match button {
				event::MouseButton::Left => {
					if valid_cell.has_mine {
						game_over = true;
						valid_cell.is_cleared = true;
					}
					else {
						self.cascade([pos[0] / 32, pos[1] / 32]);
					}
				},
				event::MouseButton::Right => valid_cell.has_flag = !valid_cell.has_flag,
				_ => {},
			}
			_ => {},
		}
		game_over
	}

	fn cascade(self: &mut Self, pos: [u16; 2]) {
		let cell = &mut self.cells[[pos[0] as usize, pos[1] as usize]];
		if cell.is_cleared {
			return
		}
		cell.is_cleared = true;
		if cell.mine_neighbours == 0 {
			for x in ((pos[0]).max(1) - 1)..=((pos[0]) + 1).min((self.cells.shape()[0] - 1) as u16) {
				for y in ((pos[1]).max(1) - 1)..=((pos[1]) + 1).min((self.cells.shape()[1] - 1) as u16) {
					if x == pos[0] && y == pos[1] {
						continue;
					}
					self.cascade([x, y]);
				}
			}
		}
	}
}