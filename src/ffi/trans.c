//#include <trans.h>
#include "../../include/trans.h"
#include <stdint.h>
#include <stdio.h>

bool
is_lefter(const PointC *a, const PointC *b, const PointC *main){
	return ( ((a->y - main->y) / (a->x - main->x) > (b->y - main->y) / (b->x - main->x)) );
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

// void
// set_w_param(int n) 
// {  int w = n; }

// int
// get_w_param(void)
// {  return w; }

// BuildingC*
// nc_hull_maker(BuildingsVec *buildings_vec)
// {
// 	BuildingC* init_hull = malloc(sizeof(merge_buildings(buildings_vec)));
	
// 	uint64_t chull_points = 0;
// 	for(uint64_t i = 0; i < buildings_vec->lenBuildings; i++){
// 		chull_points += buildings_vec->buildings[i].lenVertex;
// 	}
// 	init_hull->sides = malloc(chull_points * sizeof(VectorC));
// 	init_hull->lenVertex = chull_points;

// 	BuildingC *entirety_of_points = malloc(sizeof(BuildingC));
// 	BuildingC *insides = malloc(sizeof(BuildingC));

// 	uint64_t inside_points = 0;
// 	for (uint64_t i = 0; entirety_of_points[i] != init_hull[i]; i++){
// 		inside_points += buildings_vec->buildings[i].lenVertex;
// 	}
// 	insides->lenVertex = inside_points;

// 	// for (uint64_t i = 0; i < get_w_param()*chull_points; i++) {
// 	// 	uint64_t ind_max = ;
// 	// 	PointC* trip1 = ;
// 	// 	PointC* trip2 = ;
// 	// 	PointC* trip3;
// 	// 	VectorC* side1 =
// 	// 	// double side_quad1 = (trip2 - trip1) * (trip2 - trip1);
// 	// 	double square_max = 0;
// 	// 	uint64_t index_trip3 = ;

// 	// 	for (uint64_t i = 0; trip3 == insides[i]; i++) {
// 	// 		// double side_quad2 = (trip3 - trip1) * (trip3 - trip1);
// 	// 		// double side_quad3 = (trip3 - trip2) * (trip3 - trip2);

// 	// 		if (side_quad1 > abs(side_quad2 - side_quad3)) {
// 	// 			// double p = ((trip2 - trip1) + (trip3 - trip1) + (trip3 - trip2)) / 2;
// 	// 			// double square_of_tri = sqrt(p*(p-(trip2 - trip1))*(p-(trip3 - trip1))*(p-(trip3 - trip2)));
// 	// 			if (square_of_tri < square_max) continue;
// 	// 			if (test_if_point_inside_building(trip3, insides)) continue;
// 	// 			if (test_vector_intersection()) continue;
// 	// 			int index_trip3 = &insides[trip3] - &insides[0];
// 	// 			square_max = square_of_tri;
// 	// 		}
// 	// 	}
// 	// 	if (index_trip3 >= 0) {

// 	// 	}
// 	// 	else break
// 	// }

// 	return init_hull;
// }

// BuildingsVec
// changeVertex(BuildingsVec data)
// {
// 	// Алгоритм Грэхема
// 	for(uint64_t i = 0; i < data.lenBuildings; ++i){
// 		grahams_algorithm(&(data.buildings[i]));
// 	}

// 	// Сортировка зданий по левой границе
// 	//qsort(data.buildings, data.lenBuildings, sizeof(BuildingC), compare_buildings);
	
// 	for(uint64_t i = 0; i < data.lenBuildings; ++i){
// 		for(uint64_t j = 0; j < data.lenBuildings - 1; ++j){
// 			if(data.buildings[j].startPoint.x > data.buildings[j+1].startPoint.x){
// 				BuildingC tmp = data.buildings[j];
// 				data.buildings[j] = data.buildings[j + 1];
// 				data.buildings[j + 1] = tmp;
// 			}
// 		}
// 	}
	
// 	/* ---------- Проверка объединения зданий ---------- */
// /*
// 	BuildingsVec vec;
// 	uint64_t n = 5;
// 	vec.lenBuildings = n;
// 	vec.buildings = malloc(vec.lenBuildings * sizeof(BuildingC));

// 	for(uint64_t i = 0; i < vec.lenBuildings; i++){
// 		vec.buildings[i] = data.buildings[i];
// 	}

// 	BuildingC* result = merge_buildings(&vec);

// 	free(vec.buildings);

// 	BuildingsVec new_data;
// 	new_data.lenBuildings = data.lenBuildings - n + 1;
// 	new_data.buildings = malloc(new_data.lenBuildings * sizeof(BuildingC));

// 	new_data.buildings[0] = *result;

// 	for(uint64_t i = 1; i < new_data.lenBuildings; i++){
// 		new_data.buildings[i] = data.buildings[i+n-1];
// 	}

// 	return new_data;
// */
// 	return data;
// }


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
