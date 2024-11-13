use dlog_proof::DLogProof;
use k256::ProjectivePoint;

mod common;
mod dlog_proof;
mod tests;

fn main() {
    let sid = "sid";
    let pid = 1;

    let x = common::generate_random_number();
    let y = ProjectivePoint::GENERATOR * x; // Correctly multiply the point by the scalar

    let dlog_proof = DLogProof::prove(sid, pid, &x, &y);

    // Output the proof components
    println!("t: {:?}", dlog_proof.t);
    println!("s: {:?}", dlog_proof.s);

    let is_valid = dlog_proof.verify(sid, pid, &y);

    if is_valid {
        println!("DLOG proof is correct");
    } else {
        println!("DLOG proof is not correct");
    }
}
