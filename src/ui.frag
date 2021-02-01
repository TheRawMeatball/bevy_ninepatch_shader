#version 450

layout(location = 0) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform ColorMaterial_color {
    vec4 Color;
};

layout(set = 2, binding = 1) uniform texture2D NinepatchMaterial_texture;
layout(set = 2, binding = 2) uniform sampler NinepatchMaterial_texture_sampler;
layout(set = 2, binding = 3) uniform NinepatchMaterial_bounds {
    vec4 Bounds;
};

layout(set = 1, binding = 1) uniform Node_size {
    vec2 NodeSize;
};

void main() {
    vec4 color = Color;

    ivec2 texture_size = textureSize(sampler2D(NinepatchMaterial_texture, NinepatchMaterial_texture_sampler), 0);
    vec2 conv_ts = vec2(texture_size);

    vec2 a = Bounds.xz / conv_ts;
    vec2 b = Bounds.yw / conv_ts;
    vec2 x = v_Uv;
    vec2 s = NodeSize;

    vec2 uv = (x - a) * (1.0.xx - a - b) / (s - a - b) + a;

    color *= texture(sampler2D(NinepatchMaterial_texture, NinepatchMaterial_texture_sampler), uv);
    o_Target = 1.0.xxxx;
}
