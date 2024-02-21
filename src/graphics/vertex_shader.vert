#version 140


in vec2 position;
uniform mat4 matrix;
uniform float x_off;
uniform float y_off;


void main()
{
    vec2 pos = position;
    pos.x += x_off;
    pos.y += y_off;
    gl_Position = matrix * vec4(pos, .0, 1.);
}
