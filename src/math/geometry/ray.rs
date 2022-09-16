

pub struct Ray {
    origin: Vector<3>,
    direction: Vector<3>,
}

impl Ray {
    pub fn new(origin: Vector<3>, direction: Vector<3>) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }
}
