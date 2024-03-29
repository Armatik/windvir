use std::alloc::{alloc, Layout};


#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Building {
    pub data: *mut *mut f64,
    pub len_vertex: u64,
}


impl Building {
    unsafe fn new(data: &mut Vec<Vec<f64>>) -> Self {
        let layout = Layout::array::<*mut f64>(data.len()).expect("Overflow");
        let out_data = unsafe { alloc(layout).cast::<*mut f64>() };

        if out_data.is_null() {
            panic!("Произошло переполнение памяти!");
        }

        for (i, data) in data.iter_mut().enumerate() {
            let layout = Layout::array::<usize>(2).expect("Overflow");
            let point = unsafe { alloc(layout).cast::<f64>() };

            if point.is_null() {
                panic!("Произошло переполнение памяти!");
            }

            unsafe {
                point.offset(0).write(data[0]);
                point.offset(1).write(data[1]);
                out_data.offset(i as isize).write(point);
            };
        }

        Self {
            len_vertex: data.len() as u64,
            data: out_data,
        }
    }
}


#[repr(C)]
#[derive(Debug)]
pub struct BuildingsVec {
    pub data: *mut Building,
    pub len_buildings: u64,
}


impl BuildingsVec {
    pub unsafe fn new(mut data: Vec<Vec<Vec<f64>>>) -> Self {
        let layout = Layout::array::<Building>(data.len()).expect("Owrflow");
        let out_data = unsafe { alloc(layout).cast::<Building>() };

        if out_data.is_null() {
            panic!("Произошло переполнение памяти!");
        }

        for (i, data) in data.iter_mut().enumerate() {
            unsafe { out_data.offset(i as isize).write(Building::new(data)); };
        }
        
        Self {
            len_buildings: data.len() as u64,
            data: out_data,
        }
    }
}


extern "C" {
    pub fn changeVertex(_: BuildingsVec) -> BuildingsVec;
    pub fn freeBuildings(_: BuildingsVec);
}
