use std::{collections::HashMap, net::UdpSocket, time::SystemTime};

use bevy::{
    app::AppExit,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::exit_on_all_closed,
};
use bevy_egui::{EguiContext, EguiPlugin};
use bevy_rapier2d::prelude::*;
use bevy_renet::{
    renet::{RenetServer, ServerAuthentication, ServerConfig, ServerEvent},
    RenetServerPlugin,
};
use mini_golf::{
    server_connection_config, ClientChannel, NetworkedEntities, Player, PlayerCommand, PlayerInput,
    Projectile, ServerChannel, ServerMessages, PROTOCOL_ID, setup_camera, BG_COLOR
};
use renet_visualizer::RenetServerVisualizer;
use bevy_prototype_lyon::prelude as ly;

#[derive(Debug, Default, Resource)]
pub struct ServerLobby {
    pub players: HashMap<u64, Entity>,
}

const PLAYER_MOVE_SPEED: f32 = 50.0;

fn new_renet_server() -> RenetServer {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let connection_config = server_connection_config();
    let server_config = ServerConfig::new(64, PROTOCOL_ID, server_addr, ServerAuthentication::Unsecure);
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.insert_resource(ClearColor(BG_COLOR));

    app.add_plugin(RenetServerPlugin::default());
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
    app.add_plugin(RapierDebugRenderPlugin::default());
    // app.add_plugin(FrameTimeDiagnosticsPlugin::default());
    app.add_plugin(LogDiagnosticsPlugin::default());
    app.add_plugin(EguiPlugin);

    app.insert_resource(ServerLobby::default());
    app.insert_resource(new_renet_server());
    app.insert_resource(RenetServerVisualizer::<200>::default());

    app.add_system(server_update_system);
    app.add_system(server_network_sync);
    // app.add_system(move_players_system);    
    app.add_system(update_visulizer_system);
    app.add_system_to_stage(CoreStage::PostUpdate, disconnect_clients_on_exit.after(exit_on_all_closed));

    app.add_startup_system(setup_camera);
    app.add_startup_system(server_setup);


    app.run();
}

fn server_setup(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}


// #[allow(clippy::too_many_arguments)]
fn server_update_system(
    mut server_events: EventReader<ServerEvent>,
    mut commands: Commands,
    mut lobby: ResMut<ServerLobby>,
    mut server: ResMut<RenetServer>,
    mut visualizer: ResMut<RenetServerVisualizer<200>>,
    players: Query<(Entity, &Player, &Transform)>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, _) => {
                println!("Player {} connected.", id);
                visualizer.add_client(*id);

                // Initialize other players for this new client
                for (entity, player, transform) in players.iter() {
                    let translation: [f32; 3] = transform.translation.into();
                    let message = bincode::serialize(&ServerMessages::PlayerCreate {
                        id: player.id,
                        entity,
                        translation,
                    })
                    .unwrap();
                    server.send_message(*id, ServerChannel::ServerMessages, message);
                }

                // Spawn new player
                let transform = Transform::from_xyz(0.0, 0.0, 10.0);
                let player_entity = commands
                    .spawn(ly::GeometryBuilder::build_as(
                        &ly::shapes::Circle {
                            radius: 15.0,
                            center: Vec2::ZERO,
                        },
                        ly::DrawMode::Fill(ly::FillMode::color(Color::WHITE)),
                        transform,
                    ))
                    .insert(RigidBody::Dynamic)
                    .insert(LockedAxes::ROTATION_LOCKED)
                    .insert(Damping {
                        linear_damping: 1.5,
                        ..default()
                    })
                    .insert(Collider::ball(15.0))
                    .insert(PlayerInput::default())
                    .insert(Velocity::default())
                    .insert(Player { id: *id })
                    .id();

                lobby.players.insert(*id, player_entity);

                let translation: [f32; 3] = transform.translation.into();
                let message = bincode::serialize(&ServerMessages::PlayerCreate {
                    id: *id,
                    entity: player_entity,
                    translation,
                })
                .unwrap();
                server.broadcast_message(ServerChannel::ServerMessages, message);
            }
            ServerEvent::ClientDisconnected(id) => {
                println!("Player {} disconnected.", id);
                visualizer.remove_client(*id);
                if let Some(player_entity) = lobby.players.remove(id) {
                    commands.entity(player_entity).despawn();
                }

                let message = bincode::serialize(&ServerMessages::PlayerRemove { id: *id }).unwrap();
                server.broadcast_message(ServerChannel::ServerMessages, message);
            }
        }
    }

    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, ClientChannel::Input) {
            let impulse: Vec2 = bincode::deserialize(&message).unwrap();
            if let Some(player_entity) = lobby.players.get(&client_id) {
                // println!("message! {:?}", input);
                commands.entity(*player_entity).insert(ExternalImpulse { impulse, ..default() });
                // commands.entity(*player_entity).insert(Velocity { linvel: impulse, ..default() });
            }
        }
    }
}

fn update_visulizer_system(
    mut egui_context: ResMut<EguiContext>,
    mut visualizer: ResMut<RenetServerVisualizer<200>>,
    server: Res<RenetServer>,
) {
    visualizer.update(&server);
    visualizer.show_window(egui_context.ctx_mut());
}

#[allow(clippy::type_complexity)]
fn server_network_sync(mut server: ResMut<RenetServer>, query: Query<(Entity, &Transform), Or<(With<Player>, With<Projectile>)>>) {
    let mut networked_entities = NetworkedEntities::default();
    for (entity, transform) in query.iter() {
        networked_entities.entities.push(entity);
        networked_entities.translations.push(transform.translation.into());
    }

    let sync_message = bincode::serialize(&networked_entities).unwrap();
    server.broadcast_message(ServerChannel::NetworkedEntities, sync_message);
}

fn move_players_system(mut query: Query<(&mut Velocity, &PlayerInput)>) {
    for (mut velocity, input) in query.iter_mut() {
        let x = (input.right as i8 - input.left as i8) as f32;
        let y = (input.down as i8 - input.up as i8) as f32;
        let direction = Vec2::new(x, y).normalize_or_zero();
        velocity.linvel.x = direction.x * PLAYER_MOVE_SPEED;
        velocity.linvel.y = -direction.y * PLAYER_MOVE_SPEED;        
    }
}

fn disconnect_clients_on_exit(exit: EventReader<AppExit>, mut server: ResMut<RenetServer>) {
    if !exit.is_empty() {
        server.disconnect_clients();
    }
}
