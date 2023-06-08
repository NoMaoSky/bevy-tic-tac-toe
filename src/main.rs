use bevy::{prelude::*, window::WindowResolution};
use chess::ChessPlugin;
use components::{
    AppState, ButtonAction, ChessBox, ChessValue, GameState, GameTitle, MouseState, GAP, GRID_SIZE,
    LINE_WIDTH, SCALE,
};

mod components;

mod chess;

fn main() {
    let mut app = App::new();

    app.add_state::<AppState>().add_state::<GameState>();
    app.init_resource::<MouseState>();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Tic Tac Toe".to_string(),
                    resolution: WindowResolution::new(400., 500.),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    );
    app.add_plugin(ChessPlugin);

    app.add_startup_system(setup_system)
        .add_system(button_click_system)
        .add_system(mouse_movement_system.run_if(in_state(AppState::Playing)))
        .add_system(board_cleanup_system.run_if(in_state(AppState::Restart)));

    app.run();
}

// 系统初始化
fn setup_system(
    mut commands: Commands,
    asss: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // 添加2D摄像头
    commands.spawn(Camera2dBundle::default());

    // 渲染标题
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Px(75.)),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::GRAY),
            ..default()
        })
        .with_children(|children| {
            children.spawn((
                TextBundle::from_section(
                    "Tic Tac Toe",
                    TextStyle {
                        font: asss.load("fonts/uni.ttf"),
                        font_size: 30.,
                        color: Color::WHITE,
                    },
                ),
                GameTitle,
            ));
        });

    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::bottom(Val::Px(0.)),
                size: Size::new(Val::Percent(100.), Val::Px(75.)),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                gap: Size::all(Val::Px(15.)),
                ..default()
            },
            background_color: BackgroundColor(Color::GRAY),
            ..default()
        })
        .with_children(|children| {
            children
                .spawn((
                    ButtonBundle {
                        button: Button,
                        style: Style {
                            padding: UiRect::bottom(Val::Px(7.)),
                            size: Size::new(Val::Px(175.), Val::Px(50.)),
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::WHITE),
                        ..default()
                    },
                    ButtonAction::StartGame,
                ))
                .with_children(|children| {
                    children.spawn(TextBundle::from_section(
                        "Start",
                        TextStyle {
                            font: asss.load("fonts/uni.ttf"),
                            font_size: 30.,
                            color: Color::BLACK,
                        },
                    ));
                });
        });

    // 渲染线的方法
    let mut build_line = |size, coordinates| {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(size),
                ..default()
            },
            transform: Transform {
                translation: coordinates,
                scale: Vec3::new(SCALE, SCALE, 1.),
                ..default()
            },
            ..default()
        });
    };

    let line_size = GRID_SIZE * 3. + GAP * 2.;
    let position_offset = (GRID_SIZE + GAP) / 2.;
    let line_width = LINE_WIDTH * SCALE;

    // 渲染横线
    build_line(
        Vec2::new(line_size, line_width),
        Vec3::new(0., position_offset, 1.),
    );
    build_line(
        Vec2::new(line_size, line_width),
        Vec3::new(0., -position_offset, 1.),
    );

    // 渲染竖线
    build_line(
        Vec2::new(line_width, line_size),
        Vec3::new(position_offset, 0., 1.),
    );
    build_line(
        Vec2::new(line_width, line_size),
        Vec3::new(-position_offset, 0., 1.),
    );

    // 图片合集
    let text_handle = asss.load("imgs/chess.png");
    let texture_atlas = TextureAtlas::from_grid(text_handle, Vec2::new(96., 96.), 3, 2, None, None);
    let chess = texture_atlases.add(texture_atlas);

    // 渲染棋子的方法
    let mut build_chess = |index, coordinates| {
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: chess.clone(),
                transform: Transform {
                    translation: coordinates,
                    scale: Vec3::new(SCALE, SCALE, 1.),
                    ..default()
                },
                ..default()
            },
            ChessBox(index, ChessValue::Null),
        ));
    };

    // 定义一个九宫格的中心点
    let grid_center = Vec3::new(0.0, 0.0, 0.0);
    // 用两个循环来创建9个格子
    for i in 0..3 {
        for j in 0..3 {
            // 计算索引，从1到9
            let index = i * 3 + j + 1;
            // 计算x坐标，从左到右
            let x = grid_center.x - GRID_SIZE + (j as f32) * GRID_SIZE + (j as f32 - 1.) * GAP;
            // 计算y坐标，从上到下
            let y = grid_center.y + GRID_SIZE - (i as f32) * GRID_SIZE - (i as f32 - 1.) * GAP;
            // 调用build_cell函数，传入索引和坐标
            build_chess(index, Vec3::new(x, y, 0.));
        }
    }
}

// 鼠标移动系统
fn mouse_movement_system(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse: ResMut<MouseState>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, global_transform) = q_camera.get_single().unwrap();

    for event in cursor_moved_events.iter() {
        if let Some(pos) = camera.viewport_to_world_2d(global_transform, event.position) {
            mouse.position = pos;
            mouse.movable = true;
        }
    }
}

// 按钮点击事件
fn button_click_system(
    interaction_query: Query<
        (&Interaction, &ButtonAction, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    app_state: Res<State<AppState>>,
    mut app_state_next: ResMut<NextState<AppState>>,
) {
    for (interaction, button_action, children) in interaction_query.iter() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        if let Interaction::Clicked = *interaction {
            if ButtonAction::StartGame == *button_action {
                text.sections[0].value = "Restart".to_string();

                if AppState::WaitStart == app_state.0 {
                    app_state_next.set(AppState::Playing);
                } else {
                    app_state_next.set(AppState::Restart)
                }
            }
        }
    }
}

// 棋盘清理
fn board_cleanup_system(
    mut chess_query: Query<&mut ChessBox>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for mut chess in chess_query.iter_mut() {
        chess.1 = ChessValue::Null;
    }
    game_state.set(GameState::Circle);
    app_state.set(AppState::Playing);
}
