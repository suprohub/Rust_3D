use std::collections::HashMap;

use crate::math::Matrix::Matrix;
use crate::math::Vec3D::Vec3D;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ObjectIdTag {
    id: usize
}

pub struct ObjectStruct {

}

pub trait Object {
    fn transform<T>(me: T, t: &Matrix) {
        
    }
    fn transform_relative_point<T>(me: T, p: &Vec3D, t: &Matrix) {
        
    }
    fn translate<T>(me: T, dv: &Vec3D) {
        
    }
    fn translate_to_point<T>(me: T, p: &Vec3D) {
        
    }
    fn attract_to_point<T>(me: T, p: &Vec3D, value: f64) {
        
    }
    fn scale<T>(me: T, s: &Vec3D) {
        
    }
    fn rotate<T>(me: T, r: &Vec3D) {
        
    }
    fn rotaterv<T>(me: T, r: &Vec3D, rv: f64) {
        
    }
    fn rotate_to_angle<T>(me: T, v: &Vec3D) {
        
    }
    fn rotate_relative_point<T>(me: T, s: &Vec3D, r: &Vec3D) {
        
    }
    fn rotate_relative_pointr<T>(me: T, s: &Vec3D, v: &Vec3D, r: f64) {
        
    }
    fn rotate_left<T>(me: T, rl: f64) {
        
    }
    fn rotate_up<T>(me: T, ru: f64) {
        
    }
    fn rotate_look_at<T>(me: T, rl_at: f64) {
        
    }
    fn attach<T, Y>(me: T, obj: &Y) {
        
    }
    fn unattach<T, Y>(me: T, obj: &Y) {
        
    }
    fn attached<T, Y>(me: T, obj: &Y) {
        
    }
    

}