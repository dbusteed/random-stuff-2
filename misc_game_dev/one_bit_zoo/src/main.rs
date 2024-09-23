use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
};
use bevy_prototype_lyon::prelude::*;

const TILE_SIZE: f32 = 32.0;
const TILE_X: f32 = 29.0;
const TILE_Y: f32 = 19.0;
const MID_X: isize = (TILE_X / 2.0) as isize;
const MID_Y: isize = (TILE_Y / 2.0) as isize;
const SPRITE_SIZE: f32 = 16.0;
const SPRITE_SCALE: Vec3 = Vec3::new(TILE_SIZE / SPRITE_SIZE, TILE_SIZE / SPRITE_SIZE, 1.0);
const OFFSET_Y: f32 = -TILE_SIZE / 2.0;

const WIDTH: f32 = (TILE_SIZE * TILE_X) + (TILE_SIZE / 2.0);
const HEIGHT: f32 = (TILE_SIZE * TILE_Y) + (TILE_SIZE / 2.0) + TILE_SIZE;

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
                        title: "One Bit Zoo".into(),
                        resolution: WindowResolution::new(WIDTH, HEIGHT)
                            .with_scale_factor_override(1.0),
                        present_mode: PresentMode::AutoVsync,
                        prevent_default_event_handling: false,
                        // fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                }),
            ShapePlugin,
            // bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (gizmos))
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
                            transform: Transform {
                                translation: Vec3::new(
                                    (((x as isize) - MID_X) as f32) * TILE_SIZE,
                                    OFFSET_Y + (((y as isize) - MID_Y) as f32) * TILE_SIZE,
                                    1.0,
                                ),
                                scale: SPRITE_SCALE,
                                ..default()
                            },
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
                            transform: Transform {
                                translation: Vec3::new(
                                    (((x as isize) - MID_X) as f32) * TILE_SIZE,
                                    OFFSET_Y + (((y as isize) - MID_Y) as f32) * TILE_SIZE,
                                    1.0,
                                ),
                                scale: SPRITE_SCALE,
                                ..default()
                            },
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

    // HUD
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Rectangle {
                extents: Vec2::new(WIDTH, TILE_SIZE),
                origin: RectangleOrigin::Center,
            }),
            spatial: SpatialBundle {
                transform: Transform::from_xyz(0.0, (HEIGHT / 2.0) - (TILE_SIZE / 2.0), 1.0),
                ..default()
            },
            ..default()
        },
        Fill::color(Color::hex("2f1d27").unwrap()),
    ));

    commands.spawn(SpriteBundle {
        texture: asset_server.load("gold.png"),
        transform: Transform {
            translation: Vec3::new(
                (WIDTH / 2.0) - (TILE_SIZE * 1.0),
                (HEIGHT / 2.0) - (TILE_SIZE / 2.0),
                2.0,
            ),
            scale: SPRITE_SCALE,
            ..default()
        },
        ..default()
    });

    // github button
    commands.spawn((
        ButtonBundle {
            style: Style {
                top: Val::Px(0.0),
                left: Val::Px(2.0),
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                ..default()
            },
            background_color: BackgroundColor::from(Color::WHITE),
            image: asset_server.load("build.png").into(),
            z_index: bevy::ui::ZIndex::Local(10),
            ..default()
        },
    ));

    // commands.spawn((TextBundle::from_sections([TextSection::new(
    //     "Day 1",
    //     TextStyle {
    //         font: asset_server.load("fonts/Kenney Future Square.ttf"),
    //         font_size: 28.0,
    //         color: Color::WHITE,
    //     },
    // )])
    // .with_style(Style {
    //     position_type: PositionType::Absolute,
    //     top: Val::Px(3.0),
    //     left: Val::Px(16.0),
    //     ..default()
    // }),));
}

fn gizmos(mut gizmos: Gizmos) {
    // HUD
    // gizmos.rect_2d(
    //     Vec2::new(0.0, (HEIGHT / 2.0) - (TILE_SIZE / 2.0)),
    //     0.0,
    //     Vec2::new(WIDTH, TILE_SIZE),
    //     Color::RED,
    // );

    // // map boundary
    // gizmos.rect_2d(
    //     Vec2::new(0.0, OFFSET_Y),
    //     0.0,
    //     Vec2::new(TILE_SIZE * TILE_X, TILE_SIZE * TILE_Y),
    //     Color::GREEN,
    // );
}
