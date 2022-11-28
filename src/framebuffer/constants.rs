use web_sys::WebGl2RenderingContext as gl;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct FramebufferBinding(u32);
#[allow(dead_code)]
impl FramebufferBinding{
	pub const DRAW_FRAMEBUFFER: FramebufferBinding = FramebufferBinding(gl::DRAW_FRAMEBUFFER);
	pub const READ_FRAMEBUFFER: FramebufferBinding = FramebufferBinding(gl::READ_FRAMEBUFFER);
}
impl Into<u32> for FramebufferBinding{
    fn into(self) -> u32 {
        self.0
    }
}