use crate::state::AppState;
use bevy::prelude::*;

pub fn lvl_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Game), setup_level);
}

#[derive(Component)]
pub struct LevelEntry;

#[derive(Component)]
pub struct LevelExit;

#[derive(Resource, Default)]
pub struct ReachedLevel(pub usize);

#[derive(Component)]
pub struct Wall;

pub(super) fn setup_level(_cmd: Commands) {
    // todo:
}
