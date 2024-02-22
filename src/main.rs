#[macro_use]
extern crate glium;

mod graphics;
mod json;
mod etc;

use glium::{
    glutin::surface::WindowSurface,
    Display,
    IndexBuffer,
    Surface,
    VertexBuffer
};
use graphics::Vertex;
use std::{fs, env};


struct App {
    p: json::Persistent,
    cam: graphics::Camera
}


impl App {
    pub fn new(_p: json::Persistent) -> Self {
        Self {
            p: _p,
            cam: graphics::Camera::default(),
        }
    }

    fn transform_map(&mut self, action: graphics::TransformAction) {
        const OFFSET_SCALE: f32 = 7.;
        const OFFSET_X: f32 = 0.0001;
        const OFFSET_Y: f32 = 0.00007;
        const OFFSET_THETA: f32 = 0.04;
        let transform = |mat: &mut [[f32; 4]; 4], theta: f32, scale: f32| {
            mat[0][0] = f32::cos(theta) * scale * graphics::ASPECT_RATIO_HEIGHT / graphics::ASPECT_RATIO_WIDTH;
            mat[0][1] = -f32::sin(theta) * scale;
            mat[1][0] = f32::sin(theta) * scale;
            mat[1][1] = f32::cos(theta) * scale;
        };

        match action {
            graphics::TransformAction::Increase => {
                self.cam.scale += OFFSET_SCALE;
                transform(&mut self.cam.transform_matrix, self.cam.theta, self.cam.scale);
            },
            graphics::TransformAction::Reduce => {
                if self.cam.scale - OFFSET_SCALE < 0. {
                    return;
                }

                self.cam.scale -= OFFSET_SCALE;
                transform(&mut self.cam.transform_matrix, self.cam.theta, self.cam.scale);
            },
            graphics::TransformAction::MoveUp => self.cam.offset_y -= OFFSET_Y,
            graphics::TransformAction::MoveDown => self.cam.offset_y += OFFSET_Y,
            graphics::TransformAction::MoveLeft => self.cam.offset_x += OFFSET_X,
            graphics::TransformAction::MoveRight => self.cam.offset_x -= OFFSET_X,
            graphics::TransformAction::RotateLeft => {
                self.cam.theta += OFFSET_THETA;
                transform(&mut self.cam.transform_matrix, self.cam.theta, self.cam.scale);
            },
            graphics::TransformAction::RotateRight => {
                self.cam.theta -= OFFSET_THETA;
                transform(&mut self.cam.transform_matrix, self.cam.theta, self.cam.scale);
            },
        }
    }

    fn render_frame(
        &self,
        display: &Display<WindowSurface>,
        positions: &VertexBuffer<Vertex>,
        indices: (&IndexBuffer<u16>, &IndexBuffer<u16>),
        program: &glium::Program,
    ) {
        let uniforms = uniform! {
            matrix: self.cam.transform_matrix,
            x_off: self.cam.offset_x,
            y_off: self.cam.offset_y,
        };
        let mut target = display.draw();
        target.clear_color(
            graphics::BACKGROUND_R,
            graphics::BACKGROUND_G,
            graphics::BACKGROUND_B,
            graphics::NOT_TRANSPARENT,
        );

        match self.cam.display_type {
            graphics::DisplayType::Triangles => {
                let params = glium::DrawParameters {
                    polygon_mode: glium::draw_parameters::PolygonMode::Fill,
                    multisampling: true,
                    dithering: true,
                    smooth: Some(glium::draw_parameters::Smooth::Nicest),
                    ..Default::default()
                };
                target.draw(&*positions, &*indices.0, &program, &uniforms, &params)
                    .expect("Ошибка! Не удалось отрисовать кадр!");
            },
            graphics::DisplayType::TrianglesLines => {
                let params = glium::DrawParameters {
                    polygon_mode: glium::draw_parameters::PolygonMode::Line,
                    multisampling: true,
                    dithering: true,
                    smooth: Some(glium::draw_parameters::Smooth::Nicest),
                    ..Default::default()
                };
                target.draw(&*positions, &*indices.0, &program, &uniforms, &params)
                    .expect("Ошибка! Не удалось отрисовать кадр!");
            },
            graphics::DisplayType::Lines => {
                let params = glium::DrawParameters {
                    multisampling: true,
                    dithering: true,
                    smooth: Some(glium::draw_parameters::Smooth::Nicest),
                    ..Default::default()
                };
                target.draw(&*positions, &*indices.1, &program, &uniforms, &params)
                    .expect("Ошибка! Не удалось отрисовать кадр!");
            },
        }

        target.finish()
            .expect("Ошибка! Не удалось закончить отрисовку кадра!");
    }

    pub fn start_app(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut buildings = Vec::<Vec<Vec<f64>>>::with_capacity(self.p.features.len());

        for building in &self.p.features {
            buildings.push(building.geometry.coordinates[0][0].clone());
        }
        
        let indices_triangle = graphics::get_triangle_indices(&buildings);
        let indices_line = graphics::get_line_indices(&buildings);
        let event_loop = winit::event_loop::EventLoopBuilder::new()
            .build()?;
        let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title(&self.p.name)
            .build(&event_loop);        
        implement_vertex!(Vertex, position);
        let mut shape = Vec::<Vertex>::with_capacity(buildings.len());

        for build in buildings {
            for point in build {
                shape.push(Vertex { position: etc::vec_to_arr::<f64, 2>(point) })
            }
        }

        let positions = glium::VertexBuffer::new(&display, &shape)?;
        let indices_triangle = glium::IndexBuffer::new(
            &display,
            glium::index::PrimitiveType::TrianglesList,
            &indices_triangle,
        )?;
        let indices_line = glium::IndexBuffer::new(
            &display,
            glium::index::PrimitiveType::LinesList,
            &indices_line,
        )?;
        let vertex_shader_src = fs::read_to_string(graphics::VERTEX_SHADER_PATH)?;
        let color_shader_src = fs::read_to_string(graphics::COLOR_SHADER_PATH)?;
        let program = glium::Program::from_source(
            &display,
            &vertex_shader_src,
            &color_shader_src,
            None,
        )?;
        self.render_frame(&display, &positions, (&indices_triangle, &indices_line), &program);
        
        event_loop.run(move |ev, window_target| {
            match ev {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => {
                        window_target.exit();
                    },
                    winit::event::WindowEvent::Resized(window_size) => {
                        self.render_frame(&display, &positions, (&indices_triangle, &indices_line), &program);
                        display.resize(window_size.into());
                    },
                    winit::event::WindowEvent::Focused(_) => self.render_frame(
                        &display,
                        &positions,
                        (&indices_triangle, &indices_line),
                        &program,
                    ),
                    winit::event::WindowEvent::Moved(_) => self.render_frame(
                        &display,
                        &positions,
                        (&indices_triangle, &indices_line),
                        &program,
                    ),
                    winit::event::WindowEvent::KeyboardInput { event, is_synthetic, .. } => {
                        if is_synthetic {
                            return;
                        }

                        match event.physical_key {
                            winit::keyboard::PhysicalKey::Code(key) => {
                                match key {
                                    winit::keyboard::KeyCode::KeyZ => self.transform_map(graphics::TransformAction::Increase),
                                    winit::keyboard::KeyCode::KeyX => self.transform_map(graphics::TransformAction::Reduce),
                                    winit::keyboard::KeyCode::KeyW | winit::keyboard::KeyCode::ArrowUp => 
                                        self.transform_map(graphics::TransformAction::MoveUp),
                                    winit::keyboard::KeyCode::KeyS | winit::keyboard::KeyCode::ArrowDown =>
                                        self.transform_map(graphics::TransformAction::MoveDown),
                                    winit::keyboard::KeyCode::KeyA | winit::keyboard::KeyCode::ArrowLeft => 
                                        self.transform_map(graphics::TransformAction::MoveLeft),
                                    winit::keyboard::KeyCode::KeyD | winit::keyboard::KeyCode::ArrowRight => 
                                        self.transform_map(graphics::TransformAction::MoveRight),
                                    winit::keyboard::KeyCode::KeyQ => self.transform_map(graphics::TransformAction::RotateLeft),
                                    winit::keyboard::KeyCode::KeyE => self.transform_map(graphics::TransformAction::RotateRight),
                                    winit::keyboard::KeyCode::KeyV => if event.state == winit::event::ElementState::Released {
                                        self.cam.display_type.switch();
                                    },
                                    _ => return,
                                }

                                self.render_frame(&display, &positions, (&indices_triangle, &indices_line), &program);
                            },
                            winit::keyboard::PhysicalKey::Unidentified(_) => return,
                        }
                    }
                    _ => {},
                },
                _ => {},
            }
        })?;

        Ok(())
    }
}


pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args();
    let args_len = args.len();
    let mut is_first_arg = true;

    for arg in args {
        if args_len > 1 {
            if is_first_arg {
                is_first_arg = false;

                continue;
            }

            if &arg == "--help" || &arg == "-h" {
                println!("wtf");

                return Ok(());
            } else {
                panic!("Неизвестный аргумент {}", arg);
            }
        }
        
    }

    let p = json::Persistent::default();
    
    let mut app = App::new(p);
    app.start_app()?;
    
    Ok(())
}
