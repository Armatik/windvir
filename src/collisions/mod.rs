use crate::defs::{Building, PositionVector, Vector};


// Не пременимо для паралелельных отрезков, более медленный
fn _test_vector_intersection(first: &Vector, second: &Vector) -> bool {
    let s1 = get_segment_division_parameter(first, second);
    let s2 = get_segment_division_parameter(second, first);
    s1*s1 < s1 && s2*s2 < s2
}


// На базе скалярного произведения
/// Вычисление минимальной квадрата растояния между двумя параллельными отрезками.
fn get_distance_for_parallel_segments(first: &Vector, second: &Vector) -> f64 {
    let position_difference = &first.position - &second.position;
    let second_squared_magnitude = second.offset.get_squared_magnitude();
    let s_s = PositionVector::dot(&position_difference,&second.offset)/second_squared_magnitude;
    let s_e = s_s + Vector::dot(first,second)/second_squared_magnitude;
    
    let distance = if s_s < 0.0 {
        position_difference.get_squared_magnitude()
    } else if s_s > 1.0 {
        (&position_difference - &second.offset).get_squared_magnitude()
    } else {
        return position_difference.get_normal_magnitude_to_vector(second)
    };

    let second_distance = if s_e < 0.0 {
        (&position_difference + &first.offset).get_squared_magnitude()
    } else if s_e > 1.0 {
        (&position_difference - &(&second.offset - &first.offset)).get_squared_magnitude()
    } else {
        return position_difference.get_normal_magnitude_to_vector(second)
    };

    if distance < second_distance {
        distance
    } else { 
        second_distance 
    }
}

/// Вычисление минимального квадрата расстояня между произвольными отрезами.
fn get_distance_for_crossing_segments(first: &Vector, second: &Vector) -> f64 {
    let position_difference = &first.position - &second.position;
    let first_squared_magnitude = first.offset.get_squared_magnitude();
    let second_squared_magnitude = second.offset.get_squared_magnitude();
    let s_s1 = PositionVector::dot(&position_difference,&second.offset)/second_squared_magnitude;
    let s_e1 = s_s1 + Vector::dot(first,second)/second_squared_magnitude;
    let s_s2 = -PositionVector::dot(&position_difference,&first.offset)/first_squared_magnitude;
    let s_e2 = s_s2 + Vector::dot(first,second)/first_squared_magnitude;

    let mut result_distance = if s_s1 < 0.0 {
        position_difference.get_squared_magnitude()
    } else if s_s1 > 1.0 {
        (&position_difference - &second.offset).get_squared_magnitude()
    } else {
        return position_difference.get_normal_magnitude_to_vector(second)
    };

    let mut potential_distance = if s_e1 < 0.0 {
        (&position_difference + &first.offset).get_squared_magnitude()
    } else if s_e1 > 1.0 {
        (&position_difference - &(&second.offset - &first.offset)).get_squared_magnitude()
    } else {
        return (&position_difference + &first.offset).get_normal_magnitude_to_vector(second)
    };
 
    if result_distance > potential_distance {
        result_distance = potential_distance;
    }

    potential_distance = if s_s2 < 0.0 {
        position_difference.get_squared_magnitude()
    } else if s_s2 > 1.0 {
        (&position_difference + &first.offset).get_squared_magnitude()
    } else {
        return position_difference.get_normal_magnitude_to_vector(first)
    };

    if result_distance > potential_distance {
        result_distance = potential_distance;
    }

    potential_distance = if s_e2 < 0.0 {
        (&position_difference - &second.offset).get_squared_magnitude()
    } else if s_e2 > 1.0 {
        (&position_difference - &(&second.offset - &first.offset)).get_squared_magnitude()
    } else {
        return (&position_difference - &second.offset).get_normal_magnitude_to_vector(first)
    };

    if result_distance > potential_distance {
        result_distance = potential_distance;
    }

    result_distance

}

/// Функция которая проверяет пересекаются ли отрезки или нет
fn test_vector_intersection(first: &Vector, second: &Vector) -> bool {
    let position_difference = &second.position - &first.position;
    let cross_product = Vector::cross(first,second);
    let s1 = position_difference.cross(&second.offset);
    let s2 = position_difference.cross(&first.offset);
    s1*s1 < s1*cross_product && s2*s2 < s2*cross_product
}

/// Направленная проверка пересечения бесконечного луча и отрезка
/// Если вектор { f64::Infinity, 0} пересекает точку перегиба, проверка нарушается.
fn test_if_positive_infinity_vector_crosses_side(point: &PositionVector, side: &Vector) -> isize {
    if side.offset.y == 0. { return 0isize; }
    let s = (point.y - side.position.y)/side.offset.y;
    if 
    s*s < s
    &&
    s*side.offset.x + side.position.x > point.x
    {
        if side.offset.y > 0.0 { return 1isize; }
        else { return -1isize; }
    }
    0isize
}

/// Проверка находится ли точка внутри здания или нет.
pub fn test_if_point_inside_building(point: &PositionVector, building: &Building) -> bool {
    let mut count = 0isize;
    for side in building.sides.iter() {
        count += test_if_positive_infinity_vector_crosses_side(point, side);
    }
    count != 0isize
}

#[inline]
fn get_segment_division_parameter(first: &Vector, second: &Vector) -> f64 {
    (&second.position - &first.position).cross(&second.offset)/Vector::cross(first,second)
}


/// Неоптимизированная проверка пересечения габаритных прямоугольников
#[inline]
fn _test_bounding_box_intersection(first: &Building, second: &Building) -> bool {
    (first.start_point.x - second.end_point.x)*(first.end_point.x - second.start_point.x) < 0.
    &&
    (first.start_point.y - second.end_point.y)*(first.end_point.y - second.start_point.y) < 0.
}

/// Оптимизированая проверка проверка габаритных прямоугольников
#[inline]
fn test_bounding_box_intersection(first: &Building, second: &Building) -> bool {
    first.start_point.x < second.end_point.x
    &&
    (first.start_point.y - second.end_point.y)*(first.end_point.y - second.start_point.y) < 0.
}

/// Критеризированная расстоянием проверка пересечения габаритных прямоугольников
#[inline]
fn test_criterized_bounding_box_intersection(first: &Building, second: &Building, distance: f64) -> bool {
    first.start_point.x < second.end_point.x + 2.0f64*distance
    &&
    (first.start_point.y - second.end_point.y - 2.0f64*distance)*(first.end_point.y - second.start_point.y + 2.0f64*distance) < 0.
}

// UNUSED
fn _test_building_intersection(first: &Building, second: &Building) -> bool {
    if _test_bounding_box_intersection(first, second) {
        for first_building_side in first.sides.iter() {
            for second_building_side in second.sides.iter() {
                if test_vector_intersection(first_building_side, second_building_side) {
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
            if _test_building_intersection(&sorted_map[i], &sorted_map[n]) {
                todo!(); // Функция которая определит что делать с пересечениями
            }
        }
    }
}

/// Проверка пересечния, нахождения внутри и расстояния между зданиями
pub fn test_criterized_building_intersection(first: &Building, second: &Building, distance: f64) -> bool {
    if test_criterized_bounding_box_intersection(first, second, distance) {
        for first_building_side in first.sides.iter() {
            for second_building_side in second.sides.iter() {
                if get_distance_for_crossing_segments(first_building_side, second_building_side) < distance*distance {
                    return true
                }
            }
        }
        return test_if_point_inside_building(&first.sides[0].position, second); 
    }
    false
}

/// Проверка пересечения зданий и нахождения вутри
pub fn test_building_intersection(first: &Building, second: &Building) -> bool {
    if test_bounding_box_intersection(first, second) {
        for first_building_side in first.sides.iter() {
            for second_building_side in second.sides.iter() {
                if test_vector_intersection(first_building_side, second_building_side) {
                    return true
                }
            }
        }
        return test_if_point_inside_building(&first.sides[0].position, second); 
    }
    false
}


// Оптимизировано
fn optimize_map(sorted_map: &Vec<Building>) -> () {
    if sorted_map.len() < 2usize { return; }
    for i in 0usize..sorted_map.len() - 1usize {
        for n in i + 1usize..sorted_map.len() {
            if &sorted_map[i].end_point.x < &sorted_map[n].start_point.x { break; }
            if test_building_intersection(&sorted_map[i], &sorted_map[n]) {
                todo!(); // Функция которая определит что делать с пересечениями
            }
        }
    }
}
