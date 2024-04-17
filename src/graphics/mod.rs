pub mod app;

use glium::IndexBuffer;
use crate::json::default_json;
use crate::defs::{Building, synthetic};


#[derive(Copy, Clone, Debug)]
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

pub enum TransformAction {
    Increase,
    Reduce,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    RotateRight,
    RotateLeft,
    Resize,
}


#[derive(PartialEq)]
pub enum DisplayType {
    TrianglesFill,
    TrianglesFillLines,
    Triangles,
    TrianglesLines,
    Lines,
    ObjectSpawn,
}


impl DisplayType {
    pub fn switch(&mut self) {
        *self = match self {
            Self::TrianglesFill => Self::TrianglesFillLines,
            Self::TrianglesFillLines => Self::Triangles,
            Self::Triangles => Self::TrianglesLines,
            Self::TrianglesLines => Self::Lines,
            Self::Lines => Self::ObjectSpawn,
            Self::ObjectSpawn => Self::TrianglesFill,
        }
    }
}


pub fn get_triangle_indices(buildings: &Vec<Building>) -> Vec<u16> {
    let mut indices = Vec::<u16>::new();
    let mut index = 0;

    for building in buildings {
        let last_iter = building.sides.len() - 1;
        let penultimate_iter = building.sides.len() - 2;
        let init_index = index;

        'point_loop: for i in 0..building.sides.len() {
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

            for j in i..building.sides.len() {
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
        let last_iter = building.sides.len() - 1;

        'point_loop: for i in 0..building.sides.len() {
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
        
        for vertex in &building.sides {
            points.append(&mut vec![vertex.position.x, vertex.position.y]);
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


pub fn get_synthetic_triangulation_indices(figures: &Vec<Box<dyn synthetic::SyntheticData>>) -> (Vec<Vertex>, Vec<u16>) {
    let mut indices = Vec::<u16>::new();
    let mut vertices = Vec::<Vertex>::new();
    let mut sum = 0;

    for figure in figures {
        match figure.get_data() {
            synthetic::SyntheticVariant::Circle((x, y), r) => {
                const SEGMENTS: f64 = 25.;
                const DELTA_PHI: f64 = std::f64::consts::PI / SEGMENTS;
                let mut phi = 0.;
                
                vertices.push(Vertex { position: [x, y] });

                while phi < 2. * std::f64::consts::PI + DELTA_PHI {
                    let x = x + r * f64::cos(phi);
                    let y = y + r * f64::sin(phi);

                    vertices.push(Vertex { position: [x, y] });

                    phi += DELTA_PHI;
                }

                for index in 1..SEGMENTS as u16 * 2 + 1 {
                    indices.append(&mut vec![sum as u16, sum as u16 + index, sum as u16 + index + 1]);
                }

                println!("{vertices:?}");

                sum += SEGMENTS as usize * 2;
            },
            synthetic::SyntheticVariant::Rectangle((lu_x, lu_y), (rd_x, rd_y)) => {
                vertices.append(&mut vec![
                    Vertex { position: [lu_x, lu_y] }, Vertex { position: [rd_x, lu_y] }, Vertex { position: [rd_x, rd_y] }, Vertex { position: [lu_x, rd_y] }
                ]);
                indices.append(&mut vec![sum as u16, sum as u16 + 1, sum as u16 + 2, sum as u16, sum as u16 + 3, sum as u16 + 2]);

                sum += 4;
            },
            synthetic::SyntheticVariant::Segment((p0_x, p0_y), (p1_x, p1_y)) => {
                vertices.append(&mut vec![Vertex { position: [p0_x, p0_y] }, Vertex { position: [p1_x, p1_y] }]);
                indices.append(&mut vec![sum as u16, sum as u16 + 1, sum as u16]);

                sum += 2;
            },
        }
    }

    (vertices, indices)
}
