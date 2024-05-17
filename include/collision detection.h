#include "trans.h"

double
cross(const PointC* first, const PointC* second){
    return first->x*second->y - first->y*second->x;
}

double
squared_magnitude(const PointC* vector){
    return vector->x*vector->x + vector->y*vector->y;
}

bool
test_vector_intersection(const VectorC* first, const VectorC* second) {
    const PointC position_difference = { second->position.x - first->position.x, second->position.y - first->position.y }; 
    const double cross_product = cross(&first->offset,&second->offset);
    const double s1 = cross(&position_difference,&second->offset);
    const double s2 = cross(&position_difference,&first->offset);
    return s1*s1 < s1*cross_product && s2*s2 < s2*cross_product;
}

int64_t
test_if_positive_infinity_vector_crosess_side(const PointC* point, const VectorC* side){
    if ( side->offset.y == 0.0 ){ return 0; }
    const double s = (point->y - side->position.y)/side->offset.y;
    if (
    s >= 0
    &&
    s < 1
    &&
    point->x < side->position.x + s*side->offset.x
    ) {
        if ( side->offset.y > 0.0 ) { return 1; }
        else { return -1; }
    }
    return 0;
}

bool
test_if_point_inside_building(const PointC* point, const PointC* triangle){
    int64_t count = 0;
	VectorC sides[3];

	sides[0].position = triangle[0];
	sides[0].offset.x = triangle[1].x - triangle[0].x;
	sides[0].offset.y = triangle[1].y - triangle[0].y;
    
	sides[1].position = triangle[1];
	sides[1].offset.x = triangle[2].x - triangle[1].x;
	sides[1].offset.y = triangle[2].y - triangle[1].y;
	
	sides[2].position = triangle[2];
	sides[2].offset.x = triangle[0].x - triangle[2].x;
	sides[2].offset.y = triangle[0].y - triangle[2].y;

	for ( uint64_t i = 0; i < 3; ++i ){
        count += test_if_positive_infinity_vector_crosess_side(point, sides + i);
	}   
	
    return count != 0;
}
