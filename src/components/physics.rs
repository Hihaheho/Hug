use bevy::prelude::*;
use std::marker::PhantomData;

bitflags::bitflags! {
    pub struct CollisionTag: u32 {
        const PLAYER1 = 1 << 0;
        const PLAYER2 = 1 << 1;
        const ALL = u32::MAX;
    }
}

impl Into<u32> for CollisionTag {
    fn into(self) -> u32 {
        self.bits
    }
}

#[derive(Component)]
pub struct Joint<A, B>(PhantomData<(A, B)>);

impl<A, B> Default for Joint<A, B> {
    fn default() -> Self {
        Self(Default::default())
    }
}
