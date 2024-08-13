use super::{CurveFactory, EllipticCurve};
use k256::Secp256k1;

/// Elliptic curve based on Secp256k1
pub struct Secp256k1Curve;

impl EllipticCurve for Secp256k1Curve {
    fn get_name(&self) -> &str {
        "Secp256k1"
    }

    fn generate_point(&self) -> String {
        // Placeholder: In practice, this would generate a point on the curve
        "Generated point on Secp256k1".to_string()
    }
}

/// A factory for creating elliptic curves
pub struct EllipticCurveFactory;

impl CurveFactory for EllipticCurveFactory {
    fn create_curve(&self, curve_type: &str) -> Box<dyn EllipticCurve> {
        match curve_type {
            "Secp256k1" => Box::new(Secp256k1Curve),
            _ => panic!("Unknown curve type: {}", curve_type),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_secp256k1_curve() {
        let factory = EllipticCurveFactory;
        let curve = factory.create_curve("Secp256k1");

        assert_eq!(curve.get_name(), "Secp256k1");
        assert_eq!(curve.generate_point(), "Generated point on Secp256k1");
    }

    #[test]
    #[should_panic(expected = "Unknown curve type")]
    fn test_unknown_curve_type() {
        let factory = EllipticCurveFactory;
        factory.create_curve("UnknownCurve");
    }
}
