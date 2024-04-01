use crate::defs::Vector;

fn check_vector_intersection(first: &Vector, second: &Vector) -> bool {
	let s1 = get_segment_division_parameter(first, second);
	let s2 = get_segment_division_parameter(second, first);
	s1*s1 < s1 && s2*s2 < s2
}

#[inline]
fn get_segment_division_parameter(first: &Vector, second: &Vector) -> f64 {
	((first.x - second.x)*second.dy - second.dx*(first.y - second.y))/Vector::cross_product(first,second)
}



// let difference_vector = first - second;
// PositionVector::cross_product(&difference_vector,&second.offset)/Vector::cross_product(first,second)
// let x = first.x1 + S*first.dx;
// let	y = first.y1 + S*first.dy;
// let S = ((second.x1 - first.x1)*first.dy - first.dx*(second.y1 - first.y1))
// 	/(first.dx*second.dy - second.dx*first.dy);
// let x = second.x1 + S*second.dx;
// let y = second.y1 + S*second.dy;

// function getS(segment,line){

// 	const S = (line.cos*segment.y2 - segment.x2*line.sin + line.b)/((segment.x1 - segment.x2)*line.sin + line.cos*(segment.y2 - segment.y1));
// 	const x = segment.x2 + S*(segment.x1 - segment.x2);
// 	const y = segment.y2 + S*(segment.y1 - segment.y2);
// }