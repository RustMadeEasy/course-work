use bevy::prelude::Component;

/// Companion component for UI elements. This allows, for instance, for buttons to be differentiated
/// by purpose when clicked.
#[derive(Clone, Component)]
pub struct EntityInfoComponent<EnumType: Clone> {
    purpose: EnumType,
}

impl<EnumType: Clone> EntityInfoComponent<EnumType> {
    //

    /// Returns the purpose for the entity.
    pub fn get_purpose(&self) -> EnumType {
        self.purpose.clone()
    }

    /// Creates a new EntityInfoComponent instance.
    pub fn new(purpose: EnumType) -> Self {
        Self { purpose }
    }
}
