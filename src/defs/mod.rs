pub mod synthetic;
pub mod app;
pub mod error;

use std::ops::{Add, Sub, Mul};


#[derive(Clone, Debug, Default, PartialEq)]
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
        Self::new((self.x + other.x) / 2., (self.y + other.y) / 2.)
    }

    pub fn is_point_default(&self) -> bool {
        *self == Self::default()
    }
}


#[derive(Debug, PartialEq)]
pub struct Building {
    pub start_point: PositionVector,
    pub end_point: PositionVector,
    pub sides: Vec<Vector>
}


impl Building {
    pub fn new(build: Vec<Vec<f64>>) -> Self {
        let vertex = build.iter().map(
            |x| Vector::new(
                        PositionVector::new(x[0], x[1]),PositionVector::new(0., 0.)
                    )).collect::<Vec<Vector>>();
        
        Self {
            start_point: PositionVector::new(0.,0.),
            end_point: PositionVector::new(0.,0.),
            sides: vertex,
        }
    }

    pub fn new_complete(build: Vec<Vec<f64>>) -> Self {
        let mut vertex = build.iter().map(
            |x| Vector::new(
                        PositionVector::new(x[0], x[1]),PositionVector::new(0., 0.)
                    )).collect::<Vec<Vector>>();

        if vertex.len() < 3usize { panic!("У переданного здания меньше трёх сторон!\n{:?}",build); }
        for i in 0usize..vertex.len() - 1usize {
            vertex[i].offset = &vertex[i + 1usize].position - &vertex[i].position;
        }
        let last_index = vertex.len() - 1usize;
        vertex[last_index].offset = &vertex[0usize].position - &vertex[last_index].position;

        Self {
            start_point: PositionVector::new(0.,0.),
            end_point: PositionVector::new(0., 0.),
            sides: vertex,
        }.caclulate_and_setup_bounding_box_points()
    }

    fn caclulate_and_setup_bounding_box_points(mut self) -> Self {
        self.start_point = self.sides[0usize].position.clone();
        self.end_point = self.sides[0usize].position.clone();
        for side in self.sides.iter() {
            if side.position.x < self.start_point.x {
                self.start_point.x = side.position.x;
            } else if side.position.x > self.end_point.x {
                self.end_point.x = side.position.x;
            }

            if side.position.y < self.start_point.y {
                self.start_point.y = side.position.y;
            } else if side.position.y > self.end_point.y {
                self.end_point.y = side.position.y;
            }
        }
        self
    }

    pub fn triangulate(&self) -> Vec<usize> {
        let mut points = Vec::<f64>::with_capacity(self.sides.len()*2usize);
        for vertex in &self.sides {
            points.append(&mut vec![vertex.position.x, vertex.position.y]);
        }
        earcutr::earcut(&points, &[], 2).unwrap()
    }

    pub fn get_square(&self) -> f64 {
        let triangulation_indices = self.triangulate();
        let mut square = 0.;
        for i in 0..triangulation_indices.len()/3usize {
            square +=  PositionVector::get_square(&(&self.sides[i].position - &self.sides[i + 1usize].position),&(&self.sides[i].position - &self.sides[i + 2usize].position));
        }
        square
    }

    pub fn get_double_square(&self) -> f64 {
        let triangulation_indices = self.triangulate();
        let mut square = 0.;
        for i in 0..triangulation_indices.len()/3usize {
            square +=  PositionVector::get_double_square(&(&self.sides[i].position - &self.sides[i + 1usize].position),&(&self.sides[i].position - &self.sides[i + 2usize].position));
        }
        square
    }
}


#[derive(Clone, Debug, Default, PartialEq)]
pub struct Vector {
    pub position: PositionVector,
    pub offset: PositionVector,
}


impl Vector {
    pub fn new(position: PositionVector, offset: PositionVector) -> Self {
        Self { 
            position,
            offset
        }
    }

    #[inline]
    pub fn cross(&self, other: &Self) -> f64 {
        PositionVector::cross(&self.offset, &other.offset)
    }

    #[inline]
    pub fn dot(&self, other: &Self) -> f64 {
        PositionVector::dot(&self.offset, &other.offset)
    }

    pub fn get_right_normal(&self) -> PositionVector {
        PositionVector { 
            x: self.offset.y,
            y: -self.offset.x
        }
    }

    pub fn get_left_normal(&self) -> PositionVector {
        PositionVector { 
            x: -self.offset.y,
            y: self.offset.x
        }
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

impl Add for &PositionVector {
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

impl Mul<f64> for &PositionVector {
    type Output = PositionVector;

    fn mul(self, multiplier: f64) -> Self::Output {
        PositionVector::new(multiplier * self.x, multiplier * self.y)
    }
}

impl Mul<&PositionVector> for f64 {
    type Output = PositionVector;

    fn mul(self, multiplicand: &PositionVector) -> Self::Output {
        PositionVector::new(self * multiplicand.x, self * multiplicand.y)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
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

    // pub fn get_normal_magnitude_to_vector(&self, vector: &Vector<T>) -> T {
    //     let position_difference =  self - vector;
    //     let cross_product = Self::cross(position_difference, &vector.offset);
    //     cross_product*cross_product/vector.offset.get_squared_magnitude()
    // }
    
    pub fn get_normal_magnitude_to_vector(&self, vector: &Vector) -> f64 {
        let cross_product = Self::cross(self, &vector.offset);
        cross_product*cross_product/vector.offset.get_squared_magnitude()
    }

    pub fn multiply_by_scalar(&self, multiplier: f64) -> Self {
        PositionVector::new(multiplier * self.x, multiplier * self.y)
    }

    pub fn center_between_vectors(&self, other: &Self) -> Self {
        Self::new((self.x + other.x) / 2., (self.y + other.y) / 2.)
    }

    pub fn center(&self) -> Self {
        Self::new(self.x/ 2., self.y / 2.)
    }

    #[inline]
    pub fn dot(&self, other: &Self) -> f64 {
        other.x*self.x + self.y*other.y
    }

    // Если можно не использовать, лучше не использовать
    #[inline]
    pub fn get_magnitude(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y)
    }
    
    #[inline]
    pub fn get_squared_magnitude(&self) -> f64 {
        self.x*self.x + self.y*self.y
    }

    #[inline]
    pub fn cross(&self, other: &Self) -> f64 {
        self.x*other.y - other.x*self.y
    }

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
        f64::sqrt(self.x * self.x + self.y * self.y)
    }

    #[inline]
    pub fn get_squared_length(&self) -> f64 {
        self.x*self.x + self.y*self.y
    }

    #[inline]
    pub fn get_cos(&self) -> f64 {
        self.x/self.get_magnitude()
    }

    #[inline]
    pub fn get_sin(&self) -> f64 {
        self.y/self.get_magnitude()
    }

    pub fn get_cos_sin(&self) -> (f64, f64) {
        let length = self.get_magnitude();
        (self.x/length,self.y/length)
    }

    pub fn get_unit_vector(&self) -> Self {
        let length = self.get_magnitude();
        Self::new(self.x/length,self.y/length)
    }

    // Бесполезный мусор, так как делить на два нет смысла для сравнения площадей, лол
    pub fn get_square(&self, other: &Self) -> f64 {
        f64::abs(Self::cross_product(self, other)) / 2.
    }

    #[inline]
    pub fn get_double_square(&self, other: &Self) -> f64 {
        f64::abs(Self::cross_product(self, other))
    }

    #[inline]
    pub fn get_cos_between_vectors(&self, other: &Self) -> f64 {
        Self::dot(self, other)/(self.get_magnitude()*other.get_magnitude())
    }

    #[inline]
    pub fn get_sin_between_vectors(&self, other: &Self) -> f64 {
        Self::cross(self, other)/(self.get_magnitude()*other.get_magnitude())
    }

    #[inline]
    pub fn project_vector_on_vector(&self, other: &Self) -> Self {
        let dot = Self::dot(self, other);
        let denominator = Self::new(other.get_squared_magnitude() * other.x, other.get_squared_magnitude() * other.y);

        Self::new(dot / denominator.x, dot / denominator.y)
    }
    
    #[inline]
    pub fn project_vector_on_axis(&self, unit_vector: &Self) -> Self {
        let dot = Self::dot(self, unit_vector);
        
        Self::new(unit_vector.x * dot, unit_vector.y * dot)
    }

}
