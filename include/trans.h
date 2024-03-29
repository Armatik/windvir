#include <stdint.h>


#ifdef __cplusplus
extern "C" {
#endif  // __cplusplus

typedef struct
{
	double **data;
	uint64_t lenVertex;  // Кол-во вершин в векторе. Вершина указывает на x и y
} Building;


typedef struct
{
	Building *data;
	uint64_t lenBuildings;  // Кол-во зданий в векторе
} BuildingsVec;


BuildingsVec
changeVertex(BuildingsVec data);


void
freeBuildings(BuildingsVec data);

#ifdef __cplusplus
}
#endif  // __cplusplus
