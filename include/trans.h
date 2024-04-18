#include <stdint.h>
#include <stdbool.h>

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


BuildingsVec
changeVertex(BuildingsVec data);

bool
is_lefter(PointC* a, PointC* b, PointC* main); // Говорит находится ли точка b левее точки a относительно точки main

void
move_points(PointC* a, PointC* b); // Меняет местами значения точек a и b

void
grahams_algorithm(BuildingC *building);

void
freeBuildings(BuildingsVec data);

#ifdef __cplusplus
}
#endif  // __cplusplus
