use bellman::groth16::{
    create_random_proof, verify_proof, Parameters, PreparedVerifyingKey, Proof,
    prepare_verifying_key, generate_random_parameters,
};
use pairing::{
    bls12_381::{Bls12, Fr, FrRepr},
    Field, PrimeField, PrimeFieldRepr, Engine,
};
use rand::{OsRng, Rand};
use scrypto::{    
    jubjub::{edwards, fs::Fs, FixedGenerators, JubjubBls12, Unknown, PrimeOrder},    
    redjubjub::{PrivateKey, PublicKey, Signature},
};
use circuit_transfer::Transfer;
use primitives::{Diversifier, PaymentAddress, ProofGenerationKey, ValueCommitment};


pub struct TransferProof {
    proof: Proof<Bls12>,
    balance_value_commitment: ValueCommitment<Bls12>,
    transfer_value_commitment: ValueCommitment<Bls12>,
    rk: PublicKey<Bls12>, // rk, re-randomization sig-verifying key
    epk: edwards::Point<Bls12, PrimeOrder>,
}

impl TransferProof {    
    pub fn gen_proof(
        &mut self, 
        transfer_value: u64,         
        balance_value: u64,        
        ar: Fs,
        esk: Fs, 
        proving_key: &Parameters<Bls12>, 
        verifying_key: &PreparedVerifyingKey<Bls12>,
        proof_generation_key: ProofGenerationKey<Bls12>,
        recipient_payment_address: PaymentAddress<Bls12>,
        diversifier: Diversifier,
        params: &JubjubBls12,        
    ) -> Result<Self, ()>
    {
        // TODO: Change OsRng for wasm
        let mut rng = OsRng::new().expect("should be able to construct RNG");        

        let transfer_rcm = Fs::rand(&mut rng);
        let balance_rcm = Fs::rand(&mut rng);

        let transfer_value_commitment = ValueCommitment::<Bls12> {
            value: transfer_value,
            randomness: transfer_rcm,
        };

        let balance_value_commitment = ValueCommitment::<Bls12> {
            value: balance_value,
            randomness: balance_rcm,
        };

        let viewing_key = proof_generation_key.into_viewing_key(params);

        let prover_payment_address = match viewing_key.into_payment_address(diversifier, params) {
            Some(p) => p,
            None => return Err(()),
        };

        let rk = PublicKey::<Bls12>(proof_generation_key.ak.clone().into())
            .randomize(
                ar,
                FixedGenerators::SpendingKeyGenerator,
                params,
        );

        let epk = recipient_payment_address
            .g_d(params)
            .expect("should be valid")
            .mul(esk, params);

        let instance = Transfer {
            params: params,     
            transfer_value_commitment: Some(transfer_value_commitment.clone()),
            balance_value_commitment: Some(balance_value_commitment.clone()),            
            ar: Some(ar),
            proof_generation_key: Some(proof_generation_key), 
            esk: Some(esk),
            prover_payment_address: Some(prover_payment_address),
            recipient_payment_address: Some(recipient_payment_address),
        };

        // Crate proof
        let proof = create_random_proof(instance, proving_key, &mut rng)
            .expect("proving should not fail");
        
        let mut public_input = [Fr::zero(); 8];
        {
            let (x, y) = (&balance_value_commitment).cm(params).into_xy();
            public_input[0] = x;
            public_input[1] = y;
        }
        {
            let (x, y) = (&transfer_value_commitment).cm(params).into_xy();
            public_input[2] = x;
            public_input[3] = y;
        }
        {
            let (x, y) = epk.into_xy();
            public_input[4] = x;
            public_input[5] = y;
        }
        {
            let (x, y) = rk.0.into_xy();
            public_input[6] = x;
            public_input[7] = y;
        }

        match verify_proof(verifying_key, &proof, &public_input[..]) {
            Ok(true) => {},
            _ => {
                return Err(());
            }
        }

        let transfer_proof = TransferProof {
            proof: proof,
            balance_value_commitment: balance_value_commitment,
            transfer_value_commitment: transfer_value_commitment,
            rk: rk, 
            epk: epk,
        };

        Ok(transfer_proof)
    }    
}