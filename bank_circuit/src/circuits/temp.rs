use ark_bls12_381::Bls12_381;
use ark_crypto_primitives::snark::{CircuitSpecificSetupSNARK, SNARK};
use ark_ed_on_bls12_381::{constraints::*, *};
use ark_groth16::{prepare_verifying_key, Groth16};
use ark_r1cs_std::prelude::*;
use ark_relations::r1cs::*;
use ark_std::UniformRand;
use rand::rngs::OsRng;

struct MySillyCircuit {}

impl ConstraintSynthesizer<Fq> for MySillyCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fq>) -> Result<()> {
        // let cs1 = ConstraintSystem::<Fq>::new_ref();
        let mut rng = OsRng;

        let a_native = EdwardsProjective::rand(&mut rng);
        let b_native = EdwardsProjective::rand(&mut rng);

        // Allocate `a_native` and `b_native` as witness variables in `cs`.
        let a = EdwardsVar::new_witness(ark_relations::ns!(cs, "a"), || Ok(a_native))?;
        let b = EdwardsVar::new_witness(ark_relations::ns!(cs, "b"), || Ok(b_native))?;

        // Allocate `a_native` and `b_native` as constants in `cs`. This does not add any
        // constraints or variables.
        let a_const = EdwardsVar::new_constant(ark_relations::ns!(cs, "a_as_constant"), a_native)?;
        let b_const = EdwardsVar::new_constant(ark_relations::ns!(cs, "b_as_constant"), b_native)?;

        // This returns the identity of `Edwards`.
        let zero = EdwardsVar::zero();

        // Sanity check one + one = two
        let two_a = &a + &a + &zero;
        two_a.enforce_equal(&a.double()?)?;

        (&a + &b).enforce_equal(&(&a_const + &b_const))?;

        Ok(())
    }
}

#[test]
fn test123() {
    let mut rng = OsRng;

    let circuit = MySillyCircuit {};

    let (pk, vk) = Groth16::<Bls12_381>::setup(circuit, &mut rng).unwrap();
    let pvk = prepare_verifying_key(&vk);
    let proof = Groth16::<Bls12_381>::prove(&pk, MySillyCircuit {}, &mut rng).unwrap();

    println!("proof: {:?}", proof);
}
