// We define in our crate:
use crate::Error;
use ed25519_dalek::PublicKey;
use ed25519_dalek::Signature;
use rsa::RSAPublicKey;
use std::borrow::Cow;
use std::convert::TryFrom;

pub use hex_buffer_serde::Hex;
// a single-purpose type for use in `#[serde(with)]`
pub enum EdPublicKeyHex {}

impl Hex<PublicKey> for EdPublicKeyHex {
    type Error = Error;

    fn create_bytes(public_key: &PublicKey) -> Cow<[u8]> {
        public_key.as_ref().into()
    }

    fn from_bytes(bytes: &[u8]) -> Result<PublicKey, Error> {
        Ok(PublicKey::from_bytes(bytes)?)
    }
}

// a single-purpose type for use in `#[serde(with)]`
pub enum EdSignatureHex {}

impl Hex<Signature> for EdSignatureHex {
    type Error = Error;

    fn create_bytes(sig: &Signature) -> Cow<[u8]> {
        let bytes = sig.to_bytes().to_vec();
        Cow::from(bytes)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Signature, Error> {
        Ok(Signature::try_from(bytes)?)
    }
}

// a single-purpose type for use in `#[serde(with)]`
pub enum RSAPublicKeyHex {}

impl Hex<RSAPublicKey> for RSAPublicKeyHex {
    type Error = Error;

    fn create_bytes(public_key: &RSAPublicKey) -> Cow<[u8]> {
        serde_cbor::to_vec(public_key).unwrap().into()
    }

    fn from_bytes(bytes: &[u8]) -> Result<RSAPublicKey, Error> {
        Ok(serde_cbor::from_slice(bytes)?)
    }
}
