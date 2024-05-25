mod keyboard;

use crate::{
    App,
    graphics::app as graphics_app,
    defs::app as defs_app,
};
use glium::{
    glutin::{self, event_loop::ControlFlow}, Display,
};


pub enum MoveAim {
    Top,
    Right,
    Left,
    Down,
    Default,
}


impl App {
    pub fn window_loop(
        mut self,
        event_loop: glutin::event_loop::EventLoop<()>,
        display: Display,
        mut positions: defs_app::Positions,
        shaders: graphics_app::Shaders,
        mut indices: defs_app::FigureIndices<u16>,
    ) -> ! {
        let mut params = glium::DrawParameters {
            multisampling: self.p_j.graphics.multisampling_on,
            dithering: self.p_j.graphics.dithering_on,
            smooth: Some(glium::draw_parameters::Smooth::Nicest),
            ..Default::default()
        };

        event_loop.run(move |ev, _, control_flow| match ev {
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
                            &mut params,
                            &positions,
                            &indices,
                            &shaders,
                        );
                    },
                    glutin::event::WindowEvent::Moved(_) => self.render_frame(
                        &display,
                        &mut params,
                        &positions,
                        &indices,
                        &shaders,
                    ),
                    #[cfg(unix)]
                    glutin::event::WindowEvent::KeyboardInput { input, is_synthetic, .. } => {
                        if self.unix_keyboard_control(input, is_synthetic, &display, &mut positions, &mut indices) {
                            self.render_frame(
                                &display,
                                &mut params,
                                &positions,
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
                            &mut params,
                            &positions,
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
                            Some(cap) => self.windows_keyboard_control(
                                key,
                                cap,
                                &display,
                                &mut positions,
                                &mut indices,
                            ),
                            None => return,
                        };

                        if need_rerender {
                            self.render_frame(&display, &mut params, &positions, &indices, &shaders);
                        }
                    },
                    _ => {},
                }
            },
            _ => {},
        });
    }
}
