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
        program: glium::Program,
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
        let make_circle = |app: &mut Self, size: f64| {
            if app.synthetic_data[app.synthetic_data.len() - 1].is_value_default() {
                if let synthetic::SyntheticVariant::Circle(aim_center, _) = app.synthetic_data[synthetic::AIM_INDEX].get_data() {
                    app.synthetic_data[synthetic::AIM_INDEX].set_value(synthetic::SyntheticVariant::Circle(aim_center, size));

                    log::info!("Окружность размером {size} была успешно задана!");
                } else {
                    log::error!("Пропал прицел!");

                    unreachable!("Произошла пропажа прицела");
                }
            }
        };

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
                                            if self.synthetic_data.len() == 1 || !self.synthetic_data[self.synthetic_data.len() - 1].is_value_default() {
                                                self.synthetic_data.push(Box::new(synthetic::Circle::default()));

                                                log::info!("Выберите размер для окружности, используя цифры 0..=9");
                                            }
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
                                            make_circle(&mut self, 0.0005);
                                            let (synthetic_vertices, synthetic_indices) = graphics::get_synthetic_triangulation_indices(&self.synthetic_data);
                                            synthetic_vertices_buffer = glium::VertexBuffer::new(&display, &synthetic_vertices)
                                                .expect("Ошибка! Не удалось создать вектор вершин для синтетических данных!");
                                            synthetic_indices_buffer = glium::IndexBuffer::new(
                                                &display,
                                                glium::index::PrimitiveType::TrianglesList,
                                                &synthetic_indices,
                                            ).expect("Ошибка! Не удлалось соединить все вершины синтетических объектов!");                                            
                                        },
                                        glutin::event::VirtualKeyCode::Key2 => if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                                            if let synthetic::SyntheticVariant::Circle(_, _) = 
                                                self.synthetic_data[self.synthetic_data.len() - 1].get_data() {
                                                    if let synthetic::SyntheticVariant::Circle(aim_center, _) = 
                                                        self.synthetic_data[synthetic::AIM_INDEX].get_data() {
                                                            self.synthetic_data[synthetic::AIM_INDEX]
                                                                .set_value(synthetic::SyntheticVariant::Circle(aim_center, 0.1));
                                                    } else {
                                                        log::error!("Пропал прицел!");

                                                        unreachable!("Произошла пропажа прицела");
                                                    }
                                            }
                                        },
                                        glutin::event::VirtualKeyCode::Key3 => if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                                            if let synthetic::SyntheticVariant::Circle(_, _) = 
                                                self.synthetic_data[self.synthetic_data.len() - 1].get_data() {
                                                    if let synthetic::SyntheticVariant::Circle(aim_center, _) = 
                                                        self.synthetic_data[synthetic::AIM_INDEX].get_data() {
                                                            self.synthetic_data[synthetic::AIM_INDEX]
                                                                .set_value(synthetic::SyntheticVariant::Circle(aim_center, 0.15));
                                                    } else {
                                                        log::error!("Пропал прицел!");

                                                        unreachable!("Произошла пропажа прицела");
                                                    }
                                            }
                                        },
                                        glutin::event::VirtualKeyCode::Key4 => if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                                            if let synthetic::SyntheticVariant::Circle(_, _) = 
                                                self.synthetic_data[self.synthetic_data.len() - 1].get_data() {
                                                    if let synthetic::SyntheticVariant::Circle(aim_center, _) = 
                                                        self.synthetic_data[synthetic::AIM_INDEX].get_data() {
                                                            self.synthetic_data[synthetic::AIM_INDEX]
                                                                .set_value(synthetic::SyntheticVariant::Circle(aim_center, 0.2));
                                                    } else {
                                                        log::error!("Пропал прицел!");

                                                        unreachable!("Произошла пропажа прицела");
                                                    }
                                            }
                                        },
                                        glutin::event::VirtualKeyCode::Key5 => if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                                            if let synthetic::SyntheticVariant::Circle(_, _) = 
                                                self.synthetic_data[self.synthetic_data.len() - 1].get_data() {
                                                    if let synthetic::SyntheticVariant::Circle(aim_center, _) = 
                                                        self.synthetic_data[synthetic::AIM_INDEX].get_data() {
                                                            self.synthetic_data[synthetic::AIM_INDEX]
                                                                .set_value(synthetic::SyntheticVariant::Circle(aim_center, 0.25));
                                                    } else {
                                                        log::error!("Пропал прицел!");

                                                        unreachable!("Произошла пропажа прицела");
                                                    }
                                            }
                                        },
                                        glutin::event::VirtualKeyCode::Key6 => if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                                            if let synthetic::SyntheticVariant::Circle(_, _) = 
                                                self.synthetic_data[self.synthetic_data.len() - 1].get_data() {
                                                    if let synthetic::SyntheticVariant::Circle(aim_center, _) = 
                                                        self.synthetic_data[synthetic::AIM_INDEX].get_data() {
                                                            self.synthetic_data[synthetic::AIM_INDEX]
                                                                .set_value(synthetic::SyntheticVariant::Circle(aim_center, 0.3));
                                                    } else {
                                                        log::error!("Пропал прицел!");

                                                        unreachable!("Произошла пропажа прицела");
                                                    }
                                            }
                                        },
                                        glutin::event::VirtualKeyCode::Key7 => if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                                            if let synthetic::SyntheticVariant::Circle(_, _) = 
                                                self.synthetic_data[self.synthetic_data.len() - 1].get_data() {
                                                    if let synthetic::SyntheticVariant::Circle(aim_center, _) = 
                                                        self.synthetic_data[synthetic::AIM_INDEX].get_data() {
                                                            self.synthetic_data[synthetic::AIM_INDEX]
                                                                .set_value(synthetic::SyntheticVariant::Circle(aim_center, 0.35));
                                                    } else {
                                                        log::error!("Пропал прицел!");

                                                        unreachable!("Произошла пропажа прицела");
                                                    }
                                            }
                                        },
                                        glutin::event::VirtualKeyCode::Key8 => if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                                            if let synthetic::SyntheticVariant::Circle(_, _) = 
                                                self.synthetic_data[self.synthetic_data.len() - 1].get_data() {
                                                    if let synthetic::SyntheticVariant::Circle(aim_center, _) = 
                                                        self.synthetic_data[synthetic::AIM_INDEX].get_data() {
                                                            self.synthetic_data[synthetic::AIM_INDEX]
                                                                .set_value(synthetic::SyntheticVariant::Circle(aim_center, 0.4));
                                                    } else {
                                                        log::error!("Пропал прицел!");

                                                        unreachable!("Произошла пропажа прицела");
                                                    }
                                            }
                                        },
                                        glutin::event::VirtualKeyCode::Key9 => if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                                            if let synthetic::SyntheticVariant::Circle(_, _) = 
                                                self.synthetic_data[self.synthetic_data.len() - 1].get_data() {
                                                    if let synthetic::SyntheticVariant::Circle(aim_center, _) = 
                                                        self.synthetic_data[synthetic::AIM_INDEX].get_data() {
                                                            self.synthetic_data[synthetic::AIM_INDEX]
                                                                .set_value(synthetic::SyntheticVariant::Circle(aim_center, 0.45));
                                                    } else {
                                                        log::error!("Пропал прицел!");

                                                        unreachable!("Произошла пропажа прицела");
                                                    }
                                            }
                                        },
                                        _ => {},
                                    }
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
