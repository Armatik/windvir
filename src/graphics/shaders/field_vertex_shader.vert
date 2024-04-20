#version 140


in vec2 position;
in vec3 color;
out vec3 vertex_color;
uniform mat4 matrix;
uniform float x_off;
uniform float y_off;


void
main()
{
	vertex_color = color;

	vec2 pos = position;
	pos.x += x_off;
	pos.y += y_off;
	gl_Position = matrix * vec4(pos, .0, 1.);
}
