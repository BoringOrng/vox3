import bevy_pbr::render::{
    mesh_functions::{
        get_world_from_local,
        mesh_position_local_to_clip,
        mesh_position_local_to_world,
        mesh_normal_local_to_world
    },
    mesh_view_bindings::view,
    pbr_types::{
        STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT,
        PbrInput,
        pbr_input_new,
    },
    pbr_functions as fns,
    pbr_bindings,
};

import bevy_core_pipeline::tonemapping::tone_mapping;

@group(constants::MATERIAL_BIND_GROUP) @binding(0) var voxel_texture_array: texture_2d_array<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(1) var voxel_texture_sampler: sampler;

const UVS = array(
    vec2f(0, 0),
    vec2f(1, 0),
    vec2f(1, 1),
    vec2f(0, 1),
);

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) layer: u32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) @interpolate(flat) layer: u32,
};

@vertex
fn vertex(
    @builtin(vertex_index) vertex_index: u32,
    vertex: Vertex,
) -> VertexOutput {
    var out: VertexOutput;
    let world_from_local = get_world_from_local(vertex.instance_index);

    out.world_position = mesh_position_local_to_world(world_from_local, vec4<f32>(vertex.position, 1.0));
    out.position = mesh_position_local_to_clip(world_from_local, vec4<f32>(vertex.position, 1.0));
    out.world_normal = mesh_normal_local_to_world(vertex.normal, vertex.instance_index);
    out.uv = UVS[vertex_index % 4u];
    out.layer = vertex.layer;

    return out;
}

@fragment
fn fragment(
    @builtin(front_facing) is_front: bool,
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let layer = i32(mesh.layer);

    var pbr_input: PbrInput = pbr_input_new();
    pbr_input.material.base_color = textureSample(
        voxel_texture_array,
        voxel_texture_sampler,
        mesh.uv,
        layer,
    );

    let double_sided = (pbr_input.material.flags & STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT) != 0u;
    pbr_input.frag_coord = mesh.position;
    pbr_input.world_position = mesh.world_position;
    pbr_input.world_normal = fns::prepare_world_normal(
        mesh.world_normal,
        double_sided,
        is_front,
    );
    pbr_input.is_orthographic = view.clip_from_view[3].w == 1.0;
    pbr_input.N = normalize(pbr_input.world_normal);
    pbr_input.V = fns::calculate_view(mesh.world_position, pbr_input.is_orthographic);

    return tone_mapping(fns::apply_pbr_lighting(pbr_input), view.color_grading);
}
