//! Shows how to render simple primitive shapes with a single color.

use bevy::{
    prelude::*,
    window::{PresentMode, WindowTheme},
};

use rand::Rng;

const QUAD_WIDTH: f32 = 40.0;
const QUAD_HEIGHT: f32 = 175.0;
const QUAD_SPEED: f32 = 500.0;

const CAR_WIDTH: f32 = 175.0;
const CAR_HEIGHT: f32 = 80.0;

const SCREEN_WIDTH: f32 = 650.0;
const SCREEN_HEIGHT: f32 = 650.0;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dodge the Cars!".into(),
                resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                present_mode: PresentMode::AutoVsync,
                window_theme: Some(WindowTheme::Dark),
                ..default()
            }),
            ..default()
        }),))
        //.add_plugins(DefaultPlugins)
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, move_quad)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Quad;

#[derive(Component)]
struct Car;

fn setup(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    commands.spawn(Camera2dBundle::default());

    for _i in 0..5 {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        rng.gen_range(((SCREEN_WIDTH / 2.0) * -1.0)..(SCREEN_WIDTH / 2.0)),
                        0.0,
                        0.0,
                    ),
                    scale: Vec3 {
                        x: (CAR_HEIGHT),
                        y: (CAR_WIDTH),
                        z: (0.0),
                    },
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgba(
                        rng.gen_range(0.0..1.0),
                        rng.gen_range(0.0..1.0),
                        rng.gen_range(0.0..1.0),
                        1.0,
                    ),
                    ..default()
                },
                ..default()
            },
            Car,
        ));
    }

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -350.0, 0.0),
                scale: Vec3 {
                    x: (QUAD_WIDTH),
                    y: (QUAD_HEIGHT),
                    z: (0.0),
                },
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                ..default()
            },
            ..default()
        },
        Quad,
    ));
}

fn move_quad(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Quad>>,
    time_step: Res<FixedTime>,
) {
    let mut quad_transform = query.single_mut();
    let mut direction_x = 0.0;
    let mut direction_y = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        direction_x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction_x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        direction_y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        direction_y -= 1.0;
    }

    let new_quad_position_x =
        quad_transform.translation.x + direction_x * QUAD_SPEED * time_step.period.as_secs_f32();

    let new_quad_position_y =
        quad_transform.translation.y + direction_y * QUAD_SPEED * time_step.period.as_secs_f32();

    quad_transform.translation.x = new_quad_position_x.clamp(
        ((SCREEN_WIDTH / 2.0) * -1.0).floor(),
        (SCREEN_WIDTH / 2.0).floor(),
    );
    quad_transform.translation.y = new_quad_position_y.clamp(
        ((SCREEN_HEIGHT / 2.0) * -1.0).floor(),
        (SCREEN_HEIGHT / 2.0).floor(),
    );
}

fn is_car_overlapping(mut query: Query<&mut Transform, With<Car>>, car_x: f32, car_y: f32) -> bool {
    false
}
