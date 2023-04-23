use crate::matrix::Matrix;
use crate::element::Element;

pub fn gen_random_normal_matrix(
    q: u64,
    std_dev: f64,
    num_rows: usize,
    num_cols: usize,
) -> Matrix<Element> {
    let v: Vec<Vec<Element>> = vec![
        vec![Element::new(q); num_rows];
        num_cols
    ];

    let mut matrix = Matrix::<Element>::from(v);

    for i in 0..num_cols {
        for j in 0..num_rows {
            matrix[i][j] = Element::gen_normal_rand(q, std_dev);
        }
    }
    matrix
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_gen_random_normal_matrix() {
        let num_rows = 9;
        let num_cols = 10;
        let matrix = gen_random_normal_matrix(101u64, 6.4 as f64, num_rows, num_cols);
        assert_eq!(matrix.num_rows(), num_rows);
        assert_eq!(matrix.num_cols(), num_cols);
    }
}
