use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

pub fn draw() -> Vec<Vertex> {
    let vertex_1 = Vertex {
        position: [0.0, 0.5],
    };
    let vertex_2 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex_3 = Vertex {
        position: [0.5, -0.5],
    };

    vec![vertex_1, vertex_2, vertex_3]
}
