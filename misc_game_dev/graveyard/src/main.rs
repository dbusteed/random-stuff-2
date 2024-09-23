use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
};

const TILESIZE: f32 = 32.0;
const TILE_X: f32 = 29.0;
const TILE_Y: f32 = 19.0;
const MID_X: isize = (TILE_X / 2.0) as isize;
const MID_Y: isize = (TILE_Y / 2.0) as isize;
const OFFSET_Y: f32 = -TILESIZE / 2.0;

const WIDTH: f32 = (TILESIZE * TILE_X) + (TILESIZE / 2.0);
const HEIGHT: f32 = (TILESIZE * TILE_Y) + (TILESIZE / 2.0) + TILESIZE;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Resource)]
struct Graveyard {
    tiles: Vec<Entity>,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.28, 0.18, 0.24)))
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Graveyard Tycoon".into(),
                        resolution: WindowResolution::new(WIDTH, HEIGHT)
                            .with_scale_factor_override(1.0),
                        present_mode: PresentMode::AutoVsync,
                        prevent_default_event_handling: false,
                        // fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                }),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (gizmos, player_input))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let init: Vec<&str> = vec![
        "wwwwwwwwwwwwww wwwwwwwwwwwwww",
        "w                           w",
        "w                           w",
        "w                           w",
        "w                           w",
        "w                           w",
        "w      t t t                w",
        "w      ~ ~ ~                w",
        "w                           w",
        "w                           w",
        "w                           w",
        "w                           w",
        "w                           w",
        "w                           w",
        "w                           w",
        "w                           w",
        "w                           w",
        "w                           w",
        "wwwwwwwwwwwwwwwwwwwwwwwwwwwww",
    ];

    let mut tiles: Vec<Entity> = vec![];
    for (y, row) in init.iter().rev().enumerate() {
        for (x, char) in row.chars().enumerate() {
            let ent = match char {
                'w' => commands
                    .spawn((
                        SpriteBundle {
                            texture: asset_server.load("wall.png"),
                            transform: Transform::from_xyz(
                                (((x as isize) - MID_X) as f32) * TILESIZE,
                                OFFSET_Y + (((y as isize) - MID_Y) as f32) * TILESIZE,
                                1.0,
                            ),
                            ..default()
                        },
                        Position {
                            x: x as isize,
                            y: y as isize,
                        },
                    ))
                    .id(),
                't' => commands
                    .spawn((
                        SpriteBundle {
                            texture: asset_server.load("tombstone.png"),
                            transform: Transform::from_xyz(
                                (((x as isize) - MID_X) as f32) * TILESIZE,
                                OFFSET_Y + (((y as isize) - MID_Y) as f32) * TILESIZE,
                                1.0,
                            ),
                            ..default()
                        },
                        Position {
                            x: x as isize,
                            y: y as isize,
                        },
                    ))
                    .id(),
                '~' => commands
                    .spawn((
                        SpriteBundle {
                            texture: asset_server.load("tomb_dirt.png"),
                            transform: Transform::from_xyz(
                                (((x as isize) - MID_X) as f32) * TILESIZE,
                                OFFSET_Y + (((y as isize) - MID_Y) as f32) * TILESIZE,
                                1.0,
                            ),
                            ..default()
                        },
                        Position {
                            x: x as isize,
                            y: y as isize,
                        },
                    ))
                    .id(),
                ' ' => commands
                    .spawn((
                        SpriteBundle {
                            texture: asset_server.load("open.png"),
                            transform: Transform::from_xyz(
                                (((x as isize) - MID_X) as f32) * TILESIZE,
                                OFFSET_Y + (((y as isize) - MID_Y) as f32) * TILESIZE,
                                1.0,
                            ),
                            ..default()
                        },
                        Position {
                            x: x as isize,
                            y: y as isize,
                        },
                    ))
                    .id(),
                _ => panic!(),
            };
            tiles.push(ent);
        }
    }

    commands.insert_resource(Graveyard { tiles });

    // player
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("player.png"),
            ..default()
        },
        Player,
        Position { x: MID_X, y: MID_Y },
    ));

    // res 1
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("resident.png"),
            transform: Transform::from_xyz(
                (((5 as isize) - MID_X) as f32) * TILESIZE,
                OFFSET_Y + (((5 as isize) - MID_Y) as f32) * TILESIZE,
                1.0,
            ),
            ..default()
        },
        Position { x: 5, y: 5 },
    ));

    // HUD
    commands.spawn(SpriteBundle {
        texture: asset_server.load("gold.png"),
        transform: Transform {
            translation: Vec3::new(
                (WIDTH / 2.0) - (TILESIZE * 1.0),
                (HEIGHT / 2.0) - (TILESIZE / 2.0),
                1.0,
            ),
            ..default()
        },
        ..default()
    });

    commands.spawn((TextBundle::from_sections([TextSection::new(
        "08:00",
        TextStyle {
            font: asset_server.load("fonts/Kenney Future Square.ttf"),
            font_size: 28.0,
            color: Color::WHITE,
        },
    )])
    .with_style(Style {
        position_type: PositionType::Absolute,
        top: Val::Px(3.0),
        left: Val::Px(16.0),
        ..default()
    }),));
}

fn player_input(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    mut q_player: Query<(&mut Transform, &mut Position), With<Player>>,
    mut q_images: Query<&mut Handle<Image>, Without<Player>>,
    mut graveyard: ResMut<Graveyard>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((mut tran, mut pos)) = q_player.get_single_mut() {
        if let Some(keycode) = kb.get_just_pressed().last() {
            match keycode {
                // TODO position checking
                // - oob
                // - collision
                KeyCode::W => {
                    pos.y += 1;
                }
                KeyCode::A => {
                    pos.x -= 1;
                }
                KeyCode::S => {
                    pos.y -= 1;
                }
                KeyCode::D => {
                    pos.x += 1;
                }                
                KeyCode::Back => {
                    let idx = ((pos.y * (TILE_X as isize)) + pos.x) as usize;
                    let mut handle = q_images.get_mut(graveyard.tiles[idx]).unwrap();
                    *handle = asset_server.load("open.png");
                }
                KeyCode::P => {
                    println!("({:?}, {:?}), {:?}", pos.x, pos.y, tran.translation);
                }
                _ => {}
            }
        }
        tran.translation = Vec3::lerp(tran.translation, Vec3::new(
            ((pos.x - MID_X) as f32) * TILESIZE,
            OFFSET_Y + ((pos.y - MID_Y) as f32) * TILESIZE,
            5.0,
        ), 0.35);
    }
}

fn gizmos(mut gizmos: Gizmos) {
    // HUD
    // gizmos.rect_2d(
    //     Vec2::new(0.0, (HEIGHT / 2.0) - (TILESIZE / 2.0)),
    //     0.0,
    //     Vec2::new(WIDTH, TILESIZE),
    //     Color::RED,
    // );

    // // map boundary
    // gizmos.rect_2d(
    //     Vec2::new(0.0, OFFSET_Y),
    //     0.0,
    //     Vec2::new(TILESIZE * TILE_X, TILESIZE * TILE_Y),
    //     Color::GREEN,
    // );
}
