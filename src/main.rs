mod systems;

use bevy::prelude::*;

use crate::systems::puzzle::*;
use crate::systems::player::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(PuzzlePlugin)
    .add_plugin(PlayerPlugin)
    .add_system(bevy::window::close_on_esc)
    .add_startup_system(setup_camera)
    .run();
}

fn setup_camera(
  mut commands: Commands,
  puzzle_definition: Res<PuzzleDefinition>,
) {
  let mut camera_bundle = Camera2dBundle::default();

  camera_bundle.transform.translation = Vec3::new(
    (puzzle_definition.map_width as f32 * 0.5 - 0.5) * puzzle_definition.tile_size.x,
    (puzzle_definition.map_height as f32 * 0.5 + 0.5) * puzzle_definition.tile_size.y,
    0.0,
  );

  camera_bundle.transform.scale = Vec3::new(1.0/2.0, 1.0/2.0, 1.0);

  commands.spawn(camera_bundle);
}
