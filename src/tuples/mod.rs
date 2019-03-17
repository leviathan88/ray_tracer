use crate::utils::NumberUtils;

#[derive(PartialEq, Debug)]
enum TupleType {
    Point,
    Vector
}

#[derive(PartialEq, Debug)]
struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64
}


impl Tuple {

    fn new_vector(x: f64, y: f64, z:f64) -> Self {
        Tuple {
            x,
            y,
            z,
            w: 0.0
        }
    }

    fn new_point(x: f64, y: f64, z: f64) -> Self {
        Tuple {
            x,
            y,
            z,
            w: 1.0
        }
    }

    fn get_type(&self) -> TupleType {
        match self.w {
            n if NumberUtils::compare_floats(n, 0.0) => TupleType::Vector,
            _ => TupleType::Point
        }
    }

    

}

mod tests {

    use super::*;

    #[test]
    fn test_is_vector() {
        let point = Tuple {
            x: 2.4,
            y: 1.0,
            z: 0.1,
            w: 1.0
        };

        assert_eq!(point.get_type(), TupleType::Point);
    }

    #[test]
    fn test_is_point() {
        let vector = Tuple {
            x: 2.4,
            y: 1.0,
            z: 0.1,
            w: 0.0
        };

        assert_eq!(vector.get_type(), TupleType::Vector);
    }

    #[test]
    fn test_new_vector() {
        let new_vec = Tuple::new_vector(3.4, 3.1, 1.0);
        assert_eq!(new_vec.get_type(), TupleType::Vector);
        assert_eq!(new_vec.w, 0.0);
    }

    #[test]
    fn test_new_point() {
        let new_point = Tuple::new_point(2.1, 0.0, 1.9);
        assert_eq!(new_point.get_type(), TupleType::Point);
        assert_eq!(new_point.w, 1.0);
    }

}