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

//Вот это надо раскомментить ахтунг!

// void
// set_w_param(int n) 
// {  int w = n; }

// int
// get_w_param(void)
// {  return w; }

// //Все точки и функции ниже соответствуют всем точкам и вызовам в статье, в которой все описывалось

// BuildingC*											//Функция которая делает невыпуклую оболочку
// nc_hull_maker(BuildingsVec *buildings_vec)
// {
// 	BuildingC* init_hull = malloc(sizeof(BuildingC));			//Здесь я объявляю саму оболочку (сначала она выпуклая)
	
// 	uint64_t hull_points = 0;
// 	for(uint64_t i = 0; i < buildings_vec->lenBuildings; i++){
// 		hull_points += buildings_vec->buildings[i].lenVertex;
// 	}
// 	init_hull->sides = malloc(hull_points * sizeof(VectorC));
// 	init_hull->lenVertex = hull_points;

// 	BuildingC *entirety_of_points = malloc(sizeof(BuildingC));				//Тут точки всех зданий (по идее)
// 	BuildingC *insides = malloc(sizeof(BuildingC));							//Внутренние точки, нужны для вырезания невыпуклой

// 	uint64_t inside_points = 0;
// 	for (uint64_t i = 0; entirety_of_points[i] != init_hull[i]; i++){
// 		inside_points += buildings_vec->buildings[i].lenVertex;
// 	}
// 	insides->lenVertex = inside_points;

// 	for (uint64_t i = 0; i < get_w_param()*hull_points; i++) {		//Цикл, в котором все происходит
// 		uint64_t ind_max = ;										//Тут должен быть индекс самой левой точки максимальной стороны. Серега функцию написал

// 		BuildingC *cutting_triangle = malloc(3 * sizeof(PointC));	//Тут я объявил треугольник с помощью которого вырезается оболочка как здание (возможно хуйня)
// 		cutting_triangle->sides = malloc(3*sizeof(VectorC));
// 		cutting_triangle->lenVertex = 3;

		

// 		double square_max = 0;
// 		uint64_t index_trip3 = ;

// 		for (uint64_t i = 0; trip3 == insides[i]; i++) {				//по идее реализация foreach из статьи (неизвестно насколько верная)
// 			// double side_quad2 = (trip3 - trip1) * (trip3 - trip1);
// 			// double side_quad3 = (trip3 - trip2) * (trip3 - trip2);

// 			if (side_quad1 > abs(side_quad2 - side_quad3)) { 			//if с проверкой условий, выполняются ли функции из co
// 				// double p = ((trip2 - trip1) + (trip3 - trip1) + (trip3 - trip2)) / 2;
// 				// double square_of_tri = sqrt(p*(p-(trip2 - trip1))*(p-(trip3 - trip1))*(p-(trip3 - trip2)));
// 				if (square_of_tri < square_max) continue;
// 				if (test_if_point_inside_building(trip3, insides)) continue;
// 				if (test_vector_intersection()) continue;
// 				int index_trip3 = &insides[trip3] - &insides[0];
// 				square_max = square_of_tri;
// 			}
// 		}							//тут по статье происходит запись trip3 (третья точка треугольника) в оболочку
// 		if (index_trip3 >= 0) {
// 			return 0;
// 		}
// 		else break
// 	}

// 	return init_hull;
// }

BuildingC*
nc_hull_maker(BuildingsVec *buildings_vec) {
	BuildingC *init_hull = merge_buildings(buildings_vec);

	PointC *convex_hull = malloc(init_hull->lenVertex * sizeof(PointC));
	PointC *inside_points;

	uint64_t insides = 0;

	for(uint64_t i = 0; i < init_hull->lenVertex; i++){
		convex_hull[i] = init_hull->sides[i].position;
	}

	for(uint64_t i = 0; i < buildings_vec->lenBuildings; i++){
		insides += buildings_vec->buildings[i].lenVertex;
	}

	inside_points = malloc(insides * sizeof(PointC));

	for(uint64_t i = 0, k = insides; i < buildings_vec->lenBuildings, k != 0; i++){
		for(uint64_t j = 0; j < buildings_vec->buildings[i].lenVertex; j++){
			inside_points[--k] = buildings_vec->buildings[i].sides[j].position;
		}
	}

	for(uint64_t i = 0; i < init_hull->lenVertex; i++){
		for(uint64_t j = 0; j < insides; j++){
			if(convex_hull[i] == inside_points[j]){
				memmove(inside_points + j, inside_points + j + 1, insides - j - 1);
				inside_points = realloc(inside_points, (--insides) * sizeof(PointC));
			}
		}
	}

	
	for(uint64_t i = 0; i < init_hull->lenVertex - 1; i++){
		PointC *fitting_points;
		uint64_t fitting = 0;

		for(uint64_t j = 0; j < insides; j++){
			double d0 = side_len(init_hull[i], init_hull[i+1]);
			double d1 = side_len(init_hull[i], inside_points[j]);
			double d2 = side_len(init_hull[i+1], init_hull[i+1]);

			if(pow(d0, 2) > fabs(pow(d1, 2) - pow(d2, 2))){
				if(fitting != 0){
					fitting_points = realloc(fitting_points, (++fitting) * sizeof(PointC));
					fitting_points[fitting - 1] = inside_points[j];
				}
				else{PointC *fitting_points;
					uint64_t fitting = 0;
					fitting_points = malloc(sizeof(PointC));
					fitting_points[0] = inside_points[j];
				}
			}
		}

		double smax = 0;
		


		free(fitting_points);
		fitting = 0;
	}

	free(convex_hull);
	free(inside_points);

	return NULL;
}

BuildingsVec
changeVertex(BuildingsVec data)
{
	// Алгоритм Грэхема
	for(uint64_t i = 0; i < data.lenBuildings; ++i){
		grahams_algorithm(&(data.buildings[i]));
	}

	// Сортировка зданий по левой границе
	//qsort(data.buildings, data.lenBuildings, sizeof(BuildingC), compare_buildings);
	
	for(uint64_t i = 0; i < data.lenBuildings; ++i){
		for(uint64_t j = 0; j < data.lenBuildings - 1; ++j){
			if(data.buildings[j].startPoint.x > data.buildings[j+1].startPoint.x){
				BuildingC tmp = data.buildings[j];
				data.buildings[j] = data.buildings[j + 1];
				data.buildings[j + 1] = tmp;
			}
		}
	}
	
	/* ---------- Проверка объединения зданий ---------- */
/*
	BuildingsVec vec;
	uint64_t n = 5;
	vec.lenBuildings = n;
	vec.buildings = malloc(vec.lenBuildings * sizeof(BuildingC));

	for(uint64_t i = 0; i < vec.lenBuildings; i++){
		vec.buildings[i] = data.buildings[i];
	}

	BuildingC* result = merge_buildings(&vec);

	free(vec.buildings);

	BuildingsVec new_data;
	new_data.lenBuildings = data.lenBuildings - n + 1;
	new_data.buildings = malloc(new_data.lenBuildings * sizeof(BuildingC));

	new_data.buildings[0] = *result;

	for(uint64_t i = 1; i < new_data.lenBuildings; i++){
		new_data.buildings[i] = data.buildings[i+n-1];
	}

	return new_data;
*/
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
