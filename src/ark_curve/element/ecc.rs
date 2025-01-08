use crate::{Element, Fq, Fr};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::UniformRand;
use blake2::Blake2b512;
use elliptic_curve::{
    group::GroupEncoding,
    hash2curve::{ExpandMsg, ExpandMsgXmd, Expander},
    Group,
};
use rand_core::RngCore;
use subtle::{Choice, CtOption};

impl Group for Element {
    type Scalar = Fr;

    fn random(mut rng: impl RngCore) -> Self {
        Self::rand(&mut rng)
    }

    fn identity() -> Self {
        Self::IDENTITY
    }

    fn generator() -> Self {
        Self::GENERATOR
    }

    fn is_identity(&self) -> Choice {
        Choice::from(if self.is_identity() { 1u8 } else { 0u8 })
    }

    fn double(&self) -> Self {
        self + self
    }
}

impl GroupEncoding for Element {
    type Repr = [u8; 32];

    fn from_bytes(bytes: &Self::Repr) -> CtOption<Self> {
        match Element::deserialize_compressed(&bytes[..]) {
            Ok(e) => CtOption::new(e, Choice::from(1)),
            Err(_) => CtOption::new(Element::IDENTITY, Choice::from(0)),
        }
    }

    fn from_bytes_unchecked(bytes: &Self::Repr) -> CtOption<Self> {
        match Element::deserialize_compressed_unchecked(&bytes[..]) {
            Ok(e) => CtOption::new(e, Choice::from(1)),
            Err(_) => CtOption::new(Element::IDENTITY, Choice::from(0)),
        }
    }

    fn to_bytes(&self) -> Self::Repr {
        let mut bytes = [0u8; 32];
        self.serialize_compressed(&mut bytes[..])
            .expect("serialization to succeed");
        bytes
    }
}

impl gennaro_dkg::GroupHasher for Element {
    fn hash_to_curve(msg: &[u8]) -> Self {
        const DST: &'static [u8] = b"DECAF377_XMD:BLAKE2B-512_ELL_RO_";

        let mut expander = ExpandMsgXmd::<Blake2b512>::expand_message(&[msg], &[DST], 96)
            .expect("expander creation to succeed");
        let mut uniform_bytes = [0u8; 48];
        expander.fill_bytes(&mut uniform_bytes);
        let one = Fq::from_le_bytes_mod_order(&uniform_bytes);
        expander.fill_bytes(&mut uniform_bytes);
        let two = Fq::from_le_bytes_mod_order(&uniform_bytes);

        Element::hash_to_curve(&one, &two)
    }
}

impl frost_dkg::ScalarHash for Fr {
    fn hash_to_scalar(msg: &[u8]) -> Self {
        const DST: &'static [u8] = b"DECAF377_XMD:BLAKE2B-512_RO_NUL_";

        let mut expander = ExpandMsgXmd::<Blake2b512>::expand_message(&[msg], &[DST], 64)
            .expect("expander creation to succeed");
        let mut uniform_bytes = [0u8; 64];
        expander.fill_bytes(&mut uniform_bytes);
        Fr::from_le_bytes_mod_order(&uniform_bytes)
    }
}
