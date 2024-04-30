#[macro_use]
extern crate glium;

pub mod defs;
mod graphics;
mod json;
mod etc;
mod ffi;
mod collisions;
mod control;

use glium::glutin::{self, dpi, window, event_loop as ev_loop};
use std::{env, collections::LinkedList};
use json::{geojson, default_json, figures};
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
    synthetic_datas_points: Vec<defs::Point>,
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
            synthetic_datas_points: Vec::new(),
            aim: defs::Point::new(-p_j.map_offset.x, -p_j.map_offset.y),
            rainbow_field,
        }
    }

    fn init_window(&self) -> Result<(glium::Display, ev_loop::EventLoop<()>), Box<dyn std::error::Error>> {
        let event_loop = glutin::event_loop::EventLoop::new();
        let (icon, (width, height)) = graphics::get_icon()?;
        let icon = window::Icon::from_rgba(icon, width, height)?;
        let window = glutin::window::WindowBuilder::new()
            .with_title(&self.p_g.name)
            .with_window_icon(Some(icon))
            .with_inner_size(glutin::dpi::LogicalSize::new(self.p_j.resolution.width, self.p_j.resolution.height));
        let context = self.get_window_ctx(); 

        Ok((glium::Display::new(window, context, &event_loop)?, event_loop))
    }

    fn init_figures_cfg(&mut self) {
        let json = figures::PersistentF::default();
        let x_off = self.p_j.map_offset.x;
        let y_off = self.p_j.map_offset.y;

        if let Some(circles) = json.circles {
            circles.iter().for_each(|x| {
                let _x = x.x - x_off;
                let _y = x.y - y_off;
                let radius = x.radius;
                let rgb = x.rgb;
    
                self.define_figure(
                    synthetic::Circle::init(_x, _y, radius, x.is_fill, rgb),
                    &format!("Окружность была задана с центром ({_x} {_y}) и с радиусом {radius} и цветом {:?}", rgb),
                );
            });
        }
        
        if let Some(rectangles) = json.rectangles {
            rectangles.iter().for_each(|x| {
                let lu_x = x.left_up_angle_x - x_off;
                let lu_y = x.left_up_angle_y - y_off;
                let rd_x = x.right_down_angle_x - x_off;
                let rd_y = x.right_down_angle_y - y_off;
                let rgb = x.rgb;
    
                self.define_figure(
                    synthetic::Rectangle::init(lu_x, lu_y, rd_x, rd_y, x.is_fill, rgb),
                    &format!("Прямоугольник был задан с левым верхним углом ({lu_x} {lu_y}) и правым нижним ({rd_x} {rd_y}), еще имеет цвет {:?}", rgb),
                );
            });
        }
        
        if let Some(lines) = json.lines {
            lines.iter().for_each(|x| {
                let p0_x = x.p0_x - x_off;
                let p0_y = x.p0_y - y_off;
                let p1_x = x.p1_x - x_off;
                let p1_y = x.p1_y - y_off;
                let rgb = x.rgb;
    
                self.define_figure(
                    synthetic::Segment::init(p0_x, p0_y, p1_x, p1_y, rgb),
                    &format!("Отрезок был задан с начальной точкой ({p0_x} {p0_y}) и конечной ({p1_x} {p1_y}), а так же цветом {:?}", rgb),
                );
            });
        }
        
        if let Some(polygons) = json.polygons {
            polygons.iter().for_each(|x| {
                let points = x.points.clone().iter().map(|x| vec![x[0] - x_off, x[1] - y_off]).collect::<Vec<Vec<f32>>>();
                let rgb = x.rgb;
    
                self.define_figure(
                    synthetic::Polygon::init(points.clone(), x.is_fill, rgb),
                    &format!("Многоугольник был задан с точками {:?} и цветом {:?}", points, rgb),
                );
            });
        }
        
    }

    pub fn start_app(mut self, default_buildings: Vec<defs::Building>) -> Result<(), Box<dyn std::error::Error>> {
        let (display, event_loop) = self.init_window()?;

        // ================================ Инициализация индексов и вершин ================================
        let positions = self.init_positions(&display, &default_buildings)?;
        let shaders = self.init_shaders(&display)?;

        let indices = self.init_indices(&display, default_buildings)?;

        self.init_figures_cfg();
        // =================================================================================================
        
        log::info!("Все здания успешно просчитаны и заданы!");

        self.window_loop(
            event_loop,
            display,
            positions,
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
    let default_buildings = crate::App::trans_persistent(&p_g);

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

    app.start_app(default_buildings)?;

    if !ffi_out.buildings.is_null() {
        unsafe { ffi::freeBuildings(ffi_out); };
    }

    Ok(())
}
