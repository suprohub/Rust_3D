mod math;
fn main() {
    let test1 = math::Vec3D {x: 1.0, y: 2.0, z: 3.0};
    let test2 = math::Vec3D {x: 3.0, y: 2.0, z: 1.0};
    println!("{:?}", test1.cross(&test2))
}