/// https://github.com/bevyengine/bevy/blob/latest/examples/math/custom_primitives.rs
/// https://github.com/bevyengine/bevy/blob/latest/examples/math/render_primitives.rs

use bevy::{
    math::{
        primitives::{Capsule3d, Cone, Cuboid, Cylinder, Sphere},
        bounding::RayCast3d
    },
    prelude::*,
};


/// Enum listing the shapes that can be sampled
#[derive(Clone, Copy)]
enum Shape {
    Cuboid,
    Sphere,
    Capsule,
    Cylinder,
    Tetrahedron,
    Triangle,
}
struct ShapeMeshBuilder {
    shape: Shape,
}


impl Meshable for Shape {
    type Output = ShapeMeshBuilder;

    fn mesh(&self) -> Self::Output {
        ShapeMeshBuilder { shape: *self }
    }
}

impl MeshBuilder for ShapeMeshBuilder {
    fn build(&self) -> Mesh {
        match self.shape {
            Shape::Cuboid => CUBOID.mesh().into(),
            Shape::Sphere => SPHERE.mesh().into(),
            Shape::Capsule => CAPSULE_3D.mesh().into(),
            Shape::Cylinder => CYLINDER.mesh().into(),
            Shape::Tetrahedron => TETRAHEDRON.mesh().into(),
            Shape::Triangle => TRIANGLE_3D.mesh().into(),
        }
    }
}
