use bevy_app::Plugin;
use bevy_asset::{AddAsset, Assets, Handle, HandleUntyped};
use bevy_ecs::{Bundle, Resources};
use bevy_math::{Vec2, Vec4};
use bevy_reflect::TypeUuid;
use bevy_render::{
    mesh::Mesh,
    pipeline::*,
    prelude::{Color, Draw, Texture, Visible},
    render_graph::{AssetRenderResourcesNode, RenderGraph},
    renderer::RenderResources,
    shader::{Shader, ShaderStage, ShaderStages},
};
use bevy_sprite::QUAD_HANDLE;
use bevy_transform::components::{GlobalTransform, Transform};
use bevy_ui::{Node, Style};

pub struct NinepatchUIShaderPlugin;

impl Plugin for NinepatchUIShaderPlugin {
    fn build(&self, app: &mut bevy_app::AppBuilder) {
        app.add_asset::<NinepatchMaterial>();

        let resources = app.resources();
        let mut render_graph = resources.get_mut::<RenderGraph>().unwrap();
        render_graph.add_ui_graph(resources);
    }
}

#[derive(Debug, RenderResources, TypeUuid)]
#[uuid = "839eef17-69fd-4a2f-87f3-bc2b9787345f"]
pub struct NinepatchMaterial {
    pub color: Color,
    pub bounds: Vec4,
    pub scale: Vec2,
    pub texture: Handle<Texture>,
}

#[derive(Bundle, Clone, Debug)]
pub struct NinepatchBundle {
    pub node: Node,
    pub style: Style,
    pub mesh: Handle<Mesh>, // TODO: maybe abstract this out
    pub material: Handle<NinepatchMaterial>,
    pub draw: Draw,
    pub visible: Visible,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for NinepatchBundle {
    fn default() -> Self {
        NinepatchBundle {
            mesh: QUAD_HANDLE.typed(),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                NINEPATCH_PIPELINE_HANDLE.typed(),
            )]),
            visible: Visible {
                is_transparent: true,
                ..Default::default()
            },
            node: Default::default(),
            style: Default::default(),
            material: Default::default(),
            draw: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}

pub const NINEPATCH_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 2122239601228667733);

pub mod node {
    pub const NINEPATCH_MATERIAL: &str = "ninepatch";
}

mod imports {
    // These can't be accessed regularly, so they're hacked in
    pub mod node {
        pub const UI_PASS: &str = "ui_pass";
    }

    pub use bevy_ui::camera;
}

pub trait UiRenderGraphBuilder {
    fn add_ui_graph(&mut self, resources: &Resources) -> &mut Self;
}

impl UiRenderGraphBuilder for RenderGraph {
    fn add_ui_graph(&mut self, resources: &Resources) -> &mut Self {
        let mut pipelines = resources.get_mut::<Assets<PipelineDescriptor>>().unwrap();
        let mut shaders = resources.get_mut::<Assets<Shader>>().unwrap();
        pipelines.set_untracked(NINEPATCH_PIPELINE_HANDLE, PipelineDescriptor::default_config(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, include_str!("ui.vert"))),
            fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, include_str!("ui.frag")))),
        }));

        self.add_system_node(
            node::NINEPATCH_MATERIAL,
            AssetRenderResourcesNode::<NinepatchMaterial>::new(true),
        );
        self.add_node_edge(node::NINEPATCH_MATERIAL, imports::node::UI_PASS)
            .unwrap();
        self
    }
}
