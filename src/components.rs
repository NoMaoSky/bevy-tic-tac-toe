use bevy::prelude::*;

// 顶部标题
#[derive(Component)]
pub struct ChessTitle;

// 盒子索引
#[derive(Component)]
pub struct ChessIndex(pub u32);

// 盒子
#[derive(Component)]
pub struct ChessBox;

// 盒子里的棋是什么状态
pub mod chess_value {
    use bevy::prelude::Component;

    // 没有棋
    #[derive(Component)]
    pub struct Null;

    // 圈圈
    #[derive(Component)]
    pub struct Circle;

    // 叉叉
    #[derive(Component)]
    pub struct Fork;
}

// 重新开始按钮
#[derive(Component)]
pub struct RestartButton;
