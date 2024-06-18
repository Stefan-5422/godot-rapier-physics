use nalgebra::Point;
use rapier::prelude::*;

use super::ANG_ZERO;
use crate::rapier_wrapper::prelude::*;
pub enum BodyType {
    Dynamic,
    Kinematic,
    Static,
}
fn set_rigid_body_properties_internal(
    rigid_body: &mut RigidBody,
    pos: Vector<Real>,
    rot: AngVector<Real>,
    wake_up: bool,
) {
    if !rigid_body.is_kinematic() {
        rigid_body.set_position(Isometry::new(pos, rot), wake_up);
    } else {
        rigid_body.set_next_kinematic_position(Isometry::new(pos, rot));
    }
}
pub fn body_create(
    world_handle: Handle,
    pos: Vector<Real>,
    rot: AngVector<Real>,
    user_data: &UserData,
    body_type: BodyType,
) -> RigidBodyHandle {
    if let Some(physics_world) = physics_engine().get_world(world_handle) {
        let mut rigid_body: RigidBody;
        match body_type {
            BodyType::Dynamic => {
                rigid_body = RigidBodyBuilder::dynamic().build();
            }
            BodyType::Kinematic => {
                rigid_body = RigidBodyBuilder::kinematic_position_based().build();
            }
            BodyType::Static => {
                rigid_body = RigidBodyBuilder::fixed().build();
            }
        }
        // let default values better
        set_rigid_body_properties_internal(&mut rigid_body, pos, rot, true);
        rigid_body.user_data = user_data.get_data();
        let body_handle = physics_world
            .physics_objects
            .rigid_body_set
            .insert(rigid_body);
        return body_handle;
    }
    RigidBodyHandle::invalid()
}
pub fn body_change_mode(
    world_handle: Handle,
    body_handle: RigidBodyHandle,
    body_type: BodyType,
    wakeup: bool,
) {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get_mut(rigid_body_handle)
        {
            match body_type {
                BodyType::Dynamic => {
                    body.set_body_type(RigidBodyType::Dynamic, wakeup);
                }
                BodyType::Kinematic => {
                    body.set_body_type(RigidBodyType::KinematicPositionBased, wakeup);
                }
                BodyType::Static => {
                    body.set_body_type(RigidBodyType::Fixed, wakeup);
                }
            }
        }
    }
}
pub fn body_destroy(world_handle: Handle, body_handle: RigidBodyHandle) {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        physics_world.remove_rigid_body(body_handle);
    }
}
pub fn body_get_position(world_handle: Handle, body_handle: RigidBodyHandle) -> Vector<Real> {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get(rigid_body_handle)
        {
            let body_vector = body.translation();
            return *body_vector;
        }
    }
    Vector::default()
}
#[cfg(feature = "dim3")]
pub fn body_get_angle(world_handle: Handle, body_handle: RigidBodyHandle) -> AngVector<Real> {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get(rigid_body_handle)
        {
            let rotation = body.rotation().euler_angles();
            return AngVector::new(rotation.0, rotation.1, rotation.2);
        }
    }
    ANG_ZERO
}
#[cfg(feature = "dim2")]
pub fn body_get_angle(world_handle: Handle, body_handle: RigidBodyHandle) -> AngVector<Real> {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get(rigid_body_handle)
        {
            let rotation = body.rotation().angle();
            return rotation;
        }
    }
    ANG_ZERO
}
pub fn body_set_transform(
    world_handle: Handle,
    body_handle: RigidBodyHandle,
    pixel_pos: Vector<Real>,
    rot: AngVector<Real>,
    wake_up: bool,
) {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get_mut(rigid_body_handle)
        {
            set_rigid_body_properties_internal(body, pixel_pos, rot, wake_up);
        }
    }
}
pub fn body_get_linear_velocity(
    world_handle: Handle,
    body_handle: RigidBodyHandle,
) -> Vector<Real> {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get(rigid_body_handle)
        {
            let body_vel = body.linvel();
            return *body_vel;
        }
    }
    Vector::default()
}
pub fn body_set_linear_velocity(
    world_handle: Handle,
    body_handle: RigidBodyHandle,
    vel: Vector<Real>,
) {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get_mut(rigid_body_handle)
        {
            body.set_linvel(vel, true);
        }
    }
}
pub fn body_update_material(world_handle: Handle, body_handle: RigidBodyHandle, mat: &Material) {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get_mut(rigid_body_handle)
        {
            for collider in body.colliders() {
                if let Some(col) = physics_world
                    .physics_objects
                    .collider_set
                    .get_mut(*collider)
                {
                    // TODO update when https://github.com/dimforge/rapier/issues/622 is fixed
                    if mat.friction >= 0.0 {
                        col.set_friction(mat.friction);
                    }
                    if mat.restitution >= 0.0 {
                        col.set_restitution(mat.restitution);
                    }
                    if mat.contact_skin >= 0.0 {
                        col.set_contact_skin(mat.contact_skin);
                    }
                }
            }
        }
    }
}
#[cfg(feature = "dim2")]
pub fn body_get_angular_velocity(
    world_handle: Handle,
    body_handle: RigidBodyHandle,
) -> AngVector<Real> {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get(rigid_body_handle)
        {
            return body.angvel();
        }
    }
    ANG_ZERO
}
#[cfg(feature = "dim3")]
pub fn body_get_angular_velocity(
    world_handle: Handle,
    body_handle: RigidBodyHandle,
) -> AngVector<Real> {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get(rigid_body_handle)
        {
            return *body.angvel();
        }
    }
    ANG_ZERO
}
pub fn body_set_angular_velocity(
    world_handle: Handle,
    body_handle: RigidBodyHandle,
    vel: AngVector<Real>,
) {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get_mut(rigid_body_handle)
        {
            body.set_angvel(vel, true);
        }
    }
}
pub fn body_set_linear_damping(
    world_handle: Handle,
    body_handle: RigidBodyHandle,
    linear_damping: Real,
) {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get_mut(rigid_body_handle)
        {
            body.set_linear_damping(linear_damping);
        }
    }
}
pub fn body_set_angular_damping(
    world_handle: Handle,
    body_handle: RigidBodyHandle,
    angular_damping: Real,
) {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get_mut(rigid_body_handle)
        {
            body.set_angular_damping(angular_damping);
        }
    }
}
pub fn body_set_gravity_scale(
    world_handle: Handle,
    body_handle: RigidBodyHandle,
    gravity_scale: Real,
    wake_up: bool,
) {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get_mut(rigid_body_handle)
        {
            body.set_gravity_scale(gravity_scale, wake_up);
        }
    }
}
pub fn body_set_can_sleep(world_handle: Handle, body_handle: RigidBodyHandle, can_sleep: bool) {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get_mut(rigid_body_handle)
        {
            if !can_sleep {
                let activation = body.activation_mut();
                activation.angular_threshold = -1.0;
                activation.normalized_linear_threshold = -1.0;
            } else {
                let activation = body.activation_mut();
                let default_activation = RigidBodyActivation::default();
                activation.angular_threshold = default_activation.angular_threshold;
                activation.normalized_linear_threshold =
                    default_activation.normalized_linear_threshold;
            }
            // TODO: Check if is requiered
            if !can_sleep && body.is_sleeping() {
                body.wake_up(true);
            }
        }
    }
}
pub fn body_set_ccd_enabled(world_handle: Handle, body_handle: RigidBodyHandle, enable: bool) {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get_mut(rigid_body_handle)
        {
            body.enable_ccd(enable);
        }
    }
}
pub fn body_set_mass_properties(
    world_handle: Handle,
    body_handle: RigidBodyHandle,
    mass: Real,
    inertia: AngVector<Real>,
    local_com: Vector<Real>,
    wake_up: bool,
    force_update: bool,
) {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get_mut(rigid_body_handle)
        {
            body.set_additional_mass_properties(
                MassProperties::new(Point { coords: local_com }, mass, inertia),
                wake_up,
            );
            if force_update {
                body.recompute_mass_properties_from_colliders(
                    &physics_world.physics_objects.collider_set,
                );
            }
        }
    }
}
pub fn body_add_force(world_handle: Handle, body_handle: RigidBodyHandle, force: Vector<Real>) {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get_mut(rigid_body_handle)
        {
            body.add_force(force, true);
        }
    }
}
pub fn body_add_force_at_point(
    world_handle: Handle,
    body_handle: RigidBodyHandle,
    force: Vector<Real>,
    point: Vector<Real>,
) {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get_mut(rigid_body_handle)
        {
            let local_point = Point { coords: point } + body.center_of_mass().coords;
            body.add_force_at_point(force, local_point, true);
        }
    }
}
pub fn body_add_torque(
    world_handle: Handle,
    body_handle: RigidBodyHandle,
    torque: AngVector<Real>,
) {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get_mut(rigid_body_handle)
        {
            body.add_torque(torque, true);
        }
    }
}
pub fn body_apply_impulse(
    world_handle: Handle,
    body_handle: RigidBodyHandle,
    impulse: Vector<Real>,
) {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get_mut(rigid_body_handle)
        {
            body.apply_impulse(impulse, true);
        }
    }
}
pub fn body_apply_impulse_at_point(
    world_handle: Handle,
    body_handle: RigidBodyHandle,
    impulse: Vector<Real>,
    point: Vector<Real>,
) {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get_mut(rigid_body_handle)
        {
            let mut local_point = Point { coords: point };
            local_point += body.center_of_mass().coords;
            body.apply_impulse_at_point(impulse, local_point, true);
        }
    }
}
pub fn body_get_constant_force(world_handle: Handle, body_handle: RigidBodyHandle) -> Vector<Real> {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get_mut(rigid_body_handle)
        {
            return body.user_force();
        }
    }
    Vector::default()
}
pub fn body_get_constant_torque(
    world_handle: Handle,
    body_handle: RigidBodyHandle,
) -> AngVector<Real> {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get_mut(rigid_body_handle)
        {
            return body.user_torque();
        }
    }
    ANG_ZERO
}
pub fn body_apply_torque_impulse(
    world_handle: Handle,
    body_handle: RigidBodyHandle,
    torque_impulse: AngVector<Real>,
) {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get_mut(rigid_body_handle)
        {
            body.apply_torque_impulse(torque_impulse, true);
        }
    }
}
pub fn body_reset_torques(world_handle: Handle, body_handle: RigidBodyHandle) {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get_mut(rigid_body_handle)
        {
            body.reset_torques(false);
        }
    }
}
pub fn body_reset_forces(world_handle: Handle, body_handle: RigidBodyHandle) {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get_mut(rigid_body_handle)
        {
            body.reset_forces(false);
        }
    }
}
pub fn body_wake_up(world_handle: Handle, body_handle: RigidBodyHandle, strong: bool) {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get_mut(rigid_body_handle)
        {
            if body.is_sleeping() {
                body.wake_up(strong);
            }
        }
    }
}
pub fn body_force_sleep(world_handle: Handle, body_handle: RigidBodyHandle) {
    let physics_engine = physics_engine();
    if let Some(physics_world) = physics_engine.get_world(world_handle) {
        let rigid_body_handle = body_handle;
        if let Some(body) = physics_world
            .physics_objects
            .rigid_body_set
            .get_mut(rigid_body_handle)
        {
            body.sleep();
        }
    }
}
