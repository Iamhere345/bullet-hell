use bevy::{prelude::*, transform};
use bevy::core::FixedTimestep;
use bevy::math::Vec3Swizzles;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Player;

#[derive(Default)]
struct CursorPos {
    pos: Vec2
}

#[derive(Default)]
struct LastPlayerPos {
    pos: Vec3
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}

fn spawn_player(mut commands: Commands) {
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

fn movement(keyboard_input: Res<Input<KeyCode>>, mut player_transform: Query<&mut Transform, With<Player>>) {
    for mut transform in player_transform.iter_mut() {
        println!("position: {}", transform.translation);
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

fn face_cursor(cursor_pos: Res<CursorPos>, mut player_transform: Query<&mut Transform, With<Player>>) {
    for mut transform in &mut player_transform.iter_mut() {
        let mouse_rot = (cursor_pos.pos - transform.translation.xy()).normalize();
        let rotate_player = Quat::from_rotation_arc(Vec3::Y, mouse_rot.extend(0.0));
        transform.rotation = rotate_player;
    }
}

fn update_cursor_pos(mut cursor_res: ResMut<CursorPos>, windows: Res<Windows>, main_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>) {
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

fn edge_collision(windows: Res<Windows>, mut last_pos_res: ResMut<LastPlayerPos>, mut plr_transform: Query<&mut Transform, With<Player>>) {
    let mut transform = plr_transform.get_single_mut().unwrap();
    let last_pos = last_pos_res.pos;
    let window = windows.get_primary().unwrap();

    //println!("last position: {}", last_pos);
    println!("x: {} y: {}, width: {}, height: {}", transform.translation.x, transform.translation.y, window.width() / window.scale_factor() as f32, window.height() / window.scale_factor() as f32);

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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(CursorPos::default())
        .insert_resource(LastPlayerPos::default())
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_player)
        .add_system(face_cursor)
        .add_system(update_cursor_pos.before(face_cursor))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.03))
                .with_system(movement)
                .with_system(edge_collision.after(movement)),
        )
        .run();
}