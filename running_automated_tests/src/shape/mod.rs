pub trait LxWShape {
    fn calculate_area(&self) -> f32;
}

pub trait Shape<A> {
    fn can_fit(&self, other: &A) -> bool;
}

pub struct Rectangle {
    pub length: f32,
    pub width: f32,
}

impl LxWShape for Rectangle {
    fn calculate_area(&self) -> f32 {
        return self.length * self.width;
    }
}

impl<A> Shape<A> for A
where A: LxWShape {
    fn can_fit(&self, other: &A) -> bool {
        return self.calculate_area() > other.calculate_area();
    }
}
