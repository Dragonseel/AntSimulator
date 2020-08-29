use super::vertex::Vertex;
use crate::support::camera::Camera;
use glium::{index::IndexBuffer, uniform, Display, Frame, Program, Surface, VertexBuffer};

pub struct Rectangle {
    vertex_buffer: VertexBuffer<Vertex>,
    program: Program,
    indices: IndexBuffer<u32>,
    pub rotation: f32,
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub size: [f32; 2],
}

impl Rectangle {
    pub fn new(
        size: [f32; 2],
        position: [f32; 2],
        rotation: f32,
        color: [f32; 4],
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
        let vertex0 = Vertex::new([0.0, 0.0], color);
        let vertex1 = Vertex::new([size[0], 0.0], color);
        let vertex2 = Vertex::new([0.0, size[1]], color);
        let vertex3 = Vertex::new([size[0], size[1]], color);
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
uniform mat4 model_mat;
uniform mat4 view_mat;
uniform mat4 proj_mat;
out vec4 my_attr;


void main() {
    my_attr = color;
    gl_Position = proj_mat * view_mat * model_mat * vec4(position, 0.0, 1.0);
}
"#;

        let fragment_shader_src = r#"
#version 140
in vec4 my_attr;
out vec4 color;
void main() {
    color = my_attr;
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

    pub fn draw(&mut self, frame: &mut Frame, cam: &Camera) {
        let model_mat = [
            [self.rotation.cos(), -self.rotation.sin(), 0.0, 0.0],
            [self.rotation.sin(), self.rotation.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [self.position[0], self.position[1], 0.0, 1.0f32],
        ];

        let uniforms = uniform! {
            model_mat: model_mat,
            view_mat: cam.view_mat,
            proj_mat: cam.proj_mat,
        };

        frame
            .draw(
                &self.vertex_buffer,
                &self.indices,
                &self.program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
    }
}