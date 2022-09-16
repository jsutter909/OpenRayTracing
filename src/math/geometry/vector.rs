

use std::{iter::zip, ops::Index};

#[derive(Debug)]
pub struct Vector<const LENGTH: usize> {
    data: [f64; LENGTH],
}

impl<const LENGTH: usize> Vector<LENGTH> {
    pub fn new(data: [f64; LENGTH]) -> Self {
        Vector {
            data: data
        }
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
