use bevy::prelude::*;
use plugin::HelloPlugin;

mod plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}
