use crate::matrix::Matrix;
use crate::element::Element;
use crate::regev::{
    Params,
    gen_error_vec,
};

pub struct SimplePIRParams {
    pub log2_db_size: usize,
    pub regev_params: Params,
}

pub fn gen_params() -> SimplePIRParams {
    let log2_db_size = 8;
    let m = 8;
    let n = 512;
    let q = 3329;
    let p = 2;
    let std_dev = 6.4;
    let a = Matrix::gen_uniform_rand(q, n, m);

    let regev_params = Params { a, q, p, n, m, std_dev };

    SimplePIRParams {
        log2_db_size,
        regev_params,
    }
}

pub fn gen_db(params: &SimplePIRParams) -> Matrix {
    Matrix::gen_uniform_rand(
        params.regev_params.p,
        params.log2_db_size,
        params.log2_db_size,
    )
}

pub fn gen_hint(params: &SimplePIRParams, db: &Matrix) -> Matrix {
    let mut db_q = db.clone();
    db_q.change_q(params.regev_params.q);
    db_q.to_owned() * params.regev_params.a.to_owned()
}

pub fn query(
    params: &SimplePIRParams,
    idx: usize,
    s: &Vec<Element>,
) -> Vec<Element> {
    let db_size = params.log2_db_size;
    assert!(idx < db_size);
    let floor = params.regev_params.q / params.regev_params.p;
    let e = gen_error_vec(&params.regev_params);
    let err_matrix = Matrix::from_col(&e);

    let mut query = params.regev_params.a.to_owned().mul_vec(s);
    query += err_matrix.rotated();
    query[idx][0] += Element::from(params.regev_params.q, floor);

    query.rotated()[0].to_owned()
}

pub fn answer(query: &Vec<Element>, db: &Matrix) -> 
    Matrix
{
    let mut db_q = db.clone();
    for i in 0..db_q.num_cols() {
        for j in 0..db_q.num_rows() {
            db_q[i][j].q = query[0].q;
        }
    }
    db_q.to_owned().mul_vec(query)
}

pub fn recover(
    params: &SimplePIRParams,
    s: &Vec<Element>,
    idx: usize,
    hint: &Matrix,
    answer: &Matrix,
) -> Element {
    let p = params.regev_params.p;
    let q = params.regev_params.q;

    let interim = hint.to_owned().mul_vec(s);
    let mut ans = answer.to_owned();
    ans -= interim.to_owned();

    let x = ((ans[idx][0].uint * p) as f64 / q as f64).round() as u64 % p;
    Element::from(p, x)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::regev::gen_secret;

    fn test_simplepir_impl(desired_col: usize, desired_row: usize) {
        let params = gen_params();
        let db = gen_db(&params);

        let db_item = &db[desired_col][desired_row];

        let secret = gen_secret(&params.regev_params);
        let hint = gen_hint(&params, &db);

        let query = query(&params, desired_row, &secret);
        let answer = answer(&query, &db);
        let recovered = recover(&params, &secret, desired_col, &hint, &answer);
        assert_eq!(recovered, *db_item);
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

        let secret = gen_secret(&params.regev_params);

        let query = query(&params, desired_row, &secret);
        let ans = answer(&query, &db);
        let recovered = recover(&params, &secret, desired_col, &hint, &ans);

        let db_item = &db[desired_col][desired_row];
        assert_eq!(recovered, *db_item);

        // Flip all bits of one row
        let mut db = db.clone();

        let mut updated_row = Vec::with_capacity(params.log2_db_size);
        for i in 0..db.num_rows() {
            // Flip the bits in the row
            db[desired_col][i] -= Element::from(params.regev_params.p, 1);
            updated_row.push(
                Element::from(params.regev_params.q, db[desired_col][i].uint)
            );
        }
        db.change_q(params.regev_params.q);

        // Now update the hint
        let mut hint = hint.clone();

        // This operation is much more efficient than regenerating the whole hint matrix
        let updated_hint_row = Matrix::from_col(&updated_row) * params.regev_params.a.to_owned();

        for j in 0..hint.num_rows() {
            hint[desired_col][j] = updated_hint_row[0][j].clone();
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
