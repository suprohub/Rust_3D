use crate::math::Matrix::Matrix;



pub struct ObjectIdTag {
    id: usize
}

pub struct Object {
    tag: ObjectIdTag,
    transform_matrix: Matrix,

}