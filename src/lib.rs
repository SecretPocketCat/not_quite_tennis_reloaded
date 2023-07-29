#![feature(let_chains)]

use bevy::prelude::*;
use seldom_fn_plugin::FnPluginExt;


pub use tools::ecs::EntityCommandsExt;

mod debug;
mod input;
mod level;
mod physics;
mod render;
mod state;
mod time;
mod tools;

#[derive(Resource, Deref)]
pub struct AppSize(pub Vec2);

pub const GAME_NAME: &str = "not quite tennis";
pub use render::palette;

pub fn game_plugin(app: &mut App) {
    app.fn_plugin(state::state_plugin)
        .fn_plugin(physics::physics_plugin)
        .fn_plugin(render::render_plugin)
        .fn_plugin(input::input_plugin)
        .fn_plugin(time::time_plugin)
        .fn_plugin(level::lvl_plugin);

    #[cfg(debug_assertions)]
    {
        app.fn_plugin(debug::debug_plugin);
    }
}
