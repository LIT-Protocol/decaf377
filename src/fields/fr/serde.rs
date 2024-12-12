use super::Fr;
use alloc::string::ToString;
use serdect::serde::{Deserialize, Deserializer, Serialize, Serializer};

impl Serialize for Fr {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = self.to_bytes_le();
        serdect::array::serialize_hex_lower_or_bin(&bytes, s)
    }
}

impl<'de> Deserialize<'de> for Fr {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut bytes = [0u8; 32];
        let _ = serdect::array::deserialize_hex_or_bin::<D>(&mut bytes, d)?;
        Fr::from_bytes_checked(&bytes).map_err(|e| serdect::serde::de::Error::custom(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_chacha::ChaCha8Rng;
    use rand_core::SeedableRng;

    #[test]
    fn text() {
        let mut rng = ChaCha8Rng::from_entropy();
        let val = Fr::rand(&mut rng);
        let serialized = serde_json::to_string(&val).unwrap();
        let deserialized: Fr = serde_json::from_str(&serialized).unwrap();
        assert_eq!(val, deserialized);
    }

    #[test]
    fn binary() {
        let mut rng = ChaCha8Rng::from_entropy();
        let val = Fr::rand(&mut rng);
        let serialized = serde_bare::to_vec(&val).unwrap();
        let deserialized: Fr = serde_bare::from_slice(&serialized).unwrap();
        assert_eq!(val, deserialized);
    }
}
