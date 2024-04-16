use crate::{
    App,
    graphics::{self, Vertex},
};
use glium::{
    glutin::{self, event_loop::ControlFlow},
    Display,
    VertexBuffer,
};


impl App {
    pub fn window_loop(
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
                            self.transform_map(graphics::TransformAction::Resize);
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
                                        glutin::event::VirtualKeyCode::C => if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                                            // if self.synthetic_data.len() == 0 || !self.synthetic_data[self.synthetic_data.len() - 1].is_value_default() {
                                            //     self.synthetic_data.push(Box::new(defs::Circle::default()));
                                            // }
                                        },
                                        glutin::event::VirtualKeyCode::R => if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                                            // if self.synthetic_data.len() == 0 || !self.synthetic_data[self.synthetic_data.len() - 1].is_value_default() {
                                            //     self.synthetic_data.push(Box::new(defs::Rectangle::default()));
                                            // }
                                        },
                                        glutin::event::VirtualKeyCode::L => if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                                            // if self.synthetic_data.len() == 0 || !self.synthetic_data[self.synthetic_data.len() - 1].is_value_default() {
                                            //     self.synthetic_data.push(Box::new(defs::Segment::default()));
                                            // }
                                        },
                                        glutin::event::VirtualKeyCode::Key1 => if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                                            // if let defs::SyntheticVariant::Circle(radius) = self.synthetic_data[self.synthetic_data.len() - 1].get_data() {

                                            // }
                                        },
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
                                    glutin::event::VirtualKeyCode::P => if input.state == glutin::event::ElementState::Released {
                                        self.cam.display_type.switch();
                                    },
                                    _ => {},
                                },
                                None => return,
                            }

                            self.render_frame(&display, &positions, (&indices_triangle, &indices_line, &indices_triangulate), &program);
                        },
                        _ => {},
                    }
                },
                _ => {},
            }
        });
    }
}
