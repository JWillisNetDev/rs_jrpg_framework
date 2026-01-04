use rs_jrpg_framework::plugin::RpgPlugin;

use bevy::prelude::*;

fn main() {
    App::new().add_plugins((DefaultPlugins, RpgPlugin)).run();
}
