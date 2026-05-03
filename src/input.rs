use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default());
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Sprint,
    Attack,
}

pub fn movement_input_map() -> InputMap<Action> {
    let mut input_map = InputMap::default();

    let movement_map = InputMap::new([
        (Action::Up, KeyCode::ArrowUp),
        (Action::Left, KeyCode::ArrowLeft),
        (Action::Right, KeyCode::ArrowRight),
        (Action::Down, KeyCode::ArrowDown),
        (Action::Sprint, KeyCode::ShiftLeft),
        (Action::Attack, KeyCode::KeyZ),
    ]);

    input_map.merge(&movement_map);

    input_map
}
