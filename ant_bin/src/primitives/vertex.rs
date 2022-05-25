use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
    uv: [f32; 2],
}
implement_vertex!(Vertex, position, color, uv);

impl Vertex {
    pub fn new(pos: [f32; 2], col: [f32; 4], uv: [f32; 2]) -> Vertex {
        Vertex {
            position: pos,
            color: col,
            uv,
        }
    }
}
