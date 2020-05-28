#version 450

layout(location = 0) in vec4 a_position;
layout(location = 1) in vec4 a_normal;

layout(location = 0) out vec3 f_normal;

layout(set = 0, binding = 0)
#include "uniform.glsl"

void main() {
    gl_Position = u_mvp * a_position;
    f_normal = a_normal.xyz;
}