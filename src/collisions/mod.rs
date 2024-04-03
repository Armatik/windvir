use crate::defs::{Building, Vector};

fn check_vector_intersection(first: &Vector, second: &Vector) -> bool {
	let s1 = get_segment_division_parameter(first, second);
	let s2 = get_segment_division_parameter(second, first);
	s1*s1 < s1 && s2*s2 < s2
}

#[inline]
fn get_segment_division_parameter(first: &Vector, second: &Vector) -> f64 {
	(&first.offset - &second.offset).cross_product(&second.offset)/Vector::cross_product(first,second)
}

fn check_bounding_box_intersection(first: &Building, second: &Building) -> bool {
	(first.start_point.x - second.end_point.x)*(first.end_point.x - second.start_point.x) < 0.0f64
	&&
	(first.start_point.y - second.end_point.y)*(first.end_point.y - second.start_point.y) < 0.0f64}

/*
dx = c1.x - c2.x
c1.x = (l1.x + r1.x)/2
c2.x = (l2.x + r2.x)/2
dx = l1.x + r1.x - l2.x - r2.x
(l1.x + r1.x - l2.x - r2.x)^2 < (r1.x - l1.x + r2.x - l2.x)^2
// levo
l1.x*l1.x + r1.x*r1 + l2.x*l2.x + r2.x*r2.x
+ 2*l1.x*r1.x - 2*l2.x*l1.x - 2*r2.x*l1.x
- 2*l2.x*r1.x - 2*r2.x*r1.x + 2*l2.x*r2.x
// pravo
l1.x*l1.x + r1.x*r1 + l2.x*l2.x + r2.x*r2.x
- 2*l1.x*r1.x + 2*l2.x*l1.x - 2*r2.x*l1.x
- 2*l2.x*r1.x + 2*r2.x*r1.x - 2*l2.x*r2.x

summ
+ l1.x*r1.x - l2.x*l1.x
- r2.x*r1.x + l2.x*r2.x

dalee
l1.x*r1.x - l2.x*l1.x - r2.x*r1.x + l2.x*r2.x
(l1.x - r2.x)*(r1.x - l2.x) < 0

(l1.x - r2.x)*(r1.x - l2.x) < 0
((l1.x - l2.x) - l2.w)*((l1.x - l2.x) + l1.w)
*/

fn check_building_intersection(first: &Building, second: &Building) -> bool {

}