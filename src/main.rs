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
const CAR_SPEED: f32 = 300.0;

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
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (move_quad, move_cars_to_bottom))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Quad;

#[derive(Component)]
struct Car;

#[derive(Component)]
struct CarID(i32);

fn setup(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    commands.spawn(Camera2dBundle::default());

    for i in 0..5 {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        rng.gen_range(((SCREEN_WIDTH / 2.0) * -1.0)..(SCREEN_WIDTH / 2.0)),
                        rng.gen_range(0.0..(SCREEN_HEIGHT / 2.0)),
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
            CarID(i),
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

fn move_cars_to_bottom(
    mut query: Query<(&mut Transform, &CarID), With<Car>>,
    time_step: Res<FixedTime>,
) {
    let mut rng = rand::thread_rng();

    for (mut car_transform, _car_id) in query.iter_mut() {
        let new_car_position_y =
            car_transform.translation.y - CAR_SPEED * time_step.period.as_secs_f32();

        if new_car_position_y < (((SCREEN_HEIGHT / 2.0) * -1.0) - CAR_HEIGHT).floor() {
            car_transform.translation.y =
                ((SCREEN_HEIGHT / 2.0) + CAR_HEIGHT + rng.gen_range(0.0..CAR_HEIGHT * 2.0)).floor();
            car_transform.translation.x =
                rng.gen_range(((SCREEN_WIDTH / 2.0) * -1.0)..(SCREEN_WIDTH / 2.0));
        } else {
            car_transform.translation.y = new_car_position_y;
        }
    }
}

fn is_car_overlapping(car1_x: f32, car1_y: f32, car2_x: f32, car2_y: f32) -> bool {
    if car1_x < car2_x + CAR_WIDTH
        && car1_x + CAR_WIDTH > car2_x
        && car1_y < car2_y + CAR_HEIGHT
        && car1_y + CAR_HEIGHT > car2_y
    {
        return true;
    }
    false
}
