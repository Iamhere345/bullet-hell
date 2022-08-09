use bevy::prelude::*;
use bevy::core::FixedTimestep;
use projectile::move_projectiles;
use player::*;

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
        .insert_resource(CursorPos::default())
        .insert_resource(LastPlayerPos::default())
        .insert_resource(PlayerInfo::new())
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_player)
        .add_system(face_cursor)
        .add_system(update_cursor_pos.before(face_cursor))
        .add_system(tick_cooldowns)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.03))
                .with_system(movement)
                .with_system(shooting_input)
                .with_system(edge_collision.after(movement)),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.025))
                .with_system(move_projectiles)
        )
        .run();
}