use crate::read;

pub const PROJECT_NAME: &str = "Rust Rendering";
pub const PROJECT_DIRECTORY: &str = "rust-rendering";
pub const OPENGL_VERSION: (u8, u8) = (3, 3);

pub const VERTEX_SHADER_SOURCE: &str = read!("assets/shader/vertex.glsl");
pub const FRAGMENT_SHADER_SOURCE: &str = read!("assets/shader/fragment.glsl");
