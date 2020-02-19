mod shader_program;
pub use shader_program::ShaderProgram;

mod uniform;
pub use uniform::UniformValue;

pub fn deg_to_rad(degree: f32) -> f32 {
  degree * std::f32::consts::PI / 180.0
}
