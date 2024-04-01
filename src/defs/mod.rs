// use std::ops::{Add, Sub, SubAssign, AddAssign};


#[derive(Clone, Debug)]
pub struct Point {
	pub x: f64,
	pub y: f64,
}


impl Point {
	pub fn new(x: f64, y: f64) -> Self {
		Self {
			x,
			y,
		}
	}
}


#[derive(Debug)]
pub struct Building {
	pub center: Point,
	pub leftmost_point_index: u64,
	pub points: Vec<Point>,
}


impl Building {
	pub fn new(build: Vec<Vec<f64>>) -> Self {
		log::warn!("Центры и радиусы для отдельных зданий пока что не задаются. Нуждается в исправлении!");
		let vertex = build.iter().map(|x| Point::new(x[0], x[1])).collect::<Vec<Point>>();
		
		Self {
			center: Point::new(0., 0.),
			leftmost_point_index: 0u64,
			points: vertex,
		}
	}
}

#[derive(Clone, Debug, Default)]
pub struct Vector {
	pub x: f64,
	pub y: f64,
	pub dx: f64,
	pub dy: f64,
	// pub offset: PositionVector
}

impl Vector {
	pub fn new(x: f64, y: f64, dx: f64, dy: f64 ) -> Self {
		Self { 
			x, 
			y,
			dx,
			dy
		}
	}

	pub fn cross_product(&self, other: &Self) -> f64 {
		other.dx*self.dy - self.dx*other.dy
	}
}

// impl Add for Point {
// 	type Output = Vector;

// 	fn add(self, end_point: Self) -> Self::Output {
// 		Vector::new(
// 			self.x,
// 			self.y,
// 			PositionVector::new(self.x + end_point.x,self.y + end_point.y)
// 		)
// 	}
// }

// impl Sub for Point {
// 	type Output = Vector;

// 	fn sub(self, end_point: Self) -> Self::Output {
// 		Vector::new(
// 			self.x,
// 			self.y,
// 			PositionVector::new(end_point.x - self.x, end_point.y - self.y)
// 		)
// 	}
// }

// impl Add for Vector {
// 	type Output = Self;

// 	fn add(self, addor_vector: Self) -> Self::Output {
// 		Vector::new(
// 			self.x,
// 			self.y,
// 			self.offset + addor_vector.offset
// 		)
// 	}
// }

// impl Sub for Vector {
// 	type Output = Self;

// 	fn sub(self, subtractor_vector: Self) -> Self::Output {
// 		Vector::new(
// 			self.x,
// 			self.y,
// 			self.offset - subtractor_vector.offset
// 		)
// 	}
// }

// impl Add for PositionVector {
// 	type Output = Self;

// 	fn add(self, addor_vector: Self) -> Self::Output {
// 		PositionVector::new(
// 			self.dx + addor_vector.dx,
// 			self.dy + addor_vector.dy
// 		)
// 	}
// }

// impl Sub for PositionVector {
// 	type Output = Self;

// 	fn sub(self, subtractor_vector: Self) -> Self::Output {
// 		PositionVector::new(
// 			self.dx - subtractor_vector.dx,
// 			self.dy - subtractor_vector.dy
// 		)
// 	}
// }


// impl Add for &PositionVector {
// 	type Output = PositionVector;

// 	fn add(self, addor_vector: Self) -> Self::Output {
// 		PositionVector::new(
// 			self.dx + addor_vector.dx,
// 			self.dy + addor_vector.dy
// 		)
// 	}
// }

// impl Sub for &Vector {
// 	type Output = PositionVector;

// 	fn sub(self, subtractor_vector: Self) -> Self::Output {
// 		self - subtractor_vector
// 	}
// }

// impl Sub for &PositionVector {
// 	type Output = PositionVector;

// 	fn sub(self, subtractor_vector: Self) -> Self::Output {
// 		PositionVector::new(
// 			self.dx - subtractor_vector.dx,
// 			self.dy - subtractor_vector.dy
// 		)
// 	}
// }

// #[derive(Clone, Debug, Default)]
// pub struct PositionVector {
// 	pub dx: f64,
// 	pub dy: f64,
// }
// impl PositionVector {

// 	pub fn new(dx: f64, dy: f64) -> Self {
// 		Self { 
// 			dx,
// 			dy
// 		}
// 	}

// 	pub fn cross_product(&self, other: &Self) -> f64 {
// 		other.dx*self.dy - self.dx*other.dy
// 	}

// }