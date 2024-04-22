use crate::{App, graphics, etc, defs::synthetic};


pub struct FigureIndices<T> where T: glium::index::Index {
    pub buildings_indices_line: glium::IndexBuffer<T>,
    pub buildings_indices_triangulate: glium::IndexBuffer<T>,
    pub field_indices: glium::IndexBuffer<T>,
}


impl<T> FigureIndices<T> where T: glium::index::Index {
    pub fn new(
        buildings_indices_line: glium::IndexBuffer<T>,
        buildings_indices_triangulate: glium::IndexBuffer<T>,
        field_indices: glium::IndexBuffer<T>,
    ) -> Self {
        Self {
            buildings_indices_line,
            buildings_indices_triangulate,
            field_indices,
        }
    }
}


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
        is_color_rainbow: bool,
        display: &glium::Display,
    ) -> Result<glium::VertexBuffer<graphics::ShaderVertex>, glium::vertex::BufferCreationError> {
        const CORRECTION_FACTOR: f32 = 1000.;
        let field_size = self.p_j.reverse_field_size; 
        let default_width = self.p_j.resolution.width as f32 / CORRECTION_FACTOR;
        let default_height = self.p_j.resolution.height as f32 / CORRECTION_FACTOR;
        let rgb = if is_color_rainbow {
            [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.], [1., 1., 0.]]
        } else {
            let default_color = self.p_j.background_color;

            [[default_color.r, default_color.g, default_color.b]; 4]
        };

        glium::VertexBuffer::new(display, &vec![
            graphics::ShaderVertex { position: [-1. / field_size * default_width, 1. / field_size * default_height], color: rgb[0] },
            graphics::ShaderVertex { position: [1. / field_size * default_width, 1. / field_size * default_height], color: rgb[1] },
            graphics::ShaderVertex { position: [1. / field_size * default_width, -1. / field_size * default_height], color: rgb[2] },
            graphics::ShaderVertex { position: [-1. / field_size * default_width, -1. / field_size * default_height], color: rgb[3] },
        ])
    }

    pub fn init_indices(&self, display: &glium::Display) -> Result<FigureIndices<u16>, glium::index::BufferCreationError> {
        let indices_line = graphics::get_line_indices(&self.buildings);
        let indices_triangulate = graphics::get_triangulation_indices(&self.buildings);

        let indices_line = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::LinesList,
            &indices_line,
        )?;
        let indices_triangulate = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &indices_triangulate,
        )?;
        let indices_field = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &vec![0_u16, 1, 2, 0, 2, 3],
        )?;
        let indices = FigureIndices::new(indices_line, indices_triangulate, indices_field);

        Ok(indices)
    }

    pub fn is_start_polygon(&self) -> bool {
        self.synthetic_datas_points.len() == 0
    }

    pub fn is_polygon(&self) -> bool {
        return match self.synthetic_data.back() {
            Some(data) => data.get_data_simply() == synthetic::SimplySyntheticVariant::Polygon,
            None => false,
        }
    }

    pub fn spawn_point(&mut self, is_end_of_polygon: bool) {
        if self.synthetic_datas_points.get(0).is_none() {
            self.synthetic_datas_points.push(super::Point::new(self.aim.x, self.aim.y));

            log::info!("Первая точка была успешно отмечена!");
        } else {
            match self.synthetic_data.back().unwrap().get_data_simply() {
                synthetic::SimplySyntheticVariant::Rectangle | synthetic::SimplySyntheticVariant::Segment => {
                    self.synthetic_datas_points.push(super::Point::new(self.aim.x, self.aim.y));
                    self.synthetic_data.back_mut().unwrap().set_points(self.synthetic_datas_points.clone())
                        .expect("Произошла ошибка! Данные точки начали задаваться для окружности!");
                },
                synthetic::SimplySyntheticVariant::Polygon => {
                    if is_end_of_polygon {
                        self.synthetic_data.back_mut().unwrap().set_points(self.synthetic_datas_points.clone())
                            .expect("Произошла ошибка! Данные точки начали задаваться для окружности!");
                    } else {
                        self.synthetic_datas_points.push(super::Point::new(self.aim.x, self.aim.y));

                        log::info!("Точка для многоугольника была успешно задана!");
                        return;
                    }
                },
                _ => {},
            }
            
            self.synthetic_datas_points = Vec::new();
            log::info!("Фигура была успешно задана!");
        }
    }

    pub fn spawn_circle(&mut self, value: f32) {
        if let Some(figure) = self.synthetic_data.back() {
            if figure.is_value_default() {
                let size = self.p_j.aim.aim_adjusment * value;
                self.synthetic_data.back_mut().unwrap()
                    .set_value(synthetic::SyntheticVariant::Circle(self.aim.clone(), size));

                log::info!("Окружность размером {size} была успешно задана!");
            }
        }
    }

    pub fn define_figure<'a, F>(&'a mut self, figure: F, log_info: &'a str) where F: synthetic::SyntheticData + 'static {
        log::info!("{log_info}");
        
        self.synthetic_data.push_back(Box::new(figure));
    }
}
