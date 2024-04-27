use crate::{App, graphics, defs::synthetic};
use glium::glutin::{self, event};


macro_rules! check_last_for_default {
    ($data:ident) => {
        match $data.synthetic_data.back() {
            Some(figure) => if figure.is_value_default() {
                return false;
            },
            _ => {},
        }
    };
    (point $data:ident) => {
        match $data.synthetic_data.back() {
            Some(figure) => if figure.get_data_simply() == synthetic::SimplySyntheticVariant::Circle {
                return false;
            } else if !figure.is_value_default() {
                return false;
            },
            _ => return false,
        }
    };
}


impl App {
    #[cfg(unix)]
    pub fn unix_keyboard_control(&mut self, input: event::KeyboardInput, is_synthetic: bool) -> bool {
        const NUM1_KEY: u32 = 0x02;
        const NUM2_KEY: u32 = 0x03;
        const NUM3_KEY: u32 = 0x04;
        const NUM4_KEY: u32 = 0x05;
        const NUM5_KEY: u32 = 0x06;
        const NUM6_KEY: u32 = 0x07;
        const NUM7_KEY: u32 = 0x08;
        const NUM8_KEY: u32 = 0x09;
        const NUM9_KEY: u32 = 0x0a;
        const NUM0_KEY: u32 = 0x0b;
        const PLUS_KEY: u32 = 0x0d;
        const Q_KEY: u32 = 0x10;
        const W_KEY: u32 = 0x11;
        const E_KEY: u32 = 0x12;
        const R_KEY: u32 = 0x13;
        const P_KEY: u32 = 0x19;
        const LEFT_BRACKET: u32 = 0x1a;
        const RIGHT_BRACKET: u32 = 0x1b;
        const RETURN_KEY: u32 = 0x1c;
        const A_KEY: u32 = 0x1e;
        const S_KEY: u32 = 0x1f;
        const D_KEY: u32 = 0x20;
        const F_KEY: u32 = 0x21;
        const L_KEY: u32 = 0x26;
        const QUOTE_KEY: u32 = 0x28;
        const Z_KEY: u32 = 0x2c;
        const X_KEY: u32 = 0x2d;
        const C_KEY: u32 = 0x2e;
        const V_KEY: u32 = 0x2f;
        const DOT_KEY: u32 = 0x34;
        const ARROW_UP_KEY: u32 = 0x67;
        const ARROW_LEFT_KEY: u32 = 0x69;
        const ARROW_RIGHT_KEY: u32 = 0x6a;
        const ARROW_DOWN_KEY: u32 = 0x6c;
        
        if is_synthetic {
            return false;
        }

        let mut need_rerender = true;

        match input.scancode {
            V_KEY => if input.state == glutin::event::ElementState::Released {
                if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                    let data = match self.synthetic_data.back_mut() {
                        Some(data) => if data.is_value_default() {
                            log::warn!("Фигура еще не была задана!");

                            return false;
                        } else {
                            data
                        },
                        None => {
                            log::warn!("Ни одной фигуры еще не было создано!");

                            return false;
                        },
                    };

                    match data.change_primitive() {
                        Err(_) => log::warn!("Нельзя изменить режим отображения для последней созданной фигуры"),
                        _ => {},
                    };
                } else {
                    self.cam.display_type.change_visible_regime();
                }
            },
            P_KEY => if input.state == glutin::event::ElementState::Released {
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
                check_last_for_default!(self);
                self.define_figure(synthetic::Circle::new(), "Выберите размер для окружности, используя цифры 1..=9");

                need_rerender = false;
            },
            R_KEY => if self.cam.display_type == graphics::DisplayType::ObjectSpawn && input.state == glutin::event::ElementState::Released {
                check_last_for_default!(self);
                self.define_figure(synthetic::Rectangle::new(), "Отметьте 2 точки, используя <Enter>, чтобы создать прямоугольник");
                self.spawn_point(false);
            },
            L_KEY => if self.cam.display_type == graphics::DisplayType::ObjectSpawn && input.state == glutin::event::ElementState::Released {
                check_last_for_default!(self);
                self.define_figure(synthetic::Segment::new(), "Отметьте 2 точки, используя <Enter>, чтобы создать отрезок");
                self.spawn_point(false);
            },
            F_KEY => if self.cam.display_type == graphics::DisplayType::ObjectSpawn && input.state == glutin::event::ElementState::Released {
                if self.is_start_polygon() {
                    check_last_for_default!(self);
                    self.define_figure(synthetic::Polygon::new(), "Отмечайте точки, используя <Enter>, чтобы создать многоугольник");
                    self.spawn_point(false);
                } else if self.is_polygon() {
                    self.spawn_point(true);
                }
            },
            NUM0_KEY => self.transform_map(graphics::TransformAction::Default),
            value @ (NUM1_KEY | NUM2_KEY | NUM3_KEY | NUM4_KEY | NUM5_KEY | NUM6_KEY | NUM7_KEY | NUM8_KEY | NUM9_KEY) =>
                if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                    self.spawn_circle(value as f32);                               
            },
            PLUS_KEY => if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                self.move_aim(super::MoveAim::Top);
            },
            LEFT_BRACKET => if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                self.move_aim(super::MoveAim::Left);
            },
            RIGHT_BRACKET => if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                self.move_aim(super::MoveAim::Right);
            },
            QUOTE_KEY => if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                self.move_aim(super::MoveAim::Down);
            },
            DOT_KEY => if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                self.move_aim(super::MoveAim::Default);
            },
            RETURN_KEY => if self.cam.display_type == graphics::DisplayType::ObjectSpawn &&
                input.state == glutin::event::ElementState::Released {
                    check_last_for_default!(point self);

                    self.spawn_point(false);
            },
            _ => {},
        }
        
        need_rerender
    }

    #[cfg(windows)]
    pub fn windows_keyboard_control(&mut self, key: event::KeyboardInput, cap: event::VirtualKeyCode) -> bool {
        let mut need_rerender = true;

        match cap {
            glutin::event::VirtualKeyCode::V => if key.state == glutin::event::ElementState::Released {
                if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                    let data = match self.synthetic_data.back_mut() {
                        Some(data) => if data.is_value_default() {
                            log::warn!("Фигура еще не была задана!");

                            return false;
                        } else {
                            data
                        },
                        None => {
                            log::warn!("Ни одной фигуры еще не было создано!");

                            return false;
                        },
                    };

                    match data.change_primitive() {
                        None => log::warn!("Нельзя изменить режим отображения для последней созданной фигуры"),
                        _ => {},
                    };
                } else {
                    self.cam.display_type.change_visible_regime();
                }
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
                check_last_for_default!(self);
                self.define_figure(synthetic::Circle::new(None), "Выберите размер для окружности, используя цифры 1..=9");

                need_rerender = false;
            },
            glutin::event::VirtualKeyCode::R => if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                check_last_for_default!(self);
                self.define_figure(synthetic::Rectangle::new(None), "Отметьте 2 точки, используя <Enter>, чтобы создать прямоугольник");

                need_rerender = false
            },
            glutin::event::VirtualKeyCode::L => if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                check_last_for_default!(self);
                self.define_figure(synthetic::Segment::new(None), "Отметьте 2 точки, используя <Enter>, чтобы создать отрезок");

                need_rerender = false;
            },
            glutin::event::VirtualKeyCode::Key0 => self.transform_map(graphics::TransformAction::Default),
            value @ (glutin::event::VirtualKeyCode::Key1 | glutin::event::VirtualKeyCode::Key2 | 
                glutin::event::VirtualKeyCode::Key3 | glutin::event::VirtualKeyCode::Key4 |
                glutin::event::VirtualKeyCode::Key5 | glutin::event::VirtualKeyCode::Key6 |
                glutin::event::VirtualKeyCode::Key7 | glutin::event::VirtualKeyCode::Key8 |
                glutin::event::VirtualKeyCode::Key9) =>  if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                    self.spawn_circle(value as u32 as f32);
            },
            glutin::event::VirtualKeyCode::Plus => if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                self.move_aim(super::MoveAim::Top);
            },
            glutin::event::VirtualKeyCode::LBracket => if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                self.move_aim(super::MoveAim::Left);
            },
            glutin::event::VirtualKeyCode::RBracket => if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                self.move_aim(super::MoveAim::Right);
            },
            glutin::event::VirtualKeyCode::Apostrophe => if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                self.move_aim(super::MoveAim::Down);
            },
            glutin::event::VirtualKeyCode::Period => if self.cam.display_type == graphics::DisplayType::ObjectSpawn {
                self.move_aim(super::MoveAim::Default);
            },
            glutin::event::VirtualKeyCode::Return => if self.cam.display_type == graphics::DisplayType::ObjectSpawn &&
                key.state == glutin::event::ElementState::Released {
                    check_last_for_default!(point self);

                    self.spawn_point();
            },
            _ => {},
        };

        need_rerender
    }
}