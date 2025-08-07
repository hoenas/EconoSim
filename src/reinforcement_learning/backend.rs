use burn::backend::wgpu::WgpuDevice;
use burn::backend::Wgpu;

pub type Backend = Wgpu<f32, i32>;
pub type Device = WgpuDevice;
