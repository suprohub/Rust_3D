use std::collections::HashMap;

use crate::math::Matrix::Matrix;
use crate::math::Vec3D::Vec3D;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ObjectIdTag {
    id: usize
}


pub trait Object {
    pub fn transform(t: &Matrix) {
        //impl
    }
    pub fn transform_relative_point(p: &Vec3D, t: &Matrix) {
        //impl
    }
    pub fn translate(dv: &Vec3D) {
        //impl
    }
    pub fn translate_to_point(p: &Vec3D) {
        //impl
    }
    pub fn attract_to_point(p: &Vec3D, value: f64) {
        //impl
    }
    pub fn scale(s: &Vec3D) {
        //impl
    }
    pub fn rotate(r: &Vec3D) {
        //impl
    }
    pub fn rotaterv(r: &Vec3D, rv: f64) {
        //impl
    }
    pub fn rotate_to_angle(v: &Vec3D) {
        //impl
    }
    pub fn rotate_relative_point(s: &Vec3D, r: &Vec3D) {
        //impl
    }
    pub fn rotate_relative_pointr(s: &Vec3D, v: &Vec3D, r: f64) {
        //impl
    }
    pub fn rotate_left(rl: f64) {
        //impl
    }
    pub fn rotate_up(ru: f64) {
        //impl
    }
    pub fn rotate_look_at(rl_at: f64) {
        //impl
    }
    

}