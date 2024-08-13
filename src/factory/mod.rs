// This module manages the creation of objects using the Factory design pattern.
pub mod curve_factory;

// The trait that defines the factory interface
pub trait CurveFactory {
    fn create_curve(&self, curve_type: &str) -> Box<dyn EllipticCurve>;
}

// The trait that defines the interface for elliptic curves
pub trait EllipticCurve {
    fn get_name(&self) -> &str;
    fn generate_point(&self) -> String; // Placeholder for point generation logic
}
