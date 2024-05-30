#version 150


in float position_color;
// in vec3 v_normal;
uniform vec3 pos_light;

out vec4 color;


void
main()
{
	// float brightness = dot(normalize(v_normal), normalize(pos_light));
	// vec3 dark_color = vec3(.3, .3, .3);
	// vec3 regular_color = vec3(.5, .5, .5);
	// color = vec4(mix(dark_color, regular_color, brightness), 1.);
	// color = vec4(.5, .5, .5, 1.);
	if (position_color != .0)
	{
		color = vec4(.5, .5, .5, 1.);
	}
	else
	{
		color = vec4(.3, .3, .3, 1.);
	}
}
