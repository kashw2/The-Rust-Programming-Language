use shape::{LxWShape, Rectangle, Shape};

pub mod shape;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn rectangle_area_is_correct() {
        let rectangle: Rectangle = Rectangle { length: 50.0, width: 100.0 };
        let result: f32 = rectangle.calculate_area();
        assert_eq!(result, 5000.0);
    }

    #[test]
    fn can_fit_lxw_shape() {
        let rectangle_1: Rectangle = Rectangle { length: 50.0, width: 100.0 };
        let rectangle_2: Rectangle = Rectangle { length: 20.0, width: 50.0 };
        let result: bool = rectangle_1.can_fit(&rectangle_2);
        assert!(result);
    }
}
