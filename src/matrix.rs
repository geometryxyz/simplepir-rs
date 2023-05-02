use crate::element::Element;
use std::clone::Clone;
use std::default::Default;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Sub, SubAssign, Index, IndexMut, Mul, MulAssign};

#[derive(Debug, PartialEq)]
pub struct Matrix {
    pub data: Vec<Vec<Element>>,
}

impl Index<usize> for Matrix {
    type Output = Vec<Element>;
    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, i: usize) -> &mut Vec<Element> {
        &mut self.data[i]
    }
}

impl Clone for Matrix {
    fn clone(&self) -> Self {
        if self.data.is_empty() {
            return Matrix { data: vec![] };
        }
        let mut d: Vec<Vec<Element>> = Vec::with_capacity(self.data.len());
        for i in 0..self.data.len() {
            let mut e = Vec::<Element>::with_capacity(self.data[i].len());
            for j in 0..self.data[i].len() {
                e.push(self.data[i][j].clone());
            }
            d.push(e);
        }
        Matrix { data: d }
    }

    fn clone_from(&mut self, source: &Self) {
        let c = source.clone();
        *self = c;
    }
}

impl Default for Matrix {
    fn default() -> Matrix {
        Matrix::new()
    }
}

impl Matrix {
    /// Initialise an empty matrix (0 by 0)
    pub fn new() -> Matrix {
        Matrix { data: vec![] }
    }

    /// Initialise a matrix from Vectors of Vectors of elements
    pub fn from(data: &Vec<Vec<Element>>) -> Self {
        let mut matrix = Self::new();
        matrix.data = data.to_owned();
        matrix
    }

    pub fn from_single(elem: &Element) -> Self {
        Self::from(&vec![vec![elem.clone()]])
    }

    pub fn from_col(col: &Vec<Element>) -> Self {
        Self::from(&vec![col.to_owned()])
    }

    pub fn from_val(rows: usize, cols: usize, val: Element) -> Self {
        let row = vec![val; cols];
        let cols = vec![row; rows];
        Matrix::from(&cols)
    }

    pub fn gen_uniform_rand(q: u64, rows: usize, cols: usize) -> Self  {
        let mut a = Vec::with_capacity(cols);
        for _ in 0..cols {
            let mut row = Vec::with_capacity(rows);
            for _ in 0..rows {
                row.push(Element::gen_uniform_rand(q));
            }
            a.push(row);
        }
        Matrix::from(&a)
    }

    pub fn push(&mut self, row: Vec<Element>) {
        self.data.push(row);
    }

    pub fn change_q(&mut self, new_q: u64) {
        for i in 0..self.num_cols() {
            for j in 0..self.num_rows() {
                self[i][j].q = new_q;
            }
        }
    }

    pub fn num_vals(&self) -> usize {
        if !self.data.is_empty() {
            return self.data.len() * self.data[0].len();
        }
        0
    }

    #[allow(clippy::needless_range_loop)]
    pub fn rotated(self) -> Self {
        let zero = Element::zero(self.data[0][0].q);
        let new_row = vec![zero; self.num_cols()];
        let mut rotated = vec![new_row; self.num_rows()];
        for i in 0..self.num_cols() {
            for j in 0..self.num_rows() {
                rotated[j][i] = self.data[i][j].clone();
            }
        }
        Self::from(&rotated)
    }

    pub fn mul_elem(self, rhs: &Element) -> Self {
        let mut r = self.clone();
        for i in 0..self.num_cols() {
            for j in 0..self.num_rows() {
                r[i][j] *= rhs.to_owned();
            }
        }

        r
    }

    pub fn mul_vec(self, rhs: &Vec<Element>) -> Self {
        let rhs_matrix = Self::from(&vec![rhs.to_owned()]).rotated();
        self.mul(rhs_matrix)
    }

    pub fn num_rows(&self) -> usize {
        self.data[0].len()
    }

    pub fn num_cols(&self) -> usize {
        self.data.len()
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.num_rows(), self.num_cols())
    }
}

impl Add for Matrix {
    type Output = Matrix;
    #[allow(clippy::needless_range_loop)]
    fn add(self, rhs: Matrix) -> Self::Output {
        assert_eq!(self.num_rows(), rhs.num_rows());
        assert_eq!(self.num_cols(), rhs.num_cols());

        let mut s = self.clone();

        for i in 0..self.num_cols() {
            for j in 0..self.num_rows() {
                s.data[i][j] = s.data[i][j].clone() + rhs.data[i][j].clone();
            }
        }
        s
    }
}

impl AddAssign for Matrix {
    fn add_assign(&mut self, rhs: Matrix) {
        assert_eq!(self.num_rows(), rhs.num_rows());
        assert_eq!(self.num_cols(), rhs.num_cols());
        for i in 0..self.num_cols() {
            for j in 0..self.num_rows() {
                self.data[i][j] += rhs.data[i][j].clone();
            }
        }
    }
}

impl Sub for Matrix {
    type Output = Matrix;
    #[allow(clippy::needless_range_loop)]
    fn sub(self, rhs: Matrix) -> Self::Output {
        assert_eq!(self.num_rows(), rhs.num_rows());
        assert_eq!(self.num_cols(), rhs.num_cols());

        let mut s = self.clone();

        for i in 0..self.num_cols() {
            for j in 0..self.num_rows() {
                s.data[i][j] = s.data[i][j].clone() - rhs.data[i][j].clone();
            }
        }
        s
    }
}

impl SubAssign for Matrix {
    fn sub_assign(&mut self, rhs: Matrix) {
        assert_eq!(self.num_rows(), rhs.num_rows());
        assert_eq!(self.num_cols(), rhs.num_cols());
        for i in 0..self.num_cols() {
            for j in 0..self.num_rows() {
                self.data[i][j] = self.data[i][j].clone() - rhs.data[i][j].clone();
            }
        }
    }
}

impl Mul for Matrix {
    /*
     * [a00, a01, a02] [b00, b01] = [a00b00 + a01b10 + a02b20, a00b01 + a01b11 + a02b21]
     * [a10, a11, a12] [b10, b11]   [a10b00 + a11b10 * a12b20, a10b01 + a11b11 * a12b21]
     *                 [b20, b21]
     */
    type Output = Matrix;
    #[allow(clippy::needless_range_loop)]
    fn mul(self, rhs: Matrix) -> Self::Output {
        // Ensure that the rhs matrix has the correct dimensions
        assert_eq!(self.num_rows(), rhs.num_cols());

        // Assign a result matrix of the required dimensions with 0s in each cell
        let zero = Element::zero(self.data[0][0].q);
        let n = self.num_cols();
        let m = self.num_rows(); // = rhs.num_cols()
        let p = rhs.num_rows();

        let each_result_row = vec![zero.clone(); p];
        let mut result: Vec<Vec<Element>> = vec![each_result_row; n];

        for i in 0..n {
            for j in 0..p {
                let mut sum = zero.clone();
                for k in 0..m {
                    sum += self.data[i][k].clone() * rhs[k][j].clone();
                }
                result[i][j] = sum;
            }
        }

        Self::from(&result)
    }
}

impl MulAssign for Matrix {
    fn mul_assign(&mut self, rhs: Matrix) {
        let mut s = self.clone();
        s = s * rhs;
        *self = s;
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.data.iter() {
            write!(f, "[")?;
            for (j, val) in row.iter().enumerate() {
                write!(f, "{}", val)?;
                if j != row.len() - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, "]")?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::Matrix;
    use super::Element;

    fn gen_q() -> u64 {
        return 101u64;
    }

    // Tests for matrix.rs
    fn gen_matrix_3_2() -> Matrix {
        let q = gen_q();
        // 3 rows, 2 cols
        Matrix::from(
            &vec![
                vec![Element::from(q, 1u64), Element::from(q, 2u64), Element::from(q, 3u64)],
                vec![Element::from(q, 4u64), Element::from(q, 5u64), Element::from(q, 6u64)],
            ]
        )
    }

    fn gen_matrix_2_3() -> Matrix {
        let q = gen_q();
        // 2 rows, 3 cols
        Matrix::from(
            &vec![
                vec![Element::from(q, 1u64), Element::from(q, 4u64)],
                vec![Element::from(q, 2u64), Element::from(q, 5u64)],
                vec![Element::from(q, 3u64), Element::from(q, 6u64)],
            ]
        )
    }

    fn gen_matrix_2_2() -> Matrix {
        let q = gen_q();
        // 2 rows, 2 cols
        Matrix::from(
            &vec![
                vec![Element::from(q, 14u64), Element::from(q, 32u64)],
                vec![Element::from(q, 32u64), Element::from(q, 77u64)],
            ]
        )
    }

    fn gen_matrix_1_2() -> Matrix {
        let q = gen_q();
        // 1 rows, 2 cols
        Matrix::from(
            &vec![
                vec![Element::from(q, 14u64)], vec![Element::from(q, 32u64)]
            ]
        )
    }

    fn gen_vec_3() -> Vec<Element> {
        let q = gen_q();
        vec![
            Element::from(q, 1u64),
            Element::from(q, 2u64),
            Element::from(q, 3u64),
        ]
    }

    #[test]
    fn test_indices() {
        let q = gen_q();
        let mut m = gen_matrix_3_2();
        assert_eq!(m[0][0], Element::from(q, 1u64));
        assert_eq!(m[0][1], Element::from(q, 2u64));
        assert_eq!(m[1][1], Element::from(q, 5u64));

        m[1][1] = Element::from(q, 0u64);
        assert_eq!(m[1][1], Element::from(q, 0u64));
    }

    #[test]
    fn test_rotation() {
        let m = gen_matrix_3_2();
        let n = gen_matrix_2_3();
        assert_eq!(m.rotated(), n);
    }

    #[test]
    fn test_mul() {
        // 3 rows, 2 cols
        let m = gen_matrix_3_2();
        // 2 rows, 3 cols
        let n = gen_matrix_2_3();
        // Should have 3 rows and 3 columns
        let o = gen_matrix_2_2();
        assert_eq!(m * n, o);
    }

    #[test]
    fn test_mul_assign() {
        let mut m = gen_matrix_3_2();
        let n = gen_matrix_2_3();
        let o = gen_matrix_2_2();
        m *= n;
        assert_eq!(m, o);
    }

    #[test]
    fn test_mul_vec() {
        let m = gen_matrix_3_2();
        let v = gen_vec_3();
        let r = gen_matrix_1_2();
        assert_eq!(m.mul_vec(&v), r);
    }

    #[test]
    fn test_add() {
        let m = gen_matrix_3_2();
        let n = gen_matrix_3_2();
        let o = m.clone() + n.clone();
        
        for (i, row) in o.data.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                assert_eq!(*val, m[i][j].clone() + n[i][j].clone());
            }
        }
    }

    #[test]
    fn test_add_assign() {
        let mut m = gen_matrix_3_2();
        let n = gen_matrix_3_2();
        let o = m.clone();
        m += n.clone();
        
        for (i, row) in m.data.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                assert_eq!(*val, o[i][j].clone() + n[i][j].clone());
            }
        }
    }
}
