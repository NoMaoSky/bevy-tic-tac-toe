use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::{
    components::{ChessBox, ChessTitle, ChessValue},
    mouse_movement_system, AppState, GameState, MouseState,
};

pub struct ChessPlugin;

impl Plugin for ChessPlugin {
    fn build(&self, app: &mut App) {
        // 链接系统流程，保证不会出现错误 (提醒下棋，下棋，检查胜利)
        app.add_systems(
            (
                title_change_system,
                play_chess_system,
                monitor_chess_system,
                check_winer_system,
            )
                .chain()
                .after(mouse_movement_system)
                .in_set(OnUpdate(AppState::Playing)),
        );

        // 游戏结束后结算
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
    mut chess_query: Query<(&Transform, &mut ChessBox)>,
    mouse_button: Res<Input<MouseButton>>,
    mouse: Res<MouseState>,
    game_state: Res<State<GameState>>,
    mut game_state_update: ResMut<NextState<GameState>>,
) {
    for (transform, mut chess_box) in chess_query.iter_mut() {
        // 如果类型相同
        if chess_box.1.is_null() {
            // 并且范围一致
            if collide(
                mouse.position.extend(1.),
                Vec2::new(5., 5.),
                transform.translation,
                Vec2::new(100., 100.),
            )
            .is_some()
            {
                if game_state.0 == GameState::Circle {
                    if mouse_button.just_pressed(MouseButton::Left) {
                        chess_box.1 = ChessValue::Circle;
                        game_state_update.set(GameState::Fork);
                        return;
                    } else {
                        chess_box.1 = ChessValue::CircleHover;
                    }
                } else if game_state.0 == GameState::Fork {
                    if mouse_button.just_pressed(MouseButton::Left) {
                        chess_box.1 = ChessValue::Fork;
                        game_state_update.set(GameState::Circle);
                        return;
                    } else {
                        chess_box.1 = ChessValue::ForkHover;
                    }
                }
            } else {
                chess_box.1 = ChessValue::Null;
            }
        }
    }
}

// 修改状态
fn monitor_chess_system(mut query: Query<(&mut ChessBox, &mut TextureAtlasSprite)>) {
    for (chess_box, mut sprite) in query.iter_mut() {
        let index = chess_box.1.clone() as usize;
        sprite.index = index;
    }
}

// 检查方法
fn check_winer(chess_list: Vec<&ChessBox>) -> Option<(Vec<usize>, ChessValue)> {
    let lines: Vec<Vec<usize>> = vec![
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
        let chess0 = chess_list.get(line[0]).unwrap();
        let chess1 = chess_list.get(line[1]).unwrap();
        let chess2 = chess_list.get(line[2]).unwrap();

        match chess0.1 {
            ChessValue::Circle => {
                if chess0 == chess1 && chess1 == chess2 {
                    return Some((line, ChessValue::Circle));
                }
            }
            ChessValue::Fork => {
                if chess0 == chess1 && chess1 == chess2 {
                    return Some((line, ChessValue::Fork));
                }
            }
            _ => {}
        }
    }
    None
}

// 展示胜利者
fn show_winer(
    mut chess_query: Query<(&mut Transform, &ChessBox)>,
    line: Vec<usize>,
    the_tpe: ChessValue,
) {
    let scale_value = 1.5;

    for (mut transform, chess_box) in chess_query.iter_mut() {
        if line.contains(&chess_box.0) && chess_box.1 == the_tpe {
            transform.scale = Vec3::new(scale_value, scale_value, 1.);
        }
    }
}

// 判断是否胜利
fn check_winer_system(
    chess_query: Query<(&mut Transform, &ChessBox)>,
    mut app_state_update: ResMut<NextState<AppState>>,
) {
    let null_chess = chess_query
        .iter()
        .filter(|(_, chess_box)| chess_box.1.is_null())
        .count();
    if null_chess == 0 {
        app_state_update.set(AppState::GameOver);
        return;
    }
    let chess_list = chess_query
        .iter()
        .map(|(_, chess_box)| chess_box)
        .collect::<Vec<_>>();

    if let Some((line, winner)) = check_winer(chess_list) {
        match winner {
            ChessValue::Circle => {
                show_winer(chess_query, line, ChessValue::Circle);
                app_state_update.set(AppState::GameOver);
            }
            ChessValue::Fork => {
                show_winer(chess_query, line, ChessValue::Fork);
                app_state_update.set(AppState::GameOver);
            }
            _ => {}
        }

        app_state_update.set(AppState::GameOver);
    }
}

// 游戏结算
fn end_chess_system(
    chess_query: Query<&ChessBox>,
    mut text_query: Query<&mut Text, With<ChessTitle>>,
) {
    let mut title = text_query.get_single_mut().unwrap();

    let chess_list = chess_query
        .iter()
        .map(|chess_box| chess_box)
        .collect::<Vec<_>>();

    if let Some((_, winner)) = check_winer(chess_list) {
        match winner {
            ChessValue::Circle => {
                title.sections[0].value = ">>>Circle Win!!!<<<".to_string();
                title.sections[0].style.color = Color::RED;
            }
            ChessValue::Fork => {
                title.sections[0].value = ">>>Fork Win!!!<<<".to_string();
                title.sections[0].style.color = Color::RED;
            }
            _ => {}
        }
    } else {
        title.sections[0].value = ">>>no winner<<<".to_string();
        title.sections[0].style.color = Color::RED;
    }
}
