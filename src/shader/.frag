#version 330 core

out vec4 Color;

in vec2 vUv;
flat in float indices;

uniform sampler2D ourTexture;

void main() {
    float texture_y = 1.0 / (indices + 1.0);
    vec4 border_width = texture(ourTexture, vec2(0.9, texture_y));
    float border_top = border_width.x;
    float border_right = border_width.y;
    float border_bottom = border_width.z;
    float border_left = border_width.w;

    if (vUv.x <= border_left
        && (
            vUv.y / vUv.x >= border_bottom / border_left
            && 
            (vUv.y - (1.0 - border_top)) / (border_left - vUv.x) <= border_top / border_left
        )
    ) {
        Color = texture(ourTexture, vec2(0.8, texture_y));
    } else if (
        vUv.x >= 1.0 - border_right
        && (
            (vUv.y - (1.0 - border_top)) / (vUv.x - (1.0 - border_right)) <= border_top / border_right
            && 
            (vUv.y) / (border_right-(vUv.x - (1.0 - border_right))) >= border_bottom / border_right
        )
    ) {
        Color = texture(ourTexture, vec2(0.4, texture_y));
    } else if (
        vUv.y <= border_bottom
    ) {
        Color = texture(ourTexture, vec2(0.6, texture_y));
    } else if (vUv.y >= 1.0 - border_top) {
        Color = texture(ourTexture, vec2(0.2, texture_y));
    } else {
        // Color = vec4(vec3(0.0, 0.0, 0.0), 1.0);
        discard;
    }
    // Color = vec4(vec3(indices, indices, indices), 1.0);
    //  Color = texture(ourTexture, vUv);
    //  Color = texture(ourTexture, vec2(0.9, 0.0));
}