pub mod errors;
pub mod window;

pub const VERTEX_SHADER: &str = r#"
    #version 140

    in vec2 position;
    
    uniform float x;

    void main() {
        vec2 pos = position;
        pos.x += x;
        gl_Position = vec4(pos, 0.0, 1.0);
    }
"#;

pub const FRAGMENT_SHADER_SRC: &str = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;
