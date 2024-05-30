#version 150


in vec3 position;
uniform mat4 matrix;
uniform float x_off;
uniform float y_off;

// out vec3 v_normal;
out float position_color;


void
main()
{
	vec3 pos = position;
	pos.x += x_off;
	pos.y += y_off;

	position_color = pos.z;
	// v_normal = transpose(inverse(mat3(matrix))) * vec3(pos.x, pos.y, pos.z);  
	gl_Position = matrix * vec4(pos, 1.);
}
