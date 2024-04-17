use crate::{
    defs, json::geojson, App
};
use glium::{
    Display,
    Surface,
    VertexBuffer,
};


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
        }
    }

    pub fn render_frame(
        &self,
        display: &Display,
        positions: (&VertexBuffer<super::Vertex>, &VertexBuffer<super::Vertex>),
        indices: (&super::IndciesTriangles, &super::IndciesLines, &super::IndciesLines, &super::IndciesTriangles),
        program: &glium::Program,
    ) {
        let uniforms = uniform! {
            matrix: self.cam.transform_matrix,
            x_off: self.cam.offset_x,
            y_off: self.cam.offset_y,
        };
        let mut target = display.draw();
        target.clear_color(
            self.p_j.background_color.r,
            self.p_j.background_color.g,
            self.p_j.background_color.b,
            self.p_j.background_color.a,
        );
        let multisampling_on = self.p_j.graphics.multisampling_on;
        let dithering_on = self.p_j.graphics.dithering_on;

        match self.cam.display_type {
            super::DisplayType::TrianglesFill => {
                let params = glium::DrawParameters {
                    polygon_mode: glium::draw_parameters::PolygonMode::Fill,
                    multisampling: multisampling_on,
                    dithering: dithering_on,
                    smooth: Some(glium::draw_parameters::Smooth::Nicest),
                    ..Default::default()
                };
                target.draw(&*positions.0, &*indices.2, &program, &uniforms, &params)
                    .expect("Ошибка! Не удалось отрисовать кадр!");
            },
            super::DisplayType::TrianglesFillLines => {
                let params = glium::DrawParameters {
                    polygon_mode: glium::draw_parameters::PolygonMode::Line,
                    multisampling: multisampling_on,
                    dithering: dithering_on,
                    smooth: Some(glium::draw_parameters::Smooth::Nicest),
                    ..Default::default()
                };
                target.draw(&*positions.0, &*indices.2, &program, &uniforms, &params)
                    .expect("Ошибка! Не удалось отрисовать кадр!");
            },
            super::DisplayType::Triangles => {
                let params = glium::DrawParameters {
                    polygon_mode: glium::draw_parameters::PolygonMode::Fill,
                    multisampling: multisampling_on,
                    dithering: dithering_on,
                    smooth: Some(glium::draw_parameters::Smooth::Nicest),
                    ..Default::default()
                };
                target.draw(&*positions.0, &*indices.0, &program, &uniforms, &params)
                    .expect("Ошибка! Не удалось отрисовать кадр!");
            },
            super::DisplayType::TrianglesLines => {
                let params = glium::DrawParameters {
                    polygon_mode: glium::draw_parameters::PolygonMode::Line,
                    multisampling: multisampling_on,
                    dithering: dithering_on,
                    smooth: Some(glium::draw_parameters::Smooth::Nicest),
                    ..Default::default()
                };
                target.draw(&*positions.0, &*indices.0, &program, &uniforms, &params)
                    .expect("Ошибка! Не удалось отрисовать кадр!");
            },
            super::DisplayType::Lines => {
                let params = glium::DrawParameters {
                    multisampling: multisampling_on,
                    dithering: dithering_on,
                    smooth: Some(glium::draw_parameters::Smooth::Nicest),
                    ..Default::default()
                };
                target.draw(&*positions.0, &*indices.1, &program, &uniforms, &params)
                    .expect("Ошибка! Не удалось отрисовать кадр!");
            },
            super::DisplayType::ObjectSpawn => {
                let params = glium::DrawParameters {
                    polygon_mode: glium::draw_parameters::PolygonMode::Fill,
                    multisampling: multisampling_on,
                    dithering: dithering_on,
                    smooth: Some(glium::draw_parameters::Smooth::Nicest),
                    ..Default::default()
                };
                target.draw(&*positions.1, &*indices.3, &program, &uniforms, &params)
                    .expect("Ошибка! Не удалось отрисовать кадр!");
            },
        }

        target.finish()
            .expect("Ошибка! Не удалось закончить отрисовку кадра!");
    }
}
