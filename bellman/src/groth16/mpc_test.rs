#[cfg(test)]
mod mpc_tests {
    use super::*;

    use crate::groth16::mpc::*;
    use bls12_381::{Bls12, Scalar};
    #[test]
    fn common_works() {
        let g1 = bls12_381::G1Affine::generator();
        let g2 = bls12_381::G2Affine::generator();
        let mut paramter_in_storage = initial_common_paramters::<Bls12>(4);
        //under-chain
        let player1_common = mpc_common_paramters_generator(&paramter_in_storage, (1, 2, 3));
        //on-chain
        paramter_in_storage = verify_common_paramter(&paramter_in_storage, &player1_common);
        //under-chain
        let player2_common = mpc_common_paramters_generator(&paramter_in_storage, (2, 3, 4));
        //on-chain
        paramter_in_storage = verify_common_paramter(&paramter_in_storage, &player2_common);
        //under-chain
        let player3_common = mpc_common_paramters_generator(&paramter_in_storage, (3, 4, 5));
        //on-chain
        paramter_in_storage = verify_common_paramter(&paramter_in_storage, &player3_common);

        let alpha_g1 = paramter_in_storage.alpha_g1;
        let alpha_g2 = paramter_in_storage.alpha_g2;
        let beta_g1 = paramter_in_storage.beta_g1;
        let beta_g2 = paramter_in_storage.beta_g2;
        let tau_g1 = paramter_in_storage.tau_g1;
        let tau_g2 = paramter_in_storage.tau_g2;
        let alpha_mul_tau_g1 = paramter_in_storage.alpha_mul_tau_g1;
        let alpha_mul_tau_g2 = paramter_in_storage.alpha_mul_tau_g2;
        let beta_mul_tau_g1 = paramter_in_storage.beta_mul_tau_g1;
        let beta_mul_tau_g2 = paramter_in_storage.beta_mul_tau_g2;
        assert_eq!(alpha_g1, g1 * Scalar::from(6));
        assert_eq!(alpha_g2, g2 * Scalar::from(6));
        assert_eq!(beta_g1, g1 * Scalar::from(24));
        assert_eq!(beta_g2, g2 * Scalar::from(24));
        assert_eq!(tau_g1[0], g1 * Scalar::from(60));
        assert_eq!(tau_g2[0], g2 * Scalar::from(60));
        assert_eq!(tau_g1[1], g1 * Scalar::from(3600));
        assert_eq!(tau_g2[1], g2 * Scalar::from(3600));
        assert_eq!(tau_g1[2], g1 * Scalar::from(216000));
        assert_eq!(tau_g2[2], g2 * Scalar::from(216000));
        assert_eq!(alpha_mul_tau_g1[0], g1 * Scalar::from(60 * 6));
        assert_eq!(alpha_mul_tau_g2[0], g2 * Scalar::from(60 * 6));
        assert_eq!(alpha_mul_tau_g1[1], g1 * Scalar::from(3600 * 6));
        assert_eq!(alpha_mul_tau_g2[1], g2 * Scalar::from(3600 * 6));
        assert_eq!(alpha_mul_tau_g1[2], g1 * Scalar::from(216000 * 6));
        assert_eq!(alpha_mul_tau_g2[2], g2 * Scalar::from(216000 * 6));
        assert_eq!(beta_mul_tau_g1[0], g1 * Scalar::from(60 * 24));
        assert_eq!(beta_mul_tau_g2[0], g2 * Scalar::from(60 * 24));
        assert_eq!(beta_mul_tau_g1[1], g1 * Scalar::from(3600 * 24));
        assert_eq!(beta_mul_tau_g2[1], g2 * Scalar::from(3600 * 24));
        assert_eq!(beta_mul_tau_g1[2], g1 * Scalar::from(216000 * 24));
        assert_eq!(beta_mul_tau_g2[2], g2 * Scalar::from(216000 * 24));
    }

    #[test]
    fn matrix_works() {}
}