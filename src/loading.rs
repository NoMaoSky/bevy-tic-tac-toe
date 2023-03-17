use std::time::Duration;

use bevy::prelude::*;
use bevy_easings::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    components::{chess_value, ChessBox, ChessIndex, ChessTitle, RestartButton},
    AppState, GameState,
};

// 格子大小
const BOX_SIZE: Vec2 = Vec2::new(100., 100.);
// 格子数量
const BOX_COUNT: Vec2 = Vec2::new(3., 3.);
// 线款
const LINE_WIDTH: f32 = 5.;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                build_title_system,
                build_button_system,
                build_chess_system,
                build_next_system,
            )
                .chain()
                .in_schedule(OnEnter(AppState::CreateUI)),
        );

        app.add_systems(
            (build_chess_box_system, build_end_system)
                .chain()
                .in_schedule(OnEnter(AppState::BuildBox)),
        );

        app.add_system(
            button_restart_system
                .after(build_button_system)
                .run_if(in_state(AppState::Playing).or_else(in_state(AppState::GameOver))),
        );
    }
}

// 创建标题
fn build_title_system(mut commands: Commands, asss: Res<AssetServer>) {
    // 创建顶部提示
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(100.)),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|children| {
            children.spawn((
                TextBundle::from_section(
                    "Game Start",
                    TextStyle {
                        font: asss.load("fonts/uni.ttf"),
                        font_size: 50.,
                        color: Color::BLACK,
                    },
                ),
                ChessTitle,
            ));
        });
}

// 创建按钮
fn build_button_system(mut commands: Commands, asss: Res<AssetServer>) {
    // 创建底部按钮
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::bottom(Val::Px(0.)),
                size: Size::new(Val::Percent(100.0), Val::Px(100.)),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|children| {
            children
                .spawn((
                    ButtonBundle {
                        button: Button,
                        style: Style {
                            padding: UiRect::bottom(Val::Px(7.)),
                            size: Size::new(Val::Px(200.), Val::Px(60.)),
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::WHITE),
                        ..default()
                    },
                    RestartButton,
                ))
                .with_children(|children| {
                    children.spawn(TextBundle::from_section(
                        "Restart",
                        TextStyle {
                            font: asss.load("fonts/uni.ttf"),
                            font_size: 50.,
                            color: Color::BLACK,
                        },
                    ));
                });
        });
}

// 重置按钮交互
fn button_restart_system(
    mut commands: Commands,
    chess_box_query: Query<Entity, With<ChessBox>>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<RestartButton>),
    >,
    mut text_query: Query<&mut Text>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interact, mut back_ground_color, children) in button_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match interact {
            Interaction::Clicked => {
                back_ground_color.0 = Color::RED;
                text.sections[0].style.color = Color::WHITE;
                for entity in chess_box_query.iter() {
                    commands.entity(entity).despawn();
                }
                app_state.set(AppState::BuildBox);
                game_state.set(GameState::Circle);
                return;
            }
            Interaction::Hovered => {
                back_ground_color.0 = Color::GRAY;
                text.sections[0].style.color = Color::WHITE;
            }
            Interaction::None => {
                back_ground_color.0 = Color::WHITE;
                text.sections[0].style.color = Color::BLACK;
            }
        }
    }
}

// 初始化棋盘
fn build_chess_system(mut commands: Commands) {
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
                duration: Duration::from_secs_f32(0.5),
            },
        ),
    ));

    // 线颜色
    let stroke = Stroke::new(Color::GRAY, 5.);
    let mut build_line = |start, end| {
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Line(start, end)),
                ..default()
            },
            stroke,
        ));
    };

    // 开始x坐标
    let start_x = (BOX_SIZE.x * BOX_COUNT.x + LINE_WIDTH * BOX_COUNT.x) / 2.;
    // 结束x坐标
    let end_x = -start_x;
    // 开始y坐标
    let start_y = (BOX_SIZE.y * BOX_COUNT.y + LINE_WIDTH * BOX_COUNT.y) / 2.;
    // 结束y坐标
    let end_y = -start_y;

    // 渲染竖线
    for i in 0..(BOX_COUNT.x as u32 + 1) {
        // 竖线开始的位置
        let start_x = start_x - (start_x * 2. / 3.) * i as f32;
        let start_y = start_y + LINE_WIDTH / 2.;
        let end_y = end_y - LINE_WIDTH / 2.;

        build_line(Vec2::new(start_x, start_y), Vec2::new(start_x, end_y));
    }

    // 渲染横线
    for i in 0..(BOX_COUNT.y as u32 + 1) {
        // 横线开始的位置
        let start_y = start_y - (start_y * 2. / 3.) * i as f32;
        let start_x = start_x + LINE_WIDTH / 2.;
        let end_x = end_x - LINE_WIDTH / 2.;

        build_line(Vec2::new(start_x, start_y), Vec2::new(end_x, start_y));
    }
}

// 下一个阶段
fn build_next_system(mut state: ResMut<NextState<AppState>>) {
    state.set(AppState::BuildBox);
}

// 构建棋盘方块
fn build_chess_box_system(mut commands: Commands) {
    // 颜色
    let mut build_box = |translation, index| {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(0., 0.)),
                    ..default()
                },
                transform: Transform {
                    translation,
                    ..default()
                },
                ..default()
            },
            ChessIndex(index),
            ChessBox,
            chess_value::Null,
        ));
    };

    // 开始x坐标
    let end_x = -(BOX_SIZE.x * BOX_COUNT.x + LINE_WIDTH * BOX_COUNT.x) / 2.;

    // 开始y坐标
    let start_y = (BOX_SIZE.y * BOX_COUNT.y + LINE_WIDTH * BOX_COUNT.y) / 2.;

    let diff = 50.;

    let mut index = 0;

    for j in 0..(BOX_COUNT.y as u32) {
        let start_y = start_y + j as f32 * (-5. + -100.);
        for i in 1..=(BOX_COUNT.x as u32) {
            let end_x = end_x + i as f32 * (5. + 100.);
            let end_x = end_x - 2.5;
            let start_y = start_y - 2.5;
            build_box(Vec3::new(end_x - diff, start_y - diff, 1.), index);
            index += 1;
        }
    }
}

// 修改状态
fn build_end_system(mut state: ResMut<NextState<AppState>>) {
    state.set(AppState::Playing);
}
