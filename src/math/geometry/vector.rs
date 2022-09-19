

use std::{iter::zip, ops::{Index, self}};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector<const LENGTH: usize> {
    data: [f64; LENGTH],
}

impl<const LENGTH: usize> Vector<LENGTH> {
    pub fn new(data: [f64; LENGTH]) -> Self {
        Vector {
            data: data
        }
    }

    pub fn length(self) -> f64 {
        return f64::sqrt(dot(&self, &self));
    }

    pub fn normalized(self) -> Vector<LENGTH> {
        return self / self.length();
    }
}

impl<const LENGTH: usize> Index<usize> for Vector<LENGTH> {
    type Output = f64;

    fn index (&self, index: usize) -> &f64 {
        return &self.data[index]
    }
}

pub fn dot<const LENGTH: usize> (a: &Vector<LENGTH>, b: &Vector<LENGTH>) -> f64 {
    let res = zip(a.data, b.data).map(|pair: (f64, f64) | pair.0 * pair.1 ).reduce(|a: f64, b: f64| a + b);

    match res {
        None => 0f64,
        Some(value) => value
    }
}

impl<const LENGTH: usize> ops::Add<Vector<LENGTH>> for Vector<LENGTH> {
    type Output = Vector<LENGTH>;


    fn add(self, rhs: Vector<LENGTH>) -> Self::Output {
        
        let mut result_arr = [0.0; LENGTH];

        for i in 0..LENGTH {
            result_arr[i] = self.data[i] + rhs.data[i];
        }

        return Vector::new(result_arr);
    }
}

impl<const LENGTH: usize> ops::Sub<Vector<LENGTH>> for Vector<LENGTH> {
    type Output = Vector<LENGTH>;


    fn sub(self, rhs: Vector<LENGTH>) -> Self::Output {
        
        let mut result_arr = [0.0; LENGTH];

        for i in 0..LENGTH {
            result_arr[i] = self.data[i] - rhs.data[i];
        }

        return Vector::new(result_arr);
    }
}

impl<const LENGTH: usize> ops::Mul<f64> for Vector<LENGTH> {
    type Output = Vector<LENGTH>;


    fn mul(self, rhs: f64) -> Self::Output {
        
        let mut result_arr = [0.0; LENGTH];

        for i in 0..LENGTH {
            result_arr[i] = self.data[i] * rhs;
        }

        return Vector::new(result_arr);
    }
}

impl<const LENGTH: usize> ops::Div<f64> for Vector<LENGTH> {
    type Output = Vector<LENGTH>;

    fn div(self, rhs: f64) -> Self::Output {
        
        let mut result_arr = [0.0; LENGTH];

        for i in 0..LENGTH {
            result_arr[i] = self.data[i] / rhs;
        }

        return Vector::new(result_arr);
    }
}


#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use super::{Vector, dot};

    #[test]
    fn test_vector_construction() {
        let _vec1 = Vector::new([1.0, 2.0]);
        let _vec2 = Vector::new([3.0, 4.0]);
    }

    #[test]
    fn test_vector_indexing() {
        let vec1 = Vector::new([1.0, 2.0]);
        let vec2 = Vector::new([3.0, 4.0]);

        assert_eq!(vec1[0], 1.0);
        assert_eq!(vec1[1], 2.0);
        assert_eq!(vec2[0], 3.0);
        assert_eq!(vec2[1], 4.0);
    }

    #[test]
    fn test_dot_product() {
        let vec1 = Vector::new([1.0, 2.0]);
        let vec2 = Vector::new([3.0, 4.0]);

        assert_eq!(dot(&vec1, &vec2), 11.0);
        assert_eq!(dot(&vec2, &vec1), 11.0);
    }

    #[test]
    fn test_length() {
        let vec1 = Vector::new([1.0, 2.0]);
        let vec2 = Vector::new([3.0, 4.0]);

        assert!(approx_eq!(f64, vec1.length(), f64::sqrt(5.0), epsilon=0.0000001));
        assert!(approx_eq!(f64, vec2.length(), f64::sqrt(25.0), epsilon=0.0000001));
    }

    #[test]
    fn test_normalize() {
        let vec1 = Vector::new([1.0, 2.0]);
        let normalized = vec1.normalized();

        assert!(approx_eq!(f64, normalized[0], 1.0/f64::sqrt(5.0), epsilon=0.0000001));
        assert!(approx_eq!(f64, normalized[1], 2.0/f64::sqrt(5.0), epsilon=0.0000001));
    }

    #[test]
    fn test_add_operator() {
        let vec1 = Vector::new([1.0, 2.0]);
        let vec2 = Vector::new([3.0, 4.0]);

        assert_eq!(vec1 + vec2, Vector::new([1.0 + 3.0, 2.0 + 4.0]));
    }

    #[test]
    fn test_subtract_operator() {
        let vec1 = Vector::new([1.0, 2.0]);
        let vec2 = Vector::new([3.0, 4.0]);

        assert_eq!(vec1 - vec2, Vector::new([1.0 - 3.0, 2.0 - 4.0]));
    }

    #[test]
    fn test_mul_operator() {
        let vec1 = Vector::new([1.0, 2.0]);

        assert_eq!(vec1 * 2.0, Vector::new([1.0 * 2.0, 2.0 * 2.0]));
    }

    #[test]
    fn test_div_operator() {
        let vec1 = Vector::new([1.0, 2.0]);

        assert_eq!(vec1 / 0.7, Vector::new([1.0 /0.7, 2.0 / 0.7]));
    }


}

