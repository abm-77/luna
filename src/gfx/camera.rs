use cgmath::Point3;
use winit::dpi::PhysicalSize;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj<T: Camera>(&mut self, camera: &T) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

pub trait Camera  {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32>;
}

pub struct OrthographicCamera {
    pub pos: cgmath::Point3<f32>,
    pub dir: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub size: PhysicalSize<u32>,
    pub znear: f32,
    pub zfar: f32,
}

impl OrthographicCamera {
    pub fn new (width: u32, height: u32) -> Self {
        Self {
            pos: (0.0, 0.0, -1.0).into(),
            dir: (0.0, 0.0, 1.0).into(),
            up: cgmath::Vector3::unit_y(),
            size: PhysicalSize::new(width, height),
            znear: -1.0,
            zfar: 1.0,
        }
    }

    pub fn translate (&mut self, delta_x: f32, delta_y: f32) {
        self.pos = Point3::new(self.pos.x + delta_x, self.pos.y + delta_y, self.pos.z);
    }
}

pub struct PerspectiveCamera {
    pub pos: cgmath::Point3<f32>,
    pub dir: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,

    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

impl Camera for PerspectiveCamera {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let target = Point3::new(self.pos.x + self.dir.x, self.pos.y + self.dir.y, self.pos.z + self.dir.z);
        let view = cgmath::Matrix4::look_at_rh(self.pos, target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

impl Camera for OrthographicCamera {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let target = Point3::new(self.pos.x + self.dir.x, self.pos.y + self.dir.y, self.pos.z + self.dir.z);
        let view = cgmath::Matrix4::look_at_rh(self.pos, target, self.up);
        let proj = cgmath::ortho(0.0, self.size.width as f32, self.size.height as f32, 0.0, self.znear, self.zfar);
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}