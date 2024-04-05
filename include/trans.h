#include <stdint.h>


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
	PointC start_point;
	PointC end_point;
	VectorC *sides;
	uint64_t lenVertex;  // Кол-во вершин в векторе
} BuildingC;


typedef struct
{
	BuildingC *buildings;
	uint64_t lenBuildings;  // Кол-во зданий в векторе
} BuildingsVec;

typedef struct
{
	PointC position;
	PointC offset;  // Кол-во зданий в векторе
} VectorC;

BuildingsVec
changeVertex(BuildingsVec data);


void
freeBuildings(BuildingsVec data);

#ifdef __cplusplus
}
#endif  // __cplusplus
