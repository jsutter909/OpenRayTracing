use super::ray::Ray;


// an object that can be intersected by a ray
pub trait Intersectable {
    fn intersect(self, ray: Ray) -> Option<Ray>;
}