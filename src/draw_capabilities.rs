use color::RGBA;
use glam::*;

use crate::{
    BlendEquation, BlendFuncFactor, CullMode, DepthFunction, Graphics, StencilFunc, StencilOp,
    WebGlCapability,
};

#[derive(Clone, Copy)]
pub struct StencilData {
    pub ref_: StencilRef,
    pub func: StencilFunc,
    pub fail: StencilOp,
    pub zfail: StencilOp,
    pub zpass: StencilOp,
}

impl StencilData {
    pub fn set_ref(&mut self, ref_: StencilRef) {
        self.ref_ = ref_;
    }
}

#[derive(Clone, Copy)]
pub struct StencilRef {
    pub value: u8,
    pub mask: u8,
}

#[derive(Clone, Copy)]
pub enum StencilTestState {
    Same(StencilData),
    Separate {
        front: StencilData,
        back: StencilData,
    },
}

#[derive(Clone, Copy)]
pub enum BlendEquationState {
    Same(BlendEquation),
    Separate {
        rgb: BlendEquation,
        alpha: BlendEquation,
    },
}

#[derive(Clone, Copy)]
pub enum BlendFactorState {
    Same {
        src: BlendFuncFactor,
        dst: BlendFuncFactor,
    },
    Separate {
        src_rgb: BlendFuncFactor,
        dst_rgb: BlendFuncFactor,

        src_alpha: BlendFuncFactor,
        dst_alpha: BlendFuncFactor,
    },
}

#[derive(Clone, Copy)]
pub struct BlendState {
    pub equation: BlendEquationState,
    pub factor: BlendFactorState,
    pub blend_color: Option<RGBA>,
}

impl BlendState {
    pub fn default_alpha() -> Self {
        Self {
            equation: BlendEquationState::Same(BlendEquation::FUNC_ADD),
            factor: BlendFactorState::Same {
                src: BlendFuncFactor::SRC_ALPHA,
                dst: BlendFuncFactor::ONE_MINUS_SRC_ALPHA,
            },
            blend_color: None,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Scissor {
    pub position: IVec2,
    pub size: IVec2,
}

#[derive(Clone, Copy)]
pub struct DrawCapabilities {
    pub blend_state: Option<BlendState>,
    pub cull_face: Option<CullMode>,
    pub depth_test: Option<DepthFunction>,
    pub stencil_test: Option<StencilTestState>,
    pub scissor_test: Option<Scissor>,
    pub color_draw_mask: (bool, bool, bool, bool),
    pub depth_draw_mask: bool,
}

impl Default for DrawCapabilities {
    fn default() -> Self {
        Self {
            blend_state: Default::default(),
            cull_face: Default::default(),
            depth_test: Default::default(),
            stencil_test: Default::default(),
            scissor_test: Default::default(),
            color_draw_mask: (true, true, true, true),
            depth_draw_mask: true,
        }
    }
}

impl DrawCapabilities {
    pub fn new(
        blend_state: Option<BlendState>,
        cull_face: Option<CullMode>,
        depth_test: Option<DepthFunction>,
        stencil_test: Option<StencilTestState>,
        scissor_test: Option<Scissor>,
        color_draw_mask: Option<(bool, bool, bool, bool)>,
        depth_draw_mask: Option<bool>,
    ) -> Self {
        Self {
            blend_state,
            cull_face,
            depth_test,
            stencil_test,
            scissor_test,
            color_draw_mask: color_draw_mask.unwrap_or((true, true, true, true)),
            depth_draw_mask: depth_draw_mask.unwrap_or(true),
        }
    }

    pub fn default_opaque_no_cull() -> Self {
        DrawCapabilities {
            depth_test: Some(DepthFunction::LEQUAL),
            ..Default::default()
        }
    }

    pub fn default_opaque() -> Self {
        DrawCapabilities {
            cull_face: Some(CullMode::BACK),
            depth_test: Some(DepthFunction::LEQUAL),
            ..Default::default()
        }
    }

    pub fn always_render() -> Self {
        DrawCapabilities::default()
    }

    pub fn get_mut_stencil_state(&mut self) -> Option<&mut StencilTestState> {
        match &mut self.stencil_test {
            Some(stencil_test) => Some(stencil_test),
            None => None,
        }
    }

    pub fn set_blend_state(&self, graphics: &Graphics) {
        match self.blend_state {
            Some(state) => {
                graphics.enable_capability(WebGlCapability::BLEND);
                graphics.set_blend_equation(state.equation);
                graphics.set_blend_factor(state.factor);
                let blend_color = state.blend_color.unwrap_or(RGBA::BLACK);
                graphics.set_blend_color(blend_color);
            }
            None => graphics.disable_capability(WebGlCapability::BLEND),
        }
    }

    pub fn set_cull_mode(&self, graphics: &Graphics) {
        match self.cull_face {
            Some(mode) => {
                graphics.enable_capability(WebGlCapability::CULL_FACE);
                graphics.set_cull_face(mode);
            }
            None => graphics.disable_capability(WebGlCapability::CULL_FACE),
        }
    }

    pub fn set_depth_test(&self, graphics: &Graphics) {
        match self.depth_test {
            Some(depth_fn) => {
                graphics.enable_capability(WebGlCapability::DEPTH_TEST);
                graphics.set_depth_func(depth_fn);
                graphics.set_depth_write(true);
            }
            None => {
                graphics.disable_capability(WebGlCapability::DEPTH_TEST);
            }
        }
    }

    pub fn set_stencil_test(&self, graphics: &Graphics) {
        match self.stencil_test {
            Some(stencil_state) => {
                graphics.enable_capability(WebGlCapability::STENCIL_TEST);
                match stencil_state {
                    StencilTestState::Same(state) => {
                        graphics.set_stencil_state(state);
                    }
                    StencilTestState::Separate { front, back } => {
                        graphics.set_stencil_state_separate(CullMode::FRONT, front);
                        graphics.set_stencil_state_separate(CullMode::BACK, back);
                    }
                }
            }
            None => graphics.disable_capability(WebGlCapability::STENCIL_TEST),
        }
    }

    pub fn set_scissor_test(&self, graphics: &Graphics) {
        match self.scissor_test {
            Some(scissor) => {
                graphics.enable_capability(WebGlCapability::SCISSOR_TEST);
                graphics.set_scissor(scissor.position, scissor.size);
            }
            None => graphics.disable_capability(WebGlCapability::SCISSOR_TEST),
        }
    }

    pub fn set_color_mask(&self, graphics: &Graphics) {
        graphics.color_mask(
            self.color_draw_mask.0,
            self.color_draw_mask.1,
            self.color_draw_mask.2,
            self.color_draw_mask.3,
        );
    }

    pub fn set_depth_mask(&self, graphics: &Graphics) {
        graphics.depth_mask(self.depth_draw_mask);
    }

    pub fn set_capabilities(&self, graphics: &Graphics) {
        self.set_blend_state(graphics);
        self.set_cull_mode(graphics);
        self.set_depth_test(graphics);
        self.set_stencil_test(graphics);
        self.set_scissor_test(graphics);
        self.set_color_mask(graphics);
        self.set_depth_mask(graphics);
    }
}
