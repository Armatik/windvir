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
	for (unsigned i = 0; i < data.data->lenVertex; i++)
	{
		free((void *)data.data->data[i]);
	}

	free((void *)data.data->data);
	free((void *)data.data);
}
