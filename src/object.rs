use std::collections::HashMap;

use crate::math::Matrix::Matrix;
use crate::math::Vec3D::Vec3D;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ObjectIdTag {
    id: usize
}


pub trait Object {
    fn transform(t: &Matrix) {
        //impl
    }
    fn transform_relative_point(p: &Vec3D, t: &Matrix) {
        //impl
    }
    fn translate(dv: &Vec3D) {
        //impl
    }
    fn translate_to_point(p: &Vec3D) {
        //impl
    }
    fn attract_to_point(p: &Vec3D, value: f64) {
        //impl
    }
    fn scale(s: &Vec3D) {
        //impl
    }
    fn rotate(r: &Vec3D) {
        //impl
    }
    fn rotaterv(r: &Vec3D, rv: f64) {
        //impl
    }
    fn rotate_to_angle(v: &Vec3D) {
        //impl
    }
    fn rotate_relative_point(s: &Vec3D, r: &Vec3D) {
        //impl
    }

}