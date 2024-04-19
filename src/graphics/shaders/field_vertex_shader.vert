#version 140


in vec2 position;
in vec3 color;
out vec3 vertex_color;
uniform mat4 matrix;


void
main()
{
  vertex_color = color;
  gl_Position = matrix * vec4(position, .0, 1.);
}
