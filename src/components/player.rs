use bevy::prelude::*;

use super::physics::CollisionTag;

#[derive(Copy, Clone, Component)]
pub struct Player1;

#[derive(Copy, Clone, Component)]
pub struct Player2;

pub trait Player: Component {
    fn get_collision_tag() -> CollisionTag;
}

impl Player for Player1 {
    fn get_collision_tag() -> CollisionTag {
        CollisionTag::PLAYER1
    }
}

impl Player for Player2 {
    fn get_collision_tag() -> CollisionTag {
        CollisionTag::PLAYER2
    }
}
