use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::{
    chess_value, mouse_movement_system, AppState, ChessBox, ChessIndex, ChessTitle, GameState,
    MouseState,
};

pub struct ChessPlugin;

impl Plugin for ChessPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(title_change_system.in_set(OnUpdate(AppState::Playing)));

        app.add_system(
            play_chess_system
                .run_if(in_state(AppState::Playing))
                .after(mouse_movement_system),
        );

        app.add_system(
            check_winer_system
                .run_if(in_state(AppState::Playing))
                .before(play_chess_system),
        );

        app.add_system(end_chess_system.in_schedule(OnEnter(AppState::GameOver)));
    }
}

// 修改标题提示用户
fn title_change_system(
    mut query: Query<&mut Text, With<ChessTitle>>,
    game_state: Res<State<GameState>>,
) {
    let mut title = query.get_single_mut().unwrap();
    match game_state.0 {
        GameState::Circle => {
            title.sections[0].value = "===wait circle chess===".to_string();
            title.sections[0].style.color = Color::WHITE;
        }
        GameState::Fork => {
            title.sections[0].value = "===wait fork chess===".to_string();
            title.sections[0].style.color = Color::WHITE;
        }
    }
}

// 下棋
fn play_chess_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &ChessIndex), With<chess_value::Null>>,
    mouse_button: Res<Input<MouseButton>>,
    mouse: Res<MouseState>,
    mut app_state_update: ResMut<NextState<AppState>>,
    game_state: Res<State<GameState>>,
    mut game_state_update: ResMut<NextState<GameState>>,
    asss: Res<AssetServer>,
) {
    if query.is_empty() {
        app_state_update.set(AppState::GameOver);
        return;
    }
    for (entity, transform, chess_index) in query.iter_mut() {
        // 如果类型相同
        if mouse.movable {
            let chess_index = ChessIndex(chess_index.0);
            let translation = transform.translation;
            // 并且范围一致
            if collide(
                mouse.position.extend(1.),
                Vec2::new(5., 5.),
                transform.translation,
                Vec2::new(100., 100.),
            )
            .is_some()
            {
                // 销毁旧实体
                commands.entity(entity).despawn();

                if let GameState::Circle = game_state.0 {
                    if mouse_button.just_pressed(MouseButton::Left) {
                        // 产生新实体
                        commands.spawn((
                            SpriteBundle {
                                texture: asss.load("imgs/circle_1.png"),
                                transform: Transform {
                                    translation,
                                    scale: Vec3::new(0.5, 0.5, 1.),
                                    ..default()
                                },
                                ..default()
                            },
                            chess_index,
                            ChessBox,
                            chess_value::Circle,
                        ));
                        game_state_update.set(GameState::Fork);
                        break;
                    } else {
                        // 产生新实体
                        commands.spawn((
                            SpriteBundle {
                                texture: asss.load("imgs/circle_0.png"),
                                transform: Transform {
                                    translation,
                                    scale: Vec3::new(0.5, 0.5, 1.),
                                    ..default()
                                },
                                ..default()
                            },
                            chess_index,
                            ChessBox,
                            chess_value::Null,
                        ));
                    }
                } else if let GameState::Fork = game_state.0 {
                    if mouse_button.just_pressed(MouseButton::Left) {
                        // 产生新实体
                        commands.spawn((
                            SpriteBundle {
                                texture: asss.load("imgs/fork_1.png"),
                                transform: Transform {
                                    translation,
                                    scale: Vec3::new(0.5, 0.5, 1.),
                                    ..default()
                                },
                                ..default()
                            },
                            chess_index,
                            ChessBox,
                            chess_value::Fork,
                        ));
                        game_state_update.set(GameState::Circle);
                        break;
                    } else {
                        // 产生新实体
                        commands.spawn((
                            SpriteBundle {
                                texture: asss.load("imgs/fork_0.png"),
                                transform: Transform {
                                    translation,
                                    scale: Vec3::new(0.5, 0.5, 1.),
                                    ..default()
                                },
                                ..default()
                            },
                            chess_index,
                            ChessBox,
                            chess_value::Null,
                        ));
                    }
                }
            } else {
                // 销毁旧实体
                commands.entity(entity).despawn();

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
                    chess_index,
                    ChessBox,
                    chess_value::Null,
                ));
            }
        }
    }

    if query.is_empty() {}
}

enum Winner {
    Circle,
    Fork,
}

// 检查方法
fn check_winer(circle_list: Vec<u32>, fork_list: Vec<u32>) -> Option<(Vec<u32>, Winner)> {
    let lines: Vec<Vec<u32>> = vec![
        vec![0, 1, 2],
        vec![3, 4, 5],
        vec![6, 7, 8],
        vec![0, 3, 6],
        vec![1, 4, 7],
        vec![2, 5, 8],
        vec![0, 4, 8],
        vec![2, 4, 6],
    ];

    for line in lines {
        if circle_list.contains(&line[0])
            && circle_list.contains(&line[1])
            && circle_list.contains(&line[2])
        {
            return Some((line, Winner::Circle));
        }
        if fork_list.contains(&line[0])
            && fork_list.contains(&line[1])
            && fork_list.contains(&line[2])
        {
            return Some((line, Winner::Fork));
        }
    }
    None
}

// 判断是否胜利
fn check_winer_system(
    mut circle_query: Query<
        (&mut Transform, &ChessIndex),
        (With<chess_value::Circle>, Without<chess_value::Fork>),
    >,
    mut fork_query: Query<
        (&mut Transform, &ChessIndex),
        (With<chess_value::Fork>, Without<chess_value::Circle>),
    >,
    mut app_state_update: ResMut<NextState<AppState>>,
) {
    let circle_list: Vec<u32> = circle_query.iter().map(|(_, index)| index.0).collect();
    let fork_list: Vec<u32> = fork_query.iter().map(|(_, index)| index.0).collect();

    let scale_value = 1.1;

    if let Some((line, winner)) = check_winer(circle_list, fork_list) {
        match winner {
            Winner::Circle => {
                for (mut tf, index) in circle_query.iter_mut() {
                    if line.contains(&index.0) {
                        tf.scale = Vec3::new(scale_value, scale_value, 1.);
                    }
                }
            }
            Winner::Fork => {
                for (mut tf, index) in fork_query.iter_mut() {
                    if line.contains(&index.0) {
                        tf.scale = Vec3::new(scale_value, scale_value, 1.);
                    }
                }
            }
        }

        app_state_update.set(AppState::GameOver);
    }
}

// 游戏结算
fn end_chess_system(
    mut query: Query<&mut Text, With<ChessTitle>>,
    circle_query: Query<&ChessIndex, With<chess_value::Circle>>,
    fork_query: Query<&ChessIndex, With<chess_value::Fork>>,
) {
    let circle_list: Vec<u32> = circle_query.iter().map(|index| index.0).collect();

    let fork_list: Vec<u32> = fork_query.iter().map(|index| index.0).collect();

    let mut title = query.get_single_mut().unwrap();

    if let Some((_, winner)) = check_winer(circle_list, fork_list) {
        match winner {
            Winner::Circle => {
                title.sections[0].value = ">>>Circle Win!!!<<<".to_string();
                title.sections[0].style.color = Color::RED;
            }
            Winner::Fork => {
                title.sections[0].value = ">>>Fork Win!!!<<<".to_string();
                title.sections[0].style.color = Color::RED;
            }
        }
    } else {
        title.sections[0].value = ">>>no winner<<<".to_string();
        title.sections[0].style.color = Color::RED;
    }
}
