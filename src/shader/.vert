#version 330 core

layout (location = 0) in vec2 Position;
layout (location = 1) in vec2 uv;
layout (location = 2) in vec4 border_color;

out vec2 vUv;
in vec4 viewport;

flat 
out vec4 vertexColor;

void main()
{
    vUv = uv;
    gl_Position = vec4((Position.x / viewport.z * 2.0 - 1.0), (1.0 - Position.y / viewport.w * 2.0), 0.0, 1.0);
    vertexColor = border_color; // 传递一个固定的色值
}