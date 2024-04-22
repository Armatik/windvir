mod keyboard;

use crate::{
    App,
    graphics::{self, Vertex, app as graphics_app},
    defs::app as defs_app,
};
use glium::{
    glutin::{self, event_loop::ControlFlow}, Display, VertexBuffer
};


pub enum MoveAim {
    Top,
    Right,
    Left,
    Down,
    Default,
}


impl App {
    pub fn window_loop<T>(
        mut self,
        event_loop: glutin::event_loop::EventLoop<()>,
        display: Display,
        positions: VertexBuffer<Vertex>,
        field_positions: VertexBuffer<graphics::ShaderVertex>,
        shaders: graphics_app::Shaders,
        indices: defs_app::FigureIndices<T>,
    ) -> ! where T: glium::index::Index {
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
                            self.resize_window(size);

                            self.render_frame(
                                &display,
                                &positions,
                                &field_positions,
                                &indices,
                                &shaders,
                            );
                        },
                        glutin::event::WindowEvent::Moved(_) => self.render_frame(
                            &display,
                            &positions,
                            &field_positions,
                            &indices,
                            &shaders,
                        ),
                        #[cfg(unix)]
                        glutin::event::WindowEvent::KeyboardInput { input, is_synthetic, .. } => {
                            if self.unix_keyboard_control(input, is_synthetic) {
                                self.render_frame(
                                    &display,
                                    &positions,
                                    &field_positions,
                                    &indices,
                                    &shaders,
                                );
                            }
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
                                &field_positions,
                                &indices,
                                &shaders,
                            );
                        },
                        _ => {},
                    }
                },
                #[cfg(windows)]
                glutin::event::Event::DeviceEvent { event, .. } => {
                    match event {
                        glutin::event::DeviceEvent::Key(key) => {
                            let need_rerender = match key.virtual_keycode {
                                Some(cap) => self.windows_keyboard_control(key, cap),
                                None => return,
                            };

                            if need_rerender {
                                self.render_frame(&display, &positions, &field_positions, &indices, &shaders);
                            }
                        },
                        _ => {},
                    }
                },
                _ => {},
            }
        });
    }
}
