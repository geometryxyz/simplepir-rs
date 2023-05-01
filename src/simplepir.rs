use crate::regev::Params;
use crate::matrix::Matrix;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_setup() {
        let params = gen_params();
        let db = gen_db(&params);
        println!("{}", db);
    }
}
