// This module manages different strategies for various operations, 
// such as factorization or solving equations.
pub mod factorization_strategy;

// Trait that defines the Strategy interface
pub trait FactorizationStrategy {
    fn factorize(&self, n: u64) -> Option<u64>;
}
