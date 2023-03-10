use std::time::Duration;

use bevy::{prelude::*, window::PresentMode};
use bevy_easings::*;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins::set(
            DefaultPlugins,
            WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoVsync,
                    ..default()
                }),
                ..default()
            },
        ))
        .add_plugin(ShapePlugin)
        .add_plugin(EasingsPlugin)
        .add_startup_system(setup_system)
        .run()
}

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle { ..default() },
        Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        }
        .ease_to(
            Sprite {
                custom_size: Some(Vec2::new(320., 320.)),
                ..default()
            },
            EaseFunction::BackIn,
            EasingType::Once {
                duration: Duration::from_secs(1),
            },
        ),
    ));

    let stroke = Stroke::new(Color::GRAY, 5.);
    let mut build_line = |start, end| {
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Line { 0: start, 1: end }),
                ..default()
            },
            stroke.clone(),
        ));
    };

    const BOX_SIZE: Vec2 = Vec2::new(100., 100.);

    const BOX_COUNT: Vec2 = Vec2::new(3., 3.);

    const LINE_WIDTH: f32 = 5.;

    let start_x = (BOX_SIZE.x * BOX_COUNT.x + LINE_WIDTH * BOX_COUNT.x) / 2.;
    println!("{start_x}");
    let end_x = -start_x;

    let start_y = (BOX_SIZE.y * BOX_COUNT.y + LINE_WIDTH * BOX_COUNT.y) / 2.;
    println!("{start_y}");
    let end_y = -start_y;

    for i in 0..(BOX_COUNT.x as u32 + 1) {
        let start_x = start_x - (start_x * 2. / 3.) * i as f32;
        let start_y = start_y + LINE_WIDTH / 2.;
        let end_y = end_y - LINE_WIDTH / 2.;

        build_line(Vec2::new(start_x, start_y), Vec2::new(start_x, end_y));
    }

    for i in 0..(BOX_COUNT.y as u32 + 1) {
        let start_y = start_y - (start_y * 2. / 3.) * i as f32;
        let start_x = start_x + LINE_WIDTH / 2.;
        let end_x = end_x - LINE_WIDTH / 2.;

        build_line(Vec2::new(start_x, start_y), Vec2::new(end_x, start_y));
    }
}
