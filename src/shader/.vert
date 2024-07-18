#version 330 core

layout (location = 0) in vec2 Position;
layout (location = 1) in vec2 uv;
layout (location = 2) in float index_num;
in vec4 viewport;

out vec2 vUv;
flat out float indices;

void main()
{
    vUv = uv;
    indices = index_num;
    gl_Position = vec4((Position.x / viewport.z * 2.0 - 1.0), (1.0 - Position.y / viewport.w * 2.0), 0.0, 1.0);
}