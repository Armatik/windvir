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

	pub fn center_point(&self, other: &Self) -> Self {
		Self::new((self.x + other.x)/2.0f64, (self.y + other.y)/2.0f64)
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

	pub fn center_point(&self, other: &Self) -> Self {
		Self::new((self.x + other.x)/2.0f64, (self.y + other.y)/2.0f64)
	}

	#[inline]
	pub fn cross_product(&self, other: &Self) -> f64 {
		other.x*self.y - self.x*other.y
	}

	#[inline]
	pub fn dot_product(&self, other: &Self) -> f64 {
		other.x*self.x + self.x*other.x
	}

	// Если можно не использовать, лучше не использовать
	#[inline]
	pub fn get_length(&self) -> f64 {
		f64::sqrt(self.x*self.x + self.y*self.y)
	}

	#[inline]
	pub fn get_squared_length(&self) -> f64 {
		self.x*self.x + self.y*self.y
	}

	#[inline]
	pub fn get_cos(&self) -> f64 {
		self.x/self.get_length()
	}

	#[inline]
	pub fn get_sin(&self) -> f64 {
		self.y/self.get_length()
	}

	pub fn get_cos_sin(&self) -> (f64,f64) {
		let length = self.get_length();
		(self.x/length,self.y/length)
	}

	// Бесполезный мусор, так как делить на два нет смысла для сравнения площадей, лол
	#[inline]
	pub fn get_square(&self, other: &Self) -> f64 {
		Self::cross_product(self, other)/2.0f64
	}

	#[inline]
	pub fn get_cos_between_vectors(&self, other: &Self) -> f64 {
		Self::dot_product(self, other)/(self.get_length()*other.get_length())
	}

	#[inline]
	pub fn get_sin_between_vectors(&self, other: &Self) -> f64 {
		Self::cross_product(self, other)/(self.get_length()*other.get_length())
	}
}