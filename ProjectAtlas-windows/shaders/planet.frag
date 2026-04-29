#version 330

in vec3 fragPosition;
in vec3 fragNormal;

uniform vec3 lightPos;
uniform vec3 cameraPos;
uniform vec3 baseColor;
uniform vec3 wgPos;

out vec4 finalColor;

void main() {
    vec3 N = normalize(fragNormal);
    vec3 L = normalize(lightPos - fragPosition);
    vec3 V = normalize(cameraPos - fragPosition);
    vec3 H = normalize(L + V);

    float diff = max(dot(N, L), 0.0);
    float spec = pow(max(dot(N, H), 0.0), 48.0) * diff;
    float fres = pow(1.0 - max(dot(N, V), 0.0), 4.0);

    vec3  wgDir  = normalize(wgPos - fragPosition);
    float wgDist = length(wgPos - fragPosition);
    float wgFall = max(0.0, 1.0 - wgDist / 120.0);
    wgFall = wgFall * wgFall * wgFall;
    float wgDiff = max(dot(N, wgDir), 0.0);

    vec3 ambient = baseColor * 0.03;
    vec3 diffuse = baseColor * diff * vec3(1.0, 0.92, 0.78);
    vec3 specular = vec3(1.0, 0.95, 0.85) * spec * 0.5;
    vec3 rim = mix(baseColor, vec3(0.5, 0.7, 1.0), 0.4) * fres * 0.65;
    vec3 bounce = vec3(0.1, 0.25, 0.9) * wgDiff * wgFall * 0.4;

    finalColor = vec4(ambient + diffuse + specular + rim + bounce, 1.0);
}