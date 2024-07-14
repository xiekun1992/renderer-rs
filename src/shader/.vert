#version 330 core

layout (location = 0) in vec2 Position;
layout (location = 1) in vec2 uv;

varying vec2 vUv;
attribute vec4 viewport;

void main()
{
    vUv = uv;
    gl_Position = vec4((Position.x / viewport.z * 2.0 - 1.0), (1.0 - Position.y / viewport.w * 2.0), 0.0, 1.0);
}