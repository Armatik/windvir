//#include <trans.h>
//#include "../../include/trans.h"
#include "../../include/collision detection.h"
#include <stdint.h>
#include <stdio.h>
#include <string.h>

bool
is_lefter(const PointC *a, const PointC *b, const PointC *main){
	return ( ((a->y - main->y) / (a->x - main->x) > (b->y - main->y) / (b->x - main->x)) );
}

double side_len(PointC *p1, PointC *p2){
	return sqrt(pow(p2->x - p1->x, 2) + pow(p2->y - p1->y, 2));
}

double triangle_area(PointC *triangle){
	return 0.5 * fabs((triangle[1].x - triangle[0].x) * (triangle[2].y - triangle[0].y) -
				(triangle[2].x - triangle[0].x) * (triangle[1].y - triangle[0].y) );
}

bool compare_points(PointC *p1, PointC *p2){
	return (p1->x == p2->x) && (p1->y == p2->y);
}

uint64_t
get_leftmost_biggest_side_point(const BuildingC* building) {
	uint64_t index = 0;
	double maxLength = 0.0;
	for (uint64_t i = 0; i < building->lenVertex; ++i) {
		const double magnitude = squared_magnitude(&building->sides[i].offset);
		if (magnitude > maxLength) {
			maxLength = magnitude;
			index = i;
		}
	}
	if (0.0 < building->sides[index].offset.x) {
		return index;
	}
	return (index + 1)%building->lenVertex;
}

int
compare_buildings(const void * b1, const void * b2){
	double x1 = (*(BuildingC*)b1).startPoint.x;
	double x2 = (*(BuildingC*)b2).startPoint.x;

	printf("%f %f\n", x1, x2);
	return (int)(x1 - x2);
}

void
swap_points(PointC *a, PointC *b){
	PointC c = {a->x, a->y};

	a->x = b->x;
	a->y = b->y;

	b->x = c.x;
	b->y = c.y;
}

BuildingC *make_building(PointC *points, uint64_t size){
	BuildingC *building = malloc(sizeof(BuildingC));
	building->sides = malloc(size * sizeof(VectorC));
	building->lenVertex = size;
	
	VectorC side;
	for(uint64_t i = 0; i < size - 1; ++i){
		side.position = points[i];
		side.offset.x = points[i+1].x - points[i].x;
		side.offset.y = points[i+1].y - points[i].y;

		building->sides[i] = side;
	}

	side.position = points[size - 1];
	side.offset.x = points[0].x - points[size - 1].x;
	side.offset.y = points[0].y - points[size - 1].y;

	building->sides[size - 1] = side;

	return building;
}

void
grahams_algorithm(BuildingC *building){
	PointC leftmost_point; //Самая левая точка здания
	register PointC* points; // Массив точек(вершин) здания
	
	points = malloc(building->lenVertex * sizeof(PointC));

	// Копирование точек из building в массив points
	for(uint64_t i = 0; i < building->lenVertex; ++i){
		points[i] = building->sides[i].position;
	}

	// Определение самой левой точки и минимальных и максимальных координат
	leftmost_point = points[0];
	double xmin = points[0].x;
	double xmax = points[0].x;
	double ymin = points[0].y;
	double ymax = points[0].y;

	uint64_t left_point_index = 0;
	for(uint64_t i = 1; i < building->lenVertex; ++i){
		if(points[i].x < leftmost_point.x){
			leftmost_point = points[i];
			left_point_index = i;
		}

		if(points[i].x < xmin){ xmin = points[i].x; }
		if(points[i].x > xmax){ xmax = points[i].x; }
		if(points[i].y < ymin){ ymin = points[i].y; }
		if(points[i].y > ymax){ ymax = points[i].y; }
	}

	building->startPoint.x = xmin;
	building->startPoint.y = ymin;
	building->endPoint.x = xmax;
	building->endPoint.y = ymax;

	swap_points(points, points + left_point_index);

	// Упорядочивание точек
	// Жесткая сортировка пузырьком
	
	for(uint64_t i = 1; i < building->lenVertex; ++i){
		for(uint64_t j = 1; j < building->lenVertex - 1; ++j){
			if(is_lefter(points + j, points + j + 1, &leftmost_point)){
				swap_points(points + j, points + j + 1);
			}
		}
	}
	
	uint64_t result_size = 2;
	register PointC *result_points = malloc(result_size * sizeof(PointC)); // Точки, составляющие выпуклую оболочку

	result_points[0] = points[0];
	result_points[1] = points[1];

	// Нахождение выпуклой оболочки
	for(uint64_t i = 2; i < building->lenVertex; ++i){
		float x1 = result_points[result_size-1].x - result_points[result_size-2].x;
		float x2 = points[i].x - result_points[result_size-1].x;

		float y1 = result_points[result_size-1].y - result_points[result_size-2].y;
		float y2 = points[i].y - result_points[result_size-1].y;

		if(x1 * y2 > x2 * y1){
			// Добавление точки в массив
			result_points = (PointC*) realloc(result_points, (result_size + 1) * sizeof(PointC));
			result_points[result_size] = points[i];
			result_size++;
		}
		else{
			// Удаление точки
			result_points = (PointC*) realloc(result_points, (result_size - 1) * sizeof(PointC));
			result_size--;
			i--;
		}
	}

	// Запись изменений в здание
	free(building->sides);
	building->sides = malloc(result_size * sizeof(VectorC));
	building->lenVertex = result_size;
	
	VectorC side;
	for(uint64_t i = 0; i < result_size - 1; ++i){
		side.position = result_points[i];
		side.offset.x = result_points[i+1].x - result_points[i].x;
		side.offset.y = result_points[i+1].y - result_points[i].y;

		building->sides[i] = side;
	}

	side.position = result_points[result_size - 1];
	side.offset.x = result_points[0].x - result_points[result_size - 1].x;
	side.offset.y = result_points[0].y - result_points[result_size - 1].y;

	building->sides[result_size - 1] = side;

	free(points);
	free(result_points);
}

BuildingC*
merge_buildings(BuildingsVec *buildings_vec)
{
	BuildingC *result_building = malloc(sizeof(BuildingC));
	uint64_t points_len = 0; // Количество точек

	for(uint64_t i = 0; i < buildings_vec->lenBuildings; i++){
		points_len += buildings_vec->buildings[i].lenVertex;
	}

	result_building->sides = malloc(points_len * sizeof(VectorC));
	result_building->lenVertex = points_len;

	// index это итератор по массиву sides
	for(uint64_t i = 0, index = 0; i < buildings_vec->lenBuildings; i++){
		for(uint64_t j = 0; j < buildings_vec->buildings[i].lenVertex; j++){
			result_building->sides[index].position = buildings_vec->buildings[i].sides[j].position;
			index++;
		}
	}

	grahams_algorithm(result_building);

	return result_building;
}

BuildingC*
nc_hull_maker(BuildingsVec *buildings_vec, uint64_t w) {
	BuildingC *init_hull = merge_buildings(buildings_vec);

	uint64_t convex_size = init_hull->lenVertex;
	PointC *convex_hull = malloc(convex_size * sizeof(PointC));
	PointC *inside_points;

	uint64_t insides = 0;

	for(uint64_t i = 0; i < convex_size; i++){
		convex_hull[i] = init_hull->sides[i].position;
	}

	uint64_t n = init_hull->lenVertex;
	free(init_hull);

	for(uint64_t i = 0; i < buildings_vec->lenBuildings; i++){
		insides += buildings_vec->buildings[i].lenVertex;
	}

	inside_points = malloc(insides * sizeof(PointC));

	for(uint64_t i = 0, k = insides; (i < buildings_vec->lenBuildings) && (k != 0); i++){
		for(uint64_t j = 0; j < buildings_vec->buildings[i].lenVertex; j++){
			inside_points[--k] = buildings_vec->buildings[i].sides[j].position;
		}
	}

	for(uint64_t i = 0; i < convex_size; i++){
		for(uint64_t j = 0; j < insides; j++){
			if(compare_points(convex_hull + i, inside_points + j)){
				if(j != insides - 1){
					memmove(inside_points + j, inside_points + j + 1, (insides - j - 1) * sizeof(PointC));
				}
				inside_points = realloc(inside_points, (insides - 1) * sizeof(PointC));
				insides--;
			}
		}
	}

	for(uint64_t i = 0; i < w * n; i++){
		uint64_t im = convex_size - 1;
		uint64_t im1 = 0;
		double d0 = side_len(convex_hull + im, convex_hull);

		for(uint64_t j = 0; j < convex_size - 1; j++){
			double d = side_len(convex_hull + j, convex_hull + j + 1);

			if(d > d0){
				im = j;
				im1 = j + 1;
				d0 = d;
			}
		}

		double smax = 0;
		uint64_t jpt = -1;

		for(uint64_t j = 0; j < insides; j++){
			double d1 = side_len(convex_hull + im, inside_points + j);
			double d2 = side_len(convex_hull + im1, inside_points + j);

			if(pow(d0, 2) > fabs(pow(d1, 2) - pow(d2, 2))){
				PointC triangle[3];

				triangle[0] = convex_hull[im];
				triangle[1] = convex_hull[im1];
				triangle[2] = inside_points[j];

				double s = triangle_area(triangle);

				if(s < smax){ continue; }

				bool has_points_inside = false;
				
				for(uint64_t k = 0; (k < insides) && (!has_points_inside); k++){
					if((k != j) && test_if_point_inside_building(inside_points + k, triangle)){
						has_points_inside = true;
					}
				}

				if(has_points_inside){ continue; }
				
				bool has_crosses = false;

				VectorC pb;
				VectorC pe;

				pb.position = convex_hull[im];
				pb.offset.x = inside_points[j].x - convex_hull[im].x;
				pb.offset.y = inside_points[j].y - convex_hull[im].y;
				
				pe.position = convex_hull[im1];
				pe.offset.x = inside_points[j].x - convex_hull[im1].x;
				pe.offset.y = inside_points[j].y - convex_hull[im1].y;

				VectorC v;
				for(uint64_t k = 0; (k < convex_size - 1) && (!has_crosses) && k != im && k != im1; k++){
					v.position = convex_hull[k];
					v.offset.x = convex_hull[k+1].x - convex_hull[k].x;
					v.offset.y = convex_hull[k+1].y - convex_hull[k].y;

					if(test_vector_intersection(&pb, &v)){ has_crosses = true; }
					if(test_vector_intersection(&pe, &v)){ has_crosses = true; }
				}
				
				if(im1 != 0){
					v.position = convex_hull[convex_size - 1];
					v.offset.x = convex_hull[0].x - convex_hull[convex_size - 1].x;
					v.offset.y = convex_hull[0].y - convex_hull[convex_size - 1].y;

					if(test_vector_intersection(&pb, &v)){ has_crosses = true; }
					if(test_vector_intersection(&pe, &v)){ has_crosses = true; }
				}
				
				if(has_crosses){ continue; }
				
				jpt = j;

				smax = s;
			}

			if(j >= 0){
				convex_hull = realloc(convex_hull, (convex_size + 1) * sizeof(PointC));
				memmove(convex_hull + im1 + 1, convex_hull + im1, (convex_size - im1) * sizeof(PointC));
				convex_hull[im1] = inside_points[j];
				
				convex_size++;
				
				if(j != insides - 1){
					memmove(inside_points + j, inside_points + j + 1, (insides - j - 1) * sizeof(PointC));
				}
			
				inside_points = realloc(inside_points, (insides - 1) * sizeof(PointC));
				insides--;
			}
			else{ break; }
		}
	}

	BuildingC *result_building = make_building(convex_hull, convex_size);

	free(convex_hull);
	free(inside_points);

	return result_building;
}

BuildingsVec
changeVertex(BuildingsVec data)
{
	// Алгоритм Грэхема
	
	for(uint64_t i = 0; i < data.lenBuildings; ++i){
		grahams_algorithm(&(data.buildings[i]));
	}
	
	for(uint64_t i = 0; i < data.lenBuildings; ++i){
		for(uint64_t j = 0; j < data.lenBuildings - 1; ++j){
			if(data.buildings[j].startPoint.x > data.buildings[j+1].startPoint.x){
				BuildingC tmp = data.buildings[j];
				data.buildings[j] = data.buildings[j + 1];
				data.buildings[j + 1] = tmp;
			}
		}
	}
	
	uint64_t w = 2;
	printf("w = %ld\n", w);
	BuildingC *b = nc_hull_maker(&data, 1);

	for(uint64_t i = 0; i < data.lenBuildings; i++){
		free(data.buildings[i].sides);
	}
	free(data.buildings);

	data.buildings = b;
	data.lenBuildings = 1;

	return data;
}


/// Не трогать!
void
freeBuildings(BuildingsVec data)
{
	for (uint64_t i = 0; i < data.lenBuildings; ++i)
	{
		free(data.buildings[i].sides);
	}

	free(data.buildings);
}
