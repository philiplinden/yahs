use avian3d::prelude::*;
use bevy::prelude::*;

const TETHER_SEGMENT_LENGTH: f32 = 0.5;

pub fn link_entities(
    mut commands: Commands,
    length: f32,
    attached_entity_one: Entity,
    attached_entity_two: Entity,

) -> Entity {
    let num_segments = (length / TETHER_SEGMENT_LENGTH).ceil() as usize;
    // Create parent entity for the tether system
    let tether_parent = commands.spawn((Name::new("Tether System"),)).id();

    let mut segment_entities = Vec::new();
    let mut joint_entities = Vec::new();
    let mut prev_entity = attached_entity_one;

    let segment_half_length = TETHER_SEGMENT_LENGTH / 2.0;

    // Create segments
    for i in 0..num_segments {
        let segment_entity = commands
            .spawn((
                Name::new(format!("Tether Segment {}", i)),
                RigidBody::Dynamic,
                Transform::from_translation(Vec3::new(
                    0.0,
                    1.0 - (i as f32 * TETHER_SEGMENT_LENGTH),
                    0.0,
                )),
                Mass(0.0),
            ))
            .id();
        segment_entities.push(segment_entity);

        let joint_entity = commands
            .spawn(
                SphericalJoint::new(prev_entity, segment_entity)
                    .with_local_anchor_1(Vec3::NEG_Y * segment_half_length)
                    .with_local_anchor_2(Vec3::Y * segment_half_length)
                    .with_compliance(0.001),
            )
            .id();
        joint_entities.push(joint_entity);
        prev_entity = segment_entity;
    }

    // Final joint
    let final_joint = commands
        .spawn(
            SphericalJoint::new(prev_entity, attached_entity_two)
                .with_local_anchor_1(Vec3::NEG_Y * segment_half_length)
                .with_local_anchor_2(Vec3::Y * segment_half_length)
                .with_compliance(0.001),
        )
        .id();
    joint_entities.push(final_joint);
    tether_parent
}
