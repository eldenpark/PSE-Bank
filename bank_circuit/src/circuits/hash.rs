// use ark_ff::PrimeField;
// use ark_r1cs_std::fields::fp::FpVar;
// use ark_r1cs_std::prelude::AllocVar;
// use ark_r1cs_std::prelude::EqGadget;
// use ark_relations::r1cs::ConstraintSynthesizer;
// use ark_relations::r1cs::ConstraintSystemRef;
// use ark_relations::r1cs::SynthesisError;
// use sha3::{Digest, Sha3_256};

// #[derive(Clone)]
// pub struct HashCheckCircuit<F: PrimeField> {
//     // statement
//     pub hash: Option<F>,
//     // witness
//     pub input: Option<F>,
// }

// impl<F: PrimeField> ConstraintSynthesizer<F> for HashCheckCircuit<F> {
//     fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
//         let mut hasher = Sha3_256::new();
//         let input = FpVar::new_witness(cs.clone(), || self.input.ok_or(SynthesisError::AssignmentMissing))?;

//         hasher.update(input);
//     }
// }