#[cfg(test)]
mod tests {
    use crate::common::generate_random_number;
    use crate::dlog_proof::DLogProof;
    // use k256::elliptic_curve::ops::Reduce;
    use k256::{
        elliptic_curve::{generic_array::GenericArray, PrimeField},
        ProjectivePoint, Scalar,
    };

    #[test]
    fn test_dlog_proof_generation_and_verification() {
        let sid = "test_session_id";
        let pid = 42;

        // Generate a random scalar and corresponding public point
        let x = generate_random_number();
        let y = ProjectivePoint::GENERATOR * x; // y = x * G

        // Generate the DLog proof
        let dlog_proof = DLogProof::prove(sid, pid, &x, &y);

        // Verify the proof
        assert!(
            dlog_proof.verify(sid, pid, &y),
            "DLOG proof verification failed"
        );
    }

    #[test]
    fn test_dlog_proof_verification_with_invalid_parameters() {
        let sid = "test_session_id";
        let pid = 42;

        // Generate a random scalar and corresponding public point
        let x = generate_random_number();
        let y = ProjectivePoint::GENERATOR * x; // y = x * GENERATOR

        // Generate the DLog proof
        let dlog_proof = DLogProof::prove(sid, pid, &x, &y);

        // Create an altered value for y that should invalidate the verification
        let invalid_y = ProjectivePoint::GENERATOR * generate_random_number(); // Different random scalar

        // Verify the proof with the altered y
        assert!(
            !dlog_proof.verify(sid, pid, &invalid_y),
            "DLOG proof verification should fail with incorrect y"
        );
    }

    #[test]
    fn test_dlog_proof_with_edge_cases() {
        let sid = "edge_case_sid";
        let pid = 1000;

        // Test with zero scalar
        let x = Scalar::ZERO;
        let y = ProjectivePoint::GENERATOR * x; // y should also be the identity point

        // Generate the DLog proof
        let dlog_proof = DLogProof::prove(sid, pid, &x, &y);

        // Verify the proof
        assert!(
            dlog_proof.verify(sid, pid, &y),
            "DLOG proof verification failed with zero scalar"
        );

        // Test with maximum scalar
        let max_scalar = Scalar::from_repr(GenericArray::clone_from_slice(&[0xFF; 32])).unwrap();
        let y_max = ProjectivePoint::GENERATOR * max_scalar;

        // Generate the DLog proof
        let dlog_proof_max = DLogProof::prove(sid, pid, &max_scalar, &y_max);
        assert!(
            dlog_proof_max.verify(sid, pid, &y_max),
            "DLOG proof verification failed with maximum scalar"
        );
    }

    #[test]
    fn test_consistency_of_multiple_proofs() {
        let sid = "consistency_sid";
        let pid = 2022;

        // Generate random scalars and corresponding public points
        let x1 = generate_random_number();
        let y1 = ProjectivePoint::GENERATOR * x1;

        let x2 = generate_random_number();
        let y2 = ProjectivePoint::GENERATOR * x2;

        // Generate DLog proofs for both
        let dlog_proof1 = DLogProof::prove(sid, pid, &x1, &y1);
        let dlog_proof2 = DLogProof::prove(sid, pid, &x2, &y2);

        // Verify both proofs
        assert!(
            dlog_proof1.verify(sid, pid, &y1),
            "DLOG proof verification failed for first proof"
        );
        assert!(
            dlog_proof2.verify(sid, pid, &y2),
            "DLOG proof verification failed for second proof"
        );
    }

    /// Trait implementation to the DLogProof struct for equality checks
    impl PartialEq for DLogProof {
        fn eq(&self, other: &Self) -> bool {
            self.t == other.t && self.s == other.s
        }
    }
}
