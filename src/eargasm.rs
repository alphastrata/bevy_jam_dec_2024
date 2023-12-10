use bevy::audio::{AudioBundle, AudioSink, AudioSource, PlaybackMode};
use bevy::ecs::system::SystemParam;
use bevy::{log, prelude::*};

pub struct EargasmPlugin;
impl Plugin for EargasmPlugin {
    fn build(&self, app: &mut App) {
        info!("Adding Eargasm plugin");
        app.add_event::<AudioRequest>()
            .add_systems(Startup, setup)
            .add_systems(Update, (play_system, once_fire_music_test));

        info!("Added Eargasm plugin");
    }
}

fn once_fire_music_test(
    keyboard_input: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<AudioRequest>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        info!("Starting music");
        event_writer.send(AudioRequest {
            component: AudioComponent::Track1(Track1),
        });
        info!("Sent event!");
    }
}

#[derive(Component, Debug)]
pub enum AudioComponent {
    Track1(Track1),
    Track2(Track2),
    Radar1(Radar1),
    Radar2(Radar2),
    TheCompanyThanksYou(TheCompanyThanksYou),
    IntroVoice(IntroVoice),
}

#[derive(Component, Debug)]
pub struct Track1;
#[derive(Component, Debug)]
pub struct Track2;
#[derive(Component, Debug)]
pub struct Radar1;
#[derive(Component, Debug)]
pub struct Radar2;
#[derive(Component, Debug)]
pub struct TheCompanyThanksYou;
#[derive(Component, Debug)]
pub struct IntroVoice;

#[derive(Event)]
pub struct AudioRequest {
    pub component: AudioComponent,
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let audio_assets = [
        ("audio/musictrack 1.mp3", AudioComponent::Track1(Track1)),
        ("audio/musictrack 2.mp3", AudioComponent::Track2(Track2)),
        // ... other assets ...
    ];

    for (path, component) in audio_assets {
        let audio_source = asset_server.load(path);
        commands
            .spawn(AudioBundle {
                source: audio_source,
                settings: PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Once,
                    // volume: todo!(),
                    // speed: todo!(),
                    paused: true,
                    // spatial: todo!(),
                    ..default()
                },
            })
            .insert(component);

        info!("Added {}", path);
    }
}

/// System: plays sounds from our `assets/audio/*.mp3`, each option there is named for an Event that you can fire with the :
/// EventWriter<[`AudioRequest`]>, which takes an [`AudioComponent`]
fn play_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<AudioRequest>,
) {
    for event in event_reader.read() {
        info!("Read AudioRequest");
        match &event.component {
            AudioComponent::Track1(_) => {
                commands
                    .spawn(AudioBundle {
                        source: asset_server.load("audio/musictrack 1.mp3"),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Once,
                            // Add any other custom settings if needed
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Track1);
            }
            AudioComponent::Track2(_) => {
                commands
                    .spawn(AudioBundle {
                        source: asset_server.load("audio/musictrack 2.mp3"),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Once,
                            // Add any other custom settings if needed
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Track2);
            }
            AudioComponent::Radar1(_) => {
                // Similar implementation for Radar1

                commands
                    .spawn(AudioBundle {
                        source: asset_server.load("audio/radar1.mp3"),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Once,
                            // Add any other custom settings if needed
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Track2);
            }
            AudioComponent::Radar2(_) => {
                // Similar implementation for Radar2

                commands
                    .spawn(AudioBundle {
                        source: asset_server.load("audio/radar2.mp3"),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Once,
                            // Add any other custom settings if needed
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Track2);
            }
            AudioComponent::TheCompanyThanksYou(_) => {
                // Similar implementation for TheCompanyThanksYou

                commands
                    .spawn(AudioBundle {
                        source: asset_server.load("audio/thecompanythanksyou.mp3"),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Once,
                            // Add any other custom settings if needed
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Track2);
            }
            AudioComponent::IntroVoice(_) => {
                // Similar implementation for IntroVoice
                commands
                    .spawn(AudioBundle {
                        source: asset_server.load("audio/introductionvoice.mp3"),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Once,
                            // Add any other custom settings if needed
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Track2);
            }
            _ => error!("Unknown audiosink/source/track pairing!"),
        }
    }
}