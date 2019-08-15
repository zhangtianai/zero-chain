#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

#[cfg(not(feature = "std"))]
mod std {
    pub use ::core::*;
    pub use crate::alloc::vec;
    pub use crate::alloc::string;
    pub use crate::alloc::boxed;
    pub use crate::alloc::borrow;
}

#[cfg(feature = "std")]
#[macro_use]
extern crate serde_derive;

pub mod enc_key;
pub mod signature;
pub mod ciphertext;
pub mod proof;
pub mod sig_vk;
pub mod prepared_vk;
pub mod nonce;
pub mod g_epoch;
pub mod right_ciphertext;
pub mod left_ciphertext;

pub use self::enc_key::EncKey;
pub use self::signature::RedjubjubSignature;
pub use self::ciphertext::{Ciphertext, ElgamalCiphertext};
pub use self::proof::Proof;
pub use self::sig_vk::{SigVerificationKey, SigVk};
pub use self::prepared_vk::PreparedVk;
pub use self::nonce::Nonce;
pub use self::g_epoch::GEpoch;

use lazy_static::lazy_static;
use jubjub::curve::JubjubBls12;

lazy_static! {
    pub static ref PARAMS: JubjubBls12 = { JubjubBls12::new() };
}
