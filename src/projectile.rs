use bevy::prelude::*;

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
}

pub fn spawn_projectile(mut commands: Commands, proj_type: ProjectileType, spawn_transform: Transform, vel: Vec2) {
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
            velocity: vel
        });
}

pub fn move_projectiles(commands: Commands, mut projectiles: Query<(&Projectile, &mut Transform), With<Projectile>>) {
    for (projectile, mut transform) in projectiles.iter_mut() {
        match projectile.projectile_type {
            ProjectileType::Laser => {
                transform.translation = transform.translation + projectile.velocity.extend(0.0)
            },
            _ => println!("unimplemented")

        }
    }
}