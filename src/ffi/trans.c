//#include <trans.h>
//#include "../../include/trans.h"
#include "../../include/collision detection.h"
#include <stdint.h>
#include <stdio.h>
#include <string.h>

double min(double n1, double n2){ return (n1 < n2) ? n1 : n2; }

VectorC make_vector(PointC *p1, PointC *p2){
	VectorC vec;

	vec.position = *p1;
	
	vec.offset.x = p2->x - p1->x;
	vec.offset.y = p2->y - p1->y;
	
	return vec;
}

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
merge_buildings(BuildingsVec *buildings_vec){
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
nc_hull_maker(BuildingsVec *buildings_vec, double w) {
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

	while(true){
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

		d0 = pow(d0,2);

		double smin = 4 * d0;
		int64_t jpt = -1;
		
		//printf("im: %ld\tim1: %ld\n", im, im1);

		for(uint64_t j = 0; j < insides; j++){

			//printf("\tj: %ld\n", j);

			double d1 = pow(side_len(convex_hull + im, inside_points + j), 2);
			double d2 = pow(side_len(convex_hull + im1, inside_points + j), 2);

			if(d1 + d2 - d0 > w * min(d1, d2)){ continue; }
			
			//printf("\tfirst condition\n");

			PointC triangle[3];

			triangle[0] = convex_hull[im];
			triangle[1] = convex_hull[im1];
			triangle[2] = inside_points[j];

			double s = triangle_area(triangle);

			if(s < smin){

				//printf("\t\ts < smin\n");

				bool has_crosses = false;

				VectorC pb = make_vector(convex_hull + im, inside_points + j);
				VectorC pe = make_vector(inside_points + j, convex_hull + im1);

				VectorC v;
				for(uint64_t k = 0; (k < convex_size - 1) && (!has_crosses); k++){
					if(k != im && k != im1){
						v = make_vector(convex_hull + k, convex_hull + k + 1);

						if(test_vector_intersection(&pb, &v)){ has_crosses = true; }
						if(test_vector_intersection(&pe, &v)){ has_crosses = true; }
					}
				}
				
				if(im1 != 0){
					v = make_vector(convex_hull + convex_size - 1, convex_hull);

					if(test_vector_intersection(&pb, &v)){ has_crosses = true; }
					if(test_vector_intersection(&pe, &v)){ has_crosses = true; }
				}
				
				if(has_crosses){ continue; }
				
				//printf("\t\thas no crosses\n");

				jpt = j;

				smin = s;
				//printf("\t\tjpt: %ld\n", jpt);
			}
		}

		if(jpt >= 0){
			convex_hull = realloc(convex_hull, (convex_size + 1) * sizeof(PointC));
			memmove(convex_hull + im1 + 1, convex_hull + im1, (convex_size - im1) * sizeof(PointC));
		
			convex_hull[im1] = inside_points[jpt];
	
			convex_size++;
	
			if(jpt != insides - 1){
				memmove(inside_points + jpt, inside_points + jpt + 1, (insides - jpt - 1) * sizeof(PointC));
			}
			
			inside_points = (PointC*)realloc(inside_points, (insides - 1) * sizeof(PointC));
			insides--;
		}
		else{ break; }
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
	/*
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
	*/
	
	double w = 2.5;
	printf("w = %f\n", w);
	BuildingC *b = nc_hull_maker(&data, w);

	for(uint64_t i = 0; i < data.lenBuildings; i++){
		free(data.buildings[i].sides);
	}
	free(data.buildings);

	data.buildings = b;
	data.lenBuildings = 1;
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
