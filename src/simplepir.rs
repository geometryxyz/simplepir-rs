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
    for i in 0..db_q.num_cols() {
        for j in 0..db_q.num_rows() {
            db_q[i][j].q = params.regev_params.q;
        }
    }
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

    fn test_simplepir_impl() {
        let params = gen_params();
        let db = gen_db(&params);

        let desired_row = 1;
        let desired_col = 2;

        // TODO: check if it should be col, row or row, col
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
        for _ in 0..50 {
            test_simplepir_impl();
        }
    }
}
