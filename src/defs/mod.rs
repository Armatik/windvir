use std::ops::{Add,Sub};


#[derive(Clone, Debug, Default)]
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
	pub start_point: PositionVector,
	pub end_point: PositionVector,
	pub sides: Vec<Vector>
}


impl Building {
	pub fn new(build: Vec<Vec<f64>>) -> Self {
		log::warn!("Центры и радиусы для отдельных зданий пока что не задаются. Нуждается в исправлении!");
		let vertex = build.iter().map(
			|x| Vector::new(
						PositionVector::new(x[0], x[1]),PositionVector::new(0.0f64, 0.0f64)
					)).collect::<Vec<Vector>>();
		
		Self {
			start_point: PositionVector::new(0.0f64,0.0f64),
			end_point: PositionVector::new(0.0f64,0.0f64),
			sides: vertex,
		}
	}
}


#[derive(Clone, Debug, Default)]
pub struct Vector {
	pub position: PositionVector,
	pub offset: PositionVector
}


impl Vector {
	pub fn new(position: PositionVector, offset: PositionVector) -> Self {
		Self { 
			position,
			offset
		}
	}

	#[inline]
	pub fn cross_product(&self, other: &Self) -> f64 {
		PositionVector::cross_product(&self.offset, &other.offset)
	}
}


impl Sub for Point {
	type Output = PositionVector;

	fn sub(self, end_point: Self) -> Self::Output {
		PositionVector::new(
			end_point.x - self.x, 
			end_point.y - self.y
		)
	}
}


impl Add for &Point {
	type Output = PositionVector;

	fn add(self, other: Self) -> Self::Output {
		PositionVector::new(
			self.x + other.x, 
			self.y + other.y
		)
	}
}


impl Sub for &Point {
	type Output = PositionVector;

	fn sub(self, start_point: Self) -> Self::Output {
		PositionVector::new(
			self.x - start_point.x, 
			self.y - start_point.y
		)
	}
}


impl Sub for &Vector {
	type Output = PositionVector;

	fn sub(self, subtractor_vector: Self) -> Self::Output {
		&self.offset - &subtractor_vector.offset
	}
}


impl Sub for &PositionVector {
	type Output = PositionVector;

	fn sub(self, subtractor_vector: Self) -> Self::Output {
		PositionVector::new(
			self.x - subtractor_vector.x,
			self.y - subtractor_vector.y
		)
	}
}


#[derive(Clone, Debug, Default)]
pub struct PositionVector {
	pub x: f64,
	pub y: f64,
}


impl PositionVector {

	pub fn new(x: f64, y: f64) -> Self {
		Self { 
			x,
			y
		}
	}

	#[inline]
	pub fn cross_product(&self, other: &Self) -> f64 {
		other.x*self.y - self.x*other.y
	}

}