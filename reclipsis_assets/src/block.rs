use avian3d::prelude::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub const BLOCK_WIDTH: f32 = 1.0;
pub const BLOCK_HEIGHT: f32 = 1.0;

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BlockMarker;

#[derive(Bundle)]
pub struct BlockPhysicsBundle {
    collider: Collider,
    rigid_body: RigidBody,
}

impl Default for BlockPhysicsBundle {
    fn default() -> Self {
        Self {
            collider: Collider::cuboid(BLOCK_WIDTH, BLOCK_HEIGHT, BLOCK_WIDTH),
            rigid_body: RigidBody::Dynamic,
        }
    }
}
