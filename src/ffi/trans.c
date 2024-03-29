#include <trans.h>
#include <stdlib.h>


BuildingsVec
changeVertex(BuildingsVec data)
{
	return data;
}


/// Не трогать!
void
freeBuildings(BuildingsVec data)
{
	for (unsigned i = 0; i < data.lenBuildings; i++)
	{
		for (unsigned j = 0; j < data.buildings[i].lenVertex; j++)
		{
			free((void *)data.buildings[i].points);
		}		
	}

	free((void *)data.buildings);
}
