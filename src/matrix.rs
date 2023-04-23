use num::traits::identities::Zero;
use std::clone::Clone;
use std::default::Default;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Sub, SubAssign, Index, IndexMut, Mul, MulAssign};

#[derive(Debug, PartialEq)]
pub struct Matrix<T> {
    pub data: Vec<Vec<T>>,
}

impl<T> Index<usize> for Matrix<T> {
    type Output = Vec<T>;
    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, i: usize) -> &mut Vec<T> {
        &mut self.data[i]
    }
}

impl<T: Clone> Clone for Matrix<T> {
    fn clone(&self) -> Self {
        if self.data.is_empty() {
            return Matrix { data: vec![] };
        }
        let mut d: Vec<Vec<T>> = Vec::with_capacity(self.data.len());
        for i in 0..self.data.len() {
            let mut e = Vec::<T>::with_capacity(self.data[i].len());
            for j in 0..self.data[i].len() {
                e.push(self.data[i][j].clone());
            }
            d.push(e);
        }
        Matrix::<T> { data: d }
    }

    fn clone_from(&mut self, source: &Self) {
        let c = source.clone();
        *self = c;
    }
}

impl<T: Clone + Zero + Mul + Add + AddAssign + Sub + Mul<Output = T>> Default for Matrix<T> {
    fn default() -> Matrix<T> {
        Matrix::new()
    }
}

impl<T: Clone + Zero + Mul + Add + AddAssign + Sub + Mul<Output = T>> Matrix<T> {
    /// Initialise an empty matrix (0 by 0)
    pub fn new() -> Matrix<T> {
        Matrix { data: vec![] }
    }

    /// Initialise a matrix from Vectors of Vectors of elements
    pub fn from(data: Vec<Vec<T>>) -> Self {
        let mut matrix = Self::new();
        matrix.data = data;
        matrix
    }

    pub fn push(&mut self, row: Vec<T>) {
        self.data.push(row);
    }

    pub fn num_vals(&self) -> usize {
        if !self.data.is_empty() {
            return self.data.len() * self.data[0].len();
        }
        0
    }

    #[allow(clippy::needless_range_loop)]
    pub fn rotated(self) -> Self {
        let zero = T::zero();
        let new_row = vec![zero; self.num_cols()];
        let mut rotated = vec![new_row; self.num_rows()];
        for i in 0..self.num_cols() {
            for j in 0..self.num_rows() {
                rotated[j][i] = self.data[i][j].clone();
            }
        }
        Self::from(rotated)
    }

    pub fn mul_vec(self, rhs: Vec<T>) -> Self {
        let rhs_matrix = Self::from(vec![rhs]).rotated();
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

impl<T: Clone + Zero + Mul + Add + AddAssign + Sub + Mul<Output = T>> Add for Matrix<T> {
    type Output = Matrix<T>;
    #[allow(clippy::needless_range_loop)]
    fn add(self, rhs: Matrix<T>) -> Self::Output {
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

impl<T: Clone + Zero + Mul + Add + AddAssign + Sub + Mul<Output = T>> AddAssign for Matrix<T> {
    fn add_assign(&mut self, rhs: Matrix<T>) {
        assert_eq!(self.num_rows(), rhs.num_rows());
        assert_eq!(self.num_cols(), rhs.num_cols());
        for i in 0..self.num_cols() {
            for j in 0..self.num_rows() {
                self.data[i][j] += rhs.data[i][j].clone();
            }
        }
    }
}

impl<T: Clone + Zero + Mul + Add + AddAssign + Sub + Mul<Output = T> + Sub<Output = T>> Sub for Matrix<T> {
    type Output = Matrix<T>;
    #[allow(clippy::needless_range_loop)]
    fn sub(self, rhs: Matrix<T>) -> Self::Output {
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

impl<T: Clone + Zero + Mul + Add + AddAssign + Sub + Mul<Output = T> + Sub<Output = T>> SubAssign for Matrix<T> {
    fn sub_assign(&mut self, rhs: Matrix<T>) {
        assert_eq!(self.num_rows(), rhs.num_rows());
        assert_eq!(self.num_cols(), rhs.num_cols());
        for i in 0..self.num_cols() {
            for j in 0..self.num_rows() {
                self.data[i][j] = self.data[i][j].clone() - rhs.data[i][j].clone();
            }
        }
    }
}

impl<T: Clone + Zero + Mul + Add + AddAssign + Sub + Mul<Output = T>> Mul for Matrix<T> {
    /*
     * [a00, a01, a02] [b00, b01] = [a00b00 + a01b10 + a02b20, a00b01 + a01b11 + a02b21]
     * [a10, a11, a12] [b10, b11]   [a10b00 + a11b10 * a12b20, a10b01 + a11b11 * a12b21]
     *                 [b20, b21]
     */
    type Output = Matrix<T>;
    #[allow(clippy::needless_range_loop)]
    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        // Ensure that the rhs matrix has the correct dimensions
        assert_eq!(self.num_rows(), rhs.num_cols());

        // Assign a result matrix of the required dimensions with 0s in each cell
        let zero = T::zero();
        let n = self.num_cols();
        let m = self.num_rows(); // = rhs.num_cols()
        let p = rhs.num_rows();

        let each_result_row = vec![zero; p];
        let mut result: Vec<Vec<T>> = vec![each_result_row; n];

        for i in 0..n {
            for j in 0..p {
                let mut sum = T::zero();
                for k in 0..m {
                    sum += self.data[i][k].clone() * rhs[k][j].clone();
                }
                result[i][j] = sum;
            }
        }

        Self::from(result)
    }
}

impl<T: Clone + Zero + Mul + Add + AddAssign + Sub + Mul<Output = T>> MulAssign for Matrix<T> {
    fn mul_assign(&mut self, rhs: Matrix<T>) {
        let mut s = self.clone();
        s = s * rhs;
        *self = s;
    }
}

impl<T: Display> Display for Matrix<T> {
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
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;

    // Tests for matrix.rs
    fn gen_matrix_3_2() -> Matrix<u64> {
        // 3 rows, 2 cols
        Matrix::<u64>::from(
            vec![
                vec![1u64, 2u64, 3u64],
                vec![4u64, 5u64, 6u64],
            ]
        )
    }

    fn gen_matrix_2_3() -> Matrix<u64> {
        // 2 rows, 3 cols
        Matrix::<u64>::from(
            vec![
                vec![1u64, 4u64],
                vec![2u64, 5u64],
                vec![3u64, 6u64],
            ]
        )
    }

    fn gen_matrix_2_2() -> Matrix<u64> {
        // 2 rows, 2 cols
        Matrix::<u64>::from(
            vec![
                vec![14u64, 32u64],
                vec![32u64, 77u64],
            ]
        )
    }

    fn gen_matrix_1_2() -> Matrix<u64> {
        // 1 rows, 2 cols
        Matrix::<u64>::from(
            vec![
                vec![14u64], vec![32u64]
            ]
        )
    }

    fn gen_vec_3() -> Vec<u64> {
        vec![1u64, 2u64, 3u64]
    }

    #[test]
    fn test_indices() {
        let mut m = gen_matrix_3_2();
        assert_eq!(m[0][0], 1u64);
        assert_eq!(m[0][1], 2u64);
        assert_eq!(m[1][1], 5u64);

        m[1][1] = 0u64;
        assert_eq!(m[1][1], 0u64);
    }

    #[test]
    fn test_rotation() {
        let m = gen_matrix_3_2();
        let n = gen_matrix_2_3();
        assert_eq!(m.rotated(), n);
    }

    #[test]
    fn test_mul() {
        let m = gen_matrix_3_2();
        let n = gen_matrix_2_3();
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
        assert_eq!(m.mul_vec(v), r);
    }

    #[test]
    fn test_add() {
        let m = gen_matrix_3_2();
        let n = gen_matrix_3_2();
        let o = m.clone() + n.clone();
        
        for (i, row) in o.data.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                assert_eq!(*val, &m[i][j] + &n[i][j]);
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
                assert_eq!(*val, &o[i][j] + &n[i][j]);
            }
        }
    }
}
