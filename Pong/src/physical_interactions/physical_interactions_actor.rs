// Pong Game
//
// © 2024 Rust Made Easy. All rights reserved.
//
// @author JoelDavisEngineering@Gmail.com

/// Specifies the actors that may physically interact with one another.
#[derive(Clone, PartialEq)]
pub(crate) enum PhysicalInteractionActor {
    Ball,
    Ceiling,
    Floor,
    None,
    Paddle,
    SideWall,
}
