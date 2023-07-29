use bevy::prelude::*;

use self::camera::update_app_size;

pub mod camera;
pub mod palette;

pub fn render_plugin(app: &mut App) {
    app.add_systems(Startup, camera::setup_camera)
        .add_systems(Update, update_app_size);
}
