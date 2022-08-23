use std::time::Duration;

use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_tweening::*;
use enemy::{EnemyInitInfo, Enemy, EnemyType, EnemyDiedEvent, enemy_movement};
use level::*;
use player::*;
use projectile::*;

pub mod enemy;
pub mod level;
pub mod player;
pub mod projectile;

fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TweeningPlugin)
        .insert_resource(CurrentLevel {
            index: 0,
            enemies_left: 0
        })
        .insert_resource(CursorPos::default())
        .insert_resource(LastPlayerPos::default())
        .insert_resource(PlayerInfo::new())
        .insert_resource(Levels(vec![
            Level { //startup level; has nothing in it.
                enemies: Vec::new(),
                difficulty: Difficulty::Normal
            },
            Level {
                enemies: vec![
                    EnemyInitInfo {
                        enemy: Enemy { enemy_type: EnemyType::Scout },
                        health: Health { health: 50.0, armour: 0.0 },
                        spawn_loc: Vec3::new(0.0, 10.0, 10.0),
                        weapon: WeaponSlot::new(ProjectileType::Laser, Duration::from_millis(500), 2.0, 10.0, KeyCode::E)
                    },
                    EnemyInitInfo {
                        enemy: Enemy { enemy_type: EnemyType::Scout },
                        health: Health { health: 50.0, armour: 0.0 },
                        spawn_loc: Vec3::new(10.0, 10.0, 10.0),
                        weapon: WeaponSlot::new(ProjectileType::Laser, Duration::from_millis(500), 2.0, 10.0, KeyCode::E)
                    },
                    EnemyInitInfo {
                        enemy: Enemy { enemy_type: EnemyType::Scout },
                        health: Health { health: 50.0, armour: 0.0 },
                        spawn_loc: Vec3::new(-10.0, 10.0, 10.0),
                        weapon: WeaponSlot::new(ProjectileType::Laser, Duration::from_millis(500), 2.0, 10.0, KeyCode::E)
                    },
                ],
                difficulty: Difficulty::Normal
            },
            Level {
                enemies: vec![
                    EnemyInitInfo {
                        enemy: Enemy { enemy_type: EnemyType::Scout },
                        health: Health { health: 50.0, armour: 0.0 },
                        spawn_loc: Vec3::new(0.0, 10.0, 10.0),
                        weapon: WeaponSlot::new(ProjectileType::Laser, Duration::from_millis(500), 2.0, 20.0, KeyCode::E)
                    },
                    EnemyInitInfo {
                        enemy: Enemy { enemy_type: EnemyType::Scout },
                        health: Health { health: 50.0, armour: 0.0 },
                        spawn_loc: Vec3::new(10.0, 10.0, 10.0),
                        weapon: WeaponSlot::new(ProjectileType::Laser, Duration::from_millis(500), 2.0, 20.0, KeyCode::E)
                    },
                    EnemyInitInfo {
                        enemy: Enemy { enemy_type: EnemyType::Scout },
                        health: Health { health: 50.0, armour: 0.0 },
                        spawn_loc: Vec3::new(-10.0, 10.0, 10.0),
                        weapon: WeaponSlot::new(ProjectileType::Laser, Duration::from_millis(500), 2.0, 20.0, KeyCode::E)
                    },
                    EnemyInitInfo {
                        enemy: Enemy { enemy_type: EnemyType::Scout },
                        health: Health { health: 50.0, armour: 0.0 },
                        spawn_loc: Vec3::new(0.0, -10.0, 10.0),
                        weapon: WeaponSlot::new(ProjectileType::Laser, Duration::from_millis(500), 2.0, 20.0, KeyCode::E)
                    },
                ],
                difficulty: Difficulty::Normal
            },
            Level {
                enemies: vec![
                    EnemyInitInfo {
                        enemy: Enemy { enemy_type: EnemyType::Scout },
                        health: Health { health: 50.0, armour: 0.0 },
                        spawn_loc: Vec3::new(0.0, 10.0, 10.0),
                        weapon: WeaponSlot::new(ProjectileType::Laser, Duration::from_millis(500), 2.0, 20.0, KeyCode::E)
                    },
                    EnemyInitInfo {
                        enemy: Enemy { enemy_type: EnemyType::Scout },
                        health: Health { health: 50.0, armour: 0.0 },
                        spawn_loc: Vec3::new(10.0, 10.0, 10.0),
                        weapon: WeaponSlot::new(ProjectileType::Laser, Duration::from_millis(500), 2.0, 20.0, KeyCode::E)
                    },
                    EnemyInitInfo {
                        enemy: Enemy { enemy_type: EnemyType::Scout },
                        health: Health { health: 50.0, armour: 0.0 },
                        spawn_loc: Vec3::new(-10.0, 10.0, 10.0),
                        weapon: WeaponSlot::new(ProjectileType::Laser, Duration::from_millis(500), 2.0, 20.0, KeyCode::E)
                    },
                    EnemyInitInfo {
                        enemy: Enemy { enemy_type: EnemyType::Scout },
                        health: Health { health: 50.0, armour: 0.0 },
                        spawn_loc: Vec3::new(0.0, -10.0, 10.0),
                        weapon: WeaponSlot::new(ProjectileType::Laser, Duration::from_millis(500), 2.0, 20.0, KeyCode::E)
                    },
                ],
                difficulty: Difficulty::Hard
            },
        ]))
        
        .add_event::<NextLevelEvent>()
        .add_event::<EnemyDiedEvent>()

        .add_startup_system(setup_camera)
        .add_startup_system(spawn_player)

        .add_system(face_cursor)
        .add_system(update_cursor_pos.before(face_cursor))
        .add_system(tick_cooldowns)

        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.03))
                .with_system(movement)
                .with_system(enemy_movement)
                .with_system(shooting_input)
                .with_system(edge_collision.after(movement))
                .with_system(projectile_collision.after(movement))
                .with_system(enemy_died.after(projectile_collision))
                .with_system(next_level.after(enemy_died)),
        )

        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.025))
                .with_system(move_projectiles)
                .with_system(projectile_range.after(move_projectiles)),
        )
        .run();
}
