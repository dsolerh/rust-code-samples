// use bevy::{prelude::*, window::PrimaryWindow};
// use rand::prelude::*;

// pub const PLAYER_SPEED: f32 = 500.0;
// pub const PLAYER_SIZE: f32 = 64.0;
// pub const ENEMY_SPEED: f32 = 200.0;
// pub const ENEMY_SIZE: f32 = 64.0;
// pub const NUMBER_OF_ENEMIES: i8 = 20;

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         // .add_plugins(PeoplePlugin)
//         .add_systems(Startup, spawn_camera)
//         .add_systems(Startup, spawn_player)
//         .add_systems(Startup, spawn_enemies)
//         .add_systems(Update, move_player)
//         .add_systems(Update, move_enemy)
//         .add_systems(Update, confine_player_movement)
//         .add_systems(Update, update_enemy_direction)
//         .add_systems(Update, confine_enemy_movement)
//         .run()
// }

// #[derive(Component)]
// pub struct Player {}

// pub fn spawn_player(
//     mut commands: Commands,
//     window_query: Query<&Window, With<PrimaryWindow>>,
//     asset_server: Res<AssetServer>,
// ) {
//     let window = window_query.get_single().unwrap();

//     commands.spawn((
//         SpriteBundle {
//             transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
//             texture: asset_server.load("sprites/ball_blue_large.png"),
//             ..default()
//         },
//         Player {},
//     ));
// }

// pub fn move_player(
//     keyboard_input: Res<Input<KeyCode>>,
//     mut player_query: Query<&mut Transform, With<Player>>,
//     time: Res<Time>,
// ) {
//     if let Ok(mut transform) = player_query.get_single_mut() {
//         let mut direction = Vec3::ZERO;

//         if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
//             direction += Vec3::new(-1.0, 0.0, 0.0)
//         }
//         if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
//             direction += Vec3::new(1.0, 0.0, 0.0)
//         }
//         if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
//             direction += Vec3::new(0.0, 1.0, 0.0)
//         }
//         if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
//             direction += Vec3::new(0.0, -1.0, 0.0)
//         }

//         if direction.length() > 0.0 {
//             direction = direction.normalize()
//         }

//         transform.translation += direction * PLAYER_SPEED * time.delta_seconds()
//     }
// }

// pub fn confine_player_movement(
//     mut player_query: Query<&mut Transform, With<Player>>,
//     window_query: Query<&Window, With<PrimaryWindow>>,
// ) {
//     if let Ok(mut player_transform) = player_query.get_single_mut() {
//         let window = window_query.get_single().unwrap();

//         let half_player_size = PLAYER_SIZE / 2.0;
//         let x_min = 0.0 + half_player_size;
//         let x_max = window.width() - half_player_size;
//         let y_min = 0.0 + half_player_size;
//         let y_max = window.height() - half_player_size;

//         let mut translation = player_transform.translation;

//         // check the player x position
//         if translation.x < x_min {
//             translation.x = x_min;
//         } else if translation.x > x_max {
//             translation.x = x_max;
//         }

//         // check the player y position
//         if translation.y < y_min {
//             translation.y = y_min;
//         } else if translation.y > y_max {
//             translation.y = y_max;
//         }

//         player_transform.translation = translation
//     }
// }

// pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
//     let window = window_query.get_single().unwrap();

//     commands.spawn(Camera2dBundle {
//         transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
//         ..default()
//     });
// }

// #[derive(Component)]
// pub struct Enemy {
//     pub direction: Vec2,
// }

// pub fn spawn_enemies(
//     mut commands: Commands,
//     window_query: Query<&Window, With<PrimaryWindow>>,
//     asset_server: Res<AssetServer>,
// ) {
//     let window = window_query.get_single().unwrap();
//     let mut rng = thread_rng();

//     let half_enemy_size = ENEMY_SIZE / 2.0;
//     let x_min = 0.0 + half_enemy_size;
//     let x_max = window.width() - half_enemy_size;
//     let y_min = 0.0 + half_enemy_size;
//     let y_max = window.height() - half_enemy_size;

//     for _ in 0..NUMBER_OF_ENEMIES {
//         let x: f32 = rng.gen_range(x_min..x_max);
//         let y: f32 = rng.gen_range(y_min..y_max);

//         commands.spawn((
//             SpriteBundle {
//                 transform: Transform::from_xyz(x, y, 0.0),
//                 texture: asset_server.load("sprites/ball_red_large.png"),
//                 ..default()
//             },
//             Enemy {
//                 direction: Vec2::new(rng.gen::<f32>(), rng.gen::<f32>()).normalize(),
//             },
//         ));
//     }
// }

// pub fn move_enemy(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
//     for (mut transform, enemy) in enemy_query.iter_mut() {
//         let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
//         transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
//     }
// }

// pub fn update_enemy_direction(
//     mut enemy_query: Query<(&Transform, &mut Enemy)>,
//     window_query: Query<&Window, With<PrimaryWindow>>,
//     asset_server: Res<AssetServer>,
// ) {
//     let window = window_query.get_single().unwrap();

//     let half_enemy_size = PLAYER_SIZE / 2.0;
//     let x_min = 0.0 + half_enemy_size;
//     let x_max = window.width() - half_enemy_size;
//     let y_min = 0.0 + half_enemy_size;
//     let y_max = window.height() - half_enemy_size;

//     for (transform, mut enemy) in enemy_query.iter_mut() {
//         let translation = transform.translation;
//         let mut direction_changed = false;

//         if translation.x < x_min || translation.x > x_max {
//             enemy.direction.x *= -1.0;
//             direction_changed = true;
//         }

//         if translation.y < y_min || translation.y > y_max {
//             enemy.direction.y *= -1.0;
//             direction_changed = true;
//         }

//         if direction_changed {
                        
//         }
//     }
// }

// pub fn confine_enemy_movement(
//     mut enemy_query: Query<&mut Transform, With<Enemy>>,
//     window_query: Query<&Window, With<PrimaryWindow>>,
// ) {
//     let window = window_query.get_single().unwrap();

//     let half_enemy_size = ENEMY_SIZE / 2.0;
//     let x_min = 0.0 + half_enemy_size;
//     let x_max = window.width() - half_enemy_size;
//     let y_min = 0.0 + half_enemy_size;
//     let y_max = window.height() - half_enemy_size;

//     for mut enemy_transform in enemy_query.iter_mut() {
//         let mut translation = enemy_transform.translation;

//         // check the player x position
//         if translation.x < x_min {
//             translation.x = x_min;
//         } else if translation.x > x_max {
//             translation.x = x_max;
//         }

//         // check the player y position
//         if translation.y < y_min {
//             translation.y = y_min;
//         } else if translation.y > y_max {
//             translation.y = y_max;
//         }

//         enemy_transform.translation = translation
//     }
// }
