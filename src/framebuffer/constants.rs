use web_sys::WebGl2RenderingContext as gl;

#[derive(Clone, Copy)]
pub struct FramebufferBindTarget(u32);
impl FramebufferBindTarget {
    pub const DRAW_FRAMEBUFFER: FramebufferBindTarget = FramebufferBindTarget(gl::DRAW_FRAMEBUFFER);
    pub const READ_FRAMEBUFFER: FramebufferBindTarget = FramebufferBindTarget(gl::READ_FRAMEBUFFER);
}
impl Into<u32> for FramebufferBindTarget {
    fn into(self) -> u32 {
        self.0
    }
}