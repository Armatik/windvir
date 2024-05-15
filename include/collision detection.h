#include "trans.h"

double
cross(const PointC* first, const PointC* second){
    return first->x*second->y - first->y*second->x;
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
test_if_positive_infinity_vectror_crosess_side(const PointC* point, const VectorC* side){
    if ( side->offset.y == 0.0 ){ return 0; }
    const double s = (point->y - side->position.y)/side->offset.y;
    if (
    s*s < s
    &&
    point->x < side->position.x + s*side->offset.x
    ) {
        if ( side ->offset.y > 0.0 ) { return 1; }
        else { return -1; }
    }
    return 0;
}

bool
test_if_point_inside_building(const PointC* point, const BuildingC* building){
    int64_t count = 0;
    for ( uint64_t i = 0; i < building->lenVertex; ++i ){
        count += test_if_positive_infinity_vectror_crosess_side(point, &building->sides[i]);
    }   
    return count != 0;
}