//! This module contains a circuit implementation for anonymous transfer.

use bellman::{
    SynthesisError,
    ConstraintSystem,
    Circuit,
};
use scrypto::jubjub::{
    JubjubEngine,
    FixedGenerators,
};
use crate::{ProofGenerationKey, EncryptionKey, DecryptionKey};
use scrypto::circuit::{
    boolean::{self, Boolean},
    ecc::{self, EdwardsPoint},
    num::AllocatedNum,
};
use scrypto::jubjub::{edwards, PrimeOrder};
use crate::{elgamal::Ciphertext, Assignment, Nonce};
use super::range_check::u32_into_bit_vec_le;
use super::anonimity_set::*;

const ANONIMITY_SIZE: usize = 11;

// Non-decoy-index is defined over 00 to 99 to represent which enc_keys are not decoys.
// First digit is for sender's index in the list of encryption keys and
// second one is for recipient's.
// The range is enough for representing non-decoys-index because
// the entire set of encryption keys are limited with 10.
pub struct AnonymousTransfer<'a, E: JubjubEngine> {
    params: &'a E::Params,
    amount: Option<u32>,
    remaining_balance: Option<u32>,
    randomness: Option<&'a E::Fs>,
    alpha: Option<&'a E::Fs>,
    proof_generation_key: Option<&'a ProofGenerationKey<E>>,
    dec_key_sender: Option<&'a DecryptionKey<E>>,
    enc_key_recipient: Option<&'a EncryptionKey<E>>,
    enc_key_decoys: &'a [Option<EncryptionKey<E>>],
    encrypted_balance: Option<&'a Ciphertext<E>>,
    fee: Option<u32>,
    g_epoch: Option<&'a edwards::Point<E, PrimeOrder>>,
    // non_decoy_index: Option<u32>,
}

impl<'a, E: JubjubEngine> Circuit<E> for AnonymousTransfer<'a, E> {
    fn synthesize<CS: ConstraintSystem<E>>(
        self,
        cs: &mut CS
    ) -> Result<(), SynthesisError>
    {
        let params = self.params;

        // Ensure the amount is u32.
        let amount_bits = u32_into_bit_vec_le(
            cs.namespace(|| "range proof of amount"),
            self.amount
        )?;

        // Ensure the remaining balance is u32.
        let remaining_balance_bits = u32_into_bit_vec_le(
            cs.namespace(|| "range proof of remaining_balance"),
            self.remaining_balance
        )?;

        let mut enc_key_set = EncKeySet::new(ANONIMITY_SIZE);

        enc_key_set
            .push_sender(
                cs.namespace(|| "push sender to enckey set"),
                self.dec_key_sender,
                params
            )?;

        enc_key_set
            .push_recipient(
                cs.namespace(|| "push recipient to enckey set"),
                self.enc_key_recipient,
                params
            )?;

        enc_key_set
            .push_decoys(
                cs.namespace(|| "push decoys to enckey set"),
                self.enc_key_decoys,
                params
            )?;


        Ok(())
    }
}






#[cfg(test)]
mod tests {
    use super::*;

}
