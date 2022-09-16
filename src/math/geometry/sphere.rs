use super::Vector;



pub struct Sphere {
    center: Vector<3>,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vector<3>, radius: f64) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}
