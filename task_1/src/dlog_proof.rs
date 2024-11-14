use crate::common::generate_random_number;
use k256::{
    elliptic_curve::{generic_array::GenericArray, sec1::ToEncodedPoint, Field, PrimeField},
    ProjectivePoint, Scalar,
};
use sha2::{Digest, Sha256};
use k256::elliptic_curve::ops::Reduce;

pub struct DLogProof {
    pub t: ProjectivePoint,
    pub s: Scalar,
}

const SECP256K1_ORDER_BYTES: [u8; 32] = [
    0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFE, // This should be n - 1
    0xBA, 0xAE, 0xD6, 0xAF, // Correct representation following the curve order
];

impl DLogProof {
    /// Generates a DLog proof using the Schnorr proof scheme.
    ///
    /// # Arguments
    /// * `sid`: Session ID as a string slice.
    /// * `pid`: Proof ID as a u32 integer.
    /// * `x`: Secret scalar used in the proof.
    /// * `y`: Public point representing `y = x * G` where `G` is the generator.
    ///
    /// # Returns
    /// A `DLogProof` struct containing the generated proof.
    pub fn prove(sid: &str, pid: u32, x: &Scalar, y: &ProjectivePoint) -> Self {
        let r = generate_random_number();
        let t = ProjectivePoint::GENERATOR * r; // Corrected multiplication

        let c = Self::hash_points(sid, pid, &[ProjectivePoint::GENERATOR, *y, t]);
        let s = r + c * x;
        DLogProof { t, s }
    }
    // pub fn prove(sid: &str, pid: u32, x: &Scalar, y: &ProjectivePoint) -> Self {
    //     let r = generate_random_number(); // Generate random scalar
    //     let t = ProjectivePoint::GENERATOR * r; // Commitment

    //     let c = Self::hash_points(sid, pid, &[ProjectivePoint::GENERATOR, *y, t]);
        
    //     // Create a scalar from the order bytes
    //     let order_generic_array = *GenericArray::from_slice(&SECP256K1_ORDER_BYTES);
    //     // let s = (r + c * x) % Scalar::from_be_bytes_reduced(order_generic_array); // Modulo the defined order
        

    //     let order_scalar = Scalar::from_be_bytes_reduced(order_generic_array);

    //     let s = (r + (c * x)) * order_scalar.invert().unwrap();
    //     DLogProof { t, s } // Return the proof
    // }

    /// Verifies the DLog proof using the Schnorr proof scheme.
    ///
    /// # Arguments
    /// * `self`: The DLog proof to verify.
    /// * `sid`: Session ID as a string slice.
    /// * `pid`: Proof ID as a u32 integer.
    /// * `y`: Public point representing `y = x * G`.
    ///
    /// # Returns
    /// A boolean indicating whether the proof is valid.
    pub fn verify(&self, sid: &str, pid: u32, y: &ProjectivePoint) -> bool {
        let c = Self::hash_points(sid, pid, &[ProjectivePoint::GENERATOR, *y, self.t]);
        let lhs = ProjectivePoint::GENERATOR * self.s;
        let rhs = self.t + (*y * c);
        lhs == rhs
    }

    /// Hashes a set of points along with session ID and proof ID using SHA-256.
    ///
    /// This function encodes the session ID and proof ID into bytes, and
    /// concatenates them with the encoded representation of elliptic curve
    /// points. It then computes the SHA-256 hash of this concatenation.
    ///
    /// # Arguments
    /// * `sid`: A string slice representing the session ID that will be included in the hash.
    /// * `pid`: A u32 integer representing the proof ID that will be included in the hash.
    /// * `points`: A slice of `ProjectivePoint` references to hash, which contains the elliptic curve points.
    ///
    /// # Returns
    /// A `k256::Scalar` representing the hash of the provided points and IDs.
    /// This scalar will be used in cryptographic operations such as proof generation and verification.
    pub fn hash_points(sid: &str, pid: u32, points: &[k256::ProjectivePoint]) -> k256::Scalar {
        let mut hasher = Sha256::new();
        hasher.update(sid.as_bytes());
        hasher.update(&pid.to_le_bytes());

        for point in points {
            hasher.update(point.to_encoded_point(false).as_ref());
        }

        let result = hasher.finalize();
        let mut bytes_array: [u8; 32] = Default::default();
        bytes_array.copy_from_slice(&result[..32]); // Ensure the length is 32 bytes
        Scalar::from_repr(GenericArray::clone_from_slice(&bytes_array)).unwrap()
    }

    /// New instance of DLogProof with default values.
    ///
    /// # Returns
    /// A `DLogProof` instance with `t` set to the identity point and `s` set to zero.
    pub fn new() -> Self {
        DLogProof {
            t: ProjectivePoint::IDENTITY, // Use identity point for default
            s: Scalar::ZERO,              // Use zero for default
        }
    }
}

impl Default for DLogProof {
    /// Creates a default instance of DLogProof.
    ///
    /// # Returns
    /// A `DLogProof` instance with `t` set to the identity point and `s` set to zero.
    fn default() -> Self {
        DLogProof {
            t: ProjectivePoint::IDENTITY, // Use identity point for default
            s: Scalar::ZERO,              // Use zero for default
        }
    }
}
