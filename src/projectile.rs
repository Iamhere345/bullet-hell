use bevy::prelude::*;
use bevy::sprite::collide_aabb::*;
use crate::player::Player;

#[derive(Clone, Copy)]
pub enum ProjectileType {
    Laser,
    HeavyLaser,
    Rocket,
    Seeker
}

#[derive(Component)]
pub struct Projectile {
    projectile_type: ProjectileType,
    velocity: Vec2,
    damage: f32
}

pub fn spawn_projectile(mut commands: &mut Commands, proj_type: ProjectileType, spawn_transform: Transform, vel: Vec2) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::Rgba { red: 0.75, green: 0.15, blue: 0.15, alpha: 0.25 },
                ..default()
            },
            transform: spawn_transform,
            ..default()
        })
        .insert(Projectile {
            projectile_type: proj_type,
            velocity: vel,
            damage: 10.0
        });
}

pub fn move_projectiles(mut projectiles: Query<(&Projectile, &mut Transform), With<Projectile>>) {
    for (projectile, mut transform) in projectiles.iter_mut() {
        match projectile.projectile_type {
            ProjectileType::Laser => {
                transform.translation = transform.translation + projectile.velocity.extend(0.0)
            },
            _ => println!("unimplemented")

        }
    }
}

pub fn projectile_collision(mut projectiles: Query<(&Projectile, &mut Transform), With<Projectile>>, mut targets: Query<&Transform, (Without<Projectile>, Without<Player>)>) {
    for (projectile, transform) in projectiles.iter_mut() {
        for target in targets.iter_mut() {
            match projectile.projectile_type {
                ProjectileType::Laser => {
                    let collision_result = collide(transform.translation, transform.scale.truncate(), target.translation, target.scale.truncate());

                    if collision_result.is_some() {
                        //don't need to unwrap the collision for anything yet
                        println!()
                    }
                },
                _ => println!("unimplemented")
            }
        }
    }
}