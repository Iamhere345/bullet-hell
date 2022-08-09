use std::time::Duration;

use bevy::prelude::*;
use bevy::math::Vec3Swizzles;
use crate::projectile::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Player;

#[derive(Default)]
pub struct CursorPos {
    pub pos: Vec2
}

pub struct WeaponSlot {
    pub cooldown: (Timer, bool),
    pub cooldown_duration: Duration,
    pub weapon_type: ProjectileType,
    pub speed: f32,
    pub damage: f32,
    pub range: f32,
    pub keybind: KeyCode
}

#[derive(Default)]
pub struct PlayerInfo {
    pub weapons: Vec<WeaponSlot>,
}

#[derive(Default)]
pub struct LastPlayerPos {
    pub pos: Vec3
}

impl WeaponSlot {
    fn new(_weapon_type: ProjectileType, _cooldown_duration: Duration,  _speed: f32, _damage: f32, _range: f32, _keybind: KeyCode) -> Self {
        WeaponSlot { 
            cooldown: (Timer::new(Duration::new(0,0), false), false),
            cooldown_duration: _cooldown_duration,
            weapon_type: _weapon_type,
            speed: _speed,
            damage: _damage,
            range: _range,
            keybind: _keybind
        }
    }
}

impl PlayerInfo {
    pub fn new() -> Self {
        PlayerInfo {
            weapons: vec![
                WeaponSlot::new(ProjectileType::Laser, Duration::from_millis(250), 2.0, 20.0, 50.0, KeyCode::E)
            ]
        }
    }
}

pub fn spawn_player(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0,1.0,1.0),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(Player);
}

pub fn movement(keyboard_input: Res<Input<KeyCode>>, mut player_transform: Query<&mut Transform, With<Player>>) {
    for mut transform in player_transform.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += 2.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= 2.0;
        }
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= 2.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += 2.0;
        }
    }
}

pub fn shooting_input(mut commands: Commands, keyboard_input: ResMut<Input<KeyCode>>, mut player_info: ResMut<PlayerInfo>, player_transform: Query<&Transform, With<Player>>) {
    let transform = player_transform.get_single().unwrap();
    
    for weapon in player_info.weapons.iter_mut() {
        if keyboard_input.pressed(weapon.keybind) && (!weapon.cooldown.1 || weapon.cooldown.0.finished()) {
            spawn_projectile(&mut commands, weapon.weapon_type, *transform, transform.local_y().truncate() * weapon.speed);
            weapon.cooldown.1 = true;
            weapon.cooldown.0.reset();
            weapon.cooldown.0.set_duration(weapon.cooldown_duration)
        }
    }

}

pub fn face_cursor(cursor_pos: Res<CursorPos>, mut player_transform: Query<&mut Transform, With<Player>>) {
    for mut transform in &mut player_transform.iter_mut() {
        let mouse_rot = (cursor_pos.pos - transform.translation.xy()).normalize();
        let rotate_player = Quat::from_rotation_arc(Vec3::Y, mouse_rot.extend(0.0));
        transform.rotation = rotate_player;
    }
}

pub fn update_cursor_pos(mut cursor_res: ResMut<CursorPos>, windows: Res<Windows>, main_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>) {
    let (camera, camera_transform) = main_camera.single();

    let window = windows.get_primary().unwrap();

    if let Some(screen_pos) = window.cursor_position() {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);

        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        let world_pos: Vec2 = world_pos.truncate();

        cursor_res.pos = world_pos;
    }
}

pub fn edge_collision(windows: Res<Windows>, mut last_pos_res: ResMut<LastPlayerPos>, mut plr_transform: Query<&mut Transform, With<Player>>) {
    let mut transform = plr_transform.get_single_mut().unwrap();
    let last_pos = last_pos_res.pos;
    let window = windows.get_primary().unwrap();

    //println!("last position: {}", last_pos);
    //println!("x: {} y: {}, width: {}, height: {}", transform.translation.x, transform.translation.y, window.width() / window.scale_factor() as f32, window.height() / window.scale_factor() as f32);

    if (transform.translation.x >= window.width() / window.scale_factor() as f32 
        || transform.translation.x <= (window.width() / window.scale_factor() as f32) * -1.0) 
        || (transform.translation.y >= window.height() / window.scale_factor() as f32 
        || transform.translation.y <= (window.height() / window.scale_factor() as f32) * -1.0) {
        println!("out of bounds");
        transform.translation = last_pos;
    } else {
        last_pos_res.pos = transform.translation;
    }
}

pub fn tick_cooldowns(time: Res<Time>, mut plr_info: ResMut<PlayerInfo>) {
    for weapon in plr_info.weapons.iter_mut() {
        weapon.cooldown.0.tick(time.delta());
    }
}