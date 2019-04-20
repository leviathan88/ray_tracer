pub struct NumberUtils {}

impl NumberUtils {
    pub fn compare_floats(a: f64, b: f64) -> bool {
        f64::abs(a-b) < 0.00001
    }

    pub fn compare_floats_32(a: f32, b: f32) -> bool {
        f32::abs(a-b) < 0.00001
    }
}

mod tests {

    use super::*;

    #[test]
    fn test_compare_floats() {
        assert_eq!(NumberUtils::compare_floats(0.1, 0.1), true);
        assert_eq!(NumberUtils::compare_floats(0.001, 0.001), true);              
        assert_eq!(NumberUtils::compare_floats(0.01, 0.01), true);

        assert_eq!(NumberUtils::compare_floats(0.01, 0.001), false);
        assert_eq!(NumberUtils::compare_floats(0.0001, 0.001), false);
    }

    #[test]
    fn test_compare_floats_with_way_too_small_numbers() {
        assert_eq!(NumberUtils::compare_floats(0.000000000001, 0.00001), true);  
    }
}