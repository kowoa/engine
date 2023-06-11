use bevy::prelude::App;
use engine::run;


fn main() {
    App::new()
        .set_runner(run)
        .run();
}
