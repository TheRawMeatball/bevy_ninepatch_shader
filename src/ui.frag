#version 450

layout(location = 0) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform NinepatchMaterial_color {
    vec4 Color;
};

layout(set = 2, binding = 1) uniform texture2D NinepatchMaterial_texture;
layout(set = 2, binding = 2) uniform sampler NinepatchMaterial_texture_sampler;
layout(set = 2, binding = 3) uniform NinepatchMaterial_bounds {
    vec4 Bounds;
};
layout(set = 2, binding = 4) uniform NinepatchMaterial_scale {
    vec2 Scale;
};

layout(set = 1, binding = 1) uniform Node_size {
    vec2 NodeSize;
};

void main() {
    ivec2 texture_size = textureSize(sampler2D(NinepatchMaterial_texture, NinepatchMaterial_texture_sampler), 0);

    // https://www.desmos.com/calculator/xfaj8rgmet

    vec2 s = NodeSize;
    vec2 t = vec2(texture_size) * Scale;
    vec2 a = Bounds.xz * Scale.x;
    vec2 b = Bounds.yw * Scale.y;
    vec2 x = v_Uv;

    vec2 one = 1.0.xx;

    vec2 e1 = x*s/t;
    vec2 e2 = (x - one)*s/t + one;
    vec2 e3 = (s*(a + b - t)*(x - a/s))/(t*(a + b - s)) + a/t;

    vec2 c1 = a/s;
    vec2 c2 = one - b/s;

    vec2 e1_out = 1.0 - step(c1, x);
    vec2 e2_out = step(c2, x);
    vec2 e3_out = step(e1_out + e2_out, 0.0.xx);

    vec2 uv = e1_out * e1 + e2_out * e2 + e3_out * e3;

    vec4 color = Color;
    color *= texture(sampler2D(NinepatchMaterial_texture, NinepatchMaterial_texture_sampler), uv);
    o_Target = color;
}
