// use ark_ff::PrimeField;
// use ark_r1cs_std::fields::fp::FpVar;
// use ark_r1cs_std::prelude::*;
// use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
// use sha3::{Digest, Sha3_256};

// #[derive(Clone, Debug)]
// pub struct Witness<F: PrimeField> {
//     pub receiver_pk: FpVar<F>,   // receiver_pk
//     pub bal_prev: FpVar<F>,      // bal_prev
//     pub opening_prev: FpVar<F>,  // o_prev
//     pub bal_new: FpVar<F>,       // bal_new
//     pub opening_new: FpVar<F>,   // o_new
//     pub value: FpVar<F>,         // value
//     pub pk_recv: FpVar<F>,       // pk_rcv
//     pub cm_prev: FpVar<F>,       // cm_prev
// }

// #[derive(Clone, Debug)]
// pub struct Instance<F: PrimeField> {
//     pub sender_pk: FpVar<F>,  // sender_pk
//     pub cm_new: FpVar<F>,    // cm_new
//     pub cm_value: FpVar<F>,  // cm_v
//     pub from: FpVar<F>,      // from
// }

// #[derive(Clone, Debug)]
// pub struct SendCircuit<F: PrimeField> {
//     pub instance: Instance<F>,
//     pub witness: Witness<F>,
// }

// impl<F: PrimeField> SendCircuit<F> {
//     pub fn new(instance: Instance<F>, witness: Witness<F>) -> Self {
//         Self { instance, witness }
//     }
// }

// impl<F: PrimeField> ConstraintSynthesizer<F> for SendCircuit<F> {
//     fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
//         let mut hasher = Sha3_256::new();
//         // 1. cm_prev = H(addr_snd, bal_prev, o_prev)
//         let cm_prev_hash_input = format!(
//             "{:?}{:?}{:?}",
//             self.witness.receiver_pk.clone(),
//             self.witness.bal_prev.clone(),
//             self.witness.opening_prev.clone()
//         );

//         hasher.update(cm_prev_hash_input);

//         let cm_prev_calc = hasher.finalize();

//         let cm_prev_calc_bits = cm_prev_calc.to_vec();

//         let cm_prev_calc_var = FpVar::<F>::new_witness(cs.clone(), || {
//             Ok(F::from_le_bytes_mod_order(&cm_prev_calc_bits))
//         })?;

//         cm_prev_calc_var.enforce_equal(&self.witness.cm_prev)?;

//         // 2. cm_new = H(addr_snd, bal_prev - value, o_new)
//         let bal_new_calc = self.witness.bal_prev.clone() - self.witness.value.clone(); // bal_prev - value
//         let cm_new_hash_input = format!(
//             "{:?}{:?}{:?}",
//             self.witness.receiver_pk.clone(),
//             bal_new_calc.clone(),
//             self.witness.opening_new.clone()
//         );

//         let mut hasher = Sha3_256::new();
//         hasher.update(cm_new_hash_input);
//         let cm_new_calc_bits = hasher.finalize();

//         let cm_new_calc_var = FpVar::<F>::new_witness(cs.clone(), || {
//             Ok(F::from_le_bytes_mod_order(&cm_new_calc_bits))
//         })?;
        
//         cm_new_calc_var.enforce_equal(&self.instance.cm_new)?;

//         // 3. bal_new = bal_prev - value
//         self.witness.bal_new.enforce_equal(&bal_new_calc)?;

//         // 4. bal_new >= 0
//         bal_new_calc.enforce_cmp(&FpVar::<F>::zero(), std::cmp::Ordering::Greater, true)?;

//         // 5. cm_v = H(0 || v || o_v)

//         let cm_v_hash_input = format!(
//             "{:?}{:?}{:?}",
//             FpVar::<F>::zero(),
//             self.witness.value.clone(),
//             self.witness.opening_prev.clone()
//         );
        
//         let mut hasher = Sha3_256::new();
//         hasher.update(cm_v_hash_input);
//         let cm_v_calc_bits = hasher.finalize();

//         let cm_v_calc_var = FpVar::<F>::new_witness(cs.clone(), || {
//             Ok(F::from_le_bytes_mod_order(&cm_v_calc_bits))
//         })?;

//         cm_v_calc_var.enforce_equal(&self.instance.cm_value)?;

//         // 6. ct = elgamal_enc(pk_recv, value, o_v)

//         Ok(())
//     }
// }