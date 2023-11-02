use ggez::{
    context::Has,
    glam::vec2,
    mint::Point2,
    graphics::{
        Canvas,
        Color,
        Drawable,
        DrawMode,
        DrawParam,
        GraphicsContext,
        Mesh,
        Rect,
    },
    Context,
    input::keyboard::KeyCode,
};

use crate::{
    core::{
        input_context::InputContext,
        traits::game_object::GameObject,
    }
};


pub struct Player {
    position: Point2<f32>,
    speed: f32,
    mesh: Mesh,

    new_position: Point2<f32>,

    game_object: GameObject,
}

impl Player {
    fn update(&mut self) {
        self.move_position(self.new_position);
    }
    fn process_input(&mut self, input_ctx: &InputContext) {
        let mut cur_pos = self.position;
        let input = input_ctx.keyboard;
        let delta_time = input_ctx.time.delta().as_secs_f32();

        if input.is_key_pressed(KeyCode::W) {
            cur_pos.y -= self.speed * delta_time;
        }
        if input.is_key_pressed(KeyCode::A) {
            cur_pos.x -= self.speed * delta_time;
        }
        if input.is_key_pressed(KeyCode::S) {
            cur_pos.y += self.speed * delta_time;
        }
        if input.is_key_pressed(KeyCode::D) {
            cur_pos.x += self.speed * delta_time;
        }

        self.new_position = cur_pos;
    }
}

impl Player {
    pub fn new(ctx: &Context) -> Self {
        Player {
            position: vec2(0., 0.).into(),
            speed: 100.0,
            mesh: Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                vec2(10.0, 10.0),
                10.0,
                2.0,
                Color::BLACK).expect("Creating mesh didn't work"),
            new_position: Point2 {
                x: 0.0,
                y: 0.0,
            },
        }
    }


    pub fn move_position(&mut self, new_pos: Point2<f32>) {
        self.position = new_pos;
    }
}

impl Drawable for Player {
    fn draw(&self, canvas: &mut Canvas, _: impl Into<DrawParam>) {
        canvas.draw(&self.mesh, self.position);
    }

    fn dimensions(&self, _: &impl Has<GraphicsContext>) -> Option<Rect> {
        todo!()
    }
}

