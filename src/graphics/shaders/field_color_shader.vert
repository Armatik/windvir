#version 140


in vec3 vertex_color;
out vec4 color;


void
main()
{
  color = vec4(vertex_color, 1.);
}
