use std::marker::PhantomData;
use ark_ec::short_weierstrass::Affine;
use ark_ec::CurveGroup;
use ark_ec::pairing::Pairing;
use ark_ff::{PrimeField, Zero};
use ark_crypto_primitives::crh::{sha256::constraints::Sha256Gadget, TwoToOneCRHScheme, CRHScheme};
use ark_r1cs_std::groups::curves::short_weierstrass::AffineVar;
use ark_r1cs_std::prelude::{PairingVar, AllocVar, CurveVar, EqGadget, ToBitsGadget, ToBytesGadget};
use ark_r1cs_std::fields::fp::FpVar;
use ark_relations::r1cs::ConstraintSynthesizer;
use ark_relations::r1cs::ConstraintSystemRef;
use ark_relations::r1cs::SynthesisError;



#[derive(Clone)]
pub struct Instance<C: CurveGroup> {
    pub g: C:: Affine,
    pub sn_cur: C:: ScalarField,
    pub cm_new: C:: Affine,
    pub cm_v: C:: Affine,
    pub rt: C:: ScalarField,
    pub ct_bar: C:: Affine,
    pub apk: C:: Affine,
    pub t: C:: ScalarField,
    pub auth: C:: ScalarField,
}

#[derive(Clone)]
pub struct Witness<C: CurveGroup>{
    pub sk_snd: Option<C:: ScalarField>,
    pub pk_snd: Option<C:: Affine>,
    pub addr_snd: Option<C:: ScalarField>,
    pub pk_rcv: Option<C:: Affine>,
    pub v_cur: Option<C:: ScalarField>,
    pub v: Option<C:: ScalarField>,
    pub o_cur: Option<C:: ScalarField>,
    pub o_v: Option<C:: ScalarField>,
    pub o_new: Option<C:: ScalarField>,
    pub sn_new: Option<C:: ScalarField>,
    pub cm_cur: Option<C:: Affine>,
    pub path: Option<Vec<C:: ScalarField>>,
}

#[derive(Clone)]
pub struct SendCircuit<C: CurveGroup> {
    pub instance: Instance<C>,
    pub witness: Witness<C>,
}

impl<C> SendCircuit<C>
where
    C: CurveGroup
{
    pub fn empty() -> Self {
        Self {
            instance: Instance {
                g: C:: Affine::default(),
                sn_cur: C:: ScalarField::zero(),
                cm_new: C:: Affine::default(),
                cm_v: C:: Affine::default(),
                rt: C:: ScalarField::zero(),
                ct_bar: C:: Affine::default(),
                apk: C:: Affine::default(),
                t: C:: ScalarField::zero(),
                auth: C:: ScalarField::zero(),
            },
            witness: Witness {
                sk_snd: Some(C:: ScalarField::zero()),
                pk_snd: Some(C:: Affine::default(),),
                addr_snd: Some(C:: ScalarField::zero()),
                pk_rcv: Some(C:: Affine::default(),),
                v_cur: Some(C:: ScalarField::zero()),
                v: Some(C:: ScalarField::zero()),
                o_cur: Some(C:: ScalarField::zero()),
                o_v: Some(C:: ScalarField::zero()),
                o_new: Some(C:: ScalarField::zero()),
                sn_new: Some(C:: ScalarField::zero()),
                cm_cur: Some(C:: Affine::default(),),
                path: Some(vec![C:: ScalarField::zero()]),
            },
        }
    }

    pub fn new(instance: Instance<C>, witness: Witness<C>) -> Self {
        Self {
            instance,
            witness,
        }
    }
}

impl<C: CurveGroup> ConstraintSynthesizer<C:: ScalarField> for SendCircuit<C>
where 
    C: CurveGroup,
{
    fn generate_constraints(self, cs: ark_relations::r1cs::ConstraintSystemRef<C:: ScalarField>) -> ark_relations::r1cs::Result<()> {
        let sk_snd_var = FpVar::<C:: ScalarField>::new_witness(cs.clone(), || self.witness.sk_snd.ok_or(SynthesisError::AssignmentMissing))?;
        
        Ok(())
    }
}

// #[cfg(test)]
// pub mod test {
//     use std::str::FromStr;
//     use ark_ff::{UniformRand, Fp};
//     use ark_crypto_primitives::snark::SNARK;
//     use ark_groth16::Groth16;
//     use ark_relations::r1cs::{ConstraintSystem, ConstraintSynthesizer}; // 추가된 부분
//     use ark_ec::{bn, pairing::Pairing, short_weierstrass::{Affine, Projective, SWCurveConfig}, AffineRepr, CurveGroup};
//     use ark_bn254::{Bn254, G1Affine, Fr, Fq};
//     use crate::{circuits::send::{Instance, Witness, SendCircuit}, snark::setup};



//     #[test]
//     fn test_send() {
//         let rng = &mut ark_std::test_rng();

//         let _instance: Instance<<Bn254 as Pairing>::G1> = Instance {
//             g: G1Affine::rand(rng),
//             sn_cur: Fr::rand(rng),
//             cm_new: G1Affine::rand(rng),
//             cm_v: G1Affine::rand(rng),
//             rt: Fr::rand(rng),
//             ct_bar: G1Affine::rand(rng),
//             apk: G1Affine::rand(rng),
//             t: Fr::rand(rng),
//             auth: Fr::rand(rng),
//         };

//         let _witness: Witness<<Bn254 as Pairing>::G1> = Witness {
//             sk_snd : Some(Fr::from(1u64)),
//             pk_snd : Some(G1Affine::rand(rng)),
//             addr_snd : Some(Fr::rand(rng)),
//             pk_rcv : Some(G1Affine::rand(rng)),
//             v_cur : Some(Fr::rand(rng)),
//             v : Some(Fr::rand(rng)),
//             o_cur : Some(Fr::from(2u64)),
//             o_v : Some(Fr::rand(rng)),
//             o_new : Some(Fr::rand(rng)),
//             sn_new : Some(Fr::rand(rng)),
//             cm_cur : Some(G1Affine::rand(rng)),
//             path : Some(vec![Fr::rand(rng), Fr::rand(rng), Fr::rand(rng)]),
//         };

//         let send_circuit = SendCircuit {
//             instance: _instance,
//             witness: _witness,
//         };

//         let cs = ark_relations::r1cs::ConstraintSystem::<Fr>::new_ref();

//         send_circuit.clone().generate_constraints(cs.clone()).unwrap();
//         assert!(cs.is_satisfied().unwrap());
//     }


//     #[test]
//     fn test_empty() {
//         let test_circuit = SendCircuit::<<Bn254 as Pairing>::G1>::empty();
//         let cs = ark_relations::r1cs::ConstraintSystem::<Fr>::new_ref();
//         test_circuit.clone().generate_constraints(cs.clone()).unwrap();
//         assert!(cs.is_satisfied().unwrap());
//     }
// }