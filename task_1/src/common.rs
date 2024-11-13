use k256::elliptic_curve::Field;

/// Generates a random scalar in the range of the SECP256k1 curve.
///
/// This function uses a secure random number generator to produce a
/// random scalar. It's important for cryptographic operations such
/// as creating nonces in DLog proofs.
///
/// # Returns
/// A random `k256::Scalar` representing a scalar value.
pub fn generate_random_number() -> k256::Scalar {
    let mut rng = rand::thread_rng();
    k256::Scalar::random(&mut rng)
}
