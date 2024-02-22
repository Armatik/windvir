#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f64; 2],
}


pub const VERTEX_SHADER_PATH: &str = "src/graphics/vertex_shader.vert";
pub const COLOR_SHADER_PATH: &str = "src/graphics/color_shader.vert";
pub const OFFSET_X: f32 = -45.395;
pub const OFFSET_Y: f32 = -52.293;
pub const BACKGROUND_R: f32 = 0.8;
pub const BACKGROUND_G: f32 = 0.98;
pub const BACKGROUND_B: f32 = 0.988;
pub const NOT_TRANSPARENT: f32 = 1.;
pub const ASPECT_RATIO_WIDTH: f32 = 16.;
pub const ASPECT_RATIO_HEIGHT: f32 = 9.;


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
    Triangles,
    TrianglesLines,
    Lines,
}


impl DisplayType {
    pub fn switch(&mut self) {
        *self = match self {
            Self::Triangles => Self::TrianglesLines,
            Self::TrianglesLines => Self::Lines,
            Self::Lines => Self::Triangles,
        }
    }
}


pub fn get_triangle_indices(buildings: &Vec<Vec<Vec<f64>>>) -> Vec<u16> {
    let mut indices = Vec::<u16>::new();
    let mut index = 0;

    for building in buildings {
        let last_iter = building.len() - 1;
        let penultimate_iter = building.len() - 2;
        let init_index = index;

        'point_loop: for i in 0..building.len() {
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

            for j in i..building.len() {
                indices.append(&mut vec![index, index + 1, init_index + j as u16]);
            }

            index += 1;
        }
    }

    indices
}


pub fn get_line_indices(buildings: &Vec<Vec<Vec<f64>>>) -> Vec<u16> {
    let mut indices = Vec::<u16>::new();
    let mut index = 0;

    for building in buildings {
        let init_index = index;

        for _ in building {
            indices.append(&mut vec![index, index + 1]);
            index += 1;
        }

        indices.append(&mut vec![index, init_index]);
        index += 1;
    }

    indices
}
