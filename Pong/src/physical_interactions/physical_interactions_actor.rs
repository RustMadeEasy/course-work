/// Specifies the actors that may interact with one another.
#[derive(Clone, PartialEq)]
pub(crate) enum PhysicalInteractionActor {
    Ball,
    Ceiling,
    Floor,
    None,
    Paddle,
    SideWall,
}
