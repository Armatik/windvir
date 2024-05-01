use crate::defs::{Building, Vector};


// Не пременимо для паралелельных отрезков, более медленный
fn _check_vector_intersection(first: &Vector, second: &Vector) -> bool {
    let s1 = get_segment_division_parameter(first, second);
    let s2 = get_segment_division_parameter(second, first);
    s1*s1 < s1 && s2*s2 < s2
}


fn check_vector_intersection(first: &Vector, second: &Vector) -> bool {
    let position_difference = &second.position - &first.position;
    let cross_product = Vector::cross(first,second);
    let s1 = position_difference.cross(&second.offset);
    let s2 = position_difference.cross(&first.offset);
    s1*s1 < s1*cross_product && s2*s2 < s2*cross_product
}


#[inline]
fn get_segment_division_parameter(first: &Vector, second: &Vector) -> f32 {
    (&second.position - &first.position).cross(&second.offset)/Vector::cross(first,second)
}


// Не оптимизировано
#[inline]
fn _check_bounding_box_intersection(first: &Building, second: &Building) -> bool {
    (first.start_point.x - second.end_point.x)*(first.end_point.x - second.start_point.x) < 0.
    &&
    (first.start_point.y - second.end_point.y)*(first.end_point.y - second.start_point.y) < 0.
}


#[inline]
fn check_bounding_box_intersection(first: &Building, second: &Building) -> bool {
    first.start_point.x < second.end_point.x
    &&
    (first.start_point.y - second.end_point.y)*(first.end_point.y - second.start_point.y) < 0.
}

// Не оптимизированно
fn _check_building_intersection(first: &Building, second: &Building) -> bool {
    if _check_bounding_box_intersection(first, second) {
        for first_building_side in first.sides.iter() {
            for second_building_side in second.sides.iter() {
                if check_vector_intersection(first_building_side, second_building_side) {
                    return true
                }
            }
        }
    }
    false
}


// Не оптимизировано
fn _optimize_map(sorted_map: &Vec<Building>) -> () {
    for i in 0usize..sorted_map.len().saturating_sub(1usize) {
        for n in i + 1usize..sorted_map.len() {
            if _check_building_intersection(&sorted_map[i], &sorted_map[n]) {
                todo!(); // Функция которая определит что делать с пересечениями
            }
        }
    }
}


fn check_building_intersection(first: &Building, second: &Building) -> bool {
    if check_bounding_box_intersection(first, second) {
        for first_building_side in first.sides.iter() {
            for second_building_side in second.sides.iter() {
                if check_vector_intersection(first_building_side, second_building_side) {
                    return true
                }
            }
        }
    }
    false
}


// Оптимизировано
fn optimize_map(sorted_map: &Vec<Building>) -> () {
    if sorted_map.len() < 2usize { return; }
    for i in 0usize..sorted_map.len() - 1usize {
        for n in i + 1usize..sorted_map.len() {
            if &sorted_map[i].end_point.x < &sorted_map[n].start_point.x { break; }
            if check_building_intersection(&sorted_map[i], &sorted_map[n]) {
                todo!(); // Функция которая определит что делать с пересечениями
            }
        }
    }
}