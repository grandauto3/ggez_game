use ggez::{
    input::{
        mouse::MouseContext,
        keyboard::KeyboardContext,
    },
    timer::TimeContext,
};

pub struct InputContext<'a> {
    pub(crate) keyboard: &'a KeyboardContext,
    pub(crate) mouse: &'a MouseContext,
    pub(crate) time: &'a TimeContext,
}

impl<'a> InputContext<'a> {
    pub fn new(contexts: (&'a KeyboardContext, &'a MouseContext, &'a TimeContext)) -> Self {
        InputContext {
            keyboard: contexts.0,
            mouse: contexts.1,
            time: contexts.2,
        }
    }
}