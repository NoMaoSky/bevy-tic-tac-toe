use bevy::prelude::*;

pub const GRID_SIZE: f32 = 96.;
pub const GAP: f32 = 10.;
pub const LINE_WIDTH: f32 = 2.;
pub const SCALE: f32 = 1.;

// App状态
#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum AppState {
    #[default]
    WaitStart,
    Restart,
    Playing,
    GameOver,
}

// 游戏状态
#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameState {
    #[default]
    Circle,
    Fork,
}

// 鼠标状态
#[derive(Resource, Default, Debug)]
pub struct MouseState {
    pub position: Vec2,
    pub movable: bool,
}

// 顶部标题
#[derive(Component)]
pub struct GameTitle;

// 运行时间！
#[derive(Component)]
pub struct NextTimer(pub Timer);

impl Default for NextTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
}

// 盒子
#[derive(Component)]
pub struct ChessBox(pub usize, pub ChessValue);

impl PartialEq for ChessBox {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

// 盒子里的棋是什么状态
#[derive(PartialEq, Clone)]
pub enum ChessValue {
    Null = 0,
    Circle,
    CircleHover,
    Fork,
    ForkHover,
}

impl ChessValue {
    pub fn is_null(&self) -> bool {
        if self == &Self::Null || self == &Self::CircleHover || self == &Self::ForkHover {
            return true;
        }
        false
    }
}

#[derive(Component, PartialEq)]
pub enum ButtonAction {
    StartGame,
}
