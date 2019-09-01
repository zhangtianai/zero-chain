use bellman::{
        groth16::{
            create_random_proof,
            verify_proof,
            Parameters,
            PreparedVerifyingKey,
            Proof,
        },
        SynthesisError,
};
use pairing::Field;
use rand::{Rand, Rng};
use scrypto::{
    jubjub::{
        JubjubEngine,
        FixedGenerators,
        edwards,
        PrimeOrder,
    },
    redjubjub::PublicKey,
};
use polkadot_rs::Api;
use zerochain_runtime::{UncheckedExtrinsic, Call, EncryptedBalancesCall, EncryptedAssetsCall};
use zprimitives::{
    EncKey as zEncKey,
    Ciphertext as zCiphertext,
    LeftCiphertext as zLeftCiphertext,
    RightCiphertext as zRightCiphertext,
    Nonce as zNonce,
    Proof as zProof
};
use crate::{
    circuit::AnonymousTransfer,
    elgamal::Ciphertext,
    EncryptionKey,
    ProofGenerationKey,
    SpendingKey,
    KeyContext,
    ProofBuilder,
    constants::*,
};
use crate::crypto_components::{
    MultiEncKeys,
    MultiCiphertexts,
    Confidential,
    CiphertextTrait,
    PrivacyConfing,
    Submitter,
    Calls,
};
use std::{
    io::{self, Write, BufWriter},
    path::Path,
    fs::File,
    marker::PhantomData,
};

impl<E: JubjubEngine> ProofBuilder<E> for KeyContext<E> {
    type Submitter = AnonymousXt;
    type PC = Anonymous;

    fn setup<R: Rng>(_rng: &mut R) -> Self {
        unimplemented!();
    }

    fn write_to_file<P: AsRef<Path>>(&self, pk_path: P, vk_path: P) -> io::Result<()> {
        let pk_file = File::create(&pk_path)?;
        let vk_file = File::create(&vk_path)?;

        let mut bw_pk = BufWriter::new(pk_file);
        let mut bw_vk = BufWriter::new(vk_file);

        let mut v_pk = vec![];
        let mut v_vk = vec![];

        self.proving_key.write(&mut &mut v_pk)?;
        self.prepared_vk.write(&mut &mut v_vk)?;

        bw_pk.write(&v_pk[..])?;
        bw_vk.write(&v_vk[..])?;

        bw_pk.flush()?;
        bw_vk.flush()?;

        Ok(())
    }

    fn read_from_path<P: AsRef<Path>>(pk_path: P, vk_path: P) -> io::Result<Self>{
        let pk_buf = Self::inner_read(pk_path)?;
        let vk_buf = Self::inner_read(vk_path)?;

        let pk = Parameters::read(&pk_buf[..], true)?;
        let vk = PreparedVerifyingKey::read(&vk_buf[..])?;

        Ok(KeyContext::new(pk, vk))
    }

    fn gen_proof<R: Rng>(
        &self,
        amount: u32,
        fee: u32,
        remaining_balance: u32,
        spending_key: &SpendingKey<E>,
        enc_keys: MultiEncKeys<E, Self::PC>,
        encrypted_balance: &Ciphertext<E>,
        g_epoch: edwards::Point<E, PrimeOrder>,
        rng: &mut R,
        params: &E::Params,
    ) -> Result<Self::Submitter, SynthesisError> {
        let randomness = E::Fs::rand(rng);
        let alpha = E::Fs::rand(rng);
        let s_index: usize = rng.gen_range(0, ANONIMITY_SIZE);
        let t_index: usize = rng.gen_range(0, ANONIMITY_SIZE);

        let pgk = ProofGenerationKey::<E>::from_spending_key(&spending_key, params);
        let dec_key = pgk.into_decryption_key()?;
        let enc_key_sender = pgk.into_encryption_key(params)?;

        let rvk = PublicKey(pgk.0.clone().into())
            .randomize(
                alpha,
                FixedGenerators::NoteCommitmentRandomness,
                params,
        );
        let nonce = g_epoch.mul(dec_key.0, params);

        let instance = AnonymousTransfer {
            params,
            amount: Some(amount),
            remaining_balance: Some(remaining_balance),
            s_index: Some(s_index),
            t_index: Some(t_index),
            randomness: Some(&randomness),
            alpha: Some(&alpha),
            proof_generation_key: Some(&pgk),
            dec_key: Some(&dec_key),
            enc_key_recipient: Some(&enc_keys.get_recipient()),
            enc_key_decoys: Some(&enc_keys.get_decoys()),
            enc_balances: Some(&encrypted_balance),
            g_epoch: Some(&g_epoch),
        };
        // Crate proof
        let proof = create_random_proof(instance, &self.proving_key, rng)?;
        

        unimplemented!();
    }
}


pub struct AnonymousXt {
    pub proof: [u8; PROOF_SIZE],
    // pub enc_keys: [u8]
}

impl Submitter for AnonymousXt {
    fn submit<R: Rng>(&self, calls: Calls, api: &Api, rng: &mut R) {
        unimplemented!();
    }
}
