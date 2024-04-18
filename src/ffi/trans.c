//#include <trans.h>
#include "../../include/trans.h"
#include <stdlib.h>
#include <stdio.h>

bool
is_lefter(PointC *a, PointC *b, PointC *main){
	return ( ((a->x - main->x) * (b->y - main->y) - (b->x - main->x) * (a->y - main->y)) > 0 );
}

void
move_points(PointC *a, PointC *b){
	PointC c = {a->x, a->y};

	a->x = b->x;
	a->y = b->y;

	b->x = c.x;
	b->y = c.y;
}

void
grahams_algorithm(BuildingC *building){
	PointC leftmost_point; //Самая левая точка здания
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

	move_points(points, points + left_point_index);

	// Упорядочивание точек
	PointC* sorted_points = malloc((building->lenVertex - 1) * sizeof(PointC));
	for(uint64_t i = 0; i < building->lenVertex - 1; ++i){
		sorted_points[i] = points[i+1];
	}

	free(points);

	// Жесткая сортировка пузырьком
	for(uint64_t i = 0; i < building->lenVertex - 1; ++i){
		for(uint64_t j = 0; j < building->lenVertex - 2; ++j){
			if(is_lefter(sorted_points + j, sorted_points + j + 1, &leftmost_point)){
				move_points(sorted_points + j, sorted_points + j + 1);
			}
		}
	}

	for(uint64_t i = 0; i < building->lenVertex - 1; ++i){
		printf("%f\t%f\n", sorted_points[i].x, sorted_points[i].y);
	}

	free(sorted_points);
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
