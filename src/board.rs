use std::ops::{Add, Sub};

#[derive(Clone, Debug)]
pub struct CubeCoord {
	pub x: i32,
	pub y: i32,
	pub z: i32
}

// Axial Coordinates
pub struct HexCoord {
	pub q: i32,
	pub r: i32
}

impl Into<CubeCoord> for HexCoord {
	fn into(self) -> CubeCoord {
		CubeCoord {
			x: self.q,
			y: - self.q - self.r,
			z: self.r
			
		}
	}
}

impl Into<HexCoord> for CubeCoord {
	fn into(self) -> HexCoord {
		HexCoord {
			q: self.x,
			r: self.z
		}
	}
}

impl<'a> From<&'a HexCoord> for CubeCoord {
	fn from(hex_coord: &HexCoord) -> CubeCoord {
		CubeCoord {
			x: hex_coord.q,
			y: -hex_coord.q - hex_coord.r,
			z: hex_coord.r
		}
	}
}

impl<'a> From<&'a CubeCoord> for HexCoord {
	fn from(cube_coord: &CubeCoord) -> HexCoord {
		HexCoord {
			q: cube_coord.x,
			r: cube_coord.z
		}
	}
}

impl<'a> Add<CubeCoord> for &'a CubeCoord {
	type Output = CubeCoord;

	fn add(self, other: CubeCoord) -> CubeCoord {
		return CubeCoord {
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z
		}
	}
}

impl<'a> Sub<CubeCoord> for &'a CubeCoord {
	type Output = CubeCoord;

	fn sub(self, other: CubeCoord) -> CubeCoord {
		return CubeCoord {
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z
		}
	}
}

impl<'a> Add<HexCoord> for &'a HexCoord {
	type Output = HexCoord;

	fn add(self, other: HexCoord) -> HexCoord {
		return HexCoord {
			q: self.q + other.q,
			r: self.r + other.r
		}
	}
}

impl<'a> Sub<HexCoord> for &'a HexCoord {
	type Output = HexCoord;

	fn sub(self, other: HexCoord) -> HexCoord {
		return HexCoord {
			q: self.q - other.q,
			r: self.r - other.r
		}
	}
}

impl Coord3D for HexCoord {
	fn distance(&self, coord_2: &Self) -> u32 {
		return CubeCoord::from(self).distance(
			&CubeCoord::from(coord_2)
		) / 2;
	}

	fn get_neighbors(&self) -> [HexCoord; 6] {
		return [
	   		self + HexCoord {r:  1, q:  0},
	   		self + HexCoord {r:  1, q: -1},
	   		self + HexCoord {r:  0, q: -1},
			self + HexCoord {r:  0, q:  1},
			self + HexCoord {r: -1, q:  0},
			self + HexCoord {r: -1, q:  1}
		];
	}
}

impl Coord3D for CubeCoord {
	fn distance(&self, coord_2: &CubeCoord) -> u32 {
		return (
			(self.x - coord_2.x).abs() + 
			(self.y - coord_2.y).abs() + 
			(self.z - coord_2.z).abs()
		) as u32;
	}

	fn get_neighbors(&self) -> [CubeCoord; 6] {
		return [
	   		self + CubeCoord {x:  1, y: -1, z:  0},
	   		self + CubeCoord {x:  1, y:  0, z: -1},
	   		self + CubeCoord {x:  0, y:  1, z: -1},
			self + CubeCoord {x:  0, y: -1, z:  1},
			self + CubeCoord {x: -1, y:  1, z:  0},
			self + CubeCoord {x: -1, y:  0, z:  1}
		];
	}
}

pub trait Coord3D {
	fn distance(&self, coord_2: &Self) -> u32;
	fn get_neighbors(&self) -> [Self; 6];
	fn is_adjacent(&self, coord_2: &Self) -> bool {
		return self.distance(coord_2) ==  1;
	}
}

pub struct HexGrid {
	pub grid_size: u8,
	pub hex_size: f64
}

impl HexGrid {
	pub fn new(size: u8, hex_size: f64) -> HexGrid {
		HexGrid {
			grid_size: size,
			hex_size: hex_size
		}
	}
}