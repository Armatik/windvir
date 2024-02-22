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
use std::fs;


const VERTEX_SHADER_PATH: &str = "src/graphics/vertex_shader.vert";
const COLOR_SHADER_PATH: &str = "src/graphics/color_shader.vert";
const OFFSET_X: f32 = -45.395;
const OFFSET_Y: f32 = -52.293;
const BACKGROUND_R: f32 = 0.8;
const BACKGROUND_G: f32 = 0.98;
const BACKGROUND_B: f32 = 0.988;
const NOT_TRANSPARENT: f32 = 1.;
const ASPECT_RATIO_WIDTH: f32 = 16.;
const ASPECT_RATIO_HEIGHT: f32 = 9.;


enum TransformAction {
    Increase,
    Reduce,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    RotateRight,
    RotateLeft,
}


struct App {
    p: json::Persistent,
    scale: f32,
    offset_x: f32,
    offset_y: f32,
    theta: f32,
    transform_matrix: [[f32; 4]; 4],
}


impl App {
    pub fn new(_p: json::Persistent) -> Self {
        const SCALE: f32 = 200.;
        const THETA: f32 = 0.;

        Self {
            p: _p,
            scale: SCALE,
            offset_x: OFFSET_X,
            offset_y: OFFSET_Y,
            theta: THETA,
            transform_matrix: [
                [f32::cos(THETA) * SCALE * ASPECT_RATIO_HEIGHT / ASPECT_RATIO_WIDTH, -f32::sin(THETA), 0., 0.],
                [f32::sin(THETA), f32::cos(THETA) * SCALE, -f32::sin(THETA), 0.],
                [0., f32::sin(THETA), f32::cos(THETA), 0.],
                [0., 0., 0., 1.],
            ]
        }
    }

    fn transform_map(&mut self, action: TransformAction) {
        const OFFSET_SCALE: f32 = 7.;
        const OFFSET_WIDTH: f32 = 0.0001;
        const OFFSET_HEIGHT: f32 = 0.00007;
        const OFFSET_THETA: f32 = 0.04;
        let transform = |mat: &mut [[f32; 4]; 4], theta: f32, scale: f32| {
            mat[0][0] = f32::cos(theta) * scale * ASPECT_RATIO_HEIGHT / ASPECT_RATIO_WIDTH;
            mat[0][1] = -f32::sin(theta) * scale;
            mat[1][0] = f32::sin(theta) * scale;
            mat[1][1] = f32::cos(theta) * scale;
        };
        
        match action {
            TransformAction::Increase => {
                self.scale += OFFSET_SCALE;
                transform(&mut self.transform_matrix, self.theta, self.scale);
            },
            TransformAction::Reduce => {
                if self.scale - OFFSET_SCALE < 0. {
                    return;
                }

                self.scale -= OFFSET_SCALE;
                transform(&mut self.transform_matrix, self.theta, self.scale);
            },
            TransformAction::MoveUp => self.offset_y -= OFFSET_HEIGHT,
            TransformAction::MoveDown => self.offset_y += OFFSET_HEIGHT,
            TransformAction::MoveLeft => self.offset_x += OFFSET_WIDTH,
            TransformAction::MoveRight => self.offset_x -= OFFSET_WIDTH,
            TransformAction::RotateLeft => {
                self.theta += OFFSET_THETA;
                transform(&mut self.transform_matrix, self.theta, self.scale);
            },
            TransformAction::RotateRight => {
                self.theta -= OFFSET_THETA;
                transform(&mut self.transform_matrix, self.theta, self.scale);
            },
        }
    }

    fn render_frame(
        &self,
        display: &Display<WindowSurface>,
        positions: &VertexBuffer<Vertex>,
        indices: &IndexBuffer<u16>,
        program: &glium::Program,
        draw_parameters: &glium::DrawParameters<'_>,
    ) {
        let uniforms = uniform! { matrix: self.transform_matrix, x_off: self.offset_x, y_off: self.offset_y };
        let mut target = display.draw();
        target.clear_color(BACKGROUND_R, BACKGROUND_G, BACKGROUND_B, NOT_TRANSPARENT);
        target.draw(&*positions, &*indices, &program, &uniforms, &draw_parameters)
            .expect("Ошибка! Не удалось отрисовать кадр!");
        target.finish()
            .expect("Ошибка! Не удалось закончить отрисовку кадра!");
    }

    pub fn start_app(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut buildings = Vec::<Vec<Vec<f64>>>::with_capacity(self.p.features.len());

        for build in &self.p.features {
            buildings.push(build.geometry.coordinates[0][0].clone());
        }
        
        let indices = graphics::get_triangle_indices(&buildings);
        let event_loop = winit::event_loop::EventLoopBuilder::new()
            .build()?;
        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
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
        let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indices)?;        
        let vertex_shader_src = fs::read_to_string(VERTEX_SHADER_PATH)?;
        let color_shader_src = fs::read_to_string(COLOR_SHADER_PATH)?;
        let program = glium::Program::from_source(
            &display,
            &vertex_shader_src,
            &color_shader_src,
            None,
        )?;
        let params = glium::DrawParameters {
            polygon_mode: glium::draw_parameters::PolygonMode::Fill,
            multisampling: true,
            dithering: true,
            smooth: Some(glium::draw_parameters::Smooth::Nicest),
            ..Default::default()
        };
        self.render_frame(&display, &positions, &indices, &program, &params);

        event_loop.run(move |ev, window_target| {
            match ev {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => {
                        window_target.exit();
                    },
                    winit::event::WindowEvent::Resized(window_size) => {
                        self.render_frame(&display, &positions, &indices, &program, &params);
                        display.resize(window_size.into());
                    },
                    winit::event::WindowEvent::Focused(_) => self.render_frame(
                        &display,
                        &positions,
                        &indices,
                        &program,
                        &params,
                    ),
                    winit::event::WindowEvent::Moved(_) => self.render_frame(
                        &display,
                        &positions,
                        &indices,
                        &program,
                        &params,
                    ),
                    winit::event::WindowEvent::KeyboardInput { event, is_synthetic, .. } => {
                        if is_synthetic {
                            return;
                        }

                        match event.physical_key {
                            winit::keyboard::PhysicalKey::Code(key) => {
                                match key {
                                    winit::keyboard::KeyCode::KeyZ => self.transform_map(TransformAction::Increase),
                                    winit::keyboard::KeyCode::KeyX => self.transform_map(TransformAction::Reduce),
                                    winit::keyboard::KeyCode::KeyW => self.transform_map(TransformAction::MoveUp),
                                    winit::keyboard::KeyCode::KeyS => self.transform_map(TransformAction::MoveDown),
                                    winit::keyboard::KeyCode::KeyA => self.transform_map(TransformAction::MoveLeft),
                                    winit::keyboard::KeyCode::KeyD => self.transform_map(TransformAction::MoveRight),
                                    winit::keyboard::KeyCode::KeyQ => self.transform_map(TransformAction::RotateLeft),
                                    winit::keyboard::KeyCode::KeyE => self.transform_map(TransformAction::RotateRight),
                                    _ => return,
                                }

                                self.render_frame(&display, &positions, &indices, &program, &params);
                            },
                            winit::keyboard::PhysicalKey::Unidentified(_) => return,
                        }
                    }
                    _ => {},
                },
                winit::event::Event::AboutToWait => {
                    window.request_redraw();
                },
                _ => {},
            }
        })?;

        Ok(())
    }
}


pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let p = json::Persistent::default();
    
    let mut app = App::new(p);
    app.start_app()?;
    
    Ok(())
}
