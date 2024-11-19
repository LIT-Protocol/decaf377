use crate::fields::fr::N_8;
use crate::Fr;
use elliptic_curve::ff::{FieldBits, PrimeFieldBits};
use elliptic_curve::{ff, Field, PrimeField};
use rand_core::RngCore;
use subtle::{Choice, ConstantTimeEq, CtOption};

impl ConstantTimeEq for Fr {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.to_bytes_le().ct_eq(&other.to_bytes_le())
    }
}

impl PrimeFieldBits for Fr {
    type ReprBits = [u64; 4];

    fn to_le_bits(&self) -> FieldBits<Self::ReprBits> {
        FieldBits::new(self.to_le_limbs())
    }

    fn char_le_bits() -> FieldBits<Self::ReprBits> {
        FieldBits::new(Self::MODULUS_LIMBS)
    }
}

impl Field for Fr {
    const ZERO: Self = Fr::ZERO;
    const ONE: Self = Fr::ONE;

    fn random(mut rng: impl RngCore) -> Self {
        let bytes = {
            let mut out = [0u8; N_8 + 16];
            rng.fill_bytes(&mut out);
            out
        };
        Self::from_le_bytes_mod_order(&bytes)
    }

    fn square(&self) -> Self {
        <Self as ark_ff::Field>::square(self)
    }

    fn double(&self) -> Self {
        <Self as ark_ff::Field>::double(self)
    }

    fn invert(&self) -> CtOption<Self> {
        match self.inverse() {
            Some(value) => CtOption::new(value, Choice::from(1)),
            None => CtOption::new(Self::default(), Choice::from(0)),
        }
    }

    fn sqrt_ratio(num: &Self, div: &Self) -> (Choice, Self) {
        ff::helpers::sqrt_ratio_generic(num, div)
    }
}

impl PrimeField for Fr {
    type Repr = [u8; 32];

    fn from_repr(repr: Self::Repr) -> CtOption<Self> {
        match Self::from_bytes_checked(&repr) {
            Ok(value) => CtOption::new(value, Choice::from(1)),
            Err(_) => CtOption::new(Self::default(), Choice::from(0)),
        }
    }

    fn to_repr(&self) -> Self::Repr {
        self.to_bytes_le()
    }

    fn is_odd(&self) -> Choice {
        Choice::from(self.to_bytes_le()[0] & 1)
    }

    const MODULUS: &'static str =
        "0x4aad957a68b2955982d1347970dec005293a3afc43c8afeb95aee9ac33fd9ff";
    const NUM_BITS: u32 = Fr::MODULUS_BIT_SIZE;
    const CAPACITY: u32 = Fr::MODULUS_BIT_SIZE - 1;
    const TWO_INV: Self = Self::from_montgomery_limbs([
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0400000000000000,
    ]);
    const MULTIPLICATIVE_GENERATOR: Self = Self::MULTIPLICATIVE_GENERATOR;
    const S: u32 = Self::TWO_ADICITY;
    const ROOT_OF_UNITY: Self = Self::from_montgomery_limbs([
        0x72b5dd35867fb3fe,
        0xa527475f887915fd,
        0x305a268f2e1bd800,
        0x0155b2af4d1652ab,
    ]);
    const ROOT_OF_UNITY_INV: Self = Self::from_montgomery_limbs([
        0x72b5dd35867fb3fe,
        0xa527475f887915fd,
        0x305a268f2e1bd800,
        0x0155b2af4d1652ab,
    ]);
    const DELTA: Self = Self::from_montgomery_limbs([
        0xe784a3d2b24c5253,
        0x3a21ee03605eef69,
        0xa962bfca067c7be5,
        0x049b889500e1993f,
    ]);
}
