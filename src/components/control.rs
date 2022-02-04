use std::{f32::consts::PI, marker::PhantomData};

use bevy::prelude::*;

use super::player::Player;

const MIN: f32 = 0.0;
const MAX: f32 = 1.0;

pub struct HandControl<T> {
    data: PhantomData<T>,
    left: Vec2,
    right: Vec2,
}

impl<T> Default for HandControl<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
            left: Vec2::new(0.0, 0.0),
            right: Vec2::new(1.0, 0.0),
        }
    }
}

const SHOULDER_Y_MAX: f32 = PI * 4.0 / 7.0 / 2.0;
const ELBOW_Y_MAX: f32 = PI * 3.8 / 7.0 / 2.0;
const SHOULDER_Z_MIN: f32 = -PI * 8.0 / 9.0 / 4.0;
const SHOULDER_Z_MAX: f32 = PI * 8.0 / 9.0 / 4.0;

impl<T: Player> HandControl<T> {
    pub fn add_left(&mut self, other: Vec2) {
        self.left += other;
        self.left.x = self.left.x.min(MAX).max(MIN);
        self.left.y = self.left.y.min(MAX).max(MIN);
    }

    pub fn add_right(&mut self, other: Vec2) {
        self.right += other;
        self.right.x = self.right.x.min(MAX).max(MIN);
        self.right.y = self.right.y.min(MAX).max(MIN);
    }

    pub fn right_sholder(&self) -> (f32, f32, f32) {
        let x = 0.0;
        let y = lerp(0.0, SHOULDER_Y_MAX, 1.0 - self.right.x);
        let z = lerp(SHOULDER_Z_MIN, SHOULDER_Z_MAX, self.right.y);
        (x, y, z)
    }

    pub fn left_sholder(&self) -> (f32, f32, f32) {
        let x = 0.0;
        let y = lerp(0.0, -SHOULDER_Y_MAX, self.left.x);
        let z = lerp(-SHOULDER_Z_MIN, -SHOULDER_Z_MAX, self.left.y);
        (x, y, z)
    }

    pub fn right_elbow(&self) -> (f32, f32, f32) {
        let x = 0.0;
        let y = lerp(0.0, ELBOW_Y_MAX, (1.0 - self.right.x).powi(2));
        let z = 0.0;
        (x, y, z)
    }

    pub fn left_elbow(&self) -> (f32, f32, f32) {
        let x = 0.0;
        let y = lerp(0.0, -ELBOW_Y_MAX, self.left.x.powi(2));
        let z = 0.0;
        (x, y, z)
    }
}

fn lerp(min: f32, max: f32, t: f32) -> f32 {
    min + (max - min) * t
}
