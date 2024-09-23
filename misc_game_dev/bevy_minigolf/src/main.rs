use bevy::{core_pipeline::bloom::BloomSettings, prelude::*, window::WindowResizeConstraints};
use bevy_prototype_lyon::prelude as ly;
use bevy_rapier2d::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
// use read_levels::read_levels;

#[derive(PartialEq, Debug)]
enum BallStatus {
    READY,
    MOVING,
    COOLDOWN,
}

#[derive(Component)]
struct Ball {
    status: BallStatus,
    timer: Timer,
}

#[derive(Resource)]
struct GameData {
    strokes: usize,
    level: usize,
}

impl Default for GameData {
    fn default() -> Self {
        GameData {
            strokes: 0,
            level: 0,
        }
    }
}

#[derive(Component)]
struct Hole;

#[derive(Component)]
struct HoleDetector;

#[derive(Component)]
struct Chaser;

#[derive(Component)]
struct PowerIndicator;

#[derive(Component)]
struct Power(f32);

#[derive(Component)]
struct Patrol {
    points: Vec<Vec2>,
    index: usize,
}

#[derive(Component)]
struct UILevel;

#[derive(Component)]
struct UIStroke;

#[derive(Component)]
struct CooldownIndicator;

#[derive(Component)]
struct LevelNode;

struct NextLevelEvent;

struct UpdateUIEvent;

mod level_data;
use level_data::read_levels;

const BG_COLOR: Color = Color::rgb(0.46, 0.46, 0.46);
const GREEN_COLOR: Color = Color::rgb(0.08, 0.70, 0.00);
const WALL_COLOR: Color = Color::BLACK;
const POWER_SEGMENTS: f32 = 4.0;
const COOLDOWN: f32 = 1.0;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(BG_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 900.0,
                height: 700.0,
                resize_constraints: WindowResizeConstraints {
                    min_width: 900.0,
                    max_width: 900.0,
                    min_height: 700.0,
                    max_height: 700.0,
                },
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_plugin(ly::ShapePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        // .add_plugin(WorldInspectorPlugin)
        .add_event::<NextLevelEvent>()
        .add_event::<UpdateUIEvent>()
        .insert_resource(GameData::default())
        .add_startup_system(setup)
        .add_system(shoot)
        .add_system(ball_cooldown)
        .add_system(run_chasers)
        .add_system(run_patrols)
        .add_system(adjust_hole_collider)
        .add_system(
            test_intersections
                .after(adjust_hole_collider)
                .after(run_chasers)
                .after(ball_cooldown),
        )
        .add_system(next_level.after(test_intersections))
        .add_system(update_ui)
        .run();
}

fn setup(
    mut commands: Commands,
    // mut windows: ResMut<Windows>,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut game_data: ResMut<GameData>,
    mut level_evt: EventWriter<NextLevelEvent>,
    asset_server: Res<AssetServer>,
) {
    rapier_config.gravity = Vec2::ZERO;

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: false,
                ..default()
            },
            ..default()
        },
        BloomSettings {
            intensity: 2.0,
            ..default()
        },
    ));

    commands
        .spawn(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.),
                    left: Val::Px(8.),
                    ..default()
                },
                ..default()
            },
            text: Text::from_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/JetBrainsMono-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::BLACK,
                },
            ),
            ..default()
        })
        .insert(UILevel);

    commands
        .spawn(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.),
                    right: Val::Px(10.),
                    ..default()
                },
                ..default()
            },
            text: Text::from_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/JetBrainsMono-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::BLACK,
                },
            ),
            ..default()
        })
        .insert(UIStroke);

    game_data.strokes = 0;
    game_data.level = 0;

    // TODO add custom cursor
    // let window = windows.get_primary_mut().unwrap();
    // window.set_cursor_visibility(false);

    level_evt.send(NextLevelEvent);
}

fn update_ui(
    mut ui_event: ResMut<Events<UpdateUIEvent>>,
    mut q_text: ParamSet<(
        Query<&mut Text, With<UILevel>>,
        Query<&mut Text, With<UIStroke>>,
    )>,
    game_data: Res<GameData>,
) {
    for _evt in ui_event.drain() {
        let level = &read_levels()[game_data.level];
        for mut text in q_text.p0().iter_mut() {
            text.sections[0].value = format!(
                "Level {} | Par {}",
                game_data.level.to_string(),
                level.par.to_string()
            );
        }

        for mut text in q_text.p1().iter_mut() {
            text.sections[0].value =
                format!("Remaining Strokes: {}", game_data.strokes.to_string());
        }
    }
}

fn next_level(
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    mut level_evt: ResMut<Events<NextLevelEvent>>,
    mut ui_event: EventWriter<UpdateUIEvent>
) {
    for _evt in level_evt.drain() {
        let level = &read_levels()[game_data.level];

        game_data.strokes += level.par;
        game_data.level += 1;

        ui_event.send(UpdateUIEvent);

        for green in level.green.iter() {
            commands
                .spawn(ly::GeometryBuilder::build_as(
                    &ly::shapes::Rectangle {
                        origin: ly::RectangleOrigin::Center,
                        extents: Vec2::new(green.width, green.height),
                    },
                    ly::DrawMode::Fill(ly::FillMode::color(GREEN_COLOR)),
                    Transform::from_xyz(green.x, green.y, 0.0),
                ))
                .insert(LevelNode);
        }

        for wall in level.wall.iter() {
            commands
                .spawn(ly::GeometryBuilder::build_as(
                    &ly::shapes::Rectangle {
                        origin: ly::RectangleOrigin::Center,
                        extents: Vec2::new(wall.width, wall.height),
                    },
                    ly::DrawMode::Fill(ly::FillMode::color(WALL_COLOR)),
                    Transform::from_xyz(wall.x, wall.y, 1.0),
                ))
                .insert(Restitution::new(0.5))
                .insert(Friction::new(0.0))
                .insert(Collider::cuboid(wall.width / 2.0, wall.height / 2.0))
                .insert(LevelNode);
        }

        for chaser in level.chaser.iter() {
            commands
                .spawn(ly::GeometryBuilder::build_as(
                    &ly::shapes::Circle {
                        radius: 15.0,
                        center: Vec2::ZERO,
                    },
                    ly::DrawMode::Fill(ly::FillMode::color(Color::RED)),
                    Transform::from_xyz(chaser.x, chaser.y, 3.0),
                ))
                .insert(RigidBody::Dynamic)
                .insert(ColliderMassProperties::Density(1.0))
                .insert(Collider::ball(15.0))
                .insert(Sensor)
                .insert(Velocity::default())
                .insert(Chaser)
                .insert(LevelNode);
        }

        commands
            .spawn(ly::GeometryBuilder::build_as(
                &ly::shapes::Circle {
                    radius: 10.0,
                    center: Vec2::ZERO,
                },
                ly::DrawMode::Fill(ly::FillMode::color(Color::WHITE)),
                Transform::from_xyz(level.ball.x, level.ball.y, 10.0),
            ))
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(10.0))
            .insert(Damping {
                linear_damping: 2.0,
                ..default()
            })
            .insert(Restitution::new(0.5))
            .insert(Friction::new(0.0))
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(ExternalImpulse::default())
            .insert(Velocity::default())
            .insert(Power(0.0))
            .insert(Ccd::enabled())
            .insert(Ball {
                status: BallStatus::READY,
                timer: Timer::default(),
            })
            .insert(LevelNode)
            .with_children(|parent| {
                parent.spawn(Collider::ball(2.0)).insert(HoleDetector);
            });

        commands
            .spawn(ly::GeometryBuilder::build_as(
                &ly::shapes::Circle {
                    radius: 15.0,
                    center: Vec2::ZERO,
                },
                ly::DrawMode::Fill(ly::FillMode::color(Color::BLACK)),
                Transform::from_xyz(level.hole.x, level.hole.y, 2.0),
            ))
            .insert(RigidBody::Fixed)
            .insert(Collider::ball(5.0))
            .insert(Sensor)
            .insert(Hole)
            .insert(LevelNode);

        // commands
        //     .spawn(ly::GeometryBuilder::build_as(
        //         &ly::shapes::Circle {
        //             radius: 15.0,
        //             center: Vec2::ZERO,
        //         },
        //         ly::DrawMode::Fill(ly::FillMode::color(Color::RED)),
        //         Transform::from_xyz(200.0, 200.0, 1.0),
        //     ))
        //     .insert(RigidBody::Dynamic)
        //     .insert(ColliderMassProperties::Density(1.0))
        //     .insert(Collider::ball(15.0))
        //     .insert(Velocity::default())
        //     .insert(Patrol {
        //         points: vec![
        //             Vec2::new(200.0, 200.0),
        //             Vec2::new(200.0, -200.0),
        //             Vec2::new(400.0, -200.0),
        //             Vec2::new(400.0, 200.0),
        //         ],
        //         index: 0,
        //     });

        // commands
        //     .spawn(ly::GeometryBuilder::build_as(
        //         &ly::shapes::Circle {
        //             radius: 15.0,
        //             center: Vec2::ZERO,
        //         },
        //         ly::DrawMode::Fill(ly::FillMode::color(Color::RED)),
        //         Transform::from_xyz(400.0, -200.0, 1.0),
        //     ))
        //     .insert(RigidBody::Dynamic)
        //     .insert(ColliderMassProperties::Density(1.0))
        //     .insert(Collider::ball(15.0))
        //     .insert(Velocity::default())
        //     .insert(Patrol {
        //         points: vec![
        //             Vec2::new(400.0, -200.0),
        //             Vec2::new(400.0, 200.0),
        //             Vec2::new(200.0, 200.0),
        //             Vec2::new(200.0, -200.0),
        //         ],
        //         index: 0,
        //     });
    }
}

fn shoot(
    mut commands: Commands,
    mouse: Res<Input<MouseButton>>,
    mut game_data: ResMut<GameData>,
    windows: Res<Windows>,
    mut q_ball: Query<(
        &mut ExternalImpulse,
        &mut Power,
        &Velocity,
        &Transform,
        &mut Ball,
    )>,
    q_power: Query<Entity, With<PowerIndicator>>,
    mut q_text: Query<&mut Text, With<UIStroke>>,
) {
    if let Ok((mut imp, mut power, _vel, trans, mut ball)) = q_ball.get_single_mut() {
        if ball.status != BallStatus::READY {
            return;
        }

        if mouse.pressed(MouseButton::Left) {
            let window = windows.get_primary().unwrap();
            let pos1 = window.cursor_position().unwrap();
            let pos2 = pos1 - Vec2::new(window.width() / 2., window.height() / 2.);

            for ent in q_power.iter() {
                commands.entity(ent).despawn();
            }

            let dist = pos2
                .distance(trans.translation.truncate())
                .clamp(0.0, 200.0);
            let angle = f32::atan2(pos2.y - trans.translation.y, pos2.x - trans.translation.x);
            let seg_x = (f32::cos(angle) * dist) / POWER_SEGMENTS;
            let seg_y = (f32::sin(angle) * dist) / POWER_SEGMENTS;

            for i in 1..=(POWER_SEGMENTS as usize) {
                commands
                    .spawn(ly::GeometryBuilder::build_as(
                        &ly::shapes::Circle {
                            radius: 5.0,
                            center: Vec2::ZERO,
                        },
                        ly::DrawMode::Fill(ly::FillMode::color(Color::rgba(1.0, 1.0, 1.0, 1.0))),
                        Transform::from_xyz(
                            trans.translation.x + seg_x * (i as f32),
                            trans.translation.y + seg_y * (i as f32),
                            4.0,
                        ),
                    ))
                    .insert(PowerIndicator);
            }

            power.0 = dist;
        }

        if mouse.just_released(MouseButton::Left) {
            let window = windows.get_primary().unwrap();
            let pos1 = window.cursor_position().unwrap();
            let pos2 = pos1 - Vec2::new(window.width() / 2., window.height() / 2.);

            for ent in q_power.iter() {
                commands.entity(ent).despawn();
            }

            ball.status = BallStatus::MOVING;
            ball.timer = Timer::from_seconds(0.1, TimerMode::Once);

            let dir = (trans.translation.truncate() - pos2).normalize() * power.0 * 3000.0;
            imp.impulse = dir;

            power.0 = 0.0;
            game_data.strokes -= 1;

            let mut text = q_text.get_single_mut().unwrap();
            text.sections[0].value =
                format!("Remaining Strokes: {}", game_data.strokes.to_string());
        }
    }
}

fn ball_cooldown(
    mut commands: Commands,
    mut q_ball: Query<(Entity, &mut Velocity, &Transform, &mut Ball)>,
    q_cooldown: Query<Entity, With<CooldownIndicator>>,
    time: Res<Time>,
    game_data: Res<GameData>,
) {
    if let Ok((ent, mut vel, trans, mut ball)) = q_ball.get_single_mut() {
        ball.timer.tick(time.delta());

        match ball.status {
            BallStatus::READY => {
                for ent in q_cooldown.iter() {
                    commands.entity(ent).despawn();
                }
            }
            BallStatus::MOVING => {
                if ball.timer.finished() && vel.linvel.length() < 15.0 {
                    vel.linvel = Vec2::ZERO;
                    commands.entity(ent).insert(ly::GeometryBuilder::build_as(
                        &ly::shapes::Circle {
                            radius: 10.0,
                            center: Vec2::ZERO,
                        },
                        ly::DrawMode::Fill(ly::FillMode::color(Color::rgba(1.0, 1.0, 1.0, 0.25))),
                        *trans,
                    ));

                    if game_data.strokes == 0 {
                        println!("{:?}", "you dead");
                    }

                    ball.status = BallStatus::COOLDOWN;
                    ball.timer = Timer::from_seconds(COOLDOWN, TimerMode::Once);
                }
            }
            BallStatus::COOLDOWN => {
                if ball.timer.finished() {
                    ball.status = BallStatus::READY;
                    commands.entity(ent).insert(ly::GeometryBuilder::build_as(
                        &ly::shapes::Circle {
                            radius: 10.0,
                            center: Vec2::ZERO,
                        },
                        ly::DrawMode::Fill(ly::FillMode::color(Color::rgba(1.0, 1.0, 1.0, 10.0))),
                        *trans,
                    ));
                } else {
                    let radius = (ball.timer.elapsed_secs() / COOLDOWN) * 10.0;
                    commands
                        .spawn(ly::GeometryBuilder::build_as(
                            &ly::shapes::Circle {
                                radius,
                                center: Vec2::ZERO,
                            },
                            ly::DrawMode::Fill(ly::FillMode::color(Color::rgba(
                                1.0, 1.0, 1.0, 1.0,
                            ))),
                            *trans,
                        ))
                        .insert(CooldownIndicator);
                }
            }
        }
    }
}

fn run_chasers(
    q_ball: Query<&Transform, With<Ball>>,
    mut q_chasers: Query<(&mut Velocity, &Transform), With<Chaser>>,
) {
    if let Ok(ball_trans) = q_ball.get_single() {
        for (mut vel, trans) in q_chasers.iter_mut() {
            let dir = ball_trans.translation - trans.translation;
            vel.linvel = dir.truncate().normalize() * 60.0;
        }
    }
}

fn run_patrols(mut q_patrols: Query<(&Transform, &mut Velocity, &mut Patrol)>) {
    for (trans, mut vel, mut patrol) in q_patrols.iter_mut() {
        if (trans.translation.truncate() - patrol.points[patrol.index])
            .abs()
            .max_element()
            < 10.0
        {
            patrol.index = (patrol.index + 1) % patrol.points.len();
        } else {
            let dir = patrol.points[patrol.index] - trans.translation.truncate();
            vel.linvel = dir.normalize() * 100.0;
        }
    }
}

fn test_intersections(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut q_ball: Query<(Entity, &Transform, &mut Velocity, &Children), With<Ball>>,
    q_child: Query<Entity, With<HoleDetector>>,
    q_hole: Query<(Entity, &Transform), With<Hole>>,
    q_chasers: Query<Entity, With<Chaser>>,
    mut level_evt: EventWriter<NextLevelEvent>,
    q_game_nodes: Query<Entity, With<LevelNode>>,
    mut game_data: ResMut<GameData>,
    mut ui_event: EventWriter<UpdateUIEvent>
) {
    if let Ok((ball, trans, mut vel, children)) = q_ball.get_single_mut() {
        if let Ok((ent2, trans2)) = q_hole.get_single() {
            for &child in children.iter() {
                let child_ent = q_child.get(child).unwrap();

                if rapier_context.intersection_pair(child_ent, ent2) == Some(true) {
                    let dist = trans2.translation.distance(trans.translation);
                    if dist < 9.1 {
                        vel.linvel = Vec2::ZERO;
                        for node in q_game_nodes.iter() {
                            commands.entity(node).despawn_recursive();
                        }
                        level_evt.send(NextLevelEvent {});
                    } else {
                        let dir = (trans2.translation - trans.translation)
                            .truncate()
                            .normalize();
                        vel.linvel = dir * 20.0;
                    }
                    // println!("{:?} {:?}", 0, dist);
                }
            }
        }

        for chaser in q_chasers.iter() {
            if rapier_context.intersection_pair(chaser, ball) == Some(true) {
                game_data.strokes -= 1;
                commands.entity(chaser).despawn();
                ui_event.send(UpdateUIEvent);
            }
        }
    }
}

fn adjust_hole_collider(
    mut commands: Commands,
    q_ball: Query<&Velocity, With<Ball>>,
    q_hole: Query<Entity, With<Hole>>,
) {
    if let Ok(vel) = q_ball.get_single() {
        if let Ok(hole) = q_hole.get_single() {
            if vel.linvel.length() < 15.0 {
                commands.entity(hole).insert(Collider::ball(15.0));
            } else if vel.linvel.length() < 50.0 {
                commands.entity(hole).insert(Collider::ball(12.5));
            } else if vel.linvel.length() < 100.0 {
                commands.entity(hole).insert(Collider::ball(10.0));
            } else if vel.linvel.length() < 200.0 {
                commands.entity(hole).insert(Collider::ball(7.5));
            } else {
                commands.entity(hole).insert(Collider::ball(5.0));
            }
        }
    }
}
