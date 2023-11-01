use crate::core::input_context::InputContext;

pub trait GameObject {
    fn init(&self) {}
    fn start(&self) {}
    fn update(&mut self) {}

    fn process_input(&mut self, _input_ctx: &InputContext) {}
}
