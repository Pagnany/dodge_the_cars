//! Shows how to render simple primitive shapes with a single color.

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowTheme},
};

use rand::Rng;

const FRAME_TIME: f32 = 1.0 / 144.0;

const QUAD_WIDTH: f32 = 40.0;
const QUAD_HEIGHT: f32 = 175.0;
const QUAD_SPEED: f32 = 500.0;

const CAR_WIDTH: f32 = 80.0;
const CAR_HEIGHT: f32 = 175.0;
const CAR_SPEED: f32 = 600.0;

const SCREEN_WIDTH: f32 = 650.0;
const SCREEN_HEIGHT: f32 = 650.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Dodge the Cars!".into(),
                    resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                    present_mode: PresentMode::AutoVsync,
                    window_theme: Some(WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            }),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .insert_resource(FixedTime::new_from_secs(FRAME_TIME))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (move_quad, text_update_system, move_cars_to_bottom),
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Component, Debug)]
struct Quad;

#[derive(Component, Debug)]
struct Car;

#[derive(Component, Debug, PartialEq, Clone, Copy, Reflect)]
struct CarID(i32);

#[derive(Component)]
struct FpsText;

fn setup(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    commands.spawn(Camera2dBundle::default());

    for i in 0..5 {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        rng.gen_range(
                            ((SCREEN_WIDTH / 2.0) * -1.0 + CAR_WIDTH)
                                ..(SCREEN_WIDTH / 2.0 - CAR_WIDTH),
                        ),
                        SCREEN_HEIGHT + CAR_HEIGHT * rng.gen_range(0.0..5.0),
                        0.0,
                    ),
                    scale: Vec3 {
                        x: (CAR_WIDTH),
                        y: (CAR_HEIGHT),
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
                translation: Vec3::new(0.0, -300.0, 0.0),
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

    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 20.0,
                color: Color::GOLD,
                ..Default::default()
            }),
        ]),
        FpsText,
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

    // Make copy of car transforms and ids to test for collisions
    let mut transform_for_cars: Vec<(Transform, CarID)> = Vec::new();
    for (car_transform, car_id) in query.iter() {
        transform_for_cars.push((car_transform.clone(), car_id.clone()));
    }

    for (mut car_transform, car_id) in query.iter_mut() {
        let new_car_position_y =
            car_transform.translation.y - CAR_SPEED * time_step.period.as_secs_f32();

        if new_car_position_y < (((SCREEN_HEIGHT / 2.0) * -1.0) - CAR_HEIGHT).floor() {
            car_transform.translation.y =
                ((SCREEN_HEIGHT / 2.0) + CAR_HEIGHT + rng.gen_range(0.0..CAR_HEIGHT)).floor();
            car_transform.translation.x = rng.gen_range(
                ((SCREEN_WIDTH / 2.0) * -1.0 + CAR_WIDTH)..(SCREEN_WIDTH / 2.0 - CAR_WIDTH),
            );

            let mut is_overlapping = true;
            while is_overlapping {
                is_overlapping = false;
                for (car_transform2, car_id2) in &transform_for_cars {
                    if car_id.0 == car_id2.0 {
                        continue;
                    }
                    if is_car_overlapping(
                        car_transform.translation.x,
                        car_transform.translation.y,
                        car_transform2.translation.x,
                        car_transform2.translation.y,
                    ) {
                        is_overlapping = true;
                        car_transform.translation.y =
                            ((SCREEN_HEIGHT / 2.0) + CAR_HEIGHT + rng.gen_range(0.0..CAR_HEIGHT))
                                .floor();
                        car_transform.translation.x = rng.gen_range(
                            ((SCREEN_WIDTH / 2.0) * -1.0 + CAR_WIDTH)
                                ..(SCREEN_WIDTH / 2.0 - CAR_WIDTH),
                        );
                        break;
                    }
                }
            }
        } else {
            car_transform.translation.y = new_car_position_y;
        }
    }
}

fn is_car_overlapping(car1_x: f32, car1_y: f32, car2_x: f32, car2_y: f32) -> bool {
    let car1_left = car1_x - CAR_WIDTH / 2.0;
    let car1_right = car1_x + CAR_WIDTH / 2.0;
    let car1_top = car1_y + CAR_HEIGHT / 2.0;
    let car1_bottom = car1_y - CAR_HEIGHT / 2.0;

    let car2_left = car2_x - CAR_WIDTH / 2.0;
    let car2_right = car2_x + CAR_WIDTH / 2.0;
    let car2_top = car2_y + CAR_HEIGHT / 2.0;
    let car2_bottom = car2_y - CAR_HEIGHT / 2.0;

    car1_left < car2_right
        && car1_right > car2_left
        && car1_top > car2_bottom
        && car1_bottom < car2_top
}

fn text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}
