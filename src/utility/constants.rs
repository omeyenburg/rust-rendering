// Globals
pub const PROJECT_NAME: &str = "Rust Rendering";
pub const PROJECT_DIRECTORY: &str = "rust-rendering";
pub const OPENGL_VERSION: (u8, u8) = (3, 3);

// Macros #[macro_export]
macro_rules! load_str {
    ($expr:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $expr))
    };
}
macro_rules! load_bytes {
    ($expr:expr) => {
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $expr))
    };
}

// Shaders
pub const VERTEX_SHADER_SOURCE: &str = load_str!("assets/shader/vertex.glsl");
pub const FRAGMENT_SHADER_SOURCE: &str = load_str!("assets/shader/fragment.glsl");

// Sprites
pub const SPRITES_IMAGE: &[u8] = load_bytes!("assets/icon/icon.png");

// Font
pub const FONT_IMAGE: &[u8] = load_bytes!("assets/font/font.png");
pub const FONT_DATA: &str = load_str!("assets/font/font.txt");
