use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InGameSet {
    DespawnEntities,
    UserInput,
    EntityUpdates,
    CollisionDetection,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                InGameSet::DespawnEntities,
                InGameSet::UserInput,
                InGameSet::EntityUpdates,
                InGameSet::CollisionDetection,
            )
                .chain(),
        )
        .configure_sets(
            FixedUpdate,
            (
                InGameSet::DespawnEntities,
                InGameSet::UserInput,
                InGameSet::EntityUpdates,
                InGameSet::CollisionDetection,
            ),
        );
    }
}
