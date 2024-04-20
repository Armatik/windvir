use crate::{App, graphics, etc};


impl App {
    pub fn get_buildings_vertices(&self) -> Vec<graphics::Vertex> {
        let mut shape = Vec::<graphics::Vertex>::with_capacity(self.buildings.len());

        for build in &self.buildings {
            for side in &build.sides {
                shape.push(graphics::Vertex { position: etc::vec_to_arr::<f32, 2>(vec![side.position.x, side.position.y]) })
            }
        }

        shape
    }

    pub fn init_field(
        &self,
        is_field_default: bool,
        display: &glium::Display
    ) -> Result<glium::VertexBuffer<graphics::ShaderVertex>, glium::vertex::BufferCreationError> {
        const CORRECTION_FACTOR: f32 = 1000.;
        let field_size = self.p_j.reverse_field_size; 
        let default_width = self.p_j.resolution.width as f32 / CORRECTION_FACTOR;
        let default_height = self.p_j.resolution.height as f32 / CORRECTION_FACTOR;
        let rgb = if is_field_default {
            let default_color = self.p_j.background_color;

            [[default_color.r, default_color.g, default_color.b]; 4]
        } else {
            [[-1., 0., 0.], [0., 1., 0.], [0., 0., 1.], [1., 1., 0.]]
        };

        glium::VertexBuffer::new(display, &vec![
            graphics::ShaderVertex { position: [-1. / field_size * default_width, 1. / field_size * default_height], color: rgb[0] },
            graphics::ShaderVertex { position: [1. / field_size * default_width, 1. / field_size * default_height], color: rgb[1] },
            graphics::ShaderVertex { position: [1. / field_size * default_width, -1. / field_size * default_height], color: rgb[2] },
            graphics::ShaderVertex { position: [-1. / field_size * default_width, -1. / field_size * default_height], color: rgb[3] },
        ])
    }
}
