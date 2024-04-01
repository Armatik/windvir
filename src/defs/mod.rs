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


pub trait SyntheticData {
    fn get_data(&self) -> Vec<f64>;
}


#[derive(Default)]
pub struct Circle {
    radius: f64,
}


impl Circle {
    pub fn new(radius: f64) -> Self {
        Self {
            radius,
        }
    }
}


impl SyntheticData for Circle {
    fn get_data(&self) -> Vec<f64> {
        vec![self.radius]
    }
}


#[derive(Default)]
pub struct Rectangle {
    width: f64,
    height: f64,
}


impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width,
            height,
        }
    }
}


impl SyntheticData for Rectangle {
    fn get_data(&self) -> Vec<f64> {
        vec![self.width, self.height]
    }
}


#[derive(Default)]
pub struct Segment {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
}


impl Segment {
    pub fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
        }
    }
}


impl SyntheticData for Segment {
    fn get_data(&self) -> Vec<f64> {
        vec![self.x0, self.y0, self.x1, self.y1]
    }
}
