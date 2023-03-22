use bevy::prelude::*;

// 顶部标题
#[derive(Component)]
pub struct ChessTitle;

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

// 重新开始按钮
#[derive(Component)]
pub struct RestartButton;
