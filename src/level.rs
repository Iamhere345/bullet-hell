use crate::enemy::*;
use bevy::prelude::*;

/*
Level: controls game difficulty and levels
*/

pub struct NextLevelEvent;

pub struct Level {
    pub enemies: Vec<EnemyInitInfo>,
    pub difficulty: Difficulty,
}

pub struct CurrentLevel {
    pub index: usize,
    pub enemies_left: usize
}

pub struct Levels(pub Vec<Level>);

pub enum Difficulty {
    Easy,
    Normal,
    Hard,
    Impossible,
}

pub fn next_level(
    mut next_level_ev: EventReader<NextLevelEvent>,
    mut commands: Commands,
    mut current_level: ResMut<CurrentLevel>,
    levels: Res<Levels>,
    enemies: Query<Entity, With<Enemy>>,
) {
    for event in next_level_ev.iter() {

        println!("next level");

        // increment level struct (this should be first so a new level isn't loaded twice)
        current_level.index = current_level.index + 1;

        // clear current level
        for ent in enemies.iter() {
            println!("enemy id: {}", ent.id());
            commands.entity(ent).despawn();
        }

        //spawn new enemies for next level
        let level_opt = levels.0.get(current_level.index);

        if level_opt.is_none() {
            //todo handle end of game properly
            panic!("out of levels");
        }

        let level = level_opt.unwrap();

        for enemy in level.enemies.iter() {
            // change health based difficulty
            let mut health_scaled = enemy.health;

            match level.difficulty {
                Difficulty::Easy => {
                    health_scaled.armour *= 0.5;
                    health_scaled.health *= 0.5;
                }
                Difficulty::Hard => {
                    health_scaled.armour *= 1.5;
                    health_scaled.health *= 1.5;
                }
                Difficulty::Impossible => {
                    health_scaled.armour *= 5.0;
                    health_scaled.health *= 5.0;
                }
                _ => {}
            }

            spawn_enemy(
                &mut commands,
                enemy.spawn_loc,
                enemy.enemy,
                health_scaled,
                enemy.weapon.clone(),
            );

            current_level.enemies_left += 1;
        }
    }
}

pub fn enemy_died(mut enemy_died_ev: EventReader<EnemyDiedEvent>, mut next_level_ev: EventWriter<NextLevelEvent>, mut current_level_res: ResMut<CurrentLevel>) {
    for event in enemy_died_ev.iter() {
        current_level_res.enemies_left += 1;

        if current_level_res.enemies_left <= 0 {
            next_level_ev.send(NextLevelEvent);
        }
    }

    if current_level_res.enemies_left <= 0 {
        next_level_ev.send(NextLevelEvent);
    }

}