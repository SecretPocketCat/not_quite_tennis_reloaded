use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::{reset_state, set_state_fn, AppState, GameState};
use crate::input::actions::{any_player_just_released, PlayerAction, UiAction};

pub(crate) fn pause_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Paused),
        toggle_input::<PlayerAction, false>,
    )
    .add_systems(
        OnExit(GameState::Paused),
        toggle_input::<PlayerAction, true>,
    )
    .add_systems(OnEnter(AppState::Game), toggle_input::<PlayerAction, true>)
    .add_systems(
        Update,
        (
            reset_state::<GameState>.run_if(state_changed::<AppState>()),
            set_state_fn(GameState::Paused).run_if(
                in_state(AppState::Game).and_then(
                    in_state(GameState::Running)
                        .and_then(any_player_just_released(PlayerAction::Pause)),
                ),
            ),
            set_state_fn(GameState::Running).run_if(in_state(AppState::Game).and_then(
                in_state(GameState::Paused).and_then(any_player_just_released(UiAction::Cancel)),
            )),
        ),
    );
}

fn toggle_input<T: Actionlike, const ENABLE: bool>(
    mut cmd: Commands,
    toggle: Option<ResMut<ToggleActions<T>>>,
) {
    if let Some(mut toggle) = toggle {
        toggle.enabled = ENABLE;
    } else {
        cmd.insert_resource(ToggleActions::<T> {
            enabled: ENABLE,
            ..default()
        });
    }
}
