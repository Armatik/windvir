use std::alloc::{alloc, Layout};
use crate::json::geojson::PersistentG;
use crate::defs;


pub trait ReprRust {
    type RustOutput;

    fn repr_rust(self) -> Self::RustOutput;
}


#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct PointC {
    x: f64,
    y: f64,
}


impl PointC {
    fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
        }
    }
}


impl ReprRust for PointC {
    type RustOutput = defs::PositionVector;
    
    fn repr_rust(self) -> Self::RustOutput {
        defs::PositionVector::new(self.x, self.y)
    }
}


#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BuildingC {
    start_point: PointC,
    end_point: PointC,
    sides: *mut VectorC,
    len_vertex: u64,
}


impl ReprRust for BuildingC {
    type RustOutput = defs::Building;

    fn repr_rust(self) -> Self::RustOutput {
        let sides = unsafe { Vec::from_raw_parts(self.sides, self.len_vertex as usize, self.len_vertex as usize) };
        let sides = sides.iter().map(|x| x.repr_rust()).collect::<Vec<defs::Vector>>();

        defs::Building {
            start_point: self.start_point.repr_rust(),
            end_point: self.end_point.repr_rust(),
            sides,
        }
    }
}


impl BuildingC {
    unsafe fn new(data: &mut defs::Building) -> Self {
        let layout = Layout::array::<VectorC>(data.sides.len()).expect("Выделено неверное кол-во памяти");
        let out_data = unsafe { alloc(layout).cast::<VectorC>() };

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
struct VectorC {
    position: PointC,
    offset: PointC,
}


impl VectorC {

    fn new(position: PointC, offset: PointC) -> Self {
        Self {
            position,
            offset
        }
    }
}


impl ReprRust for VectorC {
    type RustOutput = defs::Vector;
    
    fn repr_rust(self) -> Self::RustOutput {
        defs::Vector::new(self.position.repr_rust(), self.offset.repr_rust())
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


impl ReprRust for BuildingsVec {
    type RustOutput = Vec<defs::Building>;

    fn repr_rust(self) -> Self::RustOutput {
        let buildings = unsafe { Vec::from_raw_parts(self.buildings, self.len_buildings as usize, self.len_buildings as usize) };
        
        buildings.iter().map(|x| x.repr_rust()).collect::<Vec<defs::Building>>()
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


pub fn ffi_loop(norm_buildings: &mut Vec::<defs::Building>, p_g: &PersistentG) -> Result<(), Box<dyn std::error::Error>> {
    let data = crate::App::trans_persistent(p_g);
    let data = unsafe { BuildingsVec::new(data) };

    let out = unsafe { changeVertex(data) };
    
    let buildings = unsafe { Vec::from_raw_parts(out.buildings, out.len_buildings as usize, out.len_buildings as usize) };
    
    for building in buildings {
        let mut buildings_vertex = Vec::<defs::Vector>::with_capacity(building.len_vertex as usize);
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

    Ok(())
}


extern "C" {
    fn changeVertex(_: BuildingsVec) -> BuildingsVec;
    pub fn merge_buildings(_: *mut BuildingsVec) -> *const BuildingC;
    pub fn nc_hull_maker(_: *mut BuildingsVec, _: f64) -> *const BuildingC;
}
