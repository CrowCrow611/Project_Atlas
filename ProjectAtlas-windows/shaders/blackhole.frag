#version 330

in vec3 fragPosition;
in vec3 fragNormal;

uniform vec3 cameraPos;

out vec4 finalColor;

void main() {
    vec3  N   = normalize(fragNormal);
    vec3  V   = normalize(cameraPos - fragPosition);
    float NoV = max(dot(N, V), 0.0);

    float photon_rim = pow(1.0 - NoV, 6.5);
    vec3  photon     = vec3(1.0, 0.65, 0.1) * photon_rim * 1.4;

    float lens_rim = pow(1.0 - NoV, 3.2);
    vec3  lens     = vec3(0.28, 0.04, 0.48) * lens_rim * 0.7;

    vec3 core = vec3(0.04, 0.01, 0.08);

    finalColor = vec4(core + lens + photon, 1.0);
}