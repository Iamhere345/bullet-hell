use crate::player::Health;
use crate::player::WeaponSlot;
use bevy::prelude::*;
use bevy_tweening::*;
use bevy_tweening::lens::TransformPositionLens;
use rand::*;
use std::time::Duration;

#[derive(Clone, Copy)]
pub enum EnemyType {
    Scout,
    Tank,
}

#[derive(Component, Clone, Copy)]
pub struct Enemy {
    pub enemy_type: EnemyType,
}

pub struct EnemyInitInfo {
    pub enemy: Enemy,
    pub health: Health,
    pub spawn_loc: Vec3,
    pub weapon: WeaponSlot,
}

pub struct EnemyDiedEvent;

const MOVE_TIME_MILLIS: u64 = 2000;

pub fn spawn_enemy(
    mut commands: &mut Commands,
    spawn_loc: Vec3,
    enemy: Enemy,
    health: Health,
    weapon: WeaponSlot,
) {

    println!("enemy spawned");

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                translation: spawn_loc,
                ..default()
            },
            ..default()
        })
        .insert(enemy)
        .insert(health)
        .insert(weapon);
}

/*pub fn enemy_movement(
    windows: Res<Windows>,
    time: Res<Time>,
    mut enemies: Query<(&Enemy, &mut Transform, &WeaponSlot)>,
) {
    let window = windows.get_primary().unwrap();

    for (mut enemy, mut transform, weapon) in enemies.iter_mut() {
        let mut rng = rand::thread_rng();

        //todo make move change a property of the enemy
        let move_rng = rng.gen_bool(1.0 / 60.0);

        if !move_rng {
            continue;
        }

        let x_rng: f32 = rng.gen_range(
            (window.width()/* / window.scale_factor() as f32*/) / 2.0 * -1.0
                ..(window.width()/*  / window.scale_factor() as f32*/) / 2.0,
        );
        let y_rng: f32 = rng.gen_range(
            (window.height()/* / window.scale_factor() as f32*/) / 2.0 * -1.0
                ..(window.height()/* / window.scale_factor() as f32*/) / 2.0,
        );

        let x_range = 0..(x_rng as i32);
        let y_range = 0..(y_rng as i32);

        //no explain for you
        for (x, y) in x_range.zip(y_range) {
            transform.translation.x = x as f32;
            transform.translation.y = y as f32;

            let mut timer: Timer = Timer::new(
                Duration::from_millis((MOVE_TIME_MILLIS / ((x_rng + y_rng) as i32 / (x_rng as i32 + y_rng as i32))) as u64),
                false,
            );

            while !timer.just_finished() {
                timer.tick(time.delta());
            }
        }
    }
}
*/
pub fn tween_enemy_movement(
    mut commands: Commands,
    windows: Res<Windows>,
    mut enemies: Query<(&Enemy, &Transform, &WeaponSlot, Entity)>
) {
    let window = windows.get_primary().unwrap();

    for (mut enemy, mut transform, weapon, entity) in enemies.iter_mut() {
        let mut rng = rand::thread_rng();

        println!("enemy id: {}", entity.id());

        //todo make move change a property of the enemy
        let move_rng = rng.gen_bool(1.0 / 60.0);

        if !move_rng {
            continue;
        }

        let x_rng: f32 = rng.gen_range(
            (window.width()/* / window.scale_factor() as f32*/) / 2.0 * -1.0
                ..(window.width()/*  / window.scale_factor() as f32*/) / 2.0,
        );
        let y_rng: f32 = rng.gen_range(
            (window.height()/* / window.scale_factor() as f32*/) / 2.0 * -1.0
                ..(window.height()/* / window.scale_factor() as f32*/) / 2.0,
        );

        let mut tween = Tween::new(
            EaseFunction::QuadraticOut,
            TweeningType::Once,
            Duration::from_millis(MOVE_TIME_MILLIS * (x_rng as u64 + y_rng as u64) / 10),
            TransformPositionLens {
                start: transform.translation,
                end: Vec3::new(x_rng, y_rng, 10.0)
            }
        );

        tween.set_completed(|ent, tween_| {
            println!("tween completed")
        });

        commands.entity(entity).insert(Animator::new(tween));
    }
    println!("======");
}
