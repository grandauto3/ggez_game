mod core;
mod game;

use crate::core::app::App;

fn main() {
    let app = App::new();
    app.run();
}
