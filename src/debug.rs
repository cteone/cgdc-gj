//! Development tools for the game. This plugin is only enabled in dev builds.

//from bevy-2d starter
use bevy::{
    dev_tools::states::log_transitions, input::common_conditions::input_just_pressed, prelude::*,
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // Toggle the debug overlay for UI.
        app.add_systems(
            Update,
            toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
        );
    }
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
