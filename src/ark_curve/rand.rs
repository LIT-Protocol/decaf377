use ark_ff::UniformRand;
use ark_serialize::CanonicalSerialize;
use ark_std::rand::{
    Rng,
    distributions::{Distribution, Standard},
};

use crate::ark_curve::{AffinePoint, Element, Encoding, edwards::EdwardsProjective};

impl Distribution<Element> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Element {
        loop {
            // 1. Generate random underlying elliptic curve point.
            let test_point = EdwardsProjective::rand(rng);
            let mut bytes = [0u8; 32];
            test_point
                .serialize_compressed(&mut bytes[..])
                .expect("serialization into array should be infallible");

            // 2. Check if valid decaf point. If not continue.
            if let Ok(p) = Encoding(bytes).vartime_decompress() {
                return p;
            }
        }
    }
}

impl Distribution<AffinePoint> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> AffinePoint {
        let point: Element = self.sample(rng);
        point.into()
    }
}
