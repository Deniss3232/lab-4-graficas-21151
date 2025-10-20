use raylib::prelude::Matrix;

/// Uniformes que usa tu `vertex_shader` (shaders.rs necesita `crate::Uniforms`)
#[derive(Clone, Copy)]
pub struct Uniforms {
    pub model_matrix: Matrix,
}
