// Pong Game
//
// © 2024 Rust Made Easy. All rights reserved.
//
// @author JoelDavisEngineering@Gmail.com

pub(crate) mod collision_evaluator;
pub(super) mod collision_event;
pub(super) mod physical_interactions_actor;
pub(super) mod physical_interactions_plugin;

pub(super) const DIRECTION_BACKWARD: f32 = -1f32;
pub(super) const DIRECTION_FORWARD: f32 = 1f32;
