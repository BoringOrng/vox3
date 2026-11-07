use bevy::{
    asset::{RenderAssetUsages, io::embedded::GetAssetServer},
    image::{CompressedImageFormats, ImageSampler, ImageType},
    mesh::{MeshVertexAttribute, VertexFormat},
    prelude::*,
    render::render_resource::{AsBindGroup, TextureViewDescriptor, TextureViewDimension},
};

#[derive(Resource, Deref, DerefMut)]
pub struct VoxelMaterialHandle(Handle<VoxelMaterial>);

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Resource)]
pub struct VoxelMaterial {
    #[texture(0, dimension = "2d_array")]
    #[sampler(1)]
    pub(in crate::material) texture_handle: Handle<Image>,
}

impl VoxelMaterial {
    pub const ATTRIBUTE_LAYER: MeshVertexAttribute =
        MeshVertexAttribute::new("Layer", 239_723_840_923_589_702, VertexFormat::Uint32);
}

impl Material for VoxelMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "shaders/voxel.wesl".into()
    }

    fn vertex_shader() -> bevy::shader::ShaderRef {
        "shaders/voxel.wesl".into()
    }

    fn specialize(
        _pipeline: &bevy::pbr::MaterialPipeline,
        descriptor: &mut bevy::material::descriptor::RenderPipelineDescriptor,
        layout: &bevy::mesh::MeshVertexBufferLayoutRef,
        _key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), bevy::material::specialize::SpecializedMeshPipelineError> {
        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Self::ATTRIBUTE_LAYER.at_shader_location(2),
        ])?;

        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}

impl FromWorld for VoxelMaterial {
    fn from_world(world: &mut World) -> Self {
        // temporary until we bother with multiple textures
        let mut image = Image::from_buffer(
            include_bytes!("../../../../../assets/textures/block/dirt.png"),
            ImageType::Extension("png"),
            CompressedImageFormats::NONE,
            true, // is_srgb
            ImageSampler::nearest(),
            RenderAssetUsages::RENDER_WORLD,
        )
        .expect("failed to decode dirt.png");

        _ = image.reinterpret_stacked_2d_as_array(1);
        image.sampler = ImageSampler::nearest();
        let view_descriptor = TextureViewDescriptor {
            dimension: Some(TextureViewDimension::D2Array),
            ..default()
        };
        image.texture_view_descriptor = Some(view_descriptor);

        let asset_server = world.get_asset_server();

        let texture_handle = asset_server.add(image);
        let m = Self { texture_handle };

        world.insert_resource(VoxelMaterialHandle(asset_server.add(m.clone())));

        m
    }
}
