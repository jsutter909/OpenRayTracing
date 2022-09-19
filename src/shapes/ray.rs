use crate::math::geometry::Vector;


pub struct Ray {
    pub origin: Vector<3>,
    pub direction: Vector<3>,
}

impl Ray {
    pub fn new(origin: Vector<3>, direction: Vector<3>) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }
}

// a module for testing ray.rs
#[cfg(test)]
pub mod tests {
    use crate::math::geometry::Vector;

    use super::Ray;

    // test construction of a ray
    #[test]
    fn test_ray_construction() {
        let _ray = Ray::new(Vector::new([0.0, 0.0, 0.0]), Vector::new([1.0, 0.0, 0.0]));
    }

}

