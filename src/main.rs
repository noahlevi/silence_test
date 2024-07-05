extern crate ecdsa;
extern crate k256;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate sha2;

use crate::k256::elliptic_curve::group::GroupEncoding;
use crate::serde::ser::SerializeStruct;
use k256::elliptic_curve::sec1::ToEncodedPoint;
use k256::{elliptic_curve::PrimeField, ProjectivePoint, Scalar};
use rand::RngCore;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

use generic_array::GenericArray;

type Point = ProjectivePoint;

#[derive(Debug)]
struct DLogProof {
    t: Point,
    s: Scalar,
}

impl Serialize for DLogProof {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let t_bytes = self.t.to_bytes();
        let s_bytes = self.s.to_bytes();
        let mut state = serializer.serialize_struct("DLogProof", 2)?;
        state.serialize_field("t", &t_bytes.as_slice())?;
        state.serialize_field("s", &s_bytes.as_slice())?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for DLogProof {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct DLogProofData {
            t: Vec<u8>,
            s: Vec<u8>,
        }

        let data = DLogProofData::deserialize(deserializer)?;

        let t_option = Point::from_bytes(&GenericArray::from_slice(&data.t));
        let t = if t_option.is_some().into() {
            t_option.unwrap()
        } else {
            return Err(serde::de::Error::custom("Invalid point bytes"));
        };

        let s_option = Scalar::from_repr(GenericArray::clone_from_slice(&data.s));
        let s = if s_option.is_some().into() {
            s_option.unwrap()
        } else {
            return Err(serde::de::Error::custom("Invalid scalar bytes"));
        };

        Ok(DLogProof { t, s })
    }
}

impl DLogProof {
    fn hash_points(sid: &str, pid: u32, points: &[Point]) -> Scalar {
        let mut hasher = Sha256::new();
        hasher.update(sid.as_bytes());
        hasher.update(&pid.to_le_bytes());

        for point in points {
            let encoded_point = point.to_affine().to_encoded_point(false);
            hasher.update(encoded_point.as_bytes());
        }

        let hash = hasher.finalize();
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&hash[..32]); // Ensure the length is 32 bytes
        Scalar::from_repr(GenericArray::clone_from_slice(&bytes)).unwrap()
    }

    fn prove(sid: &str, pid: u32, x: Scalar, y: Point, base_point: Point) -> DLogProof {
        let mut rng = rand::thread_rng();
        let mut r_bytes = [0u8; 32];
        rng.fill_bytes(&mut r_bytes);
        let r = Scalar::from_repr(GenericArray::clone_from_slice(&r_bytes)).unwrap();
        let t = base_point * r;
        let c = DLogProof::hash_points(sid, pid, &[base_point, y, t]);
        let s = r + c * x;

        DLogProof { t, s }
    }

    fn verify(&self, sid: &str, pid: u32, y: Point, base_point: Point) -> bool {
        let c = DLogProof::hash_points(sid, pid, &[base_point, y, self.t]);

        let lhs = base_point * self.s;
        let rhs = self.t + (y * c);

        lhs == rhs
    }
}

fn generate_random_number() -> Scalar {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);
    Scalar::from_repr(GenericArray::clone_from_slice(&bytes)).unwrap()
}

fn main() {
    let sid = "sid";
    let pid = 1;

    let x = generate_random_number();
    println!("x: {:?}", x);

    let base_point = Point::GENERATOR;
    let y = base_point * x;

    let start_proof = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let dlog_proof = DLogProof::prove(sid, pid, x, y, base_point);
    let end_proof = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    println!("Proof computation time: {} ms", end_proof - start_proof);
    println!("t: {:?}", dlog_proof.t);
    println!("s: {:?}", dlog_proof.s);

    let start_verify = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let result = dlog_proof.verify(sid, pid, y, base_point);
    let end_verify = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    println!("Verify computation time: {} ms", end_verify - start_verify);

    if result {
        println!("DLOG proof is correct");
    } else {
        println!("DLOG proof is not correct");
    }
}
