pub mod synthetic;
pub mod app;

use std::ops::{Add,Sub};


#[derive(Clone, Debug, Default, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}


impl Point {
    pub fn new(x: f32, y: f32) -> Self {
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
    pub fn new(build: Vec<Vec<f32>>) -> Self {
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

    pub fn triangulate(&self) -> Vec<usize> {
        let mut points = Vec::<f32>::with_capacity(self.sides.len()*2usize);
        for vertex in &self.sides {
            points.push(vertex.position.x);
            points.push(vertex.position.y);
        }
        earcutr::earcut(&points, &[], 2).unwrap()
    }

    pub fn get_square(&self) -> f32 {
        let triangulation_indices = self.triangulate();
        let mut square = 0.;
        for i in 0..triangulation_indices.len()/3usize {
            square +=  PositionVector::get_square(&(&self.sides[i].position - &self.sides[i + 1usize].position),&(&self.sides[i].position - &self.sides[i + 2usize].position));
        }
        square
    }

    pub fn get_double_square(&self) -> f32 {
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
    pub fn cross_product(&self, other: &Self) -> f32 {
        PositionVector::cross_product(&self.offset, &other.offset)
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


#[derive(Clone, Debug, Default, PartialEq)]
pub struct PositionVector {
    pub x: f32,
    pub y: f32,
}


impl PositionVector {

    pub fn new(x: f32, y: f32) -> Self {
        Self { 
            x,
            y
        }
    }

    pub fn center_point(&self, other: &Self) -> Self {
        Self::new((self.x + other.x) / 2., (self.y + other.y) / 2.)
    }

    #[inline]
    pub fn cross_product(&self, other: &Self) -> f32 {
        other.x*self.y - self.x*other.y
    }

    #[inline]
    pub fn dot_product(&self, other: &Self) -> f32 {
        other.x*self.x + self.x*other.x
    }

    // Если можно не использовать, лучше не использовать
    #[inline]
    pub fn get_length(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y)
    }

    #[inline]
    pub fn get_squared_length(&self) -> f32 {
        self.x*self.x + self.y*self.y
    }

    #[inline]
    pub fn get_cos(&self) -> f32 {
        self.x/self.get_length()
    }

    #[inline]
    pub fn get_sin(&self) -> f32 {
        self.y/self.get_length()
    }

    pub fn get_cos_sin(&self) -> (f32, f32) {
        let length = self.get_length();
        (self.x/length,self.y/length)
    }

    // Бесполезный мусор, так как делить на два нет смысла для сравнения площадей, лол
    #[inline]
    pub fn get_square(&self, other: &Self) -> f32 {
        f32::abs(Self::cross_product(self, other)) / 2.
    }

    #[inline]
    pub fn get_double_square(&self, other: &Self) -> f32 {
        Self::cross_product(self, other).abs()
    }

    #[inline]
    pub fn get_cos_between_vectors(&self, other: &Self) -> f32 {
        Self::dot_product(self, other)/(self.get_length()*other.get_length())
    }

    #[inline]
    pub fn get_sin_between_vectors(&self, other: &Self) -> f32 {
        Self::cross_product(self, other)/(self.get_length()*other.get_length())
    }
}
