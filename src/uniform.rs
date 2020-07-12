use mint::{ColumnMatrix4, Vector3};

pub type Vec3 = Vector3<f32>;
pub type Matrix4 = ColumnMatrix4<f32>;

pub trait UniformValue: Copy {
  unsafe fn gl_uniform(self, location: i32);
}

impl UniformValue for bool {
  unsafe fn gl_uniform(self, location: i32) {
    gl::Uniform1i(location, self as i32);
  }
}

impl UniformValue for f32 {
  unsafe fn gl_uniform(self, location: i32) {
    gl::Uniform1f(location, self);
  }
}

impl UniformValue for i32 {
  unsafe fn gl_uniform(self, location: i32) {
    gl::Uniform1i(location, self);
  }
}

impl UniformValue for Vec3 {
  unsafe fn gl_uniform(self, location: i32) {
    gl::Uniform3f(location, self.x, self.y, self.z);
  }
}

impl UniformValue for Matrix4 {
  unsafe fn gl_uniform(self, location: i32) {
    gl::UniformMatrix4fv(location, 1, gl::FALSE, (self.as_ref() as &[f32; 16]).as_ptr());
  }
}
