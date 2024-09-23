use std::{collections::HashMap, net::UdpSocket, time::SystemTime};

use bevy::{
    app::AppExit,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::exit_on_all_closed,
};
use bevy_egui::{EguiContext, EguiPlugin};
use bevy_rapier2d::prelude::RapierConfiguration;
use bevy_renet::{
    renet::{ClientAuthentication, RenetClient, RenetError},
    run_if_client_connected, RenetClientPlugin,
};
use mini_golf::{
    client_connection_config, setup_camera, ClientChannel, NetworkedEntities, PlayerCommand, PlayerInput, ServerChannel,
    ServerMessages, PROTOCOL_ID, BG_COLOR, POWER_SEGMENTS, Power
};
use renet_visualizer::{RenetClientVisualizer, RenetVisualizerStyle};
use bevy_prototype_lyon::prelude as ly;

#[derive(Component)]
struct PowerIndicator;

#[derive(Component)]
struct ControlledPlayer;

#[derive(Default, Resource)]
struct NetworkMapping(HashMap<Entity, Entity>);

#[derive(Debug)]
struct PlayerInfo {
    client_entity: Entity,
    server_entity: Entity,
}

#[derive(Debug, Default, Resource)]
struct ClientLobby {
    players: HashMap<u64, PlayerInfo>,
}

fn new_renet_client() -> RenetClient {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let connection_config = client_connection_config();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    RenetClient::new(current_time, socket, connection_config, authentication).unwrap()
}

fn main() {
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins);
    app.insert_resource(ClearColor(BG_COLOR));

    app.add_plugin(RenetClientPlugin::default());    
    // app.add_plugin(FrameTimeDiagnosticsPlugin::default());
    app.add_plugin(LogDiagnosticsPlugin::default());
    app.add_plugin(EguiPlugin);
    app.add_plugin(ly::ShapePlugin);

    app.add_event::<PlayerCommand>();

    app.insert_resource(ClientLobby::default());
    app.insert_resource(PlayerInput::default());
    app.insert_resource(new_renet_client());
    app.insert_resource(NetworkMapping::default());

    app.add_system(player_input.with_run_criteria(run_if_client_connected));
    // app.add_system(client_send_input.with_run_criteria(run_if_client_connected));
    // app.add_system(client_send_player_commands.with_run_criteria(run_if_client_connected));
    app.add_system(client_sync_players.with_run_criteria(run_if_client_connected));
    app.add_system_to_stage(CoreStage::PostUpdate, disconnect_on_exit.after(exit_on_all_closed));

    app.insert_resource(RenetClientVisualizer::<200>::new(RenetVisualizerStyle::default()));
    app.add_system(update_visulizer_system);

    app.add_startup_system(setup_camera);

    app.add_system(panic_on_error_system);

    app.run();
}


// If any error is found we just panic
fn panic_on_error_system(mut renet_error: EventReader<RenetError>) {
    for e in renet_error.iter() {
        panic!("{}", e);
    }
}

fn update_visulizer_system(
    mut egui_context: ResMut<EguiContext>,
    mut visualizer: ResMut<RenetClientVisualizer<200>>,
    client: Res<RenetClient>,
    mut show_visualizer: Local<bool>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    visualizer.add_network_info(client.network_info());
    if keyboard_input.just_pressed(KeyCode::F1) {
        *show_visualizer = !*show_visualizer;
    }
    if *show_visualizer {
        visualizer.show_window(egui_context.ctx_mut());
    }
}

fn player_input(
    mut commands: Commands,
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut q_ball: Query<(&mut Power, &Transform), With<ControlledPlayer>>,
    q_power: Query<Entity, With<PowerIndicator>>,
    mut client: ResMut<RenetClient>
) {
    if mouse.pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();
        let pos1 = window.cursor_position().unwrap();
        let pos2 = pos1 - Vec2::new(window.width() / 2., window.height() / 2.);

        for ent in q_power.iter() {
            commands.entity(ent).despawn();
        }

        if let Ok((mut power, trans)) = q_ball.get_single_mut() {
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
    }

    if mouse.just_released(MouseButton::Left) {
        let window = windows.get_primary().unwrap();
        let pos1 = window.cursor_position().unwrap();
        let pos2 = pos1 - Vec2::new(window.width() / 2., window.height() / 2.);

        for ent in q_power.iter() {
            commands.entity(ent).despawn();
        }

        if let Ok((mut power, trans)) = q_ball.get_single_mut() {
            let dir = (trans.translation.truncate() - pos2).normalize() * power.0 * 2000.0;
            let msg = bincode::serialize(&dir).unwrap();
            println!("sending {:?}", msg);
            client.send_message(ClientChannel::Input, msg);
            power.0 = 0.0;
        }
    }

    // player_input.left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
    // player_input.right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);
    // player_input.up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
    // player_input.down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
}

// fn client_send_input(player_input: Res<PlayerInput>, mut client: ResMut<RenetClient>) {
//     let input_message = bincode::serialize(&*player_input).unwrap();
//     client.send_message(ClientChannel::Input, input_message);
// }

// fn client_send_player_commands(mut player_commands: EventReader<PlayerCommand>, mut client: ResMut<RenetClient>) {
//     for command in player_commands.iter() {
//         let command_message = bincode::serialize(command).unwrap();
//         client.send_message(ClientChannel::Command, command_message);
//     }
// }

fn client_sync_players(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<ClientLobby>,
    mut network_mapping: ResMut<NetworkMapping>,
) {
    let client_id = client.client_id();
    while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::PlayerCreate { id, translation, entity } => {
                println!("Player {} connected.", id);
                let mut client_entity = commands.spawn(ly::GeometryBuilder::build_as(
                    &ly::shapes::Circle {
                        radius: 15.0,
                        center: Vec2::ZERO,
                    },
                    ly::DrawMode::Fill(ly::FillMode::color(Color::WHITE)),
                    Transform::from_xyz(translation[0], translation[1], translation[2])
                ));

                if client_id == id {
                    client_entity.insert(ControlledPlayer);
                    client_entity.insert(Power::default());
                }

                let player_info = PlayerInfo {
                    server_entity: entity,
                    client_entity: client_entity.id(),
                };
                lobby.players.insert(id, player_info);
                network_mapping.0.insert(entity, client_entity.id());
            }
            ServerMessages::PlayerRemove { id } => {
                println!("Player {} disconnected.", id);
                if let Some(PlayerInfo {
                    server_entity,
                    client_entity,
                }) = lobby.players.remove(&id)
                {
                    commands.entity(client_entity).despawn();
                    network_mapping.0.remove(&server_entity);
                }
            }
            ServerMessages::SpawnProjectile { entity, translation } => {
                let projectile_entity = commands.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: 0.1,
                        subdivisions: 5,
                    })),
                    material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                    transform: Transform::from_translation(translation.into()),
                    ..Default::default()
                });
                network_mapping.0.insert(entity, projectile_entity.id());
            }
            ServerMessages::DespawnProjectile { entity } => {
                if let Some(entity) = network_mapping.0.remove(&entity) {
                    commands.entity(entity).despawn();
                }
            }
        }
    }

    while let Some(message) = client.receive_message(ServerChannel::NetworkedEntities) {
        let networked_entities: NetworkedEntities = bincode::deserialize(&message).unwrap();

        for i in 0..networked_entities.entities.len() {
            if let Some(entity) = network_mapping.0.get(&networked_entities.entities[i]) {
                let translation = networked_entities.translations[i].into();
                let transform = Transform {
                    translation,
                    ..Default::default()
                };
                commands.entity(*entity).insert(transform);
            }
        }
    }
}

fn disconnect_on_exit(exit: EventReader<AppExit>, mut client: ResMut<RenetClient>) {
    if !exit.is_empty() && client.is_connected() {
        client.disconnect();
    }
}
