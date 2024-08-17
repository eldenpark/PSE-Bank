use std::marker::PhantomData;
use ark_ec::CurveGroup;
use ark_ff::PrimeField;
use ark_r1cs_std::prelude::AllocVar;
use ark_r1cs_std::prelude::CurveVar;
use ark_r1cs_std::prelude::EqGadget;
use ark_relations::r1cs::ConstraintSynthesizer;
use ark_relations::r1cs::ConstraintSystemRef;
use ark_relations::r1cs::SynthesisError;

use crate::gadgets::public_encryptions::{elgamal, constraints::AsymmetricEncryptionGadget};

#[derive(Clone)]
pub struct Witness<C: CurveGroup>{
    pub m: Option<elgamal::Plaintext<C>>,
    pub r: Option<elgamal::Randomness<C>>,
}

#[derive(Clone)]
pub struct Instance<C: CurveGroup> {
    pub g : elgamal::Parameters<C>,
    pub pk: Option<elgamal::PublicKey<C>>,
    pub ct: Option<elgamal::Ciphertext<C>>,
}


#[derive(Clone)]
pub struct ElgamalCircuit<C: CurveGroup, GG: CurveVar<C, C::BaseField>> {
    pub instance: Instance<C>,
    pub witness: Witness<C>,
    pub _curve_var: PhantomData<GG>,
}

impl<C, GG> ConstraintSynthesizer<C::BaseField> for ElgamalCircuit<C, GG> 
where 
    C: CurveGroup,
    GG: CurveVar<C, C::BaseField>,
    C::BaseField: PrimeField,
{
    fn generate_constraints(self, cs: ConstraintSystemRef<C::BaseField>) -> Result<(), SynthesisError> {
        // constant
        let g = elgamal::constraints::ParametersVar::new_constant(cs.clone(), self.instance.g)?;

        // instance
        let pk = elgamal::constraints::PublicKeyVar::new_input(cs.clone(), || self.instance.pk.ok_or(SynthesisError::AssignmentMissing))?;
        let ct = elgamal::constraints::OutputVar::new_input(cs.clone(), ||self.instance.ct.ok_or(SynthesisError::AssignmentMissing))?;

        // witness
        let m = elgamal::constraints::PlaintextVar::new_witness(cs.clone(), ||self.witness.m.ok_or(SynthesisError::AssignmentMissing))?;
        let r = elgamal::constraints::RandomnessVar::new_witness(cs.clone(), ||self.witness.r.ok_or(SynthesisError::AssignmentMissing))?;

        let result_var = elgamal::constraints::ElGamalEncGadget::<C, GG>::encrypt(&g, &m, &r, &pk)?;

        result_var.enforce_equal(&ct)?;

        Ok(())
    }
}