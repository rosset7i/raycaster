pub const WINDOW_ERROR: &str = "Error While Building Window: ";
pub const PROGRAM_ERROR: &str = "Error While Building Program: ";
pub const BUFFER_ERROR: &str = "Error While Building Vertex Buffer: ";
pub const DRAWING_ERROR: &str = "Error While Drawing Buffer: ";

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
