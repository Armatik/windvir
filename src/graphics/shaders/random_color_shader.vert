#version 140


out vec4 color;
uniform float r_rand;
uniform float g_rand;
uniform float b_rand;


void
main()
{
	color = vec4(r_rand, g_rand, b_rand, 1.);
}
