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