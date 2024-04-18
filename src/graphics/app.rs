use crate::{
    defs, json::geojson, App, control,
};
use glium::{
    Display,
    Surface,
    VertexBuffer,
};


impl App {
    pub fn move_aim(&mut self, action: control::MoveAim) {
        let speed = self.p_j.aim.aim_speed;

        match action {
            control::MoveAim::Top => self.aim.1 += speed,
            control::MoveAim::Right => self.aim.0 += speed,
            control::MoveAim::Left => self.aim.0 -= speed,
            control::MoveAim::Down => self.aim.1 -= speed,
            control::MoveAim::Default => self.aim = (-self.p_j.map_offset.x as f64, -self.p_j.map_offset.y as f64),
        };
    } 

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

    pub fn render_frame(
        &self,
        display: &Display,
        positions: &VertexBuffer<super::Vertex>,
        indices: (&super::IndciesTriangles, &super::IndciesLines, &super::IndciesLines),
        program: &(glium::Program, glium::Program),
    ) {
        let mut target = display.draw();
        target.clear_color(
            self.p_j.background_color.r,
            self.p_j.background_color.g,
            self.p_j.background_color.b,
            self.p_j.background_color.a,
        );
        let multisampling_on = self.p_j.graphics.multisampling_on;
        let dithering_on = self.p_j.graphics.dithering_on;

        let (polygon_mode, indices) = match self.cam.display_type {
            super::DisplayType::TrianglesFill => (glium::draw_parameters::PolygonMode::Fill, indices.2),
            super::DisplayType::TrianglesFillLines => (glium::draw_parameters::PolygonMode::Line, indices.2),
            super::DisplayType::Triangles => (glium::draw_parameters::PolygonMode::Fill, indices.0),
            super::DisplayType::TrianglesLines => (glium::draw_parameters::PolygonMode::Line, indices.0),
            super::DisplayType::Lines => (glium::draw_parameters::PolygonMode::Line, indices.1),
            super::DisplayType::ObjectSpawn => {
                let mut params = glium::DrawParameters {
                    polygon_mode: glium::draw_parameters::PolygonMode::Fill,
                    multisampling: multisampling_on,
                    dithering: dithering_on,
                    smooth: Some(glium::draw_parameters::Smooth::Nicest),
                    point_size: None,
                    ..Default::default()
                };

                for figure in &self.synthetic_data {
                    let rgb = figure.get_rgb();
                    
                    let uniforms = uniform! {
                        matrix: self.cam.transform_matrix,
                        x_off: self.cam.offset_x,
                        y_off: self.cam.offset_y,
                        r_rand: rgb.0,
                        g_rand: rgb.1,
                        b_rand: rgb.2,
                    };
                    
                    let (positions, indices) = figure.get_vertices_and_indices();
                    let positions = glium::VertexBuffer::new(display, &positions)
                        .expect("Ошибка! Не удалось создать буффер вершин для объекта!");

                    if let Some(indices) = indices {
                        let indices = glium::IndexBuffer::new(
                            display,
                            glium::index::PrimitiveType::TrianglesList,
                            &indices,
                        ).expect("Ошибка! Не удалось создать буффер индексов для объекта!");
                        target.draw(&positions, &indices, &program.1, &uniforms, &params)
                            .expect("Ошибка! Не удалось отрисовать синтетическую фигуру!");
                    } else {
                        target.draw(
                            &positions,
                            &glium::index::NoIndices(glium::index::PrimitiveType::LinesList),
                            &program.1,
                            &uniforms,
                            &params
                        ).expect("Ошибка! Не удалось отрисовать синтетическую фигуру!");
                    }
                }

                let aim_uniforms = uniform! {
                    matrix: self.cam.transform_matrix,
                    x_off: self.cam.offset_x,
                    y_off: self.cam.offset_y,
                    r_rand: 1.0_f32,
                    g_rand: 0.0_f32,
                    b_rand: 0.0_f32,
                };
                params.polygon_mode = glium::draw_parameters::PolygonMode::Point;
                params.smooth = None;
                params.point_size = Some(self.p_j.aim.aim_size);
                let aim_position = glium::VertexBuffer::new(display, &[super::Vertex { position: [self.aim.0, self.aim.1] }])
                    .expect("Ошибка! Не удалось задать позицию для прицела!");
                target.draw(
                    &aim_position,
                    &glium::index::NoIndices(glium::index::PrimitiveType::Points),
                    &program.1,
                    &aim_uniforms,
                    &params,
                ).expect("Ошибка! Не удалось отрисовать прицел!");

                target.finish()
                    .expect("Ошибка! Не удалось закончить отрисовку кадра!");

                return;
            },
        };

        let uniforms = uniform! {
            matrix: self.cam.transform_matrix,
            x_off: self.cam.offset_x,
            y_off: self.cam.offset_y,
        };
        let params = glium::DrawParameters {
            polygon_mode,
            multisampling: multisampling_on,
            dithering: dithering_on,
            smooth: Some(glium::draw_parameters::Smooth::Nicest),
            ..Default::default()
        };
        target.draw(positions, indices, &program.0, &uniforms, &params)
            .expect("Ошибка! Не удалось отрисовать объект(ы)!");
        target.finish()
            .expect("Ошибка! Не удалось закончить отрисовку кадра!");
    }
}
