use bevy::prelude::*;
use bevy_easings::*;
use leafwing_input_manager::prelude::*;

use crate::PuzzleDefinition;

#[derive(Clone, Copy)]
enum MoveDirection {
  Left,
  Right,
}

impl MoveDirection {
  pub fn as_vector(&self, size: f32) -> Vec2 {
    match self {
      &Self::Left => Vec2::X * -size,
      &Self::Right => Vec2::X * size,
    }
  }

  pub fn as_scalar(&self) -> i32 {
    match self {
      &Self::Left => -1,
      &Self::Right => 1,
    }
  }
}

#[derive(Clone, Component, Copy)]
pub struct Player {
  tile_position: IVec2,
}

impl Player {
  fn can_move(&self, direction: MoveDirection, puzzle: &PuzzleDefinition) -> i32 {
    match direction {
      MoveDirection::Left => puzzle.lookup_tile(self.tile_position.x - 1, self.tile_position.y),
      MoveDirection::Right => puzzle.lookup_tile(self.tile_position.x + 1, self.tile_position.y),
    }
  }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    Left,
    Right,
    Cast,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(EasingsPlugin)
            .add_startup_system(player_startup)
            .add_system(player_input);
    }
}

fn player_startup(
  mut commands: Commands,
  puzzle: ResMut<PuzzleDefinition>,
) {
  commands
    .spawn(SpriteBundle {
      transform: Transform::from_xyz(puzzle.player_start.x as f32 * puzzle.tile_size.x, (puzzle.player_start.y as f32 + 1.25) * puzzle.tile_size.y, 0.0),
      ..Default::default()
    })
    .insert(Player { tile_position: puzzle.player_start })
    .insert(Sprite {
      custom_size: Some(Vec2::new(puzzle.tile_size.x, puzzle.tile_size.y * 1.5)),
      ..Default::default()
    }
  );
}

fn player_input(
  mut commands: Commands,
  keys: Res<Input<KeyCode>>,
  mut player_query: Query<(Entity, &mut Player, &Transform), Without<EasingComponent<Transform>>>,
  puzzle: Res<PuzzleDefinition>,
) {
  let move_direction = if keys.just_pressed(KeyCode::Left) {
    Some(MoveDirection::Left)
  } else if keys.just_pressed(KeyCode::Right) {
    Some(MoveDirection::Right)
  } else {
    None
  };
  
  for (entity, mut player, &transform) in player_query.iter_mut() {
    if move_direction.is_some()  && player.can_move(move_direction.unwrap(), puzzle.as_ref()) == 0 {
      let old_position = transform.translation;
      let new_position = old_position + move_direction.unwrap().as_vector(16.0).extend(0.0);

      commands.entity(entity)
        .insert(transform.ease_to(
          Transform { translation: new_position, ..transform },
          EaseFunction::CubicInOut,
          EasingType::Once { duration: std::time::Duration::from_millis(200) })
        );
      
      player.tile_position = IVec2::new(player.tile_position.x + move_direction.unwrap().as_scalar(), player.tile_position.y);
    }
  }
}
