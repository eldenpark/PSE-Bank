#[cfg(test)]
mod tests {
    use ark_crypto_primitives::snark::SNARK;
    use ark_ff::UniformRand;
    use ark_groth16::Groth16;
    use ark_bn254::Bn254;
    use crate::gadgets::public_encryptions::{elgamal, elgamal::Randomness, AsymmetricEncryptionScheme};
    use crate::snark::prove::elgamal_prove;
    use crate::{circuits::encryption::{Instance, Witness, ElgamalCircuit}, snark::setup};
    

    type C = ark_ed_on_bn254::EdwardsProjective;
    type GG = ark_ed_on_bn254::constraints::EdwardsVar;
    type Enc = elgamal::ElGamal<C>;

    #[test]
    fn test_prove() {
        let rng = &mut ark_std::test_rng();
        
        let parameters = Enc::setup(rng).unwrap();
        let (public_key, _) = Enc::keygen(&parameters, rng).unwrap();
        let parameters2 = Enc::setup(rng).unwrap();
        let (public_key2, _) = Enc::keygen(&parameters2, rng).unwrap();
        // let _msg = C::rand(rng).into();
        let msg = C::rand(rng).into();
        let randomness = Randomness::rand(rng);
        let ct = Enc::encrypt(&parameters, &public_key, &msg, &randomness).unwrap();

        println!("public_key : {:?}", public_key);
        println!("public_key2 : {:?}", public_key2);

        let _instance =  Instance {
                g: parameters, 
                pk: Some(public_key), 
                ct: Some(ct), 
            };   


        let _witness =  Witness {
                m: Some(msg), 
                r: Some(randomness), 
            };

        let test_circuit = ElgamalCircuit {

            instance: _instance,

            witness: _witness,

            _curve_var: Default::default(),
        };

        let (pk, vk, pvk) = setup::elgamal_setup(test_circuit.clone());

        let verify_inputs = [
            public_key2.x,
            public_key2.y,
            ct.0.x,
            ct.0.y,
            ct.1.x,
            ct.1.y,
        ];

        let proof = elgamal_prove(pk, test_circuit);

        assert!(
            Groth16::<Bn254>::verify(&vk, &verify_inputs.clone(), &proof.clone()).unwrap()
        );
        
        assert!(
            Groth16::<Bn254>::verify_with_processed_vk(&pvk, &verify_inputs, &proof).unwrap(),
        )
    }
}
