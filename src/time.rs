use bevy::{ecs::system::SystemParam, prelude::*};
use std::time::Duration;

use crate::state::GameState;

pub fn time_plugin(app: &mut App) {
    app.insert_resource(TimeScale(1.))
        .add_systems(OnEnter(GameState::Paused), update_time_scale::<0>)
        .add_systems(OnExit(GameState::Paused), update_time_scale::<1>);
}

#[derive(Resource)]
pub struct TimeScale(pub f32);

pub trait ScaledTimeFields {
    fn time(&self) -> &Time;
    fn time_scale(&self) -> &TimeScale;
}

pub trait ScaledTimeDelta {
    fn scale(&self) -> f32;
    fn delta(&self) -> Duration;
    fn delta_seconds(&self) -> f32;
    fn scaled_delta(&self) -> Duration;
    fn scaled_delta_seconds(&self) -> f32;
}

#[derive(SystemParam)]
pub struct ScaledTime<'w> {
    pub time: Res<'w, Time>,
    pub time_scale: Res<'w, TimeScale>,
}

#[derive(SystemParam)]
pub struct ScaledTimeMut<'w> {
    pub time: Res<'w, Time>,
    pub time_scale: ResMut<'w, TimeScale>,
}

impl ScaledTimeFields for ScaledTime<'_> {
    fn time(&self) -> &Time {
        &self.time
    }

    fn time_scale(&self) -> &TimeScale {
        &self.time_scale
    }
}

impl ScaledTimeFields for ScaledTimeMut<'_> {
    fn time(&self) -> &Time {
        &self.time
    }

    fn time_scale(&self) -> &TimeScale {
        &self.time_scale
    }
}

impl<T: ScaledTimeFields> ScaledTimeDelta for T {
    fn scale(&self) -> f32 {
        self.time_scale().0
    }

    fn delta(&self) -> Duration {
        self.time().delta()
    }

    fn scaled_delta(&self) -> Duration {
        self.delta().mul_f32(self.time_scale().0)
    }

    fn delta_seconds(&self) -> f32 {
        self.delta().as_secs_f32()
    }

    fn scaled_delta_seconds(&self) -> f32 {
        self.scaled_delta().as_secs_f32()
    }
}

#[allow(dead_code)]
impl ScaledTimeMut<'_> {
    pub fn set_time_scale(&mut self, scale: f32) {
        self.time_scale.0 = scale;
    }

    pub fn time_scale_mut(&mut self) -> &mut TimeScale {
        &mut self.time_scale
    }
}

pub(super) fn update_time_scale<const SCALE: usize>(mut time: ScaledTimeMut) {
    //todo: tween?
    time.set_time_scale(SCALE as f32);
}
