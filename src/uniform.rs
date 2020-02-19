use mint::ColumnMatrix4;

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

impl UniformValue for ColumnMatrix4<f32> {
  unsafe fn gl_uniform(self, location: i32) {
    gl::UniformMatrix4fv(location, 1, gl::FALSE, (self.as_ref() as &[f32; 16]).as_ptr());
  }
}
