use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowTheme},
};

use rand::Rng;

const FRAME_TIME: f32 = 1.0 / 144.0;

const QUAD_WIDTH: f32 = 40.0;
const QUAD_HEIGHT: f32 = 88.0;
const QUAD_SPEED: f32 = 500.0;

const CAR_WIDTH: f32 = 80.0;
const CAR_HEIGHT: f32 = 175.0;
const CAR_SPEED: f32 = 600.0;

const CAR_COUNT: i32 = 5;

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
            (
                move_quad,
                text_update_system,
                move_cars_to_bottom,
                check_quad_car_overlapp,
            ),
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .add_state::<AppState>()
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

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum AppState {
    MainMenu,
    #[default]
    InGame,
    Paused,
    GameOver,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();

    commands.spawn(Camera2dBundle::default());

    for i in 0..CAR_COUNT {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        ((SCREEN_WIDTH / 2.0) * -1.0 + (CAR_WIDTH / 2.0))
                            + SCREEN_WIDTH / CAR_COUNT as f32 * i as f32,
                        SCREEN_HEIGHT + CAR_HEIGHT * rng.gen_range(0.0..5.0),
                        0.0,
                    ),
                    rotation: Quat::from_rotation_x(std::f32::consts::PI),
                    ..default()
                },
                texture: asset_server.load("car.png"),
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
                ..default()
            },
            texture: asset_server.load("quad3.png"),
            ..default()
        },
        Quad,
    ));

    commands.spawn((
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
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Quad>>,
    time_step: Res<FixedTime>,
    app_state: Res<State<AppState>>,
) {
    match app_state.get() {
        AppState::InGame => {}
        _ => return,
    }

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

    if keyboard_input.just_pressed(KeyCode::P) {
        if app_state.get() == &AppState::InGame {
            commands.insert_resource(NextState(Some(AppState::Paused)));
        } else if app_state.get() == &AppState::Paused {
            commands.insert_resource(NextState(Some(AppState::InGame)));
        }
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
    app_state: Res<State<AppState>>,
) {
    match app_state.get() {
        AppState::InGame => {}
        _ => return,
    }

    let mut rng = rand::thread_rng();

    // Make copy of car transforms and ids to test for collisions
    let mut transform_for_cars: Vec<(Transform, CarID)> = Vec::new();
    for (car_transform, car_id) in query.iter() {
        transform_for_cars.push((*car_transform, *car_id));
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

fn check_quad_car_overlapp(
    mut commands: Commands,
    query: Query<&Transform, With<Car>>,
    quad_query: Query<&Transform, With<Quad>>,
) {
    let quad_transform = quad_query.single();

    for car_transform in query.iter() {
        if is_quad_overlapping_with_car(
            quad_transform.translation.x,
            quad_transform.translation.y,
            car_transform.translation.x,
            car_transform.translation.y,
        ) {
            commands.insert_resource(NextState(Some(AppState::GameOver)));
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

fn is_quad_overlapping_with_car(quad_x: f32, quad_y: f32, car_x: f32, car_y: f32) -> bool {
    let quad_left = quad_x - QUAD_WIDTH / 2.0;
    let quad_right = quad_x + QUAD_WIDTH / 2.0;
    let quad_top = quad_y + QUAD_HEIGHT / 2.0;
    let quad_bottom = quad_y - QUAD_HEIGHT / 2.0;

    let car_left = car_x - CAR_WIDTH / 2.0;
    let car_right = car_x + CAR_WIDTH / 2.0;
    let car_top = car_y + CAR_HEIGHT / 2.0;
    let car_bottom = car_y - CAR_HEIGHT / 2.0;

    quad_left < car_right && quad_right > car_left && quad_top > car_bottom && quad_bottom < car_top
}
