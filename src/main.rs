#[macro_use]
pub extern crate glium;

mod graphics;
mod json;

use glium::Surface;
use std::{
    thread,
    sync::{Mutex, Arc},
};
use winit::event_loop;


struct App {
    p: json::Persistent,
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
}


impl App {
    pub fn new(_p: json::Persistent) -> Self {
        Self {
            p: _p,
            min_x: f64::MAX,
            max_x: f64::MIN,
            min_y: f64::MAX,
            max_y: f64::MIN,
        }
    }

    /// Пока максимально бесполезная функция
    fn find_max_and_min(&mut self, coords: &Vec<Vec<Vec<f64>>>) {
        let _ = coords.iter().map(|_x| {
            let x = _x[0][0];
            let y = _x[0][1];

            if x > self.max_x {
                self.max_x = x;
            } else if x < self.min_x { 
                self.min_x = x;
            }
            if y > self.max_y {
                self.max_y = y;
            } else if y < self.min_y {
                self.min_y = y;
            }
        });
    }

    pub fn start_app(&mut self) {
        todo!()
    }
}


pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let p = json::Persistent::default();
    
    let mut app = App::new(p);
    app.start_app();
    
    Ok(())
}
