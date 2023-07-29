use bevy::{app::AppExit, prelude::*};
use seldom_fn_plugin::FnPluginExt;

mod pause;

pub fn state_plugin(app: &mut App) {
    app.add_state::<AppState>()
        .add_state::<GameState>()
        .configure_set(
            Update,
            UnpausedGame.run_if(in_state(AppState::Game).and_then(in_state(GameState::Running))),
        )
        .fn_plugin(pause::pause_plugin)
        .add_systems(OnEnter(AppState::Quit), quit_app);
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct UnpausedGame;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    #[default]
    Loading,
    Splash,
    Menu,
    Game,
    GameOver,
    Tutorial,
    Quit,
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Running,
    Paused,
    Fading,
}

pub fn reset_state<T: States>(mut state: ResMut<NextState<T>>) {
    state.set(T::default())
}

pub fn set_state_fn<S: States>(next_state: S) -> impl FnMut(ResMut<NextState<S>>) {
    move |mut state: ResMut<NextState<S>>| {
        state.set(next_state.clone());
    }
}

fn quit_app(mut exit_ev_w: EventWriter<AppExit>) {
    exit_ev_w.send(AppExit);
}
