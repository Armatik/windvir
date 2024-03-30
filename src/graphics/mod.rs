use glium::IndexBuffer;
use crate::json::default_json;
use crate::ffi;


#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f64; 2],
}


pub type IndciesTriangles = IndexBuffer<u16>;
pub type IndciesLines = IndexBuffer<u16>;


pub const VERTEX_SHADER_PATH: &str = "src/graphics/vertex_shader.vert";
pub const COLOR_SHADER_PATH: &str = "src/graphics/color_shader.vert";


pub struct Camera {
    pub scale: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub theta: f32,
    pub transform_matrix: [[f32; 4]; 4],
    pub display_type: DisplayType,
}


impl Default for Camera {
    fn default() -> Self {
        let p_j = default_json::PersistentJ::default();

        Self {
            scale: p_j.scale,
            offset_x: p_j.map_offset.x,
            offset_y: p_j.map_offset.y,
            theta: p_j.theta,
            transform_matrix: [
                [f32::cos(p_j.theta) * p_j.scale * p_j.resolution.height as f32 / p_j.resolution.width as f32, -f32::sin(p_j.theta) * p_j.scale , 0., 0.],
                [f32::sin(p_j.theta) * p_j.scale * p_j.resolution.height as f32 / p_j.resolution.width as f32, f32::cos(p_j.theta) * p_j.scale, 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ],
            display_type: DisplayType::TrianglesFill,
        }
    }
}


#[derive(Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}


impl Point {
    pub fn new(_x: f64, _y: f64) -> Self {
        Self {
            x: _x,
            y: _y,
        }
    }

    pub fn repr_rust(point: ffi::PointC) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
}


#[derive(Debug)]
pub struct Building {
    pub center: Point,
    pub radius: f64,
    pub points: Vec<Point>,
}


impl Building {
    pub fn new(build: Vec<Vec<f64>>) -> Self {
        log::warn!("Центры и радиусы для отдельных зданий пока что не задаются. Нуждается в исправлении!");
        let vertex = build.iter().map(|x| Point::new(x[0], x[1])).collect::<Vec<Point>>();
        
        Self {
            center: Point::new(0., 0.),
            radius: 0.,
            points: vertex,
        }
    }

    pub fn repr_rust(building: ffi::BuildingC) -> Self {
        let mut buildings_vertex = Vec::<Point>::with_capacity(building.len_vertex as usize);
        let building_vertex = unsafe { Vec::from_raw_parts(
            building.points, building.len_vertex as usize, building.len_vertex as usize
        ) };

        for vertex in building_vertex {
            buildings_vertex.push(Point::repr_rust(vertex));
        }

        Self {
            center: Point::new(building.center.x, building.center.y),
            radius: building.radius,
            points: buildings_vertex,
        }
    }
}


pub enum TransformAction {
    Increase,
    Reduce,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    RotateRight,
    RotateLeft,
}


pub enum DisplayType {
    TrianglesFill,
    TrianglesFillLines,
    Triangles,
    TrianglesLines,
    Lines,
}


impl DisplayType {
    pub fn switch(&mut self) {
        *self = match self {
            Self::TrianglesFill => Self::TrianglesFillLines,
            Self::TrianglesFillLines => Self::Triangles,
            Self::Triangles => Self::TrianglesLines,
            Self::TrianglesLines => Self::Lines,
            Self::Lines => Self::TrianglesFill,
        }
    }
}


pub fn get_triangle_indices(buildings: &Vec<Building>) -> Vec<u16> {
    let mut indices = Vec::<u16>::new();
    let mut index = 0;

    for building in buildings {
        let last_iter = building.points.len() - 1;
        let penultimate_iter = building.points.len() - 2;
        let init_index = index;

        'point_loop: for i in 0..building.points.len() {
            if i == last_iter {
                indices.append(&mut vec![index, init_index, init_index + 1]);
                index += 1;

                continue 'point_loop;
            }

            if i == penultimate_iter {
                indices.append(&mut vec![index, index + 1, init_index]);
                index += 1;

                continue 'point_loop;
            }

            for j in i..building.points.len() {
                indices.append(&mut vec![index, index + 1, init_index + j as u16]);
            }

            index += 1;
        }
    }

    indices
}


pub fn get_line_indices(buildings: &Vec<Building>) -> Vec<u16> {
    let mut indices = Vec::<u16>::new();
    let mut index = 0;

    for building in buildings {
        let init_index = index;
        let last_iter = building.points.len() - 1;

        'point_loop: for i in 0..building.points.len() {
            if i == last_iter {
                indices.append(&mut vec![index, init_index]);
                index += 1;

                continue 'point_loop;
            }

            indices.append(&mut vec![index, index + 1]);
            index += 1;
        }
    }

    indices
}


pub fn get_triangulation_indices(buildings: &Vec<Building>) -> Vec<u16> {
    let mut result = Vec::<u16>::new();
    let mut sum = 0;
    
    for building in buildings {
        let mut points = Vec::<f64>::new();
        let mut min = usize::MAX;
        let mut max = usize::MIN;
        
        for vertex in &building.points {
            points.append(&mut vec![vertex.x, vertex.y]);
        }

        let mut result_building = earcutr::earcut(&points, &[], 2).unwrap().iter().map(|x| {
                if *x < min {
                    min = *x;
                } else if *x > max {
                    max = *x;
                }

                (*x + sum) as u16
            }).collect::<Vec<u16>>();
        result.append(&mut result_building);

        sum += max - min + 2;
    }

    result
}
