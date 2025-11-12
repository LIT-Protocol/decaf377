use crate::{
    Fq, Fr,
    ark_curve::{Decaf377EdwardsConfig, EdwardsProjective, edwards::EdwardsAffine},
};
use ark_ec::{AffineRepr, CurveGroup, Group, ScalarMul, VariableBaseMSM};
use ark_serialize::Valid;
use ark_std::vec::Vec;

pub mod affine;
#[cfg(feature = "ecc-group")]
mod ecc;
pub mod projective;

pub use affine::AffinePoint;
pub use projective::Element;

impl Valid for Element {
    fn check(&self) -> Result<(), ark_serialize::SerializationError> {
        Ok(())
    }
}

impl ScalarMul for Element {
    type MulBase = AffinePoint;

    const NEGATION_IS_CHEAP: bool = true;

    fn batch_convert_to_mul_base(bases: &[Self]) -> Vec<Self::MulBase> {
        let bases_inner = bases.iter().map(|g| g.inner).collect::<Vec<_>>();
        let result = EdwardsProjective::batch_convert_to_mul_base(&bases_inner[..]);
        result
            .into_iter()
            .map(|g| AffinePoint { inner: g })
            .collect::<Vec<_>>()
    }
}

impl VariableBaseMSM for Element {}

impl Group for Element {
    type ScalarField = Fr;

    fn double_in_place(&mut self) -> &mut Self {
        let inner = *self.inner.double_in_place();
        *self = Element { inner };
        self
    }

    fn generator() -> Self {
        Self::GENERATOR
    }

    fn mul_bigint(&self, other: impl AsRef<[u64]>) -> Self {
        let inner = self.inner.mul_bigint(other);
        Element { inner }
    }
}

impl CurveGroup for Element {
    // We implement `CurveGroup` as it is required by the `CurveVar`
    // trait used in the R1CS feature. The `ProjectiveCurve` trait requires
    // an affine representation of `Element` to be defined, and `AffineRepr`
    // to be implemented on that type.
    type Config = Decaf377EdwardsConfig;

    type BaseField = Fq;

    type Affine = AffinePoint;

    // This type is supposed to represent an element of the entire elliptic
    // curve group, not just the prime-order subgroup. Since this is decaf,
    // this is just an `Element` again.
    type FullGroup = AffinePoint;

    fn normalize_batch(v: &[Self]) -> Vec<AffinePoint> {
        let v_inner = v.iter().map(|g| g.inner).collect::<Vec<_>>();
        let result = EdwardsProjective::normalize_batch(&v_inner[..]);
        result
            .into_iter()
            .map(|g| AffinePoint { inner: g })
            .collect::<Vec<_>>()
    }

    fn into_affine(self) -> Self::Affine {
        self.into()
    }
}

impl Valid for AffinePoint {
    fn check(&self) -> Result<(), ark_serialize::SerializationError> {
        Ok(())
    }
}

impl AffineRepr for AffinePoint {
    type Config = Decaf377EdwardsConfig;

    type ScalarField = Fr;

    type BaseField = Fq;

    type Group = Element;

    fn xy(&self) -> Option<(&Self::BaseField, &Self::BaseField)> {
        self.inner.xy()
    }

    fn zero() -> Self {
        AffinePoint {
            inner: EdwardsAffine::zero(),
        }
    }

    fn generator() -> Self {
        Element::GENERATOR.into()
    }

    fn from_random_bytes(bytes: &[u8]) -> Option<Self> {
        EdwardsAffine::from_random_bytes(bytes).map(|inner| AffinePoint { inner })
    }

    fn mul_bigint(&self, other: impl AsRef<[u64]>) -> Self::Group {
        Element {
            inner: self.inner.mul_bigint(other),
        }
    }

    fn clear_cofactor(&self) -> Self {
        // This is decaf so we're just returning the same point.
        *self
    }

    fn mul_by_cofactor_to_group(&self) -> Self::Group {
        self.into()
    }
}

impl From<Element> for AffinePoint {
    fn from(point: Element) -> Self {
        Self {
            inner: point.inner.into(),
        }
    }
}

impl From<AffinePoint> for Element {
    fn from(point: AffinePoint) -> Self {
        Self {
            inner: point.inner.into(),
        }
    }
}

impl From<&Element> for AffinePoint {
    fn from(point: &Element) -> Self {
        Self {
            inner: point.inner.into(),
        }
    }
}

impl From<&AffinePoint> for Element {
    fn from(point: &AffinePoint) -> Self {
        Self {
            inner: point.inner.into(),
        }
    }
}

#[cfg(feature = "serde")]
impl serdect::serde::Serialize for Element {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serdect::serde::Serializer,
    {
        use elliptic_curve::group::GroupEncoding;
        let bytes = self.to_bytes();
        serdect::array::serialize_hex_lower_or_bin(&bytes, s)
    }
}

#[cfg(feature = "serde")]
impl<'de> serdect::serde::Deserialize<'de> for Element {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serdect::serde::Deserializer<'de>,
    {
        use elliptic_curve::group::GroupEncoding;
        let mut bytes = [0u8; 32];
        let _ = serdect::array::deserialize_hex_or_bin::<D>(&mut bytes, d)?;
        Option::from(Element::from_bytes(&bytes))
            .ok_or(serdect::serde::de::Error::custom("Invalid encoding"))
    }
}
