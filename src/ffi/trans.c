//#include <trans.h>
#include "../../include/trans.h"
#include <stdio.h>

bool
is_lefter(PointC *a, PointC *b, PointC *main){
	return ( ((a->y - main->y) / (a->x - main->x) > (b->y - main->y) / (b->x - main->x)) );
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
	PointC* points; // Массив точек(вершин) здания
	
	points = malloc(building->lenVertex * sizeof(PointC));

	// Копирование точек из building в массив points
	for(uint64_t i = 0; i < building->lenVertex; ++i){
		points[i] = building->sides[i].position;
	}

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
	
	for(uint64_t i = 1; i < building->lenVertex; ++i){
		for(uint64_t j = 1; j < building->lenVertex - 1; ++j){
			if(is_lefter(points + j, points + j + 1, &leftmost_point)){
				swap_points(points + j, points + j + 1);
			}
		}
	}
	
	uint64_t result_size = 2;
	PointC *result_points = malloc(result_size * sizeof(PointC)); // Точки, составляющие выпуклую оболочку

	result_points[0] = points[0];
	result_points[1] = points[1];

	// Нахождение выпуклой оболочки
	for(uint64_t i = 2; i < building->lenVertex; ++i){
		double x1 = result_points[result_size-1].x - result_points[result_size-2].x;
		double x2 = points[i].x - result_points[result_size-1].x;

		double y1 = result_points[result_size-1].y - result_points[result_size-2].y;
		double y2 = points[i].y - result_points[result_size-1].y;

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

BuildingsVec
changeVertex(BuildingsVec data)
{
	// Алгоритм Грэхема
	for(uint64_t i = 0; i < data.lenBuildings; ++i){
		grahams_algorithm(&(data.buildings[i]));
	}

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
