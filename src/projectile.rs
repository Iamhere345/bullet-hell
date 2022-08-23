use crate::player::*;
use crate::enemy::EnemyDiedEvent;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::*;

#[derive(Clone, Copy)]
pub enum ProjectileType {
    Laser,
    HeavyLaser,
    Rocket,
    Seeker,
}

#[derive(Component)]
pub struct Projectile {
    projectile_type: ProjectileType,
    velocity: Vec2,
    damage: f32,
    origin: Vec2,
    range: f32,
    player: bool, //if the projectile was fired by a player or not
}
#[allow(dead_code)]
pub fn spawn_projectile(
    mut commands: &mut Commands,
    proj_type: ProjectileType,
    spawn_transform: Transform,
    vel: Vec2,
    _damage: f32,
    _range: f32,
    _player: bool,
) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::Rgba {
                    red: 0.75,
                    green: 0.15,
                    blue: 0.15,
                    alpha: 0.25,
                },
                ..default()
            },
            transform: spawn_transform,
            ..default()
        })
        .insert(Projectile {
            projectile_type: proj_type,
            velocity: vel,
            damage: _damage,
            origin: spawn_transform.translation.truncate(),
            range: _range,
            player: _player,
        });
}

pub fn move_projectiles(mut projectiles: Query<(&Projectile, &mut Transform), With<Projectile>>) {
    for (projectile, mut transform) in projectiles.iter_mut() {
        match projectile.projectile_type {
            ProjectileType::Laser => {
                transform.translation = transform.translation + projectile.velocity.extend(0.0)
            }
            _ => println!("unimplemented"),
        }
    }
}

pub fn projectile_collision(
    mut commands: Commands,
    mut projectiles: Query<(&Projectile, &mut Transform, Entity), With<Projectile>>,
    mut targets: Query<(&Transform, &mut Health, Entity), Without<Projectile>>,
    player_q: Query<Entity, With<Player>>,
    mut enemy_died_ev: EventWriter<EnemyDiedEvent>
) {
    let player = player_q.single();

    for (projectile, transform, entity) in projectiles.iter_mut() {
        let mut collision = false;

        for (target_transform, mut target_health, target_entity) in targets.iter_mut() {
            match projectile.projectile_type {
                ProjectileType::Laser => {
                    let collision_result = collide(
                        transform.translation,
                        transform.scale.truncate(),
                        target_transform.translation,
                        target_transform.scale.truncate(),
                    );

                    if collision_result.is_some() {
                        //don't need to unwrap the collision for anything yet

                        // so the player can't shoot themself
                        if target_entity.id() == player.id() && projectile.player {
                            continue;
                        }

                        println!("hit");

                        let new_health = target_health.take_damage(projectile.damage);

                        target_health.health = new_health.health;
                        target_health.armour = new_health.armour;

                        println!("{:?}", target_health);

                        if target_health.health <= 0.0 && target_entity.id() != player.id() && projectile.player {
                            println!("enemy died");
                            commands.entity(target_entity).despawn();
                            enemy_died_ev.send(EnemyDiedEvent);
                        }
                        
                        collision = true

                    }
                }
                _ => println!("unimplemented"),
            }
        }

        if collision {
            commands.entity(entity).despawn();
        }
    }
}

pub fn projectile_range(
    mut commands: Commands,
    windows: Res<Windows>,
    mut projectiles: Query<(&Projectile, &Transform, Entity), With<Projectile>>,
) {
    let window = windows.get_primary().unwrap();

    for (projectile, transform, entity) in projectiles.iter_mut() {

        if (transform.translation.x >= window.width() / window.scale_factor() as f32
            || transform.translation.x <= (window.width() / window.scale_factor() as f32) * -1.0)
            || (transform.translation.y >= window.height() / window.scale_factor() as f32
                || transform.translation.y
                    <= (window.height() / window.scale_factor() as f32) * -1.0)
        {
            commands.entity(entity).despawn();
        }
    }
}
