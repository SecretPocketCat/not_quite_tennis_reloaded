use bevy::{prelude::*, reflect::TypePath};
use leafwing_input_manager::prelude::*;

pub(super) fn actions_plugin(app: &mut App) {
    app.add_plugins((
        InputManagerPlugin::<PlayerAction>::default(),
        InputManagerPlugin::<UiAction>::default(),
    ));
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, TypePath)]
pub enum PlayerAction {
    Move,
    Echo,
    Pause,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, TypePath)]
pub enum UiAction {
    Move,
    Confirm,
    Cancel,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, TypePath)]
pub enum DebugAction {
    ToggleRapierDebug,
    RestartGame,
}

pub fn any_player_just_released<A: Actionlike>(
    action: A,
) -> impl FnMut(Query<&ActionState<A>>) -> bool {
    move |input_q: Query<&ActionState<A>>| {
        input_q
            .iter()
            .any(|input| input.just_released(action.clone()))
    }
}
