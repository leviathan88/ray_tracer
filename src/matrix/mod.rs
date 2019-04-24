use crate::utils::NumberUtils;

type MatrixType = Vec<Vec<f64>>;

pub struct Matrix;

impl Matrix {
    pub fn construct_empty_4x4() -> MatrixType {
        let inner = vec![0., 0., 0., 0.];
        vec![inner.clone(), inner.clone(), inner.clone(), inner.clone()]
    }

    pub fn construct_empty_3x3() -> MatrixType {
        let inner = vec![0., 0., 0.];
        vec![inner.clone(), inner.clone(), inner.clone()]
    }

    pub fn construct_empty_2x2() -> MatrixType {
        let inner = vec![0., 0.];
        vec![inner.clone(), inner.clone()]
    }

    pub fn are_equal(m1: &MatrixType, m2: &MatrixType) -> bool {
        if m1.len() != m2.len() {
            return false
        }

        if m1[0].len() != m2[0].len() {
            return false
        }

        for i in 0..m1.len() {
            for j in 0..m1.len() {
                if !NumberUtils::compare_floats(m1[i][j], m2[i][j]) {
                    return false
                }
            }
        }

        true
    }
}

mod tests {

    use super::*;

    #[test]
    fn test_matrix_4x4() {
        let m = Matrix::construct_empty_4x4();

        assert_eq!(m[0][0], 0.0);
        assert_eq!(m[3][3], 0.0);
    }

    #[test]
    fn test_matrix_equality() {
        let mut m1 = Matrix::construct_empty_4x4();
        let m2 = Matrix::construct_empty_4x4();
        let m3 = Matrix::construct_empty_3x3();

        assert_eq!(Matrix::are_equal(&m1, &m2), true);
        assert_eq!(Matrix::are_equal(&m2, &m3), false);
        
        m1[2][3] = 15.34;

        assert_eq!(Matrix::are_equal(&m1, &m2), false);

    }

}