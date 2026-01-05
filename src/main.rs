use rs_jrpg_framework::JRPGFrameworkPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, JRPGFrameworkPlugin))
        .run();
}
