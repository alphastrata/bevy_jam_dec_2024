use std::ops::ControlFlow;

use crate::prelude::*;
use bevy::{asset::processor::ProcessorTransactionLog, ecs::bundle, prelude::*};
use rand::Rng;

pub struct CreepPlugin;
impl Plugin for CreepPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        app.insert_resource(CreepCount(0));

        app.add_event::<SpawnCreep>();

        app.add_systems(Update, (move_sprite_toward_core, spawn_creep));

        #[cfg(debug_assertions)]
        app.add_systems(Update, (dbg_send_spawn_creep_on_enter, dbg_count_creeps));
    }
}

#[derive(Component)]
pub struct Creep;

#[derive(Event)]
pub struct SpawnCreep;

#[cfg(debug_assertions)]
#[derive(Resource)]
pub struct CreepCount(pub usize);

#[cfg(debug_assertions)]
fn dbg_count_creeps(q: Query<&Transform, With<Creep>>, mut count: ResMut<CreepCount>) {
    (*count) = CreepCount(q.iter().count());
}

#[cfg(debug_assertions)]
fn dbg_send_spawn_creep_on_enter(mut spawner: EventWriter<SpawnCreep>, kb: Res<Input<KeyCode>>) {
    if kb.just_released(KeyCode::Return) {
        spawner.send(SpawnCreep);
        println!("Creep will spawn!");
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

fn spawn_creep(
    mut spawner: EventReader<SpawnCreep>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    //TODO: random xy spawn at distance from center of map...
    spawner.read().for_each(|_spawn_event| {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-500.0..=500.);
        let y = rng.gen_range(-500.0..=500.);
        //TODO: Keep a creep count, resource for UI?
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.10),
                texture: asset_server.load("textures/creep.png"),
                //
                ..default()
            },
            Creep,
            //TODO: creep stats should probs be set by .csv or something?
            MovementSpeed(10),
            AttackSpeed(10),
            Health(100),
            Experience(100),
            Heading::Stationary,
        ));
        return;
    })
}

fn move_sprite_toward_core(
    time: Res<Time>,
    mut sprite_position: Query<(&mut Heading, &mut Transform), With<Creep>>,
) {
    sprite_position
        .iter_mut()
        .for_each(|(mut creep, mut transform)| {
            // Target position (0,0) //QUESTION: this is the core right?
            let target = Vec3::new(0.0, 0.0, 0.0);

            let current = transform.translation;
            let direction = target - current;
            if direction.length() < 0.1 {
                ControlFlow::Continue::<()>(());
                // attack!
            }

            let speed = 5.0; //
            let normalized_direction = direction.normalize() * speed * time.delta_seconds();

            transform.translation += normalized_direction;

            // Update the creep's heading based on the direction
            if normalized_direction.x > 0.0 {
                *creep = Heading::East;
            } else if normalized_direction.x < 0.0 {
                *creep = Heading::West;
            } else if normalized_direction.y > 0.0 {
                *creep = Heading::North;
            } else if normalized_direction.y < 0.0 {
                *creep = Heading::South;
            }
        });
}
