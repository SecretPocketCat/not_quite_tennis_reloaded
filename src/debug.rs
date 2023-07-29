use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_editor_pls::prelude::*;
use bevy_rapier2d::render::{DebugRenderContext, RapierDebugRenderPlugin};
use leafwing_input_manager::{common_conditions::action_just_pressed, prelude::*};

use crate::input::actions::DebugAction;

pub fn debug_plugin(app: &mut App) {
    #[cfg(debug_assertions)]
    {
        app.add_plugins((
            EditorPlugin::default(),
            RapierDebugRenderPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            InputManagerPlugin::<DebugAction>::default(),
        ))
        .init_resource::<ActionState<DebugAction>>()
        .insert_resource(
            InputMap::default()
                .insert(KeyCode::R, DebugAction::RestartGame)
                .insert(KeyCode::C, DebugAction::ToggleRapierDebug)
                .build(),
        )
        .add_systems(
            Update,
            toggle_rapier_debug.run_if(action_just_pressed(DebugAction::ToggleRapierDebug)),
        );
    }
}

fn toggle_rapier_debug(mut dbg_ctx: ResMut<DebugRenderContext>) {
    dbg_ctx.enabled = !dbg_ctx.enabled;
}
