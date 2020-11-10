use bevy::prelude::*;
use bevy_rapier2d::physics::RigidBodyHandleComponent;
use bevy_rapier2d::rapier::dynamics::RigidBodySet;

use crate::components::LockRotation;

// Currently there is no way to lock a RB's rotation on build, so we have to
// manually set it after creation.
pub fn remove_rotation(
    mut commands: Commands,
    mut rigid_bodies: ResMut<RigidBodySet>,
    entity: Entity,
    rb_handle_component: &RigidBodyHandleComponent,
    _remove_rotation_component: &LockRotation,
) {
    if let Some(mut rigid_body) = rigid_bodies.get_mut(rb_handle_component.handle()) {
        eprintln!("Removing rotation.");
        rigid_body.mass_properties.inv_principal_inertia_sqrt = 0.0;
        commands.remove_one::<LockRotation>(entity);
    }
}
