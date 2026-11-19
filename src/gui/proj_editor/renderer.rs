use std::{mem::size_of, num::NonZeroU64};

use eframe::{
    egui,
    egui_wgpu::{wgpu, Callback, CallbackResources, CallbackTrait, ScreenDescriptor},
};

type Mat4 = [[f32; 4]; 4];

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct CubeUniform {
    mvp: Mat4,
}

struct CubeRenderer {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    index_count: u32,
}

#[derive(Clone, Copy)]
struct CubeCallback {
    target_format: wgpu::TextureFormat,
    angle: f32,
    aspect_ratio: f32,
}

pub fn cube_callback(
    rect: egui::Rect,
    target_format: wgpu::TextureFormat,
    angle: f32,
) -> egui::Shape {
    let aspect_ratio = (rect.width() / rect.height()).max(0.001);
    egui::Shape::Callback(Callback::new_paint_callback(
        rect,
        CubeCallback {
            target_format,
            angle,
            aspect_ratio,
        },
    ))
}

impl CallbackTrait for CubeCallback {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &ScreenDescriptor,
        _egui_encoder: &mut wgpu::CommandEncoder,
        callback_resources: &mut CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        if callback_resources.get::<CubeRenderer>().is_none() {
            callback_resources.insert(CubeRenderer::new(device, self.target_format));
        }

        let renderer = callback_resources
            .get::<CubeRenderer>()
            .expect("cube renderer was initialized above");
        let mvp = cube_mvp(self.angle, self.aspect_ratio);
        queue.write_buffer(
            &renderer.uniform_buffer,
            0,
            bytemuck::bytes_of(&CubeUniform { mvp }),
        );

        Vec::new()
    }

    fn paint(
        &self,
        _info: egui::epaint::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        callback_resources: &CallbackResources,
    ) {
        let Some(renderer) = callback_resources.get::<CubeRenderer>() else {
            return;
        };

        render_pass.set_pipeline(&renderer.pipeline);
        render_pass.set_bind_group(0, &renderer.bind_group, &[]);
        render_pass.set_vertex_buffer(0, renderer.vertex_buffer.slice(..));
        render_pass.set_index_buffer(renderer.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..renderer.index_count, 0, 0..1);
    }
}

impl CubeRenderer {
    fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self {
        use wgpu::util::DeviceExt as _;

        let vertices = cube_vertices();
        let indices = cube_indices();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("dimension_cube_vertices"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("dimension_cube_indices"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("dimension_cube_uniform"),
            size: size_of::<CubeUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("dimension_cube_bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU64::new(size_of::<CubeUniform>() as u64),
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("dimension_cube_bind_group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("dimension_cube_pipeline_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("dimension_cube_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("cube.wgsl").into()),
        });
        let vertex_attributes = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("dimension_cube_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: size_of::<Vertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &vertex_attributes,
                }],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
            cache: None,
        });

        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            uniform_buffer,
            bind_group,
            index_count: indices.len() as u32,
        }
    }
}

fn cube_vertices() -> [Vertex; 24] {
    let face =
        |color, positions: [[f32; 3]; 4]| positions.map(|position| Vertex { position, color });
    let faces = [
        face(
            [0.95, 0.24, 0.20],
            [
                [-1.0, -1.0, 1.0],
                [1.0, -1.0, 1.0],
                [1.0, 1.0, 1.0],
                [-1.0, 1.0, 1.0],
            ],
        ),
        face(
            [0.15, 0.72, 0.92],
            [
                [1.0, -1.0, -1.0],
                [-1.0, -1.0, -1.0],
                [-1.0, 1.0, -1.0],
                [1.0, 1.0, -1.0],
            ],
        ),
        face(
            [0.24, 0.82, 0.43],
            [
                [1.0, -1.0, 1.0],
                [1.0, -1.0, -1.0],
                [1.0, 1.0, -1.0],
                [1.0, 1.0, 1.0],
            ],
        ),
        face(
            [0.48, 0.38, 0.95],
            [
                [-1.0, -1.0, -1.0],
                [-1.0, -1.0, 1.0],
                [-1.0, 1.0, 1.0],
                [-1.0, 1.0, -1.0],
            ],
        ),
        face(
            [0.98, 0.75, 0.20],
            [
                [-1.0, 1.0, 1.0],
                [1.0, 1.0, 1.0],
                [1.0, 1.0, -1.0],
                [-1.0, 1.0, -1.0],
            ],
        ),
        face(
            [0.88, 0.34, 0.72],
            [
                [-1.0, -1.0, -1.0],
                [1.0, -1.0, -1.0],
                [1.0, -1.0, 1.0],
                [-1.0, -1.0, 1.0],
            ],
        ),
    ];
    let mut vertices = [Vertex {
        position: [0.0; 3],
        color: [0.0; 3],
    }; 24];
    for (face_index, face_vertices) in faces.into_iter().enumerate() {
        vertices[face_index * 4..face_index * 4 + 4].copy_from_slice(&face_vertices);
    }
    vertices
}

fn cube_indices() -> [u16; 36] {
    let mut indices = [0; 36];
    for face in 0..6 {
        let base = (face * 4) as u16;
        let offset = face * 6;
        indices[offset..offset + 6].copy_from_slice(&[
            base,
            base + 1,
            base + 2,
            base,
            base + 2,
            base + 3,
        ]);
    }
    indices
}

fn cube_mvp(angle: f32, aspect_ratio: f32) -> Mat4 {
    let eye = [3.2, 2.4, 4.8];
    let target = [0.0, 0.0, 0.0];
    let up = [0.0, 1.0, 0.0];

    let view = look_at_rh(eye, target, up);
    let projection = perspective_rh_wgpu(45.0_f32.to_radians(), aspect_ratio, 0.1, 100.0);
    let rotate_y = rotation_y(angle);
    let rotate_x = rotation_x(angle * 0.73);
    multiply(multiply(projection, view), multiply(rotate_y, rotate_x))
}

fn multiply(left: Mat4, right: Mat4) -> Mat4 {
    let mut result = [[0.0; 4]; 4];
    for column in 0..4 {
        for row in 0..4 {
            result[column][row] = left[0][row] * right[column][0]
                + left[1][row] * right[column][1]
                + left[2][row] * right[column][2]
                + left[3][row] * right[column][3];
        }
    }
    result
}

fn rotation_x(angle: f32) -> Mat4 {
    let (sin, cos) = angle.sin_cos();
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, cos, sin, 0.0],
        [0.0, -sin, cos, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

fn rotation_y(angle: f32) -> Mat4 {
    let (sin, cos) = angle.sin_cos();
    [
        [cos, 0.0, -sin, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [sin, 0.0, cos, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

fn perspective_rh_wgpu(fov_y: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4 {
    let focal_length = 1.0 / (fov_y * 0.5).tan();
    [
        [focal_length / aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, focal_length, 0.0, 0.0],
        [0.0, 0.0, far / (near - far), -1.0],
        [0.0, 0.0, (near * far) / (near - far), 0.0],
    ]
}

fn look_at_rh(eye: [f32; 3], target: [f32; 3], up: [f32; 3]) -> Mat4 {
    let z_axis = normalize(subtract(eye, target));
    let x_axis = normalize(cross(up, z_axis));
    let y_axis = cross(z_axis, x_axis);
    [
        [x_axis[0], y_axis[0], z_axis[0], 0.0],
        [x_axis[1], y_axis[1], z_axis[1], 0.0],
        [x_axis[2], y_axis[2], z_axis[2], 0.0],
        [-dot(x_axis, eye), -dot(y_axis, eye), -dot(z_axis, eye), 1.0],
    ]
}

fn subtract(left: [f32; 3], right: [f32; 3]) -> [f32; 3] {
    [left[0] - right[0], left[1] - right[1], left[2] - right[2]]
}

fn dot(left: [f32; 3], right: [f32; 3]) -> f32 {
    left[0] * right[0] + left[1] * right[1] + left[2] * right[2]
}

fn cross(left: [f32; 3], right: [f32; 3]) -> [f32; 3] {
    [
        left[1] * right[2] - left[2] * right[1],
        left[2] * right[0] - left[0] * right[2],
        left[0] * right[1] - left[1] * right[0],
    ]
}

fn normalize(vector: [f32; 3]) -> [f32; 3] {
    let length = dot(vector, vector).sqrt().max(f32::EPSILON);
    [vector[0] / length, vector[1] / length, vector[2] / length]
}
