#[macro_use]
extern crate glium;

pub mod defs;
mod graphics;
mod json;
mod etc;
mod ffi;
mod collisions;
mod control;

use glium::glutin::{self, dpi, window};
use std::{env, collections::LinkedList};
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
    rainbow_field: bool,
}


impl App {
    pub fn new(p_g: geojson::PersistentG, p_j: default_json::PersistentJ, def_buildings: Option<Vec<defs::Building>>, rainbow_field: bool) -> Self {
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
            rainbow_field,
        }
    }

    pub fn start_app(self) -> Result<(), Box<dyn std::error::Error>> {
        let event_loop = glutin::event_loop::EventLoop::new();
        let (icon, (width, height)) = graphics::get_icon()?;
        let icon = window::Icon::from_rgba(icon, width, height)?;
        let window = glutin::window::WindowBuilder::new()
            .with_title(&self.p_g.name)
            .with_window_icon(Some(icon))
            .with_inner_size(glutin::dpi::LogicalSize::new(self.p_j.resolution.width, self.p_j.resolution.height));
        let context = self.get_window_ctx(); 
        let display = glium::Display::new(window, context, &event_loop)?;

        let shape = self.get_buildings_vertices();
        let building_vertices = glium::VertexBuffer::new(&display, &shape)?;
        
        let field_positions = self.init_field(self.rainbow_field, &display)?;
        let shaders = self.init_shaders(&display)?;

        let indices = self.init_indices(&display)?;
        
        log::info!("Все здания успешно просчитаны и заданы!");

        self.window_loop(
            event_loop,
            display,
            building_vertices,
            field_positions,
            shaders,
            indices,
        )
    }

    fn get_window_ctx(&self) ->  glutin::ContextBuilder<'_, glutin::NotCurrent> {
        return if self.p_j.graphics.multisampling_on {
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
    }

    pub fn resize_window(&mut self, size: dpi::PhysicalSize<u32>) {
        self.window_size.0 = size.width as f32;
        self.window_size.1 = size.height as f32;
        self.transform_map(graphics::TransformAction::Resize);
    }
}


pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_env().unwrap();

    let args = env::args();
    let args_len = args.len();
    let mut is_first_arg = true;
    let mut rainbow = false;
    let p_g = geojson::PersistentG::default();
    let p_j = default_json::PersistentJ::default();
    let mut ffi_buildigs = Vec::<defs::Building>::with_capacity(p_g.features.len());
    let mut ffi_out = ffi::BuildingsVec::default();

    for arg in args {
        if args_len > 1 {
            if is_first_arg {
                is_first_arg = false;

                continue;
            }

            if &arg == "--help" || &arg == "-h" {
                etc::print_help();

                return Ok(());
            } else if &arg == "-c" {
                log::info!("Приложение запущено с FFI режимом");

                ffi_out = ffi::ffi_loop(&mut ffi_buildigs, &p_g)?;
            } else if &arg == "-r" {
                log::info!("Приложение запущено с разноцветным полем");

                rainbow = true;
            } else {
                panic!("Неизвестный аргумент {}", arg);
            }
        }
        
    }
    
    let app = if ffi_buildigs == Vec::new() {
        App::new(p_g, p_j, None, rainbow)
    } else {
        App::new(p_g, p_j, Some(ffi_buildigs), rainbow)
    };

    app.start_app()?;

    if !ffi_out.buildings.is_null() {
        unsafe { ffi::freeBuildings(ffi_out); };
    }
    
    Ok(())
}
