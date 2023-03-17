use bevy::prelude::*;
use bevy_easings::*;
use bevy_prototype_lyon::prelude::*;
use chess::ChessPlugin;
use loading::LoadingPlugin;

mod loading;

mod chess;

mod components;

mod fps;

fn main() {
    let mut app = App::new();

    // 添加状态（游戏，下棋）
    app.add_state::<AppState>().add_state::<GameState>();
    //添加资源（鼠标）
    app.init_resource::<MouseState>();

    app.add_plugins(DefaultPlugins) // 默认插件
        .add_plugin(ShapePlugin) // 图形插件
        .add_plugin(EasingsPlugin) // 动态插件
        // .add_plugin(FpsPlugin) // fps显示
        .add_plugin(LoadingPlugin) // 加载插件
        .add_plugin(ChessPlugin); // 下棋插件

    // 系统初始化 / 鼠标移动
    app.add_startup_system(setup_system)
        .add_system(mouse_movement_system);

    app.run();
}

// 初始化添加2D摄像头
fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// 检测鼠标移动，获取position
fn mouse_movement_system(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse: ResMut<MouseState>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = q_camera.single();

    // 遍历鼠标移动事件
    for event in cursor_moved_events.iter() {
        // 获取摄像头在2D坐标轴上的位置
        if let Some(pos) = camera.viewport_to_world_2d(camera_transform, event.position) {
            mouse.position = pos;
            mouse.movable = true; // 已经移动了！
        }
    }
}

// App状态
#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
enum AppState {
    #[default]
    CreateUI,
    BuildBox,
    Playing,
    GameOver,
}

// 游戏状态
#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
enum GameState {
    #[default]
    Circle,
    Fork,
}

// 鼠标状态
#[derive(Resource, Default, Debug)]
struct MouseState {
    position: Vec2,
    movable: bool,
}
