use bevy::prelude::*;
use bevy_butler::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum CameraAction {
    #[actionlike(DualAxis)]
    Pan,
    #[actionlike(Axis)]
    Zoom,
    #[actionlike(Axis)]
    Rotate,
}

#[cfg(feature = "dev")]
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum DebugInput {
    ToggleWireframe,
}

#[add_plugin(to_plugin = super::Plugin)]
pub struct Plugin;

#[butler_plugin]
impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InputManagerPlugin::<CameraAction>::default(),
            #[cfg(feature = "dev")]
            InputManagerPlugin::<DebugInput>::default(),
        ));
    }
}

#[cfg(feature = "dev")]
#[add_system(plugin = Plugin, schedule = Startup)]
fn setup_debug_input(mut commands: Commands) {
    let input_map = InputMap::new([(DebugInput::ToggleWireframe, KeyCode::Backquote)]);

    commands.spawn(input_map);
}
