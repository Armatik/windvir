use std::alloc::{alloc, Layout};
use crate::json::{default_json, geojson};
use crate::graphics;


#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PointC {
    pub x: f64,
    pub y: f64,
}


impl PointC {
    fn new(_x: f64, _y: f64) -> Self {
        Self {
            x: _x,
            y: _y,
        }
    }
}


#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BuildingC {
    pub center: PointC,
    pub radius: f64,
    pub points: *mut PointC,
    pub len_vertex: u64,
}


impl BuildingC {
    unsafe fn new(data: &mut graphics::Building) -> Self {
        let layout = Layout::array::<PointC>(data.points.len()).expect("Выделено неверное кол-во памяти");
        let out_data = unsafe { alloc(layout).cast::<PointC>() };

        if out_data.is_null() {
            panic!("Произошло переполнение памяти!");
        }

        for (i, point) in data.points.iter_mut().enumerate() {
            let point = PointC::new(point.x, point.y);
            unsafe { out_data.offset(i as isize).write(point); };
        }

        Self {
            center: PointC::new(data.center.x, data.center.y),
            radius: data.radius,
            points: out_data,
            len_vertex: data.points.len() as u64,
        }
    }
}


#[repr(C)]
#[derive(Debug)]
struct BuildingsVec {
    buildings: *mut BuildingC,
    len_buildings: u64,
}


impl BuildingsVec {
    pub unsafe fn new(mut data: Vec<graphics::Building>) -> Self {
        let layout = Layout::array::<BuildingC>(data.len()).expect("Выделено неверное кол-во памяти");
        let out_data = unsafe { alloc(layout).cast::<BuildingC>() };

        if out_data.is_null() {
            panic!("Произошло переполнение памяти!");
        }

        for (i, data) in data.iter_mut().enumerate() {
            unsafe { out_data.offset(i as isize).write(BuildingC::new(data)); };
        }
        
        Self {
            buildings: out_data,
            len_buildings: data.len() as u64,
        }
    }
}


pub fn ffi_loop() -> Result<(), Box<dyn std::error::Error>> {
    let p_g = geojson::PersistentG::default();
    let p_j = default_json::PersistentJ::default();
    let data = crate::App::trans_persistent(&p_g);
    let data = unsafe { BuildingsVec::new(data) };

    let out = unsafe { changeVertex(data) };
    
    let mut norm_buildings = Vec::<graphics::Building>::with_capacity(p_g.features.len());
    let buildings = unsafe { Vec::from_raw_parts(out.buildings, out.len_buildings as usize, out.len_buildings as usize) };
    
    for building in buildings {
        let mut buildings_vertex = Vec::<graphics::Point>::with_capacity(building.len_vertex as usize);
        let building_points = unsafe { Vec::from_raw_parts(
            building.points, building.len_vertex as usize, building.len_vertex as usize
        ) };

        for vertex in building_points {
            buildings_vertex.push(graphics::Point::repr_rust(vertex));
        }

        norm_buildings.push(graphics::Building {
            center: graphics::Point::new(building.center.x, building.center.y),
            radius: building.radius,
            points: buildings_vertex,
        })
    }

    let app = crate::App::new(p_g, p_j, Some(norm_buildings));
    app.start_app()?;
    unsafe { freeBuildings(out); };

    Ok(())

}


extern "C" {
    fn changeVertex(_: BuildingsVec) -> BuildingsVec;
    fn freeBuildings(_: BuildingsVec);
}