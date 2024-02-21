#[macro_use]
extern crate glium;

mod graphics;
mod json;
mod etc;

use glium::Surface;
use graphics::Vertex;
use std::fs;


const VERTEX_SHADER_PATH: &str = "src/graphics/vertex_shader.vert";
const COLOR_SHADER_PATH: &str = "src/graphics/color_shader.vert";
const TRANSFORM_MATRIX: [[f32; 4]; 4] = [
    [1., 0., 0., 0.],
    [0., 1., 0., 0.],
    [0., 0., 1., 0.],
    [0., 0., 0., 1.],
];
const OFFSET_X: f32 = -45.;
const OFFSET_Y: f32 = -52.;


struct App {
    p: json::Persistent,
}


impl App {
    pub fn new(_p: json::Persistent) -> Self {
        Self {
            p: _p,
        }
    }

    fn get_indices(buildings: &Vec<Vec<Vec<f64>>>) -> Vec<u16> {
        let mut indices = Vec::<u16>::new();
        let mut index = 0;

        for building in buildings {
            let last_iter = building.len() - 1;
            let penultimate_iter = building.len() - 2;
            let init_index = index;

            'point_loop: for i in 0..building.len() {
                if i == last_iter {
                    indices.append(&mut vec![index, init_index, init_index + 1]);
                    index += 1;

                    continue 'point_loop;
                }

                if i == penultimate_iter {
                    indices.append(&mut vec![index, index + 1, init_index]);
                    index += 1;

                    continue 'point_loop;
                }

                for j in i..building.len() {
                    indices.append(&mut vec![index, index + 1, init_index + j as u16]);
                }

                index += 1;
            }
        }

        indices
    }

    pub fn start_app(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut buildings = Vec::<Vec<Vec<f64>>>::with_capacity(self.p.features.len());

        for build in &self.p.features {
            buildings.push(build.geometry.coordinates[0][0].clone());
        }
        
        println!("{:?}", buildings);
        // let buildings = vec![vec![vec![0.2, 0.2], vec![0.2, 0.4], vec![0.4, 0.4], vec![0.4, 0.6], vec![0.6, 0.6],
            // vec![0.6, 0.4], vec![0.8, 0.4], vec![0.8, 0.2]], vec![vec![45.2, 52.2], vec![45.2, 52.7], vec![45.7, 52.7], vec![45.7, 52.2]]];
        let indices = Self::get_indices(&buildings);
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
            None
        )?;
        let mut target = display.draw();
        target.clear_color(0.8, 0.98, 0.988, 1.0);
        let uniforms = uniform! { matrix: TRANSFORM_MATRIX, x_off: OFFSET_X, y_off: OFFSET_Y };
        target.draw(&positions, &indices, &program, &uniforms,&Default::default())?;
        target.finish()?;

        event_loop.run(move |ev, window_target| {
            match ev {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => {
                        window_target.exit();
                    },
                    winit::event::WindowEvent::Resized(window_size) => {
                        display.resize(window_size.into());
                    },
                    winit::event::WindowEvent::Focused(_) => {
                        let mut target = display.draw();
                        target.clear_color(0.8, 0.98, 0.988, 1.0);
                        target.draw(&positions, &indices, &program, &uniforms,&Default::default())
                            .expect("Ошибка! Не удалось отрисовать кадр!");
                        target.finish()
                            .expect("Ошибка! Не удалось закончить отрисовку кадра!");
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
