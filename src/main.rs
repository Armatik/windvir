#[macro_use]
extern crate glium;

mod defs;
mod graphics;
mod json;
mod etc;
mod ffi;
mod collisions;
mod control;

use glium::glutin;
use graphics::Vertex;
use std::{fs, env, collections::LinkedList};
use json::{geojson, default_json};
use defs::synthetic;


type WindowWidth = f32;
type WindowHeight = f32;


pub struct App {    
    p_g: geojson::PersistentG,
    p_j: default_json::PersistentJ,
    cam: graphics::Camera,
    window_size: (WindowWidth, WindowHeight),
    buildings: Vec<defs::Building>,
    synthetic_data: LinkedList<Box<dyn synthetic::SyntheticData>>,
    synthetic_datas_point: defs::Point,
    aim: defs::Point,
}


impl App {
    pub fn new(p_g: geojson::PersistentG, p_j: default_json::PersistentJ, def_buildings: Option<Vec<defs::Building>>) -> Self {
        let buildings = match def_buildings {
            Some(data) => data,
            None => Self::trans_persistent(&p_g),
        };

        Self {
            p_g,
            p_j,
            cam: graphics::Camera::default(),
            window_size: (p_j.resolution.width as f32, p_j.resolution.height as f32),
            buildings,
            synthetic_data: LinkedList::new(),
            synthetic_datas_point: defs::Point::default(),
            aim: defs::Point::new(-p_j.map_offset.x, -p_j.map_offset.y),
        }
    }

    pub fn start_app(self) -> Result<(), Box<dyn std::error::Error>> {
        let indices_line = graphics::get_line_indices(&self.buildings);
        let indices_triangulate = graphics::get_triangulation_indices(&self.buildings);
        let event_loop = glutin::event_loop::EventLoop::new();
        let window = glutin::window::WindowBuilder::new()
            .with_title(&self.p_g.name)
            .with_inner_size(glutin::dpi::LogicalSize::new(self.p_j.resolution.width, self.p_j.resolution.height));
        let context = if self.p_j.graphics.multisampling_on {
            if self.p_j.graphics.depth_buffering_on {
                glutin::ContextBuilder::new()
                    .with_gl(glutin::GlRequest::Latest)
                    .with_gl_profile(glutin::GlProfile::Core)
                    .with_pixel_format(0, 0)
                    .with_vsync(self.p_j.graphics.vsync_on)
                    .with_depth_buffer(self.p_j.graphics.depth_buffer)
                    .with_multisampling(self.p_j.graphics.multisampling)
            } else {
                glutin::ContextBuilder::new()
                    .with_gl(glutin::GlRequest::Latest)
                    .with_gl_profile(glutin::GlProfile::Core)
                    .with_pixel_format(0, 0)
                    .with_vsync(self.p_j.graphics.vsync_on)
                    .with_multisampling(self.p_j.graphics.multisampling)
            }
        } else {
            if self.p_j.graphics.depth_buffering_on {
                glutin::ContextBuilder::new()
                    .with_gl(glutin::GlRequest::Latest)
                    .with_gl_profile(glutin::GlProfile::Core)
                    .with_pixel_format(0, 0)
                    .with_vsync(self.p_j.graphics.vsync_on)
                    .with_depth_buffer(self.p_j.graphics.depth_buffer)
            } else {
                glutin::ContextBuilder::new()
                    .with_gl(glutin::GlRequest::Latest)
                    .with_gl_profile(glutin::GlProfile::Core)
                    .with_pixel_format(0, 0)
                    .with_vsync(self.p_j.graphics.vsync_on)
            }
        };
        let display = glium::Display::new(window, context, &event_loop)?;
        let mut shape = Vec::<Vertex>::with_capacity(self.buildings.len());

        for build in &self.buildings {
            for side in &build.sides {
                shape.push(Vertex { position: etc::vec_to_arr::<f32, 2>(vec![side.position.x, side.position.y]) })
            }
        }
        
        let positions = glium::VertexBuffer::new(&display, &shape)?;
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
        const CORRECTION_FACTOR: f32 = 1000.;
        let field_size = self.p_j.reverse_field_size; 
        let default_width = self.p_j.resolution.width as f32 / CORRECTION_FACTOR;
        let default_height = self.p_j.resolution.height as f32 / CORRECTION_FACTOR;
        let field_positions = glium::VertexBuffer::new(&display, &vec![
            graphics::ShaderVertex { position: [-1. / field_size * default_width, 1. / field_size * default_height], color: [1., 0., 0.] },
            graphics::ShaderVertex { position: [1. / field_size * default_width, 1. / field_size * default_height], color: [0., 1., 0.] },
            graphics::ShaderVertex { position: [1. / field_size * default_width, -1. / field_size * default_height], color: [0., 0., 1.] },
            graphics::ShaderVertex { position: [-1. / field_size * default_width, -1. / field_size * default_height], color: [1., 1., 0.] },
        ])?;
        let indices_field = glium::IndexBuffer::new(
            &display,
            glium::index::PrimitiveType::TrianglesList,
            &vec![0_u16, 1, 2, 0, 2, 3],
        )?;
        let vertex_shader_src = fs::read_to_string(graphics::VERTEX_SHADER_PATH)?;
        let color_shader_src = fs::read_to_string(graphics::COLOR_SHADER_PATH)?;
        let random_color_shader_src = fs::read_to_string(graphics::RANDOM_COLOR_SHADER_PATH)?;
        let field_vertex_shader_src = fs::read_to_string(graphics::FIELD_VERTEX_SHADER_PATH)?;
        let field_color_shader_src = fs::read_to_string(graphics::FIELD_COLOR_SHADER_PATH)?;
        let program = glium::Program::from_source(
            &display,
            &vertex_shader_src,
            &color_shader_src,
            None,
        )?;
        let random_program = glium::Program::from_source(
            &display,
            &vertex_shader_src,
            &random_color_shader_src,
            None,
        )?;
        let field_program = glium::Program::from_source(
            &display,
            &field_vertex_shader_src,
            &field_color_shader_src,
            None,
        )?;
        log::info!("Все здания успешно просчитаны и заданы!");

        self.window_loop(
            event_loop,
            display,
            positions,
            field_positions,
            program,
            random_program,
            field_program,
            indices_line,
            indices_triangulate,
            indices_field,
        )
    }
}


pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_env().unwrap();

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
                println!("Чтобы запустить программу с FFI режимом, используйте флаг -c.");

                return Ok(());
            } else if &arg == "-c" {
                log::info!("Приложение запущено с FFI режимом");

                ffi::ffi_loop()?;

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
