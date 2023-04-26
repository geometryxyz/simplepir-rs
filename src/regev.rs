use crate::matrix::Matrix;
use crate::element::Element;

pub struct Params {
    // Public A matrix
    pub a: Matrix,
    // The integer modulus
    pub q: u64,
    // The plaintext modulus
    pub p: u64,
    // The LWE secret length
    pub n: usize,
    // The number of samples
    pub m: usize,
    // The standard deviation for sampling random elements
    pub std_dev: f64,
}

pub fn simple_params() -> Params {
    Params {
        a: Matrix::from_col(&vec![
           // Make sure the modulus q is the same as below!
           Element::from(3329u64, 604),
           Element::from(3329u64, 735),
           Element::from(3329u64, 3216),
           Element::from(3329u64, 1804),
       ]),
        q: 3329,
        p: 2,
        n: 4,
        m: 1,
        std_dev: 1.0,
    }
}

fn check_secret_length(params: &Params, secret: &Vec<Element>) {
    // Check that the secret has the correct number of elements
    assert_eq!(secret.len(), params.n);
}

fn check_plaintext_mod(params: &Params, plaintext: &Element) {
    // Check that each element of the plaintext is within range
    assert!(plaintext.uint < params.p);
    assert_eq!(plaintext.q, params.p);
}

fn check_ciphertext_mod(params: &Params, ciphertext: &Element) {
    // Check that the ciphertext is in range
    assert!(ciphertext.uint < params.q);
}

fn check_error_length(params: &Params, error: &Vec<Element>) {
    // Check that the error has the correct number of elements
    assert_eq!(error.len(), params.m);
}

pub fn encrypt(
    params: &Params,
    secret: &Vec<Element>,
    e: &Vec<Element>,
    plaintext: &Element,
) -> Element {
    check_secret_length(params, secret);
    check_plaintext_mod(params, plaintext);
    check_error_length(params, e);

    // Compute As
    let a_s = params.a.clone().mul_vec(secret);
    
    // Compute As + e
    let a_s_e = a_s + Matrix::from(&vec![e.clone()]);
    //println!("As + e: {}", a_s_e);

    let floor = params.q / params.p;
    let floor = Matrix::from_single(&Element::from(params.q, floor));

    // Convert the plaintext to a matrix with Element mod q instead of p
    let plaintext_as_matrix = Matrix::from_single(&Element::from(params.q, plaintext.uint));
    //println!("plaintext_as_matrix: {}", plaintext_as_matrix.clone());

    // Compute the ciphertext
    let c = a_s_e + (floor * plaintext_as_matrix);
    
    c[0][0].clone()
}

pub fn decrypt(
    params: &Params,
    secret: &Vec<Element>,
    ciphertext: &Element,
) -> Element {
    check_secret_length(params, secret);
    check_ciphertext_mod(params, ciphertext);
    // Compute As
    let a_s = params.a.clone().mul_vec(secret);

    assert_eq!(ciphertext.q, params.q);
    assert_eq!(a_s[0][0].q, params.q);

    // Compute c - As
    let c_a_s = Matrix::from_single(ciphertext) - a_s;

    let x = ((c_a_s[0][0].uint * 2) / params.q) % params.p;

    Element::from(params.p, x)
}

pub fn gen_random_normal_matrix(
    q: u64,
    std_dev: f64,
    num_rows: usize,
    num_cols: usize,
) -> Matrix {
    let v: Vec<Vec<Element>> = vec![
        vec![Element::new(q); num_rows];
        num_cols
    ];

    let mut matrix = Matrix::from(&v);

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

    fn gen_secret(params: &Params) -> Vec<Element> {
        vec![
            Element::from(params.q, 1503),
            Element::from(params.q, 1137),
            Element::from(params.q, 738),
            Element::from(params.q, 1775),
        ]
    }

    #[test]
    fn test_gen_random_normal_matrix() {
        let num_rows = 9;
        let num_cols = 10;
        let matrix = gen_random_normal_matrix(101u64, 6.4 as f64, num_rows, num_cols);
        assert_eq!(matrix.num_rows(), num_rows);
        assert_eq!(matrix.num_cols(), num_cols);
    }

    fn test_encrypt_and_decrypt_impl(pu: u64) {
        let params = simple_params();
        let secret = gen_secret(&params);
        let e = vec![Element::from(params.q, 2)];

        let plaintext = Element::from(params.p, pu);
        let ciphertext = encrypt(&params, &secret, &e, &plaintext);
        assert_eq!(plaintext, decrypt(&params, &secret, &ciphertext));
    }

    #[test]
    fn test_encrypt_and_decrypt() {
        test_encrypt_and_decrypt_impl(0);
        test_encrypt_and_decrypt_impl(1);
    }
}
