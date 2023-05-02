use crate::element::Element;
use crate::matrix::Matrix;
use crate::regev::{
    Params,
    gen_error_vec,
    encrypt,
};

/// Generates a database of db_size item where each item is a bit.
pub fn gen_db(db_size: usize, params: &Params) -> Vec<Element> {
    gen_db_q(db_size, params.p)
}

/// Generates a database of db_size item where each item is an element mod q.
pub fn gen_db_q(db_size: usize, q: u64) -> Vec<Element> {
    let mut db = Vec::with_capacity(db_size);
    for _ in 0..db_size {
        let val = Element::gen_uniform_rand(q);
        db.push(val);
    }
    db
}

pub fn query(
    params: &Params,
    idx: usize,
    s: &Vec<Element>,
    db_size: usize,
) -> Vec<Element> {
    assert!(idx < db_size);
    let mut query = Vec::with_capacity(db_size);
    for i in 0..db_size {
        let bit = if i == idx {
            1
        } else {
            0
        };
        let e = gen_error_vec(params);
        let enc = encrypt(
            params,
            s,
            &e,
            &Element::from(params.p, bit)
        );
        query.push(enc);
    }
    query
}

/// The server returns the encrypted result of the query. The result is a single
/// element. Since it uses homomorphic encryption to produce the result, it learns
/// nothing about the desired index.  The server does know the contents of the
/// database, which are either 0s or 1s.
/// 
/// This implementation does the following: start with a zero matrix A and zero
/// vector c. For each item in the database, if the item is 1, add the
/// corresponding ciphertext to A and c.
/// 
/// For example:
/// query  = [enc(0), enc(1), enc(0), enc(0)] -- such that the desired index is 1
/// db     = [1, 1, 0, 0]
/// result = enc(0) + enc(1)
/// 
/// This is much simpler than the scheme described in the SimplePIR paper where the
/// database is multiplied by the query vector.
pub fn answer(params: &Params, query: &[Element], db: &[Element]) ->
    (Matrix, Element)
{
    let zero = Element::zero(params.q);
    let mut summed_a = Matrix::from_val(params.m, params.n, zero);
    let mut summed_c = Element::zero(params.q);

    for (i, item) in db.iter().enumerate() {
        if item.uint == 1 {
            summed_a += params.a.clone();
            summed_c += query[i].clone();
        }
    }
    (summed_a, summed_c)
}


/// Return an answer to a private query using a much less efficient method though it is closer to
/// what is described in the SimplePIR paper. In this technique, for each i-th database entry, we
/// take each ciphertext in the query (A, c) and multiply it by db[i].
/// e.g:
/// query = [enc(0), enc(1)]
/// db = [1, 2]
/// answer = enc(0 * 1) + enc(1 * 2) = enc(2)
pub fn answer_q(params: &Params, query: &[Element], db: &[Element]) -> 
    (Matrix, Element)
{
    let zero = Element::zero(params.q);
    let mut summed_a = Matrix::from_val(params.m, params.n, zero);
    let mut summed_c = Element::zero(params.q);
    for (i, item) in db.iter().enumerate() {
        let db_item = Element::from(params.q, item.uint);
        summed_a += params.a.to_owned().mul_elem(&db_item);
        summed_c += query[i].to_owned() * db_item.to_owned();
    }

    (summed_a, summed_c)
}

#[cfg(test)]
pub mod tests {
    use crate::regev::{
        gen_secret,
        simple_params,
        decrypt,
    };
    use super::{
        gen_db,
        query,
        answer,
        answer_q,
        Element,
        Params
    };

    fn test_pir_impl(
        params: &Params,
        s: &Vec<Element>,
    ) {
        let db_size = 50;
        let db = gen_db(db_size, params);

        let desired_idx = 24;
        let query = query(&params, desired_idx, &s, db_size);

        // Test answer_q()
        let ans = answer_q(&params, &query, &db);

        let mut p = params.clone();
        p.a = ans.0;
        let result = decrypt(&p, &s, &ans.1);
        assert_eq!(result, db[desired_idx]);

        // Test answer()
        let ans = answer(&params, &query, &db);
        let mut p = params.clone();
        p.a = ans.0;
        let result = decrypt(&p, &s, &ans.1);
        assert_eq!(result, db[desired_idx]);
    }

    #[test]
    fn test_pir() {
        let params = simple_params();
        let s = gen_secret(&params);
        for _ in 0..50 {
            test_pir_impl(&params, &s);
        }
    }
}
