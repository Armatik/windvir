use std::alloc::{alloc, Layout};
use crate::json::{default_json, geojson};
use crate::defs;


#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PointC {
    pub x: f32,
    pub y: f32,
}


impl PointC {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn repr_rust(self) -> defs::PositionVector {
        defs::PositionVector {
            x: self.x,
            y: self.y,
        }
    }
}


#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BuildingC {
    pub start_point: PointC,
    pub end_point: PointC,
    pub sides: *mut VectorC,
    pub len_vertex: u64,
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
pub struct VectorC {
    pub position: PointC,
    pub offset: PointC,
}


impl VectorC {

    fn new(position: PointC, offset: PointC) -> Self {
        Self {
            position,
            offset
        }
    }

    pub fn repr_rust(self) -> defs::Vector {
        defs::Vector { 
            position: self.position.repr_rust(),
            offset: self.offset.repr_rust()
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


pub fn ffi_loop() -> Result<(), Box<dyn std::error::Error>> {
    let p_g = geojson::PersistentG::default();
    let p_j = default_json::PersistentJ::default();
    let data = crate::App::trans_persistent(&p_g);
    let data = unsafe { BuildingsVec::new(data) };

    let out = unsafe { changeVertex(data) };
    
    let mut norm_buildings = Vec::<defs::Building>::with_capacity(p_g.features.len());
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

    let app = crate::App::new(p_g, p_j, Some(norm_buildings));
    app.start_app()?;
    unsafe { freeBuildings(out); };

    Ok(())

}


extern "C" {
    fn changeVertex(_: BuildingsVec) -> BuildingsVec;
    fn freeBuildings(_: BuildingsVec);
}
