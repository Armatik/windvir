use crate::defs::{Vector, Point};

fn detect_vector_intersection(first: &mut Vector, second: &mut Vector) -> bool {
	first += second;
}

#[inline]
fn get_segment_parameter(first: &Vector, second: &Vector) -> f64 {
	let S = ((first.x1 - second.x1)*second.dy - second.dx*(first.y1 - second.y1))
		/(second.dx*first.dy - first.dx*second.dy);
	// let x = first.x1 + S*first.dx;
	// let	y = first.y1 + S*first.dy;
	// let S = ((second.x1 - first.x1)*first.dy - first.dx*(second.y1 - first.y1))
	// 	/(first.dx*second.dy - second.dx*first.dy);
	// let x = second.x1 + S*second.dx;
	// let y = second.y1 + S*second.dy;
}


// function getS(segment,line){

// 	const S = (line.cos*segment.y2 - segment.x2*line.sin + line.b)/((segment.x1 - segment.x2)*line.sin + line.cos*(segment.y2 - segment.y1));
// 	const x = segment.x2 + S*(segment.x1 - segment.x2);
// 	const y = segment.y2 + S*(segment.y1 - segment.y2);
// }