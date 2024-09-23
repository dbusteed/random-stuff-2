use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
// use bevy_mod_picking::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;
use rand::seq::SliceRandom;

#[derive(Component)]
struct Die;

#[derive(Component)]
struct SelectedDie;

struct AssetHandles {
    die: Handle<Scene>,
}

struct LastClick(Vec2);

mod flycam;
use flycam::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        // .add_plugin(PlayerPlugin)
        // .add_plugins(DefaultPickingPlugins)
        .add_startup_system(setup)
        .add_system(debug)
        .add_system(dev)
        .add_system(move_camera)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    // let asset_handles = AssetHandles {
    //     die: asset_server.load("die.glb#Scene0"),
    // };
    // commands.insert_resource(asset_handles);

    let die: Handle<Scene> = asset_server.load("die.glb#Scene0");

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(100.0, 1.0, 200.0))),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            material: materials.add(StandardMaterial {
                base_color: Color::DARK_GREEN,
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(50., 0.5, 100.));

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(10.0, 4.0, 10.0))),
            transform: Transform::from_xyz(-10.0, 2.0, -10.0),
            material: materials.add(StandardMaterial {
                base_color: Color::BLACK,
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(5., 2., 5.));

    // light
    // commands.spawn_bundle(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 2000.0,
    //         shadows_enabled: false,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(2.0, 10.0, 2.0),
    //     ..default()
    // });

    ambient_light.color = Color::WHITE;
    ambient_light.brightness = 0.75;

    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(15.0, 30.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert_bundle(SceneBundle {
            scene: die.clone(),
            transform: Transform {
                translation: Vec3::new(0., 1., 0.),
                scale: Vec3::new(0.5, 0.5, 0.5),
                ..default()
            },
            ..default()
        })
        .insert(Collider::cuboid(1.0, 1.0, 1.0))
        .insert(Name::new("Die 1"))
        .insert(Die);

    commands.insert_resource(LastClick(Vec2::ZERO));
}

fn debug(
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<&Transform, With<Die>>,
    rapier_context: Res<RapierContext>,
) {
    if keyboard_input.just_released(KeyCode::Space) {
        for trans in &query {

            let faces: [Vec3; 6] = [
                Vec3::X,  // 1
                -Vec3::Y, // 2
                Vec3::Z,  // 3
                -Vec3::Z, // 4
                Vec3::Y,  // 5
                -Vec3::X, // 6
            ];

            let mut rolled_num: usize = 1;
            let mut max_distance: f32 = -10.0;
                        
            for (i, dir) in faces.iter().enumerate() {
                let val = trans.rotation.mul_vec3(*dir).y;
                if val > max_distance {
                    rolled_num = i + 1;
                    max_distance = val;
                }
            }

            println!("=> {:?}", rolled_num);
        }
    }
}

fn dev(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    rapier_context: Res<RapierContext>,
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    mut last_click: ResMut<LastClick>,
    query: Query<Entity, With<SelectedDie>>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    if let Ok(ent) = query.get_single() {
        if buttons.just_released(MouseButton::Left) {
            let (camera, camera_transform) = camera.get_single().unwrap();
            let window = windows.get_primary().unwrap();
            let cursor_pos_screen = window.cursor_position().unwrap();
            let view = camera_transform.compute_matrix();
            let (viewport_min, viewport_max) = camera.logical_viewport_rect().unwrap();
            let screen_size = camera.logical_target_size().unwrap();
            let viewport_size = viewport_max - viewport_min;
            let adj_cursor_pos =
                cursor_pos_screen - Vec2::new(viewport_min.x, screen_size.y - viewport_max.y);
            let projection = camera.projection_matrix();
            let far_ndc = projection.project_point3(Vec3::NEG_Z).z;
            let near_ndc = projection.project_point3(Vec3::Z).z;
            let cursor_ndc = (adj_cursor_pos / viewport_size) * 2.0 - Vec2::ONE;
            let ndc_to_world: Mat4 = view * projection.inverse();
            let near = ndc_to_world.project_point3(cursor_ndc.extend(near_ndc));
            let far = ndc_to_world.project_point3(cursor_ndc.extend(far_ndc));
            let ray_direction = far - near;

            let mut diff = last_click.0 - Vec2::new(near.x, near.z);
            // println!("\nnear {:?}", near);
            // println!("diff {:?}", diff);

            diff *= -20.0;
            let mut rng = rand::thread_rng();

            if let Some((_ent, toi)) =
                rapier_context.cast_ray(near, ray_direction, 50.0, true, QueryFilter::default())
            {
                let hit_point = near + ray_direction * toi;
                // println!("Entity {:?} hit at point {}", _ent, hit_point);

                let rots = vec![
                    rng.gen_range(1.0..=2.0),
                    rng.gen_range(1.0..=2.0),
                    rng.gen_range(1.0..=2.0),
                    rng.gen_range(-2.0..=-1.0),
                    rng.gen_range(-2.0..=-1.0),
                    rng.gen_range(-2.0..=-1.0),
                ];

                commands.entity(ent).insert(ExternalImpulse {
                    impulse: Vec3::new(diff.x, 10.0, diff.y),
                    torque_impulse: Vec3::new(
                        *rots.choose(&mut rng).unwrap(),
                        *rots.choose(&mut rng).unwrap(),
                        *rots.choose(&mut rng).unwrap(),
                    ),
                });
                commands.entity(ent).remove::<SelectedDie>();
                // commands.entity(ent).insert(SelectedDie);
                // commands.entity(ent).despawn_recursive();
            }
        }
    } else {
        if buttons.just_pressed(MouseButton::Left) {
            let (camera, camera_transform) = camera.get_single().unwrap();
            let window = windows.get_primary().unwrap();
            let cursor_pos_screen = window.cursor_position().unwrap();
            let view = camera_transform.compute_matrix();
            let (viewport_min, viewport_max) = camera.logical_viewport_rect().unwrap();
            let screen_size = camera.logical_target_size().unwrap();
            let viewport_size = viewport_max - viewport_min;
            let adj_cursor_pos =
                cursor_pos_screen - Vec2::new(viewport_min.x, screen_size.y - viewport_max.y);
            let projection = camera.projection_matrix();
            let far_ndc = projection.project_point3(Vec3::NEG_Z).z;
            let near_ndc = projection.project_point3(Vec3::Z).z;
            let cursor_ndc = (adj_cursor_pos / viewport_size) * 2.0 - Vec2::ONE;
            let ndc_to_world: Mat4 = view * projection.inverse();
            let near = ndc_to_world.project_point3(cursor_ndc.extend(near_ndc));
            let far = ndc_to_world.project_point3(cursor_ndc.extend(far_ndc));
            let ray_direction = far - near;

            last_click.0 = Vec2::new(near.x, near.z);

            // println!("\n{:?}", near);

            if let Some((ent, toi)) = rapier_context.cast_ray(
                near,
                ray_direction,
                50.0,
                true,
                QueryFilter::exclude_fixed(),
            ) {
                let hit_point = near + ray_direction * toi;
                // println!("Entity {:?} hit at point {}", ent, hit_point);
                commands.entity(ent).insert(SelectedDie);
                // commands.entity(ent).despawn_recursive();
            }
        }
    }
}

fn move_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
    rapier_context: Res<RapierContext>,
) {
    let mut trans = query.get_single_mut().unwrap();
    let mut velocity = Vec3::ZERO;
    let local_z = trans.local_z();
    let forward = -Vec3::new(local_z.x, 0., local_z.z);
    let right = Vec3::new(local_z.z, 0., -local_z.x);

    for key in keyboard_input.get_pressed() {
        match key {
            KeyCode::W => velocity += forward,
            KeyCode::S => velocity -= forward,
            KeyCode::D => velocity += right,
            KeyCode::A => velocity -= right,
            KeyCode::R => velocity += Vec3::Y,
            KeyCode::F => velocity -= Vec3::Y,
            _ => (),
        }
    }

    velocity = velocity.normalize_or_zero();
    trans.translation += velocity * 0.1;

    let translation = trans.translation;
    let rot = trans.rotation;

    if keyboard_input.just_released(KeyCode::Q) {
        // if let Some((ent, toi)) = rapier_context.cast_ray(
        //     translation,
        //     -Vec3::new(15., 30., 0.),
        //     50.0,
        //     true,
        //     QueryFilter::default(),
        // ) {
        //     let hit_point = translation + -Vec3::new(15., 30., 0.) * toi;
        //     println!("Entity {:?} hit at point {}", ent, hit_point);
        // }

        let mut offset = Vec3::new(15., 30., 0.);
        let x = rot.to_euler(EulerRot::XYZ).2;
        println!("{:?}", x);

        trans.rotate_around(
            translation - rot.mul_vec3(Vec3::new(15., 30., 0.)),
            Quat::from_rotation_y(f32::to_radians(1.0))
        );
        // trans.rotate(Quat::from_rotation_y(f32::to_radians(45.0)));
        // let x = Quat::from_rotation_y(f32::to_radians(45.0)).mul_vec3(Vec3::new(15., 30., 0.));
        // println!("{:?}", x);
    }
    if keyboard_input.just_released(KeyCode::E) {
        trans.rotate(Quat::from_rotation_y(-f32::to_radians(45.0)));
    }
}