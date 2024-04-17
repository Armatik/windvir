use crate::{
    App,
    graphics::{self, Vertex},
    defs::synthetic,
};
use glium::{
    glutin::{self, event_loop::ControlFlow}, Display, VertexBuffer
};


enum MoveAim {
    Top,
    Right,
    Left,
    Down,
}


impl App {
    fn change_aim(&mut self, action: MoveAim) {
        match action {
            MoveAim::Top => self.synthetic_data[synthetic::AIM_INDEX].move_aim(0., self.p_j.movement.aim_speed as f64)
                .expect("Ошибка! Прицел был утерян!"),
            MoveAim::Right => self.synthetic_data[synthetic::AIM_INDEX].move_aim(self.p_j.movement.aim_speed as f64, 0.)
                .expect("Ошибка! Прицел был утерян!"),
            MoveAim::Left => self.synthetic_data[synthetic::AIM_INDEX].move_aim(-self.p_j.movement.aim_speed as f64, 0.)
                .expect("Ошибка! Прицел был утерян!"),
            MoveAim::Down => self.synthetic_data[synthetic::AIM_INDEX].move_aim(0., -self.p_j.movement.aim_speed as f64)
                .expect("Ошибка! Прицел был утерян!"),
        };
    } 

    pub fn window_loop(
        mut self,
        event_loop: glutin::event_loop::EventLoop<()>,
        display: Display,
        positions: VertexBuffer<Vertex>,
        program: (glium::Program, glium::Program),
        indices_triangle: graphics::IndciesTriangles,
        indices_line: graphics::IndciesLines,
        indices_triangulate: graphics::IndciesTriangles
    ) -> ! {
        let (synthetic_vertices, synthetic_indices) = graphics::get_synthetic_triangulation_indices(&self.synthetic_data);
        let mut synthetic_vertices_buffer = glium::VertexBuffer::new(&display, &synthetic_vertices)
            .expect("Ошибка! Не удалось создать вектор вершин для синтетических данных!");
        let mut synthetic_indices_buffer = glium::IndexBuffer::new(
            &display,
            glium::index::PrimitiveType::TrianglesList,
            &synthetic_indices,
        ).expect("Ошибка! Не удлалось соединить все вершины синтетических объектов!");

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
                                (&positions, &synthetic_vertices_buffer),
                                (&indices_triangle, &indices_line, &indices_triangulate, &synthetic_indices_buffer),
                                &program,
                            );
                        },
                        glutin::event::WindowEvent::Moved(_) => self.render_frame(
                            &display,
                            (&positions, &synthetic_vertices_buffer),
                            (&indices_triangle, &indices_line, &indices_triangulate, &synthetic_indices_buffer),
                            &program
                        ),
                        #[cfg(unix)]
                        glutin::event::WindowEvent::KeyboardInput { input, is_synthetic, .. } => {
                            const NUM1_KEY: u32 = 0x02;
                            const NUM2_KEY: u32 = 0x03;
                            const NUM3_KEY: u32 = 0x04;
                            const NUM4_KEY: u32 = 0x05;
                            const NUM5_KEY: u32 = 0x06;
                            const NUM6_KEY: u32 = 0x07;
                            const NUM7_KEY: u32 = 0x08;
                            const NUM8_KEY: u32 = 0x09;
                            const NUM9_KEY: u32 = 0x0a;
                            const Q_KEY: u32 = 0x10;
                            const W_KEY: u32 = 0x11;
                            const E_KEY: u32 = 0x12;
                            const R_KEY: u32 = 0x13;
                            const A_KEY: u32 = 0x1e;
                            const S_KEY: u32 = 0x1f;
                            const D_KEY: u32 = 0x20;
                            const L_KEY: u32 = 0x26;
                            const Z_KEY: u32 = 0x2c;
                            const X_KEY: u32 = 0x2d;
                            const C_KEY: u32 = 0x2e;
                            const V_KEY: u32 = 0x2f;
                            const ARROW_UP_KEY: u32 = 0x67;
                            const ARROW_LEFT_KEY: u32 = 0x69;
                            const ARROW_RIGHT_KEY: u32 = 0x6a;
                            const ARROW_DOWN_KEY: u32 = 0x6c;
                            
                            if !is_synthetic {
                                match input.scancode {
                                    V_KEY => if input.state == glutin::event::ElementState::Released {
                                        self.cam.display_type.switch();
                                    },
                                    W_KEY | ARROW_UP_KEY => self.transform_map(graphics::TransformAction::MoveUp),
                                    A_KEY | ARROW_LEFT_KEY => self.transform_map(graphics::TransformAction::MoveLeft),
                                    S_KEY | ARROW_DOWN_KEY => self.transform_map(graphics::TransformAction::MoveDown),
                                    D_KEY | ARROW_RIGHT_KEY => self.transform_map(graphics::TransformAction::MoveRight),
                                    Q_KEY => self.transform_map(graphics::TransformAction::RotateLeft),
                                    E_KEY => self.transform_map(graphics::TransformAction::RotateRight),
                                    Z_KEY => self.transform_map(graphics::TransformAction::Increase),
                                    X_KEY => self.transform_map(graphics::TransformAction::Reduce),
                                    C_KEY => if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                                        if self.synthetic_data.len() == 1 || !self.synthetic_data[self.synthetic_data.len() - 1].is_value_default() {
                                            self.synthetic_data.push(Box::new(synthetic::Circle::default()));

                                            log::info!("Выберите размер для окружности, используя цифры 0..=9");
                                        }
                                    },
                                    R_KEY => if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                                        // if self.synthetic_data.len() == 0 || !self.synthetic_data[self.synthetic_data.len() - 1].is_value_default() {
                                        //     self.synthetic_data.push(Box::new(defs::Rectangle::default()));
                                        // }
                                    },
                                    L_KEY => if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                                        // if self.synthetic_data.len() == 0 || !self.synthetic_data[self.synthetic_data.len() - 1].is_value_default() {
                                        //     self.synthetic_data.push(Box::new(defs::Segment::default()));
                                        // }
                                    },
                                    value @ (NUM1_KEY | NUM2_KEY | NUM3_KEY | NUM4_KEY | NUM5_KEY | NUM6_KEY | NUM7_KEY | NUM8_KEY | NUM9_KEY) =>
                                        if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                                            let index_elem = self.synthetic_data.len() - 1;

                                            if self.synthetic_data[index_elem].is_value_default() {
                                                if let synthetic::SyntheticVariant::Circle(aim_center, _) =
                                                    self.synthetic_data[synthetic::AIM_INDEX].get_data() {
                                                        let size = 0.00025 * value as f64;
                                                        self.synthetic_data[index_elem].set_value(synthetic::SyntheticVariant::Circle(aim_center, size));

                                                        log::info!("Окружность размером {size} была успешно задана!");
                                                } else {
                                                    log::error!("Пропал прицел!");

                                                    unreachable!("Произошла пропажа прицела");
                                                }
                                            }
                                            let (synthetic_vertices, synthetic_indices) = graphics::get_synthetic_triangulation_indices(&self.synthetic_data);
                                            synthetic_vertices_buffer = glium::VertexBuffer::new(&display, &synthetic_vertices)
                                                .expect("Ошибка! Не удалось создать вектор вершин для синтетических данных!");
                                            synthetic_indices_buffer = glium::IndexBuffer::new(
                                                &display,
                                                glium::index::PrimitiveType::TrianglesList,
                                                &synthetic_indices,
                                            ).expect("Ошибка! Не удлалось соединить все вершины синтетических объектов!");                                            
                                    },
                                    _ => {},
                                }
                            }

                            self.render_frame(
                                &display,
                                (&positions, &synthetic_vertices_buffer),
                                (&indices_triangle, &indices_line, &indices_triangulate, &synthetic_indices_buffer),
                                &program
                            );
                        },
                        _ => {},
                    }
                },
                glutin::event::Event::NewEvents(cause) => {
                    match cause {
                        glutin::event::StartCause::Init => {
                            self.render_frame(
                                &display,
                                (&positions, &synthetic_vertices_buffer),
                            (&indices_triangle, &indices_line, &indices_triangulate, &synthetic_indices_buffer),
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
