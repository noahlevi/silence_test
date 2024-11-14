# Zero-Knowledge Proof using Schnorr Protocol in Rust

This project implements a Non-interactive Schnorr Zero-Knowledge Discrete Logarithm (DLog) Proof Scheme with a Fiat-Shamir transformation in Rust. The implementation uses the SECP256k1 elliptic curve, leveraging the `k256` crate for cryptographic operations.

## Overview

Zero-Knowledge Proofs (ZKPs) enable one party to prove knowledge of a discrete logarithm without revealing the value itself. Schnorr's protocol, specifically, provides an efficient means to achieve such proofs.


## Logic Explanation
The implementation follows the Schnorr ZKP structure:

Commitment: randomness is generated, creating a commitment that indirectly links to the secret scalar.
Challenge: Derived non-interactively to securely operate without direct synchrony between parties.
Response Calculation: Constrained modulo the curve order, maintaining operations within valid elliptic curve parameters.
Verification: Confirmation through recomputation of values via curve operations to assert validity.

## Key Components

### Scalar

- Represents an element from the field defined by the order of the elliptic curve.
- Scalar operations such as addition, multiplication, and modular arithmetic align with the cryptographic practices needed for secure ZKP systems.

### ProjectivePoint

- Represents a point on the SECP256k1 elliptic curve. 
- Used for generating and verifying commitments in the proof protocol.

### DLogProof Structure

- **DLogProof**: Contains two main components:
  - `t`: A point on the curve (commitment).
  - `s`: A scalar value representing the response used in the proof.

### Functions

- `prove`: Generates a zero-knowledge proof given a secret scalar `x` and a corresponding point `y = x * G` on the curve. It internally performs:
  1. Generates a random scalar `r`.
  2. Computes a commitment `t = r * G`.
  3. Derives a challenge `c` based on public parameters and computes the response `s = (r + c * x) mod n`, ensuring it remains within the scalar field order.
  
- `verify`: Validates the proof using the original point `y` and ensures the recomputed left-hand side matches what is expected for valid proofs of `y = x * G`.

- `hash_points`: Creates a deterministic challenge by hashing various curve points and session identifiers, providing the Fiat-Shamir transformation necessary for non-interactive protocols.

## Setup
```bash
git clone https://github.com/yourusername/zkp_project.git
cd zkp_project
```

### Build
```bash
cargo build
```

### Test
```bash
cargo test
```

