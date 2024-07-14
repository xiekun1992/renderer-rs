#version 330 core

out vec4 Color;
varying vec2 vUv;

float border_left = 0.1;
float border_top = 0.25;
float border_right = 0.1;
float border_bottom = 0.25;

void main() {

    if (vUv.x <= border_left
        && (
            vUv.y / vUv.x >= border_bottom / border_left
            && 
            (vUv.y - (1.0 - border_top)) / (border_left - vUv.x) <= border_top / border_left
        )
    ) {
        Color = vec4(vec3(1.0, 0.0, 0.0), 1.0);
    } else if (
        vUv.x >= 1.0 - border_right
        && (
            (vUv.y - (1.0 - border_top)) / (vUv.x - (1.0 - border_right)) <= border_top / border_right
            && 
            (vUv.y) / (border_right-(vUv.x - (1.0 - border_right))) >= border_bottom / border_right
        )
    ) {
        Color = vec4(vec3(1.0, 1.0, 0.0), 1.0);
    } else if (
        vUv.y <= border_bottom
    ) {
        Color = vec4(vec3(0.0, 1.0, 0.0), 1.0);
    } else if (vUv.y >= 1.0 - border_top) {
        Color = vec4(vec3(0.0, 0.0, 1.0), 1.0);
    } else {
        Color = vec4(vec3(0.0, 0.0, 0.0), 1.0);
    }
    // Color = vec4(1.0, 0.902, 0.0, 1.0);
}