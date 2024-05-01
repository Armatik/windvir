use std::alloc::{alloc, Layout};
use crate::json::geojson::PersistentG;
use crate::defs;


#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct PointC<T> where T: num::Float + Default {
    x: T,
    y: T,
}


impl<T> PointC<T> where T: num::Float + Default {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn repr_rust(self) -> defs::PositionVector<T> {
        defs::PositionVector {
            x: self.x,
            y: self.y,
        }
    }
}


#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BuildingC {
    start_point: PointC<f64>,
    end_point: PointC<f64>,
    sides: *mut VectorC<f64>,
    len_vertex: u64,
}


impl BuildingC {
    unsafe fn new(data: &mut defs::Building) -> Self {
        let layout = Layout::array::<VectorC<f64>>(data.sides.len()).expect("Выделено неверное кол-во памяти");
        let out_data = unsafe { alloc(layout).cast::<VectorC<f64>>() };

        if out_data.is_null() {
            panic!("Произошло переполнение памяти!");
        }

        for (i, side) in data.sides.iter_mut().enumerate() {
            let point = VectorC::new(
                PointC::new(side.position.x,side.position.y),
                PointC::new(side.offset.x, side.offset.y)
            );
            unsafe { out_data.offset(i as isize).write(point); };
        }

        Self {
            start_point: PointC::new(data.start_point.x, data.end_point.y),
            end_point: PointC::new(data.end_point.x,data.end_point.y),
            sides: out_data,
            len_vertex: data.sides.len() as u64,
        }
    }
}


#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct VectorC<T> where T: num::Float + Default {
    position: PointC<T>,
    offset: PointC<T>,
}


impl<T> VectorC<T> where T: num::Float + Default {

    fn new(position: PointC<T>, offset: PointC<T>) -> Self {
        Self {
            position,
            offset
        }
    }

    pub fn repr_rust(self) -> defs::Vector<T> {
        defs::Vector { 
            position: self.position.repr_rust(),
            offset: self.offset.repr_rust()
        }
    }
}


#[repr(C)]
#[derive(Debug)]
pub struct BuildingsVec {
    pub buildings: *mut BuildingC,
    len_buildings: u64,
}


impl Default for BuildingsVec {
    fn default() -> Self {
        Self {
            buildings: std::ptr::null_mut(),
            len_buildings: u64::default(),
        }
    }
}


impl BuildingsVec {
    pub unsafe fn new(mut data: Vec<defs::Building>) -> Self {
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


pub fn ffi_loop(norm_buildings: &mut Vec::<defs::Building>, p_g: &PersistentG) -> Result<BuildingsVec, Box<dyn std::error::Error>> {
    let data = crate::App::trans_persistent(p_g);
    let data = unsafe { BuildingsVec::new(data) };

    let out = unsafe { changeVertex(data) };
    
    let buildings = unsafe { Vec::from_raw_parts(out.buildings, out.len_buildings as usize, out.len_buildings as usize) };
    
    for building in buildings {
        let mut buildings_vertex = Vec::<defs::Vector<f64>>::with_capacity(building.len_vertex as usize);
        let building_points = unsafe { Vec::from_raw_parts(
            building.sides, building.len_vertex as usize, building.len_vertex as usize
        ) };

        for vertex in building_points {
            buildings_vertex.push(vertex.repr_rust());
        }

        norm_buildings.push(defs::Building {
            start_point: building.start_point.repr_rust(),
            end_point: building.end_point.repr_rust(),
            sides: buildings_vertex,
        })
    }

    Ok(out)
}


extern "C" {
    fn changeVertex(_: BuildingsVec) -> BuildingsVec;
    pub fn freeBuildings(_: BuildingsVec);
}
