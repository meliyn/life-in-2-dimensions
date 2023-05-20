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
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_system(intro_text_spawn.in_schedule(OnEnter(GameState::Intro)))
        .add_system(intro_text_fade.in_set(OnUpdate(GameState::Intro)))
        .add_system(camera_spawn)
        .add_systems((player_spawn, player_movement).in_set(OnUpdate(GameState::InGame)))
        .run();
}

#[derive(Component)]
struct MainCamera;

fn camera_spawn(mut commands: Commands, current_dimension: Res<CurrentDimension>) {
    if current_dimension.is_changed() {
        let mut camera = commands.spawn(MainCamera);
        match *current_dimension {
            CurrentDimension::Earth => {
                camera.insert(Camera2dBundle { ..default() });
            }
            CurrentDimension::SecondDimension => {
                camera.insert(Camera3dBundle { ..default() });
            }
        }
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
                        "This is my life in 2 dimensions...",
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
struct Player;

fn player_spawn(
    mut commands: Commands,
    current_dimension: Res<CurrentDimension>,
    mut meshes: ResMut<Assets<Mesh>>,
    player_query: Query<Entity, With<Player>>,
) {
    if current_dimension.is_changed() {
        for player in player_query.iter() {
            commands.entity(player).despawn();
        }

        let mut player = commands.spawn(Player);
        match *current_dimension {
            CurrentDimension::Earth => {
                player.insert((
                    Collider::cuboid(0.5, 0.5),
                    RigidBody::KinematicPositionBased,
                    SpriteBundle {
                        transform: Transform {
                            scale: Vec2::new(50.0, 50.0).extend(1.0),
                            ..default()
                        },
                        ..default()
                    },
                ));
            }
            CurrentDimension::SecondDimension => {
                player.insert(PbrBundle {
                    mesh: meshes.add(shape::Cube { size: 50.0 }.into()),
                    ..default()
                });
            }
        }
    }
}

fn player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    for mut transform in player_query.iter_mut() {
        let mut velocity = Vec2::ZERO;
        if input.any_pressed([KeyCode::Left]) {
            velocity.x -= 1.0;
        }
        if input.any_pressed([KeyCode::Right]) {
            velocity.x += 1.0;
        }
        if input.any_pressed([KeyCode::Up]) {
            velocity.y += 1.0;
        }
        if input.any_pressed([KeyCode::Down]) {
            velocity.y -= 1.0;
        }
        transform.translation += velocity.extend(0.0);
    }
}

#[derive(Component)]
struct Portal;
