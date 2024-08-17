// use ark_ff::PrimeField;
// use ark_r1cs_std::fields::fp::FpVar;
// use ark_r1cs_std::prelude::AllocVar;
// use ark_r1cs_std::prelude::EqGadget;
// use ark_relations::r1cs::ConstraintSynthesizer;
// use ark_relations::r1cs::ConstraintSystemRef;
// use ark_relations::r1cs::SynthesisError;

// // c =? a * b
// #[derive(Clone)]
// pub struct MultiplyCircuit<F: PrimeField> {
//     // statement
//     pub c: Option<F>,
//     // witness
//     pub a: Option<F>,
//     pub b: Option<F>,
// }

// impl<F: PrimeField> ConstraintSynthesizer<F> for MultiplyCircuit<F> {
//     fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
//         let c = FpVar::new_input(cs.clone(), || self.c.ok_or(SynthesisError::AssignmentMissing))?;
        
//         let a = FpVar::new_witness(cs.clone(), || self.a.ok_or(SynthesisError::AssignmentMissing))?;
//         let b = FpVar::new_witness(cs.clone(), || self.b.ok_or(SynthesisError::AssignmentMissing))?;
        
//         c.enforce_equal(&(a*b))?;

//         Ok(())
//     }
// }

// #[cfg(test)]
// pub mod test {
//     use std::str::FromStr;

//     use ark_bn254::Fr;
//     use ark_ff::Fp;
//     use ark_relations::r1cs::ConstraintSynthesizer;

//     use crate::circuits::multiply::MultiplyCircuit;

//     #[test]
//     fn test_mul() {
//         let a: Fr = Fp::from_str("3").unwrap();
//         let b: Fr = Fp::from_str("2").unwrap();

//         let c: Fr = Fp::from_str("6").unwrap();

//         let test_circuit = MultiplyCircuit {a: Some(a), b: Some(b), c: Some(c)};
//         let cs = ark_relations::r1cs::ConstraintSystem::new_ref();

//         test_circuit.clone().generate_constraints(cs.clone()).unwrap();
//         assert!(cs.is_satisfied().unwrap());
//     }
// }