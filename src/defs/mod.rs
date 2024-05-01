pub mod synthetic;
pub mod app;
pub mod error;

use std::ops::{Add,Sub};


#[derive(Clone, Debug, Default, PartialEq)]
pub struct Point<T> where T: num::Float + Default {
    pub x: T,
    pub y: T,
}


impl<T> Point<T> where T: num::Float + Default {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn center_point(&self, other: &Self) -> Self {
        let half = num::cast(2.).unwrap();

        Self::new((self.x + other.x) / half, (self.y + other.y) / half)
    }

    pub fn is_point_default(&self) -> bool {
        *self == Self::default()
    }
}


#[derive(Debug, PartialEq)]
pub struct Building {
    pub start_point: PositionVector<f64>,
    pub end_point: PositionVector<f64>,
    pub sides: Vec<Vector<f64>>
}


impl Building {
    pub fn new(build: Vec<Vec<f64>>) -> Self {
        let vertex = build.iter().map(
            |x| Vector::new(
                        PositionVector::new(x[0], x[1]),PositionVector::new(0., 0.)
                    )).collect::<Vec<Vector<f64>>>();
        
        Self {
            start_point: PositionVector::new(0.,0.),
            end_point: PositionVector::new(0.,0.),
            sides: vertex,
        }
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
pub struct Vector<T> where T: num::Float + Default {
    pub position: PositionVector<T>,
    pub offset: PositionVector<T>,
}


impl<T> Vector<T> where T: num::Float + Default {
    pub fn new(position: PositionVector<T>, offset: PositionVector<T>) -> Self {
        Self { 
            position,
            offset
        }
    }

    #[inline]
    pub fn cross_product(&self, other: &Self) -> T {
        PositionVector::cross_product(&self.offset, &other.offset)
    }

    pub fn get_right_normal(&self) -> PositionVector<T> {
        PositionVector { 
            x: self.offset.y,
            y: -self.offset.x
        }
    }

    pub fn get_left_normal(&self) -> PositionVector<T> {
        PositionVector { 
            x: -self.offset.y,
            y: self.offset.x
        }
    }
}


impl<T> Sub for Point<T> where T: num::Float + Default {
    type Output = PositionVector<T>;

    fn sub(self, end_point: Self) -> Self::Output {
        PositionVector::new(
            end_point.x - self.x, 
            end_point.y - self.y
        )
    }
}


impl<T> Add for &Point<T> where T: num::Float + Default {
    type Output = PositionVector<T>;

    fn add(self, other: Self) -> Self::Output {
        PositionVector::new(
            self.x + other.x, 
            self.y + other.y
        )
    }
}


impl<T> Sub for &Point<T> where T: num::Float + Default {
    type Output = PositionVector<T>;

    fn sub(self, start_point: Self) -> Self::Output {
        PositionVector::new(
            self.x - start_point.x, 
            self.y - start_point.y
        )
    }
}


impl<T> Sub for &Vector<T> where T: num::Float + Default {
    type Output = PositionVector<T>;

    fn sub(self, subtractor_vector: Self) -> Self::Output {
        &self.offset - &subtractor_vector.offset
    }
}


impl<T> Sub for &PositionVector<T> where T: num::Float + Default {
    type Output = PositionVector<T>;

    fn sub(self, subtractor_vector: Self) -> Self::Output {
        PositionVector::new(
            self.x - subtractor_vector.x,
            self.y - subtractor_vector.y
        )
    }
}


#[derive(Clone, Debug, Default, PartialEq)]
pub struct PositionVector<T> where T: num::Float + Default {
    pub x: T,
    pub y: T,
}


impl<T> PositionVector<T> where T: num::Float + Default {

    pub fn new(x: T, y: T) -> Self {
        Self { 
            x,
            y
        }
    }

    pub fn center_point(&self, other: &Self) -> Self {
        let half = num::cast(2.).unwrap();

        Self::new((self.x + other.x) / half, (self.y + other.y) / half)
    }

    #[inline]
    pub fn cross_product(&self, other: &Self) -> T {
        other.x*self.y - self.x*other.y
    }

    #[inline]
    pub fn dot_product(&self, other: &Self) -> T {
        other.x*self.x + self.x*other.x
    }

    // Если можно не использовать, лучше не использовать
    #[inline]
    pub fn get_length(&self) -> T {
        T::sqrt(self.x * self.x + self.y * self.y)
    }

    #[inline]
    pub fn get_squared_length(&self) -> T {
        self.x*self.x + self.y*self.y
    }

    #[inline]
    pub fn get_cos(&self) -> T {
        self.x/self.get_length()
    }

    #[inline]
    pub fn get_sin(&self) -> T {
        self.y/self.get_length()
    }

    pub fn get_cos_sin(&self) -> (T, T) {
        let length = self.get_length();
        (self.x/length,self.y/length)
    }

    // Бесполезный мусор, так как делить на два нет смысла для сравнения площадей, лол
    #[inline]
    pub fn get_square(&self, other: &Self) -> T {
        T::abs(Self::cross_product(self, other)) / num::cast(2.).unwrap()
    }

    #[inline]
    pub fn get_double_square(&self, other: &Self) -> T {
        T::abs(Self::cross_product(self, other))
    }

    #[inline]
    pub fn get_cos_between_vectors(&self, other: &Self) -> T {
        Self::dot_product(self, other)/(self.get_length()*other.get_length())
    }

    #[inline]
    pub fn get_sin_between_vectors(&self, other: &Self) -> T {
        Self::cross_product(self, other)/(self.get_length()*other.get_length())
    }
}
