//#include <trans.h>
#include "../../include/trans.h"
#include <stdio.h>

PointC leftmost_point; // Самая левая точка здания

bool
is_lefter(PointC *a, PointC *b, PointC *main){
	return ( ((a->x - main->x) * (b->y - main->y) - (b->x - main->x) * (a->y - main->y)) < 0 );
}

int
compare_points(const void *p1, const void *p2){
	double x1 = ((PointC*)p1)->x - leftmost_point.x;
	double x2 = ((PointC*)p2)->x - leftmost_point.x;
	double y1 = ((PointC*)p1)->y - leftmost_point.y;
	double y2 = ((PointC*)p2)->y - leftmost_point.y;

	return (x2 * y1 - x1 * y2);
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
	//PointC leftmost_point; //Самая левая точка здания
	PointC* points; // Массив точек(вершин) здания
	
	points = malloc(building->lenVertex * sizeof(PointC));

	// Копирование точек из building в массив points
	for(uint64_t i = 0; i < building->lenVertex; ++i){
		points[i] = building->sides[i].position;
	}

	for(uint64_t i = 0; i < building->lenVertex; ++i){
		printf("%f\t%f\n", points[i].x, points[i].y);
	}
	printf("\n");

	// Определение самой левой точки
	leftmost_point = points[0];
	uint64_t left_point_index = 0;
	for(uint64_t i = 1; i < building->lenVertex; ++i){
		if(points[i].x < leftmost_point.x){
			leftmost_point = points[i];
			left_point_index = i;
		}
	}

	swap_points(points, points + left_point_index);

	// Упорядочивание точек
	// Жесткая сортировка пузырьком
	/*
	for(uint64_t i = 1; i < building->lenVertex - 1; ++i){
		for(uint64_t j = 1; j < building->lenVertex - 2; ++j){
			if(is_lefter(points + j, points + j + 1, &leftmost_point)){
				swap_points(points + j, points + j + 1);
			}
		}
	}
	*/

	qsort(points + 1, building->lenVertex - 1, sizeof(PointC), compare_points);

	for(uint64_t i = 0; i < building->lenVertex; ++i){
		printf("%f\t%f\n", points[i].x, points[i].y);
	}

	PointC *result_points = malloc(2 * sizeof(PointC)); // Точки, составляющие выпуклую оболочку

	result_points[0] = points[0];
	result_points[1] = points[1];



	free(points);
	free(result_points);
}

BuildingsVec
changeVertex(BuildingsVec data)
{
	// Алгоритм Грэхема
	
	grahams_algorithm(&(data.buildings[0]));
	


	return data;
}


/// Не трогать!
void
freeBuildings(BuildingsVec data)
{
	for (unsigned i = 0; i < data.lenBuildings; ++i)
	{
		for (unsigned j = 0; j < data.buildings[i].lenVertex; ++j)
		{
			free((void *)data.buildings[i].sides);
		}		
	}

	free((void *)data.buildings);
}
