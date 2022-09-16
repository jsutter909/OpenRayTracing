

// a module for testing ray.rs
#[cfg(test)]
pub mod ray_test {
    // test construction of a ray
    #[test]
    fn test_ray_construction() {
        let _ray = Ray::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    }

}
