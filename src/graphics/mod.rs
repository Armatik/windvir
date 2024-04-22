pub mod app;

use crate::json::default_json;
use crate::defs::Building;
use image::io::Reader;


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vertex {
    pub position: [f32; 2],
}


implement_vertex!(Vertex, position);



#[derive(Clone, Copy)]
pub struct ShaderVertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}


implement_vertex!(ShaderVertex, position, color);


pub const VERTEX_SHADER_PATH: &str = "src/graphics/shaders/vertex_shader.vert";
pub const COLOR_SHADER_PATH: &str = "src/graphics/shaders/color_shader.vert";
pub const RANDOM_COLOR_SHADER_PATH: &str = "src/graphics/shaders/random_color_shader.vert";
pub const FIELD_VERTEX_SHADER_PATH: &str = "src/graphics/shaders/field_vertex_shader.vert";
pub const FIELD_COLOR_SHADER_PATH: &str = "src/graphics/shaders/field_color_shader.vert";
const ICON_PATH: &str = "icons/icon.png";


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
    Default,
}


#[derive(PartialEq, Clone)]
pub enum DisplayType {
    TrianglesFill,
    TrianglesFillLines,
    Lines,
    ObjectSpawn,
}


impl DisplayType {
    pub fn switch(&mut self) {
        *self = match self {
            Self::TrianglesFill => Self::Lines,
            Self::TrianglesFillLines => Self::Lines,
            Self::Lines => Self::ObjectSpawn,
            Self::ObjectSpawn => Self::TrianglesFill,
        }
    }

    pub fn change_visible_regime(&mut self) {
        *self = match self {
            Self::TrianglesFill => Self::TrianglesFillLines,
            Self::TrianglesFillLines => Self::TrianglesFill,
            _ => {
                log::warn!("У данного режима отсутвует другой вид отображения!");

                return;
            },
        }
    }
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
        
        let mut min = usize::MAX;
        let mut max = usize::MIN;
        
        let mut result_building = building.triangulate().iter().map(|x| {
                if *x < min {
                    min = *x;
                } else if *x > max {
                    max = *x;
                }

                (*x + sum) as u16
            }).collect::<Vec<u16>>();
        result.append(&mut result_building);

        sum += max - min + 1;
    }

    result
}


pub fn get_icon() -> Result<(Vec<u8>, (u32, u32)), Box<dyn std::error::Error>> {
    let img = Reader::open(ICON_PATH)?.decode()?;
    let width = img.width();
    let height = img.width();

    Ok((img.as_bytes().to_vec(), (width, height)))
}
