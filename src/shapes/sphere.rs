use crate::math::geometry::{Vector, dot};
use super::{intersectable::Intersectable, ray::Ray};


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

impl Intersectable for Sphere {
    fn intersect(self, ray: Ray) -> Option<Ray> {

        let v = ray.origin - self.center;
        let discriminant = dot(&v, &ray.direction) - (dot(&v, &v) - self.radius * self.radius);
        
        if discriminant < 0.0 {
            return None;
        } else {
            let center_value = -1.0 * dot(&v, &ray.direction);
            let root_discriminant = f64::sqrt(discriminant);

            let t1 = center_value - root_discriminant;
            let t2 = center_value + root_discriminant;

            if t1 < 0.0 && t2 < 0.0 {
                return None;
            }

            let t = if t1 < 0.0 {
                t2
            } else {
                t1
            };

            let new_position = ray.origin + ray.direction * t;
            let circle_normal = (new_position - self.center).normalized();

            let new_direction = ray.direction - circle_normal * 2.0 * (dot(&circle_normal, &ray.direction));

            return Some(Ray::new(new_position, new_direction));
        }
    }
}

#[cfg(test)]
mod test{
    use crate::math::geometry::Vector;

    use super::Sphere;
 
    #[test]
    fn test_sphere_construction() {
        let _sphere = Sphere::new(Vector::new([0.0, 0.0, 0.0]), 1.0);
    }
}