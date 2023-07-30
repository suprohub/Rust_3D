use std::collections::HashMap;

use crate::math::Matrix::Matrix;
use crate::math::Vec3D::Vec3D;
pub struct ObjectIdTag {
    id: usize
}

pub struct Object<'a> {
    tag: ObjectIdTag,
    transform_matrix: Matrix,
    position: Vec3D,
        /*
     * Take into account when you rotate body,
     * you change 'angle' & 'angle_left_up_look_at' only for this particular body,
     * but not for attached objects! This way during rotation
     * 'angle' & 'angle_left_up_look_at' stays constant all attached objects.
     */
    angle: Vec3D,
    angle_left_up_look_at: Vec3D,
    attached_objects: HashMap<ObjectIdTag, &'a Object<'a>>
}