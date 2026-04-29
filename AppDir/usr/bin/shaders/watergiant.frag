#version 330

in vec3 fragPosition;
in vec3 fragNormal;
in vec2 fragTexCoord;

uniform vec3  lightPos;
uniform vec3  cameraPos;
uniform float time;

out vec4 finalColor;

float hash(vec2 p) {
    p = fract(p * vec2(127.1, 311.7));
    p += dot(p, p + 74.31);
    return fract(p.x * p.y);
}

float noise(vec2 p) {
    vec2 i = floor(p);
    vec2 f = fract(p);
    f = f * f * (3.0 - 2.0 * f);
    return mix(
        mix(hash(i),                  hash(i + vec2(1.0, 0.0)), f.x),
        mix(hash(i + vec2(0.0, 1.0)), hash(i + vec2(1.0, 1.0)), f.x),
        f.y
    );
}

void main() {
    vec3 N = normalize(fragNormal);
    vec3 L = normalize(lightPos - fragPosition);
    vec3 V = normalize(cameraPos - fragPosition);
    vec3 H = normalize(L + V);

    const float PI = 3.14159265;
    vec2 uv = vec2(atan(N.x, N.z) / (2.0 * PI) + 0.5, N.y * 0.5 + 0.5);

    float b1 = sin(uv.y * 14.0 * PI + time * 0.15) * 0.5 + 0.5;
    float b2 = sin(uv.y *  6.0 * PI - time * 0.09) * 0.5 + 0.5;
    float turb = noise(uv * 6.0 + vec2(time * 0.04, 0.0));

    float c1 = noise(uv * 22.0 + vec2( time * 0.26,  time * 0.18));
    float c2 = noise(uv * 14.0 + vec2(-time * 0.13,  time * 0.22));
    float caustic = pow(c1 * c2, 2.2);

    vec3 deep = vec3(0.018, 0.055, 0.32);
    vec3 mid = vec3(0.04,  0.13,  0.60);
    vec3 bright = vec3(0.08,  0.28,  0.88);

    vec3 surface = mix(deep, mid, b1);
    surface = mix(surface, bright, b2 * 0.4 + turb * 0.3);

    float diff = max(dot(N, L), 0.0);
    float spec = pow(max(dot(N, H), 0.0), 200.0);
    float fres = pow(1.0 - max(dot(N, V), 0.0), 3.5);
    float sss  = pow(max(dot(-N, L) + 0.35, 0.0), 3.0) * 0.28;

    vec3 color = surface  * (diff * 1.1 + 0.22)
           + vec3(0.78, 0.93, 1.0) * spec * 2.2
           + vec3(0.12, 0.38, 0.95) * fres * 1.4
           + deep * sss
           + vec3(0.3, 0.65, 1.0) * caustic * diff * 0.35;
}