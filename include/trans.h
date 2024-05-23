#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <math.h>


#ifdef __cplusplus
extern "C" {
#endif  // __cplusplus

typedef struct
{
	double x;
	double y;
} PointC;


typedef struct
{
	PointC position;
	PointC offset;
} VectorC;


typedef struct
{
	PointC startPoint;
	PointC endPoint;
	VectorC *sides;
	uint64_t lenVertex;  // Кол-во вершин в векторе
} BuildingC;


typedef struct
{
	BuildingC *buildings;
	uint64_t lenBuildings;  // Кол-во зданий в векторе
} BuildingsVec;


/// `main` функция
BuildingsVec
changeVertex(BuildingsVec data);

bool
is_lefter(const PointC* a, const PointC* b, const PointC* main); // Говорит находится ли точка b левее точки a относительно точки main

int
compare_buildings(const void * b1, const void * b2); // Для сортировки зданий по левой границе

void
swap_points(PointC* a, PointC* b); // Меняет местами значения точек a и b

void
grahams_algorithm(BuildingC *building);

BuildingC*
merge_buildings(BuildingsVec *buildings_vec); // Функция объединяет здания в массиве и возвращает указатель на здание, которое является объединением всех зданий


BuildingC*
nc_hull_maker(BuildingsVec *buildings_vec, double w);


void
freeBuildings(BuildingsVec data);

uint64_t
get_leftmost_biggest_side_point(const BuildingC* building);

//Длина стороны 
double side_len(PointC *p1, PointC *p2);

double triangle_area(PointC *triangle);

bool compare_points(PointC *p1, PointC *p2);

BuildingC *make_building(PointC *points, uint64_t size);

double min(double n1, double n2);

VectorC make_vector(PointC *p1, PointC *p2);

#ifdef __cplusplus
}
#endif  // __cplusplus
