#version 140


in vec2 position;
uniform float x_off;


void main()
{
    vec2 pos = position;
    pos.x += x_off;
    gl_Position = vec4(pos, .0, 1.);
}
