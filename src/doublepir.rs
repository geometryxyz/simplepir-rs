use crate::matrix::Matrix;
use crate::element::Element;
use crate::regev::gen_error_vec;

pub struct DoublePIRParams {
    // Public A matrices
    pub a_1: Matrix,
    pub a_2: Matrix,

    // The integer modulus
    pub q: u64,

    // The plaintext modulus
    pub p: u64,

    // The LWE secret length
    pub n: usize,

    // The number of columns of the database
    pub l: usize,

    // The number of rows of the database
    pub m: usize,

    // The standard deviation for sampling random elements
    pub std_dev: f64,
}

pub fn gen_params() -> DoublePIRParams {
    // Database size: l x m
    let l = 4;
    let m = 8;
    let n = 32;
    let p = 2;
    let q = 3329;
    let std_dev = 6.4;
    // rows, cols - TODO: change gen_uniform_rand
    let a_1 = Matrix::gen_uniform_rand(q, m, n);
    let a_2 = Matrix::gen_uniform_rand(q, l, n);
    DoublePIRParams { a_1, a_2, q, l, p, n, m, std_dev }
}

pub fn gen_db(params: &DoublePIRParams) -> Matrix {
    Matrix::gen_uniform_rand(
        params.p,
        params.l,
        params.m,
    )
}

pub fn gen_hints(params: &DoublePIRParams, db: &Matrix) -> (Matrix, Matrix) {
    let mut db_q = db.clone();
    db_q.change_q(params.q);

    // hint_s = A transposed * db transposed
    let mut hint_s =  params.a_1.to_owned().rotated() * db_q.to_owned().rotated();
    hint_s = hint_s.decomposed(params.p);

    // hint_c = hint_s * A_2
    let hint_c = hint_s.to_owned() * params.a_2.to_owned();

    (hint_s, hint_c)
}

pub fn query(
    params: &DoublePIRParams,
    col_i: usize,
    row_i: usize,
    s_1: &Vec<Element>,
    s_2: &Vec<Element>,
) -> (Matrix, Matrix) {
    assert!(row_i < params.m);
    assert!(col_i < params.l);

    // q / p
    let floor = Element::from(params.q, params.q / params.p);

    // Generate error vectors
    let e_1 = Matrix::from_col(&gen_error_vec(params.q, params.m));
    let e_2 = Matrix::from_col(&gen_error_vec(params.q, params.l));

    // Compute c_1 = A_1 * s_1 + e_1 + floor * u_i_row
    // NOTE: perhaps due to a bug in our Matrix implementation, row and col are reversed. As such,
    // while the paper notes that c_1 contains floor at u_i_col, we instead use row_i.
    let mut c_1 = params.a_1.to_owned().mul_vec(s_1) + e_1.rotated();
    c_1[row_i][0] += floor.clone();

    assert_eq!(c_1.num_cols(), params.m);

    // Compute c_2 = A_2 * s_2 + e_2 + floor * u_i_col
    // NOTE: same bug as above
    let mut c_2 = params.a_2.to_owned().mul_vec(s_2) + e_2.rotated();
    c_2[col_i][0] += floor;
    assert_eq!(c_2.num_cols(), params.l);

    (c_1, c_2)
}

pub fn answer(
    params: &DoublePIRParams,
    db: &Matrix,
    hint_s: &Matrix,
    query: &(Matrix, Matrix),
) -> (Matrix, Matrix) {
    let k = ((params.q - 1) as f64).log(params.p as f64).ceil() as usize;
    let c_1 = query.to_owned().0;
    let c_2 = query.to_owned().1;

    let mut db_q = db.clone();
    db_q.change_q(params.q);
    let ans_1 = (c_1.rotated() * db_q.rotated()).decomposed(params.p);

    assert_eq!(ans_1.num_cols(), k);
    assert_eq!(ans_1.num_rows(), params.l);

    let h = ans_1.to_owned() * params.a_2.to_owned();

    // (ans_h || ans_2) = (hint_s || ans_1) * c_2
    let mut hint_s_ans_1 = hint_s.clone();
    for col in ans_1.data {
        hint_s_ans_1.append_col(col.clone());
    }

    let ans_h_ans_2 = hint_s_ans_1 * c_2;

    // Check the dimensions of the answer
    assert_eq!(h.num_cols(), k);
    assert_eq!(h.num_rows(), params.n);

    assert_eq!(ans_h_ans_2.num_cols(), k * (params.n + 1));
    assert_eq!(ans_h_ans_2.num_rows(), 1);

    (h, ans_h_ans_2)
}

pub fn recover(
    params: &DoublePIRParams,
    hint_c: &Matrix,
    answer: &(Matrix, Matrix),
    s_1: &Vec<Element>,
    s_2: &Vec<Element>,
) -> Element {
    let k = ((params.q - 1) as f64).log(params.p as f64).ceil() as usize;
    let p = params.p;
    let q = params.q as f64;
    let h = answer.to_owned().0;
    let ans_h_ans_2 = answer.to_owned().1;

    assert_eq!(hint_c.num_cols(), k * params.n);
    assert_eq!(hint_c.num_rows(), params.n);

    assert_eq!(h.num_cols(), k);
    assert_eq!(h.num_rows(), params.n);

    // hint_c_h =  hint_c || h
    let mut hint_c_h = hint_c.to_owned();
    for i in 0..h.num_cols() {
        hint_c_h.append_col(h[i].to_owned());
    }

    // hint_c_h * s_2
    let hhs = hint_c_h.mul_vec(s_2);
    assert_eq!(hhs.num_cols(), k * (params.n + 1));
    assert_eq!(hhs.num_rows(), 1);

    // h1_hat_a1_hat = (ans_h || ans_2) - (hint_c || h) *  s_2
    let mut h1_hat_a1_hat = ans_h_ans_2 - hhs;

    assert_eq!(h1_hat_a1_hat.num_cols(), k * (params.n + 1));
    assert_eq!(h1_hat_a1_hat.num_rows(), 1);

    for i in 0..h1_hat_a1_hat.num_cols() {
        for j in 0..h1_hat_a1_hat.num_rows() {
            h1_hat_a1_hat[i][j].uint =
                ((h1_hat_a1_hat[i][j].uint * p) as f64 / q).round() as u64 % p;
        }
    }

    // ans_h: k x n
    // ans_2: k x 1
    // ans_h_ans_2: k(n+1) x 1
    // hint_c: kn x n
    // h: k x n
    // hint_c_h: k(n+1) x n
    // s2: n x 1
    // h1_hat_a1_hat: (k(n+1) x 1) - (k(n+1) x 1)
    let h1_a1 = h1_hat_a1_hat.recompose(p, params.q);

    let mut h_1 = Vec::with_capacity(params.n);
    for i in 0..params.n {
        h_1.push(h1_a1[i].clone());
    }
    let h_1 = Matrix::from(&h_1);
    let a_1 = Matrix::from_col(&h1_a1[params.n]);

    let d_hat = a_1 - Matrix::from_col(s_1) * h_1;

    let d = ((d_hat[0][0].uint * p) as f64 / q).round() as u64 % p;
    Element::from (params.p, d)
}

#[cfg(test)]
mod tests {
    use crate::regev::gen_secret;
    use super::*;

    #[test]
    pub fn test_doublepir() {
        let params = gen_params();
        let db = gen_db(&params);

        for i in 0..db.num_cols() {
            for j in 0..db.num_rows() {
                test_doublepir_impl(&params, &db, i, j);
            }
        }
    }

    pub fn test_doublepir_impl(params: &DoublePIRParams, db: &Matrix, col: usize, row: usize) {
        // Generate (hint_s, hint_c)
        let hints = gen_hints(&params, &db);

        let s_1 = gen_secret(params.q, params.n);
        let s_2 = gen_secret(params.q, params.n);

        let query = query(&params, col, row, &s_1, &s_2);

        let answer = answer(&params, &db, &hints.0, &query);

        let recovered = recover(&params, &hints.1, &answer, &s_1, &s_2);
        assert_eq!(recovered, db[col][row]);
    }
}
