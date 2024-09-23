use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};
use bevy_prototype_lyon::prelude as ly;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Hole;

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

const BG_COLOR: Color = Color::rgb(0.06, 0.06, 0.06);
const POWER_SEGMENTS: f32 = 4.0;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(BG_COLOR))
        .add_plugins(DefaultPlugins)
        .add_plugin(ly::ShapePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_system)
        .add_system(shoot)
        .add_system(run_chasers)
        .add_system(run_patrols)
        .add_system(test_intersections)
        .run();
}

fn setup_system(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vec2::ZERO;

    // let window = windows.get_primary_mut().unwrap();
    // window.set_cursor_visibility(false);

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: false,
                ..default()
            },
            ..default()
        },
        BloomSettings {
            intensity: 1.0,
            ..default()
        },
    ));

    commands
        .spawn(ly::GeometryBuilder::build_as(
            &ly::shapes::Circle {
                radius: 15.0,
                center: Vec2::ZERO,
            },
            ly::DrawMode::Fill(ly::FillMode::color(Color::WHITE)),
            Transform::from_xyz(0.0, 0.0, 10.0),
        ))
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(15.0))
        .insert(Damping {
            linear_damping: 1.5,
            ..default()
        })
        .insert(ColliderMassProperties::Density(1.0))
        .insert(ExternalImpulse::default())
        .insert(Velocity::default())
        .insert(Power(0.0))
        .insert(Ball);

    commands
        .spawn(ly::GeometryBuilder::build_as(
            &ly::shapes::Circle {
                radius: 20.0,
                center: Vec2::ZERO,
            },
            ly::DrawMode::Fill(ly::FillMode::color(Color::GRAY)),
            Transform::from_xyz(100.0, 0.0, 1.0),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::ball(5.0))
        .insert(Sensor)
        .insert(Hole);

    commands
        .spawn(ly::GeometryBuilder::build_as(
            &ly::shapes::RegularPolygon {
                sides: 6,
                center: Vec2::ZERO,
                feature: ly::RegularPolygonFeature::SideLength(15.0),
            },
            ly::DrawMode::Fill(ly::FillMode::color(Color::RED)),
            Transform::from_xyz(-200.0, 100.0, 1.0),
        ))
        .insert(RigidBody::Dynamic)
        .insert(ColliderMassProperties::Density(1.0))
        .insert(Collider::ball(15.0))
        .insert(Velocity::default())
        .insert(Chaser);

    // commands
    //     .spawn(ly::GeometryBuilder::build_as(
    //         &ly::shapes::Circle {
    //             radius: 15.0,
    //             center: Vec2::ZERO,
    //         },
    //         ly::DrawMode::Fill(ly::FillMode::color(Color::GREEN)),
    //         Transform::from_xyz(-100.0, 400.0, 1.0),
    //     ))
    //     .insert(RigidBody::Dynamic)
    //     .insert(ColliderMassProperties::Density(1.0))
    //     .insert(Collider::ball(15.0))
    //     .insert(Velocity::default())
    //     .insert(Chaser);

    commands
        .spawn(ly::GeometryBuilder::build_as(
            &ly::shapes::Circle {
                radius: 15.0,
                center: Vec2::ZERO,
            },
            ly::DrawMode::Fill(ly::FillMode::color(Color::RED)),
            Transform::from_xyz(200.0, 200.0, 1.0),
        ))
        .insert(RigidBody::Dynamic)
        .insert(ColliderMassProperties::Density(1.0))
        .insert(Collider::ball(15.0))
        .insert(Velocity::default())
        .insert(Patrol {
            points: vec![
                Vec2::new(200.0, 200.0),
                Vec2::new(200.0, -200.0),
                Vec2::new(400.0, -200.0),
                Vec2::new(400.0, 200.0),
            ],
            index: 0,
        });

    commands
        .spawn(ly::GeometryBuilder::build_as(
            &ly::shapes::Circle {
                radius: 15.0,
                center: Vec2::ZERO,
            },
            ly::DrawMode::Fill(ly::FillMode::color(Color::RED)),
            Transform::from_xyz(400.0, -200.0, 1.0),
        ))
        .insert(RigidBody::Dynamic)
        .insert(ColliderMassProperties::Density(1.0))
        .insert(Collider::ball(15.0))
        .insert(Velocity::default())
        .insert(Patrol {
            points: vec![
                Vec2::new(400.0, -200.0),
                Vec2::new(400.0, 200.0),
                Vec2::new(200.0, 200.0),
                Vec2::new(200.0, -200.0),
            ],
            index: 0,
        });
}

fn shoot(
    mut commands: Commands,
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut q_ball: Query<(&mut ExternalImpulse, &mut Power, &Velocity, &Transform), With<Ball>>,
    q_power: Query<Entity, With<PowerIndicator>>,
) {
    if mouse.pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();
        let pos1 = window.cursor_position().unwrap();
        let pos2 = pos1 - Vec2::new(window.width() / 2., window.height() / 2.);

        for ent in q_power.iter() {
            commands.entity(ent).despawn();
        }

        let (_, mut power, _, trans) = q_ball.get_single_mut().unwrap();

        let dist = pos2
            .distance(trans.translation.truncate())
            .clamp(0.0, 300.0);
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
                    ly::DrawMode::Fill(ly::FillMode::color(Color::rgba(0.8, 0.8, 0.8, 0.5))),
                    Transform::from_xyz(
                        trans.translation.x + seg_x * (i as f32),
                        trans.translation.y + seg_y * (i as f32),
                        1.0,
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

        let (mut imp, mut power, _, trans) = q_ball.get_single_mut().unwrap();

        let dir = (trans.translation.truncate() - pos2).normalize() * power.0 * 2000.0;
        imp.impulse = dir;

        power.0 = 0.0;
    }

    // for (_, _, _, trans) in q_ball.iter_mut() {
    //     println!("{:?}", trans.translation);
    // }
}

fn run_chasers(
    q_ball: Query<&Transform, With<Ball>>,
    mut q_chasers: Query<(&mut Velocity, &Transform), With<Chaser>>,
) {
    let ball_trans = q_ball.get_single().unwrap();
    for (mut vel, trans) in q_chasers.iter_mut() {
        let dir = ball_trans.translation - trans.translation;
        vel.linvel = dir.truncate().normalize() * 50.0;
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
    rapier_context: Res<RapierContext>,
    q_ball: Query<(Entity, &Transform, &Collider), With<Ball>>,
    q_hole: Query<(Entity, &Transform, &Collider), With<Hole>>,
) {
    let (ent, trans, collider) = q_ball.get_single().unwrap();
    let (ent2, trans2, collider2) = q_hole.get_single().unwrap();
    // let shape_pos = trans.translation.truncate();
    // let shape_rot = 1.0;
    // let filter = QueryFilter::default();

    // rapier_context.intersections_with_shape(shape_pos, shape_rot, &collider, filter, |entity| {
    //     if ent != entity {   
    //         println!("The entity {:?} intersects our shape.", entity);
    //         true // Return `false` instead if we want to stop searching for other colliders that contain this point.
    //     } else {
    //         false
    //     }
    // });

    if rapier_context.intersection_pair(ent, ent2) == Some(true) {
        println!("The entities {:?} and {:?} have intersecting colliders!", ent, ent2);
    }
}
