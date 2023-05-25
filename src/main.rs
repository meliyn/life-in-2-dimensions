mod resources;
mod states;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use resources::*;
use states::*;

fn main() {
    App::new()
        .add_state::<GameState>()
        .insert_resource(CurrentDimension::Earth)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::BLACK))
        .add_system(intro_text_spawn.in_schedule(OnEnter(GameState::Intro)))
        .add_system(intro_text_fade.in_set(OnUpdate(GameState::Intro)))
        .add_system(camera_spawn)
        .add_system(button_spawn.in_schedule(OnEnter(GameState::InGame)))
        .add_system((button_onclick).in_set(OnUpdate(GameState::InGame)))
        .run();
}

#[derive(Component)]
struct MainCamera;

fn camera_spawn(mut commands: Commands, current_dimension: Res<CurrentDimension>) {
    if current_dimension.is_changed() {
        let mut camera = commands.spawn(MainCamera);
        camera.insert(Camera2dBundle { ..default() });
    }
}

#[derive(Component)]
struct IntroText;

fn intro_text_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            background_color: Color::BLACK.into(),
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::width(Val::Percent(100.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                IntroText,
                TextBundle {
                    text: Text::from_section(
                        "This is my life in 2 dimensions...\n(Your job is to click button)",
                        TextStyle {
                            font: asset_server.load("serif.otf"),
                            font_size: 50.0,
                            ..default()
                        },
                    ),
                    ..default()
                },
            ));
        });
}

fn intro_text_fade(
    mut commands: Commands,
    mut text_query: Query<(&Parent, &mut Text), With<IntroText>>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (parent, mut text) in text_query.iter_mut() {
        let color = &mut (text.sections[0].style.color);
        color.set_a(color.a() - time.delta_seconds() * 0.5);
        if color.a() <= 0.0 {
            commands.entity(**parent).despawn_recursive();
            next_state.set(GameState::InGame);
        }
    }
}

#[derive(Component)]
struct Button;

fn button_spawn(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                size: Size::width(Val::Percent(100.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "COINS: 0",
                    TextStyle {
                        color: Color::WHITE,
                        font: asset_server.load("sans.otf"),
                        font_size: 50.0,
                    },
                ),
                ..default()
            });
            parent
                .spawn((Button, ButtonBundle { ..default() }))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Press me!",
                            TextStyle {
                                color: Color::BLACK,
                                font: asset_server.load("sans.otf"),
                                font_size: 50.0,
                            },
                        ),
                        ..default()
                    });
                });
        });
}

fn button_onclick(button_query: Query<&Interaction, With<Button>>, mut clicked: Local<bool>) {
    for &interaction in button_query.iter() {
        match interaction {
            Interaction::Clicked => {
                if *clicked {
                    println!("ckucjed");
                }
                *clicked = false;
            }
            _ => {
                *clicked = true;
            }
        }
    }
}
