use ark_bn254::Bn254;
use ark_groth16::{Groth16, PreparedVerifyingKey, ProvingKey, VerifyingKey};
use ark_std::rand::thread_rng;
use ark_ec::pairing::Pairing;
use ark_crypto_primitives::snark::SNARK;
use crate::circuits::{encryption::ElgamalCircuit, send::SendCircuit};

type C = ark_ed_on_bn254::EdwardsProjective;
type GG = ark_ed_on_bn254::constraints::EdwardsVar;

pub fn elgamal_setup(circuit: ElgamalCircuit<C, GG>) -> (ProvingKey<Bn254>, VerifyingKey<Bn254>, PreparedVerifyingKey<Bn254>) {
    let rng = &mut thread_rng();
    let (pk, vk) = Groth16::<Bn254>::circuit_specific_setup(circuit, rng).unwrap();

    let pvk = Groth16::<Bn254>::process_vk(&vk).unwrap();

    (pk, vk, pvk)
}


// pub fn send_setup(circuit: SendCircuit<Bn254>) -> (ProvingKey<Bn254>, VerifyingKey<Bn254>, PreparedVerifyingKey<Bn254>) {
//     let rng = &mut thread_rng();

//     let (pk, vk) = Groth16::<Bn254>::circuit_specific_setup(circuit, rng).unwrap();

//     let pvk = Groth16::<Bn254>::process_vk(&vk).unwrap();

//     (pk, vk, pvk)
// }
pub fn send_setup(circuit: SendCircuit<<Bn254 as Pairing>::G1>) -> (ProvingKey<Bn254>, VerifyingKey<Bn254>, PreparedVerifyingKey<Bn254>) {
    let rng = &mut thread_rng();

    let (pk, vk) = Groth16::<Bn254>::circuit_specific_setup(circuit, rng).unwrap();

    let pvk = Groth16::<Bn254>::process_vk(&vk).unwrap();

    (pk, vk, pvk)
}
