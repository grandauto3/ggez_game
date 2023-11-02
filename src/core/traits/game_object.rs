use ggez::context::Has;
use ggez::graphics::{Canvas, Drawable, DrawParam, GraphicsContext, Rect};
use crate::core::input_context::InputContext;

pub struct GameObject;

impl GameObject {
    pub fn init(&self) {}
    pub fn start(&self) {}
    pub fn update(&mut self) {}

    pub fn process_input(&mut self, _input_ctx: &InputContext) {}
}

impl Drawable for GameObject {
    fn draw(&self, canvas: &mut Canvas, param: impl Into<DrawParam>) {
        todo!()
    }

    fn dimensions(&self, gfx: &impl Has<GraphicsContext>) -> Option<Rect> {
        todo!()
    }
}
