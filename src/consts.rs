pub mod errors;
pub mod window;

pub const VELOCITY_MODIFIER: f32 = 10.0;
pub const TILE_SIZE: f32 = 100.0;

pub const MAP: [[u8; 10]; 10] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 1, 1, 0, 0, 0, 1, 0, 1],
    [1, 0, 1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

pub const VERTEX_SHADER: &str = r#"
    #version 140

    in vec2 position;
    in vec3 color;
    out vec3 vertex_color;

    uniform mat4 matrix;

    void main() {
        vertex_color = color;
        gl_Position = matrix * vec4(position, 0.0, 1.0);
    }
"#;

pub const FRAGMENT_SHADER_SRC: &str = r#"
    #version 140

    in vec3 vertex_color;
    out vec4 color;

    void main() {
        color = vec4(vertex_color, 1.0);
    }
"#;
