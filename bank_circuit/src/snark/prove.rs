use ark_bn254::Bn254;
use ark_groth16::{Groth16, Proof, ProvingKey};

use ark_std::rand::thread_rng;
use ark_crypto_primitives::snark::SNARK;
use crate::circuits::{encryption::ElgamalCircuit, send::SendCircuit};

type C = ark_ed_on_bn254::EdwardsProjective;
type GG = ark_ed_on_bn254::constraints::EdwardsVar;

pub fn elgamal_prove(pk: ProvingKey<Bn254>, circuit: ElgamalCircuit<C, GG>) -> Proof<Bn254> {
    let rng = &mut thread_rng();

    let proof = Groth16::<Bn254>::prove(&pk, circuit, rng).unwrap();

    proof
}

// pub fn send_prove(pk: ProvingKey<Bn254>, circuit: SendCircuit<Bn254>) -> Proof<Bn254> {
//     let rng = &mut thread_rng();

//     let proof
// }