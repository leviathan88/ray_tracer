use std::ops;

use crate::utils::NumberUtils;

pub type Float = f64;

#[derive(PartialEq, Debug)]
pub enum TupleType {
    Point,
    Vector,
    Tuple
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Tuple {
    pub x: Float,
    pub y: Float,
    pub z: Float,
    pub w: Float
}


impl Tuple {

    pub fn new_vector(x: Float, y: Float, z:Float) -> Self {
        Tuple {
            x,
            y,
            z,
            w: 0.0
        }
    }

    pub fn new_point(x: Float, y: Float, z: Float) -> Self {
        Tuple {
            x,
            y,
            z,
            w: 1.0
        }
    }

    pub fn new_tuple(x: Float, y: Float, z: Float, w: Float) -> Self {
        Tuple {
            x,
            y,
            z,
            w
        }
    }

    pub fn get_type(&self) -> TupleType {
        match self.w {
            n if NumberUtils::compare_floats(n, 0.0) => TupleType::Vector,
            n if NumberUtils::compare_floats(n, 1.0) => TupleType::Point,
            _ => TupleType::Tuple
        }
    }

    pub fn is_equal_to(&self, tuple: &Tuple) -> bool {
        NumberUtils::compare_floats(self.x, tuple.x) &&
        NumberUtils::compare_floats(self.y, tuple.y) &&
        NumberUtils::compare_floats(self.z, tuple.z) &&
        NumberUtils::compare_floats(self.w, tuple.w)
    }

    pub fn negate(&self) -> Self {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: match self.get_type() {
                TupleType::Tuple => -self.w,
                _ => self.w
            }
        }
    }

    pub fn get_magnitude(&self) -> Float {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.get_magnitude();
        Self::new_tuple(self.x / magnitude, self.y / magnitude, self.z / magnitude, self.w / magnitude)
    }

    pub fn calculate_dot_product(&self, tuple: &Tuple) -> Float {
        (self.x * tuple.x) + (self.y * tuple.y) + (self.z * tuple.z) + (self.w * tuple.w)
    }

    pub fn get_vector_cross_product(&self, vector: &Tuple) -> Self {
        Tuple::new_vector(
            (self.y * vector.z) - (self.z * vector.y),
            (self.z * vector.x) - (self.x * vector.z),
            (self.x * vector.y) - (self.y * vector.x)
        )
    }

}

impl ops::Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, tuple: Tuple) -> Tuple {
        Tuple {
            x: self.x + tuple.x,
            y: self.y + tuple.y,
            z: self.z + tuple.z,
            w: self.w + tuple.w
        }
    }

}

impl ops::Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, tuple: Tuple) -> Tuple {
        Tuple {
            x: self.x - tuple.x,
            y: self.y - tuple.y,
            z: self.z - tuple.z,
            w: self.w - tuple.w
        }
    }
}

impl ops::Div<Float> for Tuple {
    type Output = Tuple;

    fn div(self, scalar: Float) -> Tuple {
        Tuple {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }

}

impl ops::Mul<Float> for Tuple {
    type Output = Tuple;

    fn mul(self, scalar: Float) -> Self {
        Tuple {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: match self.get_type() {
                TupleType::Tuple => self.w * scalar,
                _ => self.w
            }
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

    #[test]
    fn test_compare_two_tuples() {
        let tuple_1 = Tuple::new_point(1.2, 3.2, 2.0);
        let tuple_2 = Tuple::new_point(1.2, 3.2, 2.000000000000000002);

        assert_eq!(tuple_1.is_equal_to(&tuple_2), true);
    }

    #[test]
    fn test_add_point_and_vector() {        
        let tuple_1 = Tuple::new_point(1.2, 3.2, 2.0);
        let tuple_2 = Tuple::new_vector(1.2, 3.2, 2.000000000000000002);

        let tuple_3 = tuple_1 + tuple_2; //tuple_1.add(&tuple_2);
        assert_eq!(tuple_3, Tuple { x: 2.4, y: 6.4, z: 4.0, w: 1.0});
        assert_eq!(tuple_3.get_type(), TupleType::Point);
    }

    #[test]
    fn test_subtract_two_points() {        
        let tuple_1 = Tuple::new_point(3.0, 2.0, 1.0);
        let tuple_2 = Tuple::new_point(5.0, 6.0, 7.0);

        let tuple_3 = tuple_1 - tuple_2; // tuple_1.subtract(&tuple_2);

        assert_eq!(tuple_3, Tuple { x: -2.0, y: -4.0, z: -6.0, w: 0.0});
        assert_eq!(tuple_3.get_type(), TupleType::Vector);
    }

    #[test]
    fn test_subtract_point_and_vector() {
        let tuple_1 = Tuple::new_point(3.0, 2.0, 1.0);
        let tuple_2 = Tuple::new_vector(5.0, 6.0, 7.0);

        let tuple_3 = tuple_1 - tuple_2; // tuple_1.subtract(&tuple_2);

        assert_eq!(tuple_3, Tuple { x: -2.0, y: -4.0, z: -6.0, w: 1.0});
        assert_eq!(tuple_3.get_type(), TupleType::Point);
    }

     #[test]
    fn test_subtract_two_vectors() {        
        let tuple_1 = Tuple::new_vector(3.0, 2.0, 1.0);
        let tuple_2 = Tuple::new_vector(5.0, 6.0, 7.0);

        let tuple_3 = tuple_1 - tuple_2; // tuple_1.subtract(&tuple_2);

        assert_eq!(tuple_3, Tuple { x: -2.0, y: -4.0, z: -6.0, w: 0.0});
        assert_eq!(tuple_3.get_type(), TupleType::Vector);
    }

    #[test]
    fn test_subtract_vector_from_zero_vector() {
        let tuple_1 = Tuple::new_vector(0.0, 0.0, 0.0);
        let tuple_2 = Tuple::new_vector(1.0, -2.0, 3.0);

        let tuple_3 = tuple_1 - tuple_2; // tuple_1.subtract(&tuple_2);

        assert_eq!(tuple_3, Tuple { x: -1.0, y: 2.0, z: -3.0, w: 0.0});
        assert_eq!(tuple_3.get_type(), TupleType::Vector);
    }

    #[test]
    fn test_negate_tuple() {
        let tuple_1 = Tuple::new_point(-2.0, 3.0, -1.2);
        let tuple_2 = tuple_1.negate();

        assert_eq!(tuple_2, Tuple::new_point(2.0, -3.0, 1.2));
        assert_eq!(tuple_2.get_type(), TupleType::Point);
    }

    #[test]
    fn test_multiply_by_scalar() {
        let tuple_1 = Tuple::new_tuple(1.0, -2.0, 3.0, -4.0);
        let tuple_multiplied = tuple_1 * 3.5_f64;
        assert_eq!(tuple_multiplied, Tuple { x: 3.5, y: -7.0, z: 10.5, w: -14.0 });
    }

    #[test]
    fn test_divide_by_scalar() {
        let tuple_1 = Tuple::new_tuple(1.0, -2.0, 3.0, -4.0);
        let tuple_divided = tuple_1 / 2_f64; // tuple_1.divide_by(2.);
        assert_eq!(tuple_divided, Tuple { x: 0.5, y: -1.0, z: 1.5, w: -2.0 });
    }

    #[test]
    fn test_vector_magnitude() {
        let vector = Tuple::new_vector(0., 1., 0.);
        assert_eq!(vector.get_magnitude(), 1.);

        let vector = Tuple::new_vector(1., 2., 3.);
        let result = 14_f64;
        assert_eq!(vector.get_magnitude(), result.sqrt());
    }

    #[test]
    fn test_normalize() {
        let vector = Tuple::new_vector(4., 0., 0.);
        let vector_normalized = vector.normalize();
        assert_eq!(vector_normalized, Tuple::new_vector(1., 0., 0.));

        let vector = Tuple::new_vector(1., 2., 3.);
        let vector_normalized = vector.normalize();
        assert_eq!(vector_normalized.is_equal_to(&Tuple::new_vector(0.26726, 0.53452, 0.80178)), true);
    }

    #[test]
    fn test_calculate_dot_product() {
        let vec_1 = Tuple::new_vector(1., 2., 3.);
        let vec_2 = Tuple::new_vector(2., 3., 4.);
        assert_eq!(vec_1.calculate_dot_product(&vec_2), 20.);
    }

    #[test]
    fn test_calculate_vector_cross_product() {
        let vec_1 = Tuple::new_vector(1., 2., 3.);
        let vec_2 = Tuple::new_vector(2., 3., 4.);
        assert_eq!(vec_1.get_vector_cross_product(&vec_2), Tuple::new_vector(-1., 2., -1.));
        assert_eq!(vec_2.get_vector_cross_product(&vec_1), Tuple::new_vector(1., -2., 1.));
    }

}