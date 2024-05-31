#version 150


in float position_color;
uniform vec3 pos_light;

out vec4 color;


void
main()
{
	if (position_color != 0.)
	{
		color = vec4(.5, .5, .5, 1.);
	}
	else
	{
		color = vec4(1., .0, .0, 1.);
	}
}
