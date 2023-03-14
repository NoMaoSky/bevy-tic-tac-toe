use bevy::prelude::*;
use bevy_easings::*;
use bevy_prototype_lyon::prelude::*;
use chess::ChessPlugin;
use loading::LoadingPlugin;

mod loading;

mod chess;

mod fps;

fn main() {
    let mut app = App::new();

    app.add_state::<AppState>().add_state::<GameState>();
    app.init_resource::<MouseState>();

    app.add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(EasingsPlugin)
        // .add_plugin(FpsPlugin)
        .add_plugin(LoadingPlugin)
        .add_plugin(ChessPlugin);

    app.add_startup_system(setup_system)
        .add_system(mouse_movement_system);

    app.run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn mouse_movement_system(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse: ResMut<MouseState>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = q_camera.single();

    for event in cursor_moved_events.iter() {
        if let Some(pos) = camera.viewport_to_world_2d(camera_transform, event.position) {
            mouse.position = pos;
            mouse.movable = true;
        }
    }
}

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

#[derive(Resource, Default, Debug)]
struct MouseState {
    position: Vec2,
    movable: bool,
}

// 顶部标题
#[derive(Component)]
struct ChessTitle;

// 盒子
#[derive(Component)]
struct ChessIndex(u32);

#[derive(Component)]
struct ChessBox;

mod chess_value {
    use bevy::prelude::Component;

    #[derive(Component)]
    pub struct Null;

    #[derive(Component)]
    pub struct Circle;

    #[derive(Component)]
    pub struct Fork;
}

#[derive(Component)]
struct RestartButton;
