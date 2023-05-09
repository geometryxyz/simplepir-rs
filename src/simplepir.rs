use crate::matrix::Matrix;
use crate::element::Element;
use crate::regev::gen_error_vec;

pub struct SimplePIRParams {
    // Public A matrix
    pub a: Matrix,
    // The integer modulus
    pub q: u64,
    // The plaintext modulus
    pub p: u64,
    // The LWE secret length
    pub n: usize,
    // The number of samples or the width of the (square) database
    pub m: usize,
    // The standard deviation for sampling random elements
    pub std_dev: f64,
}

pub fn gen_params() -> SimplePIRParams {
    let m = 8;
    let n = 64;
    let q = 3329;
    let p = 2;
    let std_dev = 6.4;
    let a = Matrix::gen_uniform_rand(q, n, m);

    SimplePIRParams { a, q, p, n, m, std_dev }
}

/// Generate a database of random values mod the plaintext modulus p
pub fn gen_db(params: &SimplePIRParams) -> Matrix {
    Matrix::gen_uniform_rand(
        params.p,
        params.m,
        params.m,
    )
}

/// Generates the client's hint, which is the database multiplied by A. Also known as the setup.
pub fn gen_hint(params: &SimplePIRParams, db: &Matrix) -> Matrix {
    let mut db_q = db.clone();
    db_q.change_q(params.q);
    db_q.to_owned() * params.a.to_owned()
}

/// Generate a query to be sent to the server.
pub fn query(
    params: &SimplePIRParams,
    idx: usize,
    s: &Vec<Element>,
) -> Vec<Element> {
    let db_size = params.m;
    assert!(idx < db_size);
    // q / p
    let floor = params.q / params.p;

    // The error term
    let e = gen_error_vec(params.q, params.m);
    let err_matrix = Matrix::from_col(&e);

    // query = A * s + e + q/p * u_i_col
    let mut query = params.a.to_owned().mul_vec(s);
    query += err_matrix.rotated();

    // Add q/p * 1 only to the index corresponding to the desired column
    query[idx][0] += Element::from(params.q, floor);

    query.rotated()[0].to_owned()
}

pub fn answer(query: &Vec<Element>, db: &Matrix) -> 
    Matrix
{
    let mut db_q = db.clone();
    db_q.change_q(query[0].q);
    db_q.to_owned().mul_vec(query)
}

pub fn recover_row(
    params: &SimplePIRParams,
    s: &Vec<Element>,
    hint: &Matrix,
    answer: &Matrix,
) -> Vec<Element> {
    let p = params.p;
    let q = params.q as f64;

    let interim = hint.to_owned().mul_vec(s);
    let mut ans = answer.to_owned();
    ans -= interim;

    ans.data.iter().map(
        |v| Element::from(p, ((v[0].uint * p) as f64 / q).round() as u64 % p)
    ).collect()
}

pub fn recover(
    params: &SimplePIRParams,
    s: &Vec<Element>,
    idx: usize,
    hint: &Matrix,
    answer: &Matrix,
) -> Element {
    let p = params.p;
    let q = params.q as f64;

    let interim = hint.to_owned().mul_vec(s);
    let mut ans = answer.to_owned();
    ans -= interim;

    let x = ((ans[idx][0].uint * p) as f64 / q).round() as u64 % p;
    Element::from(p, x)
}

#[cfg(test)]
mod tests {
    use crate::regev::gen_secret;
    use super::*;

    fn test_simplepir_impl(desired_col: usize, desired_row: usize) {
        let params = gen_params();
        let db = gen_db(&params);

        let db_item = &db[desired_col][desired_row];

        let secret = gen_secret(params.q, params.n);
        let hint = gen_hint(&params, &db);

        let query = query(&params, desired_row, &secret);
        let answer = answer(&query, &db);
        let recovered_item = recover(&params, &secret, desired_col, &hint, &answer);
        assert_eq!(recovered_item, *db_item);

        let recovered_row = recover_row(&params, &secret, &hint, &answer);
        assert_eq!(recovered_row, db.rotated()[desired_row]);
    }

    #[test]
    pub fn test_simplepir() {
        for i in 0..8 {
            for j in 0..8 {
                test_simplepir_impl(i, j);
            }
        }
    }

    fn test_simplepir_updates_impl(desired_col: usize, desired_row: usize) {
        let params = gen_params();
        let db = gen_db(&params);
        let hint = gen_hint(&params, &db);

        let secret = gen_secret(params.q, params.n);

        let query = query(&params, desired_row, &secret);
        let ans = answer(&query, &db);
        let recovered = recover(&params, &secret, desired_col, &hint, &ans);

        let db_item = &db[desired_col][desired_row];
        assert_eq!(recovered, *db_item);

        // Flip all bits of one row
        let row_to_flip = desired_col;
        let mut db = db.clone();

        let mut updated_row = Vec::with_capacity(params.m);
        for i in 0..db.num_rows() {
            // Flip the bits in the row
            db[row_to_flip][i] -= Element::from(params.p, 1);
            updated_row.push(
                Element::from(params.q, db[row_to_flip][i].uint)
            );
        }
        db.change_q(params.q);

        // Now update the hint
        let mut hint = hint.clone();

        // This operation is much more efficient than regenerating the whole hint matrix
        let updated_hint_row = Matrix::from_col(&updated_row) * params.a.to_owned();

        for j in 0..hint.num_rows() {
            hint[row_to_flip][j] = updated_hint_row[0][j].clone();
        }

        let ans = answer(&query, &db);
        let recovered = recover(&params, &secret, desired_col, &hint, &ans);
        let db_item = &db[desired_col][desired_row];
        assert_eq!(recovered.uint, db_item.uint);
    }

    #[test]
    pub fn test_simplepir_updates() {
        for i in 0..8 {
            for j in 0..8 {
                test_simplepir_updates_impl(i, j);
            }
        }
    }
}
