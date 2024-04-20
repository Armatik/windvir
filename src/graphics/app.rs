use crate::{
    defs::{self, app}, json::geojson, App, control,
};
use std::fs;
use glium::{
    Display,
    Surface,
    VertexBuffer,
};


pub struct Shaders {
    pub default_shader: glium::Program,
    pub random_shader: glium::Program,
    pub field_shader: glium::Program,
}


impl Shaders {
    pub fn new(default_shader: glium::Program, random_shader: glium::Program, field_shader: glium::Program) -> Self {
        Self {
            default_shader,
            random_shader,
            field_shader,
        }
    }
}


impl App {
    pub fn trans_persistent(p_g: &geojson::PersistentG) -> Vec<defs::Building> {
        let mut buildings = Vec::<defs::Building>::with_capacity(p_g.features.len());
        for building in &p_g.features {
            let mut temp_building = building.geometry.coordinates[0][0].clone();
            temp_building.pop();
            buildings.push(defs::Building::new(temp_building));
        }

        buildings
    }

    pub fn transform_map(&mut self, action: super::TransformAction) {
        let transform = |mat: &mut [[f32; 4]; 4], theta: f32, scale: f32| {
            mat[0][0] = f32::cos(theta) * scale * self.window_size.1 / self.window_size.0;
            mat[0][1] = -f32::sin(theta) * scale;
            mat[1][0] = f32::sin(theta) * scale * self.window_size.1 / self.window_size.0;
            mat[1][1] = f32::cos(theta) * scale;
        };

        match action {
            super::TransformAction::Increase => {
                self.cam.scale += self.p_j.movement.scale;
                transform(&mut self.cam.transform_matrix, self.cam.theta, self.cam.scale);
            },
            super::TransformAction::Reduce => {
                if self.cam.scale - self.p_j.movement.scale < 0. {
                    return;
                }

                self.cam.scale -= self.p_j.movement.scale;
                transform(&mut self.cam.transform_matrix, self.cam.theta, self.cam.scale);
            },
            super::TransformAction::MoveUp => {
                self.cam.offset_x += self.p_j.movement.x*f32::sin(self.cam.theta)*self.window_size.1 / self.window_size.0;
                self.cam.offset_y -= self.p_j.movement.y*f32::cos(self.cam.theta);
            },
            super::TransformAction::MoveDown => {
                self.cam.offset_x -= self.p_j.movement.x*f32::sin(self.cam.theta)*self.window_size.1 / self.window_size.0;
                self.cam.offset_y += self.p_j.movement.y*f32::cos(self.cam.theta);
            },
            super::TransformAction::MoveLeft => {
                self.cam.offset_x += self.p_j.movement.x*f32::cos(self.cam.theta)*self.window_size.1 / self.window_size.0;
                self.cam.offset_y += self.p_j.movement.y*f32::sin(self.cam.theta);
            },
            super::TransformAction::MoveRight => {
                self.cam.offset_x -= self.p_j.movement.x*f32::cos(self.cam.theta)*self.window_size.1 / self.window_size.0;
                self.cam.offset_y -= self.p_j.movement.y*f32::sin(self.cam.theta);
            },
            super::TransformAction::RotateLeft => {
                self.cam.theta += self.p_j.movement.theta;
                transform(&mut self.cam.transform_matrix, self.cam.theta, self.cam.scale);
            },
            super::TransformAction::RotateRight => {
                self.cam.theta -= self.p_j.movement.theta;
                transform(&mut self.cam.transform_matrix, self.cam.theta, self.cam.scale);
            },
            super::TransformAction::Resize => transform(&mut self.cam.transform_matrix, self.cam.theta, self.cam.scale),
            super::TransformAction::Default => {
                self.cam.offset_x = self.p_j.map_offset.x;
                self.cam.offset_y = self.p_j.map_offset.y;
                self.cam.theta = self.p_j.theta;
                self.cam.scale = self.p_j.scale;
                transform(&mut self.cam.transform_matrix, self.cam.theta, self.cam.scale);
            },
        }
    }

    pub fn render_frame<T>(
        &self,
        display: &Display,
        positions: &VertexBuffer<super::Vertex>,
        field_positions: &VertexBuffer<super::ShaderVertex>,
        indices: &app::FigureIndices<T>,
        shaders: &Shaders,
    ) where T: glium::index::Index {
        let mut target = display.draw();
        target.clear_color(
            self.p_j.background_color.r,
            self.p_j.background_color.g,
            self.p_j.background_color.b,
            self.p_j.background_color.a,
        );
        let multisampling_on = self.p_j.graphics.multisampling_on;
        let dithering_on = self.p_j.graphics.dithering_on;
        
        let (polygon_mode, indices_buildings) = match self.cam.display_type.clone() {
            super::DisplayType::TrianglesFill => (glium::draw_parameters::PolygonMode::Fill, &indices.buildings_indices_triangulate),
            super::DisplayType::TrianglesFillLines => (glium::draw_parameters::PolygonMode::Line, &indices.buildings_indices_triangulate),
            super::DisplayType::Lines => (glium::draw_parameters::PolygonMode::Line, &indices.buildings_indices_line),
            super::DisplayType::ObjectSpawn => {
                let mut params = glium::DrawParameters {
                    polygon_mode: glium::draw_parameters::PolygonMode::Fill,
                    multisampling: multisampling_on,
                    dithering: dithering_on,
                    smooth: Some(glium::draw_parameters::Smooth::Nicest),
                    point_size: None,
                    ..Default::default()
                };
                let mut rgb = (f32::default(), f32::default(), f32::default());

                for figure in &self.synthetic_data {
                    rgb = figure.get_rgb();
                    
                    let uniforms = uniform! {
                        matrix: self.cam.transform_matrix,
                        x_off: self.cam.offset_x,
                        y_off: self.cam.offset_y,
                        r_rand: rgb.0,
                        g_rand: rgb.1,
                        b_rand: rgb.2,
                    };
                    
                    let (positions, indices) = figure.get_vertices_and_indices();
                    let primitive = figure.get_primitive();
                    let polygon_mode = match primitive {
                        glium::index::PrimitiveType::TrianglesList => glium::draw_parameters::PolygonMode::Fill,
                        glium::index::PrimitiveType::LineLoop => glium::draw_parameters::PolygonMode::Line,
                        _ => unreachable!("Попался невозможный примитив!"),
                    };
                    params.polygon_mode = polygon_mode;
                    let positions = glium::VertexBuffer::new(display, &positions)
                        .expect("Ошибка! Не удалось создать буффер вершин для объекта!");

                    if let Some(indices) = indices {
                        let indices = glium::IndexBuffer::new(
                            display,
                            primitive,
                            &indices,
                        ).expect("Ошибка! Не удалось создать буффер индексов для объекта!");
                        target.draw(&positions, &indices, &shaders.random_shader, &uniforms, &params)
                            .expect("Ошибка! Не удалось отрисовать синтетическую фигуру!");
                    } else {
                        target.draw(
                            &positions,
                            &glium::index::NoIndices(glium::index::PrimitiveType::LinesList),
                            &shaders.random_shader,
                            &uniforms,
                            &params
                        ).expect("Ошибка! Не удалось отрисовать синтетическую фигуру!");
                    }
                }

                params.polygon_mode = glium::draw_parameters::PolygonMode::Point;
                params.smooth = None;
                params.point_size = Some(self.p_j.aim.aim_size / 2.);
                let figure_point_uniforms = uniform! {
                    matrix: self.cam.transform_matrix,
                    x_off: self.cam.offset_x,
                    y_off: self.cam.offset_y,
                    r_rand: rgb.0,
                    g_rand: rgb.1,
                    b_rand: rgb.2,
                };
                let figure_points = glium::VertexBuffer::new(display, &[
                    super::Vertex { position: [self.synthetic_datas_point.x, self.synthetic_datas_point.y] },
                ]).expect("Ошибка! Не удалось задать позицию для прицела!");
                target.draw(
                    &figure_points,
                    &glium::index::NoIndices(glium::index::PrimitiveType::Points),
                    &shaders.random_shader,
                    &figure_point_uniforms,
                    &params,
                ).expect("Ошибка! Не удалось отрисовать точку для задания фигуры!");

                params.point_size = Some(self.p_j.aim.aim_size);
                let aim_uniforms = uniform! {
                    matrix: self.cam.transform_matrix,
                    x_off: self.cam.offset_x,
                    y_off: self.cam.offset_y,
                    r_rand: 1.0_f32,
                    g_rand: 0.0_f32,
                    b_rand: 0.0_f32,
                };
                
                let aim_position = glium::VertexBuffer::new(display, &[super::Vertex { position: [self.aim.x, self.aim.y] }])
                    .expect("Ошибка! Не удалось задать позицию для прицела!");
                target.draw(
                    &aim_position,
                    &glium::index::NoIndices(glium::index::PrimitiveType::Points),
                    &shaders.random_shader,
                    &aim_uniforms,
                    &params,
                ).expect("Ошибка! Не удалось отрисовать прицел!");

                target.finish()
                    .expect("Ошибка! Не удалось закончить отрисовку кадра!");

                return;
            },
        };

        
        let mut params = glium::DrawParameters {
            polygon_mode: glium::draw_parameters::PolygonMode::Fill,
            multisampling: multisampling_on,
            dithering: dithering_on,
            smooth: Some(glium::draw_parameters::Smooth::Nicest),
            ..Default::default()
        };
        let field_uniforms = uniform! {
            matrix: self.cam.transform_matrix,
            x_off: self.cam.offset_x - self.p_j.map_offset.x,
            y_off: self.cam.offset_y - self.p_j.map_offset.y,
        };
        target.draw(field_positions, &indices.field_indices, &shaders.field_shader, &field_uniforms, &Default::default())
            .expect("Ошибка! Не удлаось отрисовать поле!");
        params.polygon_mode = polygon_mode;
        let uniforms = uniform! {
            matrix: self.cam.transform_matrix,
            x_off: self.cam.offset_x,
            y_off: self.cam.offset_y,
        };
        target.draw(positions, indices_buildings, &shaders.default_shader, &uniforms, &params)
            .expect("Ошибка! Не удалось отрисовать объект(ы)!");
        target.finish()
            .expect("Ошибка! Не удалось закончить отрисовку кадра!");
    }

    pub fn move_aim(&mut self, action: control::MoveAim) {
        let speed = self.p_j.aim.aim_speed;

        match action {
            control::MoveAim::Top => self.aim.y += speed,
            control::MoveAim::Right => self.aim.x += speed,
            control::MoveAim::Left => self.aim.x -= speed,
            control::MoveAim::Down => self.aim.y -= speed,
            control::MoveAim::Default => self.aim = defs::Point::new(-self.p_j.map_offset.x, -self.p_j.map_offset.y),
        };
    }

    pub fn init_shaders(&self, display: &glium::Display) -> Result<Shaders, Box<dyn std::error::Error>> {
        let vertex_shader_src = fs::read_to_string(super::VERTEX_SHADER_PATH)?;
        let color_shader_src = fs::read_to_string(super::COLOR_SHADER_PATH)?;
        let random_color_shader_src = fs::read_to_string(super::RANDOM_COLOR_SHADER_PATH)?;
        let field_vertex_shader_src = fs::read_to_string(super::FIELD_VERTEX_SHADER_PATH)?;
        let field_color_shader_src = fs::read_to_string(super::FIELD_COLOR_SHADER_PATH)?;

        let program = glium::Program::from_source(
            display,
            &vertex_shader_src,
            &color_shader_src,
            None,
        )?;
        let random_program = glium::Program::from_source(
            display,
            &vertex_shader_src,
            &random_color_shader_src,
            None,
        )?;
        let field_program = glium::Program::from_source(
            display,
            &field_vertex_shader_src,
            &field_color_shader_src,
            None,
        )?;
        let shaders = Shaders::new(program, random_program, field_program);

        Ok(shaders)
    }
}
