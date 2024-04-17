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
	PointC position;
	PointC offset;
} VectorC;

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


void
freeBuildings(BuildingsVec data);

#ifdef __cplusplus
}
#endif  // __cplusplus
