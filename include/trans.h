#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

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

void
set_w_param(int n);

int
get_w_param(void);

int 
compare_sides(const void * s1, const void * s2);

BuildingC*
nc_hull_maker(BuildingsVec *buildings_vec);

void
freeBuildings(BuildingsVec data);

#ifdef __cplusplus
}
#endif  // __cplusplus
