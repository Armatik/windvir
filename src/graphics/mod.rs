#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f64; 2],
}


pub fn get_triangle_indices(buildings: &Vec<Vec<Vec<f64>>>) -> Vec<u16> {
    let mut indices = Vec::<u16>::new();
    let mut index = 0;

    for building in buildings {
        let last_iter = building.len() - 1;
        let penultimate_iter = building.len() - 2;
        let init_index = index;

        'point_loop: for i in 0..building.len() {
            if i == last_iter {
                indices.append(&mut vec![index, init_index, init_index + 1]);
                index += 1;

                continue 'point_loop;
            }

            if i == penultimate_iter {
                indices.append(&mut vec![index, index + 1, init_index]);
                index += 1;

                continue 'point_loop;
            }

            for j in i..building.len() {
                indices.append(&mut vec![index, index + 1, init_index + j as u16]);
            }

            index += 1;
        }
    }

    indices
}
