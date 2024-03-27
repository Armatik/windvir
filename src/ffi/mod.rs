#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Building {
    pub data: *mut *mut f64,
    pub len_vertex: u64,
}


impl Building {
    fn new(mut data: Vec<Vec<f64>>) -> Self {
        let mut out_data = data.iter_mut().map(|x| x.as_mut_ptr()).collect::<Vec<*mut f64>>();

        Self {
            len_vertex: out_data.len() as u64,
            data: out_data.as_mut_ptr(),
        }
    }
}


#[repr(C)]
#[derive(Debug)]
pub struct Data {
    pub data: *mut Building,
    pub len_buildings: u64,
}


impl Data {
    pub fn new(mut data: Vec<Vec<Vec<f64>>>) -> Self {
        let mut out_data = data.iter_mut().map(|x| 
            Building::new(x.clone())).collect::<Vec<Building>>();
        
        println!("{out_data:?}");

        Self {
            len_buildings: out_data.len() as u64,
            data: out_data.as_mut_ptr(),
        }
    }
}


extern "C" {
    pub fn c_func_test(_: Data) -> Data;
}
