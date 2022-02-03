use bevy::prelude::*;

use super::BodyPart;

#[derive(Component, Default)]
pub struct Hip;
#[derive(Component, Default)]
pub struct Spine;
#[derive(Component, Default)]
pub struct Chest;
#[derive(Component, Default)]
pub struct Neck;
#[derive(Component, Default)]
pub struct Head;
#[derive(Component, Default)]
pub struct UpperArmLeft;
#[derive(Component, Default)]
pub struct ForearmLeft;
#[derive(Component, Default)]
pub struct HandLeft;
#[derive(Component, Default)]
pub struct UpperArmRight;
#[derive(Component, Default)]
pub struct ForearmRight;
#[derive(Component, Default)]
pub struct HandRight;
#[derive(Component, Default)]
pub struct ThighLeft;
#[derive(Component, Default)]
pub struct ShinLeft;
#[derive(Component, Default)]
pub struct FootLeft;
#[derive(Component, Default)]
pub struct ThighRight;
#[derive(Component, Default)]
pub struct ShinRight;
#[derive(Component, Default)]
pub struct FootRight;

impl BodyPart for Hip {}
impl BodyPart for Spine {}
impl BodyPart for Chest {}
impl BodyPart for Neck {}
impl BodyPart for Head {}
impl BodyPart for UpperArmLeft {}
impl BodyPart for ForearmLeft {}
impl BodyPart for HandLeft {}
impl BodyPart for UpperArmRight {}
impl BodyPart for ForearmRight {}
impl BodyPart for HandRight {}
impl BodyPart for ThighLeft {}
impl BodyPart for ShinLeft {}
impl BodyPart for FootLeft {}
impl BodyPart for ThighRight {}
impl BodyPart for ShinRight {}
impl BodyPart for FootRight {}
