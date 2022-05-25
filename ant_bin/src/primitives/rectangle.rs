use std::io::Cursor;

use super::vertex::Vertex;
use crate::support::camera::Camera;
use glium::{
    index::IndexBuffer, uniform, Display, Frame, Program, Surface, Texture2d, VertexBuffer, DrawParameters, Blend,
};

use common::helper::{Color, Rotation, Vector2D};

pub struct Rectangle {
    vertex_buffer: VertexBuffer<Vertex>,
    program: Program,
    indices: IndexBuffer<u32>,
    pub rotation: Rotation,
    pub position: Vector2D,
    pub color: Color,
    pub size: Vector2D,
}

impl Rectangle {
    pub fn new(
        size: Vector2D,
        position: Vector2D,
        rotation: Rotation,
        color: Color,
        display: &Display,
    ) -> Rectangle {
        //      x
        //  0--------1
        //  |       /|
        //  |      / |
        //  |     /  |
        //  |    /   |
        //  |   /    | y
        //  |  /     |
        //  | /      |
        //  |/       |
        //  2--------3
        // at rotation 0
        //
        let vertex0 = Vertex::new(
            [-0.5 * size.x(), -0.5 * size.y()],
            color.get_data(),
            [0.0, 0.0],
        );
        let vertex1 = Vertex::new(
            [0.5 * size.x(), -0.5 * size.y()],
            color.get_data(),
            [1.0, 0.0],
        );
        let vertex2 = Vertex::new(
            [-0.5 * size.x(), 0.5 * size.y()],
            color.get_data(),
            [0.0, 1.0],
        );
        let vertex3 = Vertex::new(
            [0.5 * size.x(), 0.5 * size.y()],
            color.get_data(),
            [1.0, 1.0],
        );
        let shape = vec![vertex0, vertex1, vertex2, vertex3];

        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
        let indices = glium::index::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &[0u32, 1u32, 2u32, 1u32, 3u32, 2u32],
        )
        .unwrap();

        let vertex_shader_src = r#"
#version 330

in vec2 position;
in vec4 color;
in vec2 uv;
uniform mat4 model_mat;
uniform mat4 view_mat;
uniform mat4 proj_mat;
out vec4 my_attr;
out vec2 out_uv;

void main() {
    my_attr = color;
    out_uv = uv;
    gl_Position = proj_mat * view_mat * model_mat * vec4(position, 0.0, 1.0);
}
"#;

        let fragment_shader_src = r#"
#version 140

uniform sampler2D tex;

in vec4 my_attr;
in vec2 out_uv;
out vec4 color;
void main() {
    color = texture(tex, out_uv);
}
"#;

        let program =
            glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();

        Rectangle {
            vertex_buffer,
            indices,
            program,
            position,
            rotation,
            color,
            size,
        }
    }

    pub fn draw(&mut self, texture: &Texture2d, frame: &mut Frame, cam: &Camera) {
        let model_mat = [
            [
                self.rotation.get_rad().cos(),
                -self.rotation.get_rad().sin(),
                0.0,
                0.0,
            ],
            [
                self.rotation.get_rad().sin(),
                self.rotation.get_rad().cos(),
                0.0,
                0.0,
            ],
            [0.0, 0.0, 1.0, 0.0],
            [self.position.x(), self.position.y(), 0.0, 1.0f32],
        ];

        let uniforms = uniform! {
            model_mat: model_mat,
            view_mat: cam.view_mat,
            proj_mat: cam.proj_mat,
            tex: texture,
        };

        let mut draw_params = DrawParameters::default();
        draw_params.blend = Blend::alpha_blending();

        frame
            .draw(
                &self.vertex_buffer,
                &self.indices,
                &self.program,
                &uniforms,
                &draw_params,
            )
            .unwrap();
    }
}
