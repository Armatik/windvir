#[macro_use]
extern crate glium;

mod graphics;
mod json;
mod etc;
mod ffi;

use glium::{
    glutin::{self, event_loop::ControlFlow},
    Display,
    Surface,
    VertexBuffer,
};
use graphics::Vertex;
use std::{fs, env};
use json::{geojson, default_json};


type WindowWidth = f32;
type WindowHeight = f32;


pub struct App {    
    p_g: geojson::PersistentG,
    p_j: default_json::PersistentJ,
    cam: graphics::Camera,
    window_size: (WindowWidth, WindowHeight),
    buildings: Vec<Vec<Vec<f64>>>,
}


impl App {
    pub fn new(_p_g: geojson::PersistentG, _p_j: default_json::PersistentJ, def_buildings: Option<Vec<Vec<Vec<f64>>>>) -> Self {
        let _buildings = match def_buildings {
            Some(data) => data,
            None => Self::trans_persistent(&_p_g),
        };

        Self {
            p_g: _p_g,
            p_j: _p_j,
            cam: graphics::Camera::default(),
            window_size: (_p_j.resolution.width as f32, _p_j.resolution.height as f32),
            buildings: _buildings,
        }
    }

    pub fn trans_persistent(p_g: &geojson::PersistentG) -> Vec<Vec<Vec<f64>>> {
        let mut buildings = Vec::<Vec<Vec<f64>>>::with_capacity(p_g.features.len());

        for building in &p_g.features {
            buildings.push(building.geometry.coordinates[0][0].clone());
        }

        buildings
    }

    fn transform_map(&mut self, action: graphics::TransformAction) {
        let transform = |mat: &mut [[f32; 4]; 4], theta: f32, scale: f32| {
            mat[0][0] = f32::cos(theta) * scale * self.window_size.1 / self.window_size.0;
            mat[0][1] = -f32::sin(theta) * scale;
            mat[1][0] = f32::sin(theta) * scale * self.window_size.1 / self.window_size.0;
            mat[1][1] = f32::cos(theta) * scale;
        };

        match action {
            graphics::TransformAction::Increase => {
                self.cam.scale += self.p_j.movement.scale;
                transform(&mut self.cam.transform_matrix, self.cam.theta, self.cam.scale);
            },
            graphics::TransformAction::Reduce => {
                if self.cam.scale - self.p_j.movement.scale < 0. {
                    return;
                }

                self.cam.scale -= self.p_j.movement.scale;
                transform(&mut self.cam.transform_matrix, self.cam.theta, self.cam.scale);
            },
            graphics::TransformAction::MoveUp => self.cam.offset_y -= self.p_j.movement.y,
            graphics::TransformAction::MoveDown => self.cam.offset_y += self.p_j.movement.y,
            graphics::TransformAction::MoveLeft => self.cam.offset_x += self.p_j.movement.x,
            graphics::TransformAction::MoveRight => self.cam.offset_x -= self.p_j.movement.x,
            graphics::TransformAction::RotateLeft => {
                self.cam.theta += self.p_j.movement.theta;
                transform(&mut self.cam.transform_matrix, self.cam.theta, self.cam.scale);
            },
            graphics::TransformAction::RotateRight => {
                self.cam.theta -= self.p_j.movement.theta;
                transform(&mut self.cam.transform_matrix, self.cam.theta, self.cam.scale);
            },
        }
    }

    fn render_frame(
        &self,
        display: &Display,
        positions: &VertexBuffer<Vertex>,
        indices: (&graphics::IndciesTriangles, &graphics::IndciesLines, &graphics::IndciesLines),
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
            graphics::DisplayType::TrianglesFill => {
                let params = glium::DrawParameters {
                    polygon_mode: glium::draw_parameters::PolygonMode::Fill,
                    multisampling: multisampling_on,
                    dithering: dithering_on,
                    smooth: Some(glium::draw_parameters::Smooth::Nicest),
                    ..Default::default()
                };
                target.draw(&*positions, &*indices.2, &program, &uniforms, &params)
                    .expect("Ошибка! Не удалось отрисовать кадр!");
            },
            graphics::DisplayType::TrianglesFillLines => {
                let params = glium::DrawParameters {
                    polygon_mode: glium::draw_parameters::PolygonMode::Line,
                    multisampling: multisampling_on,
                    dithering: dithering_on,
                    smooth: Some(glium::draw_parameters::Smooth::Nicest),
                    ..Default::default()
                };
                target.draw(&*positions, &*indices.2, &program, &uniforms, &params)
                    .expect("Ошибка! Не удалось отрисовать кадр!");
            },
            graphics::DisplayType::Triangles => {
                let params = glium::DrawParameters {
                    polygon_mode: glium::draw_parameters::PolygonMode::Fill,
                    multisampling: multisampling_on,
                    dithering: dithering_on,
                    smooth: Some(glium::draw_parameters::Smooth::Nicest),
                    ..Default::default()
                };
                target.draw(&*positions, &*indices.0, &program, &uniforms, &params)
                    .expect("Ошибка! Не удалось отрисовать кадр!");
            },
            graphics::DisplayType::TrianglesLines => {
                let params = glium::DrawParameters {
                    polygon_mode: glium::draw_parameters::PolygonMode::Line,
                    multisampling: multisampling_on,
                    dithering: dithering_on,
                    smooth: Some(glium::draw_parameters::Smooth::Nicest),
                    ..Default::default()
                };
                target.draw(&*positions, &*indices.0, &program, &uniforms, &params)
                    .expect("Ошибка! Не удалось отрисовать кадр!");
            },
            graphics::DisplayType::Lines => {
                let params = glium::DrawParameters {
                    multisampling: multisampling_on,
                    dithering: dithering_on,
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

    fn window_loop(
        mut self,
        event_loop: glutin::event_loop::EventLoop<()>,
        display: Display,
        positions: VertexBuffer<Vertex>,
        program: glium::Program,
        indices_triangle: graphics::IndciesTriangles,
        indices_line: graphics::IndciesLines,
        indices_triangulate: graphics::IndciesTriangles
    ) -> ! {
        event_loop.run(move |ev, _, control_flow| {
            match ev {
                glutin::event::Event::WindowEvent { event, .. } => {
                    match event {
                        glutin::event::WindowEvent::CloseRequested | glutin::event::WindowEvent::Destroyed => {
                            *control_flow = ControlFlow::Exit;

                            #[cfg(windows)]
                            {
                                std::process::exit(0);
                            }
                        },
                        glutin::event::WindowEvent::Resized(size) => {
                            self.window_size.0 = size.width as f32;
                            self.window_size.1 = size.height as f32;
                            self.render_frame(
                                &display,
                                &positions,
                                (&indices_triangle, &indices_line, &indices_triangulate),
                                &program,
                            );
                        },
                        glutin::event::WindowEvent::Moved(_) => self.render_frame(
                            &display,
                            &positions,
                            (&indices_triangle, &indices_line, &indices_triangulate),
                            &program
                        ),
                        #[cfg(unix)]
                        glutin::event::WindowEvent::KeyboardInput { input, is_synthetic, .. } => {
                            if !is_synthetic {
                                if let Some(key) = input.virtual_keycode {
                                    match key {
                                        glutin::event::VirtualKeyCode::V => if input.state == glutin::event::ElementState::Released {
                                            self.cam.display_type.switch();
                                        },
                                        glutin::event::VirtualKeyCode::W | glutin::event::VirtualKeyCode::Up => self.transform_map(graphics::TransformAction::MoveUp),
                                        glutin::event::VirtualKeyCode::A | glutin::event::VirtualKeyCode::Left =>
                                            self.transform_map(graphics::TransformAction::MoveLeft),
                                        glutin::event::VirtualKeyCode::S | glutin::event::VirtualKeyCode::Down =>
                                            self.transform_map(graphics::TransformAction::MoveDown),
                                        glutin::event::VirtualKeyCode::D | glutin::event::VirtualKeyCode::Right =>
                                            self.transform_map(graphics::TransformAction::MoveRight),
                                        glutin::event::VirtualKeyCode::Q => self.transform_map(graphics::TransformAction::RotateLeft),
                                        glutin::event::VirtualKeyCode::E => self.transform_map(graphics::TransformAction::RotateRight),
                                        glutin::event::VirtualKeyCode::Z => self.transform_map(graphics::TransformAction::Increase),
                                        glutin::event::VirtualKeyCode::X => self.transform_map(graphics::TransformAction::Reduce),
                                        _ => {},
                                    }
                                }
                            }

                            self.render_frame(&display, &positions, (&indices_triangle, &indices_line, &indices_triangulate), &program);
                        },
                        _ => {},
                    }
                },
                glutin::event::Event::NewEvents(cause) => {
                    match cause {
                        glutin::event::StartCause::Init => {
                            self.render_frame(
                                &display,
                                &positions,
                            (&indices_triangle, &indices_line, &indices_triangulate),
                                &program,
                            );
                        },
                        _ => {},
                    }
                },
                #[cfg(windows)]
                glutin::event::Event::DeviceEvent { event, .. } => {
                    match event {
                        glutin::event::DeviceEvent::Key(key) => {
                            match key.virtual_keycode {
                                Some(cap) => match cap {
                                    glutin::event::VirtualKeyCode::V => if key.state == glutin::event::ElementState::Released {
                                        self.cam.display_type.switch();
                                    },
                                    glutin::event::VirtualKeyCode::W | glutin::event::VirtualKeyCode::Up => self.transform_map(graphics::TransformAction::MoveUp),
                                    glutin::event::VirtualKeyCode::A | glutin::event::VirtualKeyCode::Left =>
                                        self.transform_map(graphics::TransformAction::MoveLeft),
                                    glutin::event::VirtualKeyCode::S | glutin::event::VirtualKeyCode::Down =>
                                        self.transform_map(graphics::TransformAction::MoveDown),
                                    glutin::event::VirtualKeyCode::D | glutin::event::VirtualKeyCode::Right =>
                                        self.transform_map(graphics::TransformAction::MoveRight),
                                    glutin::event::VirtualKeyCode::Q => self.transform_map(graphics::TransformAction::RotateLeft),
                                    glutin::event::VirtualKeyCode::E => self.transform_map(graphics::TransformAction::RotateRight),
                                    glutin::event::VirtualKeyCode::Z => self.transform_map(graphics::TransformAction::Increase),
                                    glutin::event::VirtualKeyCode::X => self.transform_map(graphics::TransformAction::Reduce),
                                    _ => {},
                                },
                                None => return,
                            }

                            self.render_frame(&display, &positions, (&indices_triangle, &indices_line), &program);
                        },
                        _ => {},
                    }
                },
                _ => {},
            }
        });
    }

    pub fn start_app(self) -> Result<(), Box<dyn std::error::Error>> {
        let indices_triangle = graphics::get_triangle_indices(&self.buildings);
        let indices_line = graphics::get_line_indices(&self.buildings);
        let indices_triangulate = graphics::get_triangulation_indices(&self.buildings);
        let event_loop = glutin::event_loop::EventLoop::new();
        let window = glutin::window::WindowBuilder::new()
            .with_title(&self.p_g.name)
            .with_inner_size(glutin::dpi::LogicalSize::new(self.p_j.resolution.width, self.p_j.resolution.height));
        let context = if self.p_j.graphics.multisampling_on {
            if self.p_j.graphics.depth_buffering_on {
                glutin::ContextBuilder::new()
                    .with_vsync(self.p_j.graphics.vsync_on)
                    .with_depth_buffer(self.p_j.graphics.depth_buffer)
                    .with_multisampling(self.p_j.graphics.multisampling)
            } else {
                glutin::ContextBuilder::new()
                    .with_vsync(self.p_j.graphics.vsync_on)
                    .with_multisampling(self.p_j.graphics.multisampling)
            }
        } else {
            if self.p_j.graphics.depth_buffering_on {
                glutin::ContextBuilder::new()
                    .with_vsync(self.p_j.graphics.vsync_on)
                    .with_depth_buffer(self.p_j.graphics.depth_buffer)
            } else {
                glutin::ContextBuilder::new()
                    .with_vsync(self.p_j.graphics.vsync_on)
            }
        };
        let display = glium::Display::new(window, context, &event_loop)?;
        implement_vertex!(Vertex, position);
        let mut shape = Vec::<Vertex>::with_capacity(self.buildings.len());

        for build in &self.buildings {
            for point in build {
                shape.push(Vertex { position: etc::vec_to_arr::<f64, 2>(point.clone()) })
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
        let indices_triangulate = glium::IndexBuffer::new(
            &display,
            glium::index::PrimitiveType::TrianglesList,
            &indices_triangulate,
        )?;
        let vertex_shader_src = fs::read_to_string(graphics::VERTEX_SHADER_PATH)?;
        let color_shader_src = fs::read_to_string(graphics::COLOR_SHADER_PATH)?;
        let program = glium::Program::from_source(
            &display,
            &vertex_shader_src,
            &color_shader_src,
            None,
        )?;
        log::info!("Все здания успешно просчитаны и заданы!");

        self.window_loop(event_loop, display, positions, program, indices_triangle, indices_line, indices_triangulate)
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
            } else if &arg == "-c" {
                let p_g = geojson::PersistentG::default();
                let p_j = default_json::PersistentJ::default();
                let data = App::trans_persistent(&p_g);
                let data = unsafe { ffi::BuildingsVec::new(data) };

                let out = unsafe { ffi::changeVertex(data) };
                
                let mut norm_buildings = Vec::<Vec<Vec<f64>>>::with_capacity(p_g.features.len());
                let buildings = unsafe { Vec::from_raw_parts(out.data, out.len_buildings as usize, out.len_buildings as usize) };
                
                for building in buildings {
                    let mut norm_vertex = Vec::<Vec<f64>>::with_capacity(2);
                    let building = unsafe { Vec::from_raw_parts(building.data, building.len_vertex as usize, building.len_vertex as usize) };

                    for vertex in building {
                        let vertex = unsafe { Vec::from_raw_parts(vertex, 2, 2) };
                        norm_vertex.push(vertex);
                    }

                    norm_buildings.push(norm_vertex);
                }
                
                let app = App::new(p_g, p_j, Some(norm_buildings));
                app.start_app()?;

                unsafe { ffi::freeBuildings(out); };

                return Ok(());
            } else {
                panic!("Неизвестный аргумент {}", arg);
            }
        }
        
    }

    let p_g = geojson::PersistentG::default();
    let p_j = default_json::PersistentJ::default();
    
    let app = App::new(p_g, p_j, None);
    app.start_app()?;
    
    Ok(())
}
