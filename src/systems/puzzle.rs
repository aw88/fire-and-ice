use bevy::prelude::*;

pub struct PuzzlePlugin;

impl Plugin for PuzzlePlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(PuzzleDefinition::new())
      .add_startup_system(setup_level)
      .add_startup_system(setup_fire);
  }
}

pub struct PuzzleDefinition {
  tiles: Vec<Vec<i32>>,
  pub tile_size: Vec2,
  pub map_width: i32,
  pub map_height: i32,
  fire_atlas_handle: Option<Handle<TextureAtlas>>,
  fire_positions: Vec<Vec2>,
}

impl PuzzleDefinition {
  fn new() -> Self {
    Self {
      tiles: vec![
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
      ],
      tile_size: Vec2::splat(16.0),
      map_width: 16,
      map_height: 16,
      fire_atlas_handle: None,
      fire_positions: vec![
        Vec2::new(6.0, 5.0),
        Vec2::new(7.0, 8.0),
        Vec2::new(7.0, 9.0),
        Vec2::new(7.0, 10.0),
        Vec2::new(9.0, 11.0),
      ]
    }
  }

  fn lookup_tile(&self, x: i32, y: i32) -> i32 {
    if x >= 0 && x < self.map_width && y >= 0 && y < self.map_height {
      self.tiles[y as usize][x as usize]
    } else {
      0
    }
  }
}

fn setup_fire(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  mut puzzle: ResMut<PuzzleDefinition>,
) {
  let texture_handle = asset_server.load("fire0.png");
  let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::splat(16.0), 6, 1);
  let texture_atlas_handle = texture_atlases.add(texture_atlas);

  puzzle.fire_atlas_handle = Some(texture_atlas_handle.clone());

  for fire_position in &puzzle.fire_positions {
    info!("Creating fire: {:?}", fire_position);
    let position = Vec2::new(fire_position.x, puzzle.map_height as f32 - fire_position.y) * puzzle.tile_size;

    commands.spawn_bundle(SpriteSheetBundle {
      texture_atlas: texture_atlas_handle.clone(),
      transform: Transform::from_translation(position.extend(-0.1)),
      sprite: TextureAtlasSprite {
        index: 0,
        ..Default::default()
      },
      ..Default::default()
    });
  }
}

fn setup_level(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  puzzle: Res<PuzzleDefinition>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
  let texture_handle = asset_server.load("world0.png");
  let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 8, 1);
  let texture_atlas_handle = texture_atlases.add(texture_atlas);

  for y in 0..puzzle.map_height {
    for x in 0..puzzle.map_width {
      let left_tile = puzzle.lookup_tile(x - 1, y);
      let right_tile = puzzle.lookup_tile(x + 1, y);
      let tile = puzzle.lookup_tile(x, y);

      let tile_index = match (tile, left_tile, right_tile) {
          (1, 0, 0) => 1,
          (1, 0, 1) => 2,
          (1, 1, 0) => 4,
          (1, 1, 1) => 3,
          _ => 0,
      };

      commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform::from_xyz(x as f32 * puzzle.tile_size.x, (puzzle.map_height - y) as f32 * puzzle.tile_size.y, -0.2),
        sprite: TextureAtlasSprite {
          index: tile_index as usize,
          ..Default::default()
        },
        ..Default::default()
      });
    }
  }
}
